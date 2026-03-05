//! This module defines functions for classifying and declassifying various types.
//! We give definitions for all conversions so that they can be tested.
//! We define no-ops here and force inlining, to ensure that these are free.

use crate::traits::*;

// TODO: Remove hax exemptions once this is supported.
//       See https://github.com/cryspen/hax/issues/1674.

// Immutable references to slices can be classified
impl<'a, T: Scalar> ClassifyRef for &'a [T] {
    type ClassifiedRef = &'a [T];
    #[inline(always)]
    fn classify_ref(self) -> &'a [T] {
        self
    }
}

// Immutable references to slices can be declassified
impl<'a, T: Scalar> DeclassifyRef for &'a [T] {
    type DeclassifiedRef = &'a [T];
    #[inline(always)]
    fn declassify_ref(self) -> &'a [T] {
        self
    }
}

// Classify any mutable reference (identity)
#[hax_lib::exclude]
impl<'a, T> ClassifyRefMut for &'a mut T {
    type ClassifiedRefMut = &'a mut T;
    #[inline(always)]
    fn classify_ref_mut(self) -> &'a mut T {
        self
    }
}

// Declassify any mutable reference (identity)
#[hax_lib::exclude]
impl<'a, T> DeclassifyRefMut for &'a mut T {
    type DeclassifiedRefMut = &'a mut T;
    #[inline(always)]
    fn declassify_ref_mut(self) -> &'a mut T {
        self
    }
}
