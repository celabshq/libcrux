//! This module provides tools for annotating functions such that calls are traced.
//! That means that whenever the function is called and when it returns, an entry
//! is written to the specified trace.
//!
//! Usually, that trace will be a static variable with interior mutability. If in
//! doubt, define it something like this in the library/application code:
//!
//! In this example we use `cfg` and `cfg_attr` to ensure that the tracing only runs
//! during testing. In many cases one might want to also restrict this to only trace
//! if a certain feature is enabled. This can be done using the same mechanism.
//!
//! ```rust
//! # #[cfg(test)]
//! # use std::{
//! #  sync::LazyLock,
//! #  time::Instant,
//! # };
//! # #[cfg(test)]
//! # use libcrux_test_utils::tracing::MutexTrace;
//! #
//! #[cfg(test)]
//! static TRACE: LazyLock<MutexTrace<&'static str, Instant>> =
//!     LazyLock::new(|| MutexTrace::default());
//! ```
//!
//! The `MutexTrace` can be defined and used as a static variable without unsafe, but,
//! depending on the setting, my introduce too much overhead. Any type that implements
//! `Trace` works.
//!
//! Then, annotate a function like this:
//!
//! ```rust
//! # #[cfg(test)]
//! # use std::{
//! #  sync::LazyLock,
//! #  time::Instant,
//! # };
//! # #[cfg(test)]
//! # use libcrux_test_utils::tracing::{MutexTrace, trace_span};
//! # #[cfg(test)]
//! # static TRACE: LazyLock<MutexTrace<&'static str, Instant>> =
//! #     LazyLock::new(|| MutexTrace::default());
//! #
//! // trace this function into `TRACE` if we are running the tests
//! #[cfg_attr(test, trace_span("my_app_fun", TRACE))]
//! fn my_app_function() {
//!   // ... some long-running code ...
//! }
//! ```
//!
//! The macro is called `trace_span` because it traces a start and end,
//! identified by a label. The type of the label can be chosen generically, here
//! it is `&'static str`. There also are on-the-fly tracing facilities.
//!
//! After the code in question ran, the trace can be inspected. Due to the use
//! of  interior mutability, there is no generic way to get a reference to the
//! inner slice; therefore, it is easiest to just clone it. We expect that this
//! already happens in test code, so we don't add the `#[cfg(test)]` here.
//!
//! ```rust
//! # use std::{
//! #  sync::LazyLock,
//! #  time::Instant,
//! # };
//! # use libcrux_test_utils::tracing::MutexTrace;
//! # static TRACE: LazyLock<MutexTrace<&'static str, Instant>> =
//! #     LazyLock::new(|| MutexTrace::default());
//! #
//! // make sure the trait is in scope
//! use libcrux_test_utils::tracing::Trace as _;
//!
//! println!("{:?}", TRACE.clone().report());
//! ```

use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    fmt::Display,
    sync::Mutex,
};

pub use libcrux_macros::trace_span;

/// This trait describes a trace that is behind some sort of interior mutability mechanism. It can
/// log trace events and later make these available. This is usually an argument to the
/// `trace_span` function attribute macro in `libcrux-macros``, but it can also be called manually.
///
/// When used with the `trace_span` macros, this needs to be a global static. For defining and
/// instantiating this, take a look at [`std::sync::LazyLock`].
pub trait Trace: Sized {
    /// The label type used in events. Typically either `&'static str` or an enum.
    type Label: Clone;

    /// The type used for timing the events. Typically `std::time::Instant` or a cycle counter.
    type TimeStamp: TimeStamp;

    /// Writes an event to the trace.
    fn emit(&self, ev: TraceEvent<Self::Label, Self::TimeStamp>);

    /// Returns a vector of all entries in the trace.
    fn report(self) -> Vec<TraceEvent<Self::Label, Self::TimeStamp>>;

    /// Emits an event of type [EventType::SpanOpen] and returns a [`SpanHandle`] that emits another
    /// [`EventType::SpanClose`] when dropped.
    fn emit_span(&self, label: Self::Label) -> SpanHandle<'_, Self> {
        let event = TraceEvent {
            ty: EventType::SpanOpen,
            at: Self::TimeStamp::now(),
            label: label.clone(),
        };

        self.emit(event);
        SpanHandle {
            trace: self,
            label: Some(label),
        }
    }

    /// Emits an [`EventType::OnTheFly`] event.
    fn emit_on_the_fly(&self, label: Self::Label) {
        let event = TraceEvent {
            ty: EventType::OnTheFly,
            at: Self::TimeStamp::now(),
            label: label.clone(),
        };

        self.emit(event);
    }
}

/// Describes types that can be used for timestamping. This allows using the tracing facilities in
/// no_std environments, where [`std::time::Instant`] is not available.
pub trait TimeStamp {
    /// Returns the current time
    fn now() -> Self;
}

/// The type of the event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    SpanOpen,
    SpanClose,
    OnTheFly,
}

/// A trace event.
#[derive(Debug, Clone)]
pub struct TraceEvent<Label, TimeStamp> {
    pub ty: EventType,
    pub at: TimeStamp,
    pub label: Label,
}

impl<Label: Display, TimeStamp: Display> Display for TraceEvent<Label, TimeStamp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { ty, at, label } = self;
        write!(f, "{ty:?}: {label}@{at}")
    }
}

/// This type emits a [`EventType::SpanClose`] event on drop.
pub struct SpanHandle<'a, T: Trace> {
    trace: &'a T,

    // NOTE: We need an owned version of this value in `Drop::drop`. `Drop::drop` has an &mut self
    //       receiver, so we can't just move the value out of this struct.
    //       This is an option so we can call `Option::take` to move the value out.
    label: Option<T::Label>,
}

impl<'a, T: Trace> Drop for SpanHandle<'a, T> {
    fn drop(&mut self) {
        let event = TraceEvent {
            ty: EventType::SpanClose,
            at: T::TimeStamp::now(),
            label: self.label.take().unwrap(),
        };
        self.trace.emit(event)
    }
}

/// An implementation of [`Trace`] using a [`Mutex`] for interior mutability.
#[derive(Debug)]
pub struct MutexTrace<Label, TimeStamp>(Mutex<Vec<TraceEvent<Label, TimeStamp>>>);

impl<Label: Clone, TS: TimeStamp> Trace for MutexTrace<Label, TS> {
    type Label = Label;
    type TimeStamp = TS;

    fn emit(&self, ev: TraceEvent<Self::Label, Self::TimeStamp>) {
        self.0.lock().unwrap().push(ev)
    }

    fn report(self) -> Vec<TraceEvent<Self::Label, Self::TimeStamp>> {
        self.0.into_inner().unwrap()
    }
}

// This doesn't work yet because the method is still unstable;
// see issue #117108 <https://github.com/rust-lang/rust/issues/117108>
//
// impl<Label: Clone, TS: TimeStamp> MutexTrace<Label, TS> {
//     pub fn entries<'a>(&'a self) -> MappedMutexGuard<&'a [TraceEvent<Label, TS>]> {
//         std::sync::MutexGuard::map(self.0.lock().unwrap(), |x| Vec::as_mut_slice(x))
//     }
// }

impl<Label: Clone, TS: TimeStamp> Default for MutexTrace<Label, TS> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Label: Clone, TS: TimeStamp + Clone> Clone for MutexTrace<Label, TS> {
    fn clone(&self) -> Self {
        let guard = self.0.lock().unwrap();
        let vector: &Vec<_> = guard.borrow();
        Self(Mutex::new(vector.clone()))
    }
}

/// An implementation of [`Trace`] using a [`RefCell`] for interior mutability. Note that it is
/// unsafe to use [`RefCell`] in static variables.
#[derive(Debug)]
pub struct RefCellTrace<Label, TimeStamp>(RefCell<Vec<TraceEvent<Label, TimeStamp>>>);

impl<Label: Clone, TS: TimeStamp> Trace for RefCellTrace<Label, TS> {
    type Label = Label;
    type TimeStamp = TS;

    fn emit(&self, ev: TraceEvent<Self::Label, Self::TimeStamp>) {
        self.0.borrow_mut().push(ev);
    }

    fn report(self) -> Vec<TraceEvent<Self::Label, Self::TimeStamp>> {
        self.0.into_inner()
    }
}

impl<Label: Clone, TS: TimeStamp> RefCellTrace<Label, TS> {
    pub fn entries(&self) -> Ref<'_, [TraceEvent<Label, TS>]> {
        Ref::map(self.0.borrow(), Vec::as_slice)
    }
}

impl<Label: Clone, TS: TimeStamp> Default for RefCellTrace<Label, TS> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Label: Clone, TS: TimeStamp + Clone> Clone for RefCellTrace<Label, TS> {
    fn clone(&self) -> Self {
        let vector = self.0.borrow().as_slice().to_vec();
        Self(RefCell::new(vector))
    }
}

impl TimeStamp for std::time::Instant {
    fn now() -> Self {
        Self::now()
    }
}
