use crate::mem_requests::{ct_classify, ct_declassify};
/// This file defines functions for classifying and declassifying various types.
/// We give definitions for all conversions so that they can be tested
/// However, this file is only meant to be used when using feature "check-secret-independence"
/// That is, it should not be used when running the Rust code in production.
/// Otherwise, the crate defaults to public integers.
use crate::{secret, traits::*};
use core::ptr;

/// A type for secret values
#[repr(transparent)]
pub struct Secret<T>(pub(crate) T);

// Secrets are clonable if the underlying type is
impl<T: Clone> Clone for Secret<T> {
    fn clone(&self) -> Self {
        Secret(self.0.clone())
    }
}

// Any scalar type can be classified
impl<T: Scalar> From<T> for Secret<T> {
    fn from(x: T) -> Secret<T> {
        ct_classify(&x);
        Secret(x)
    }
}

// Secrets are copyable if the underlying type is
impl<T: Clone + Copy> Copy for Secret<T> {}

// IMPORTANT NOTE: These impls must be kept in sync with those in `classify_public.rs`.
// Not keeping these synchronized can lead to confusing method resolution errors and differences
// when the check-secret-independence feature is enabled or not.

// Classify a scalar
impl<T: Scalar> Classify for T {
    type Classified = Secret<T>;
    fn classify(self) -> Secret<Self> {
        secret(self)
    }
}

// Declassify a scalar
impl<T: Scalar> Declassify for Secret<T> {
    type Declassified = T;
    fn declassify(self) -> T {
        ct_declassify(&self);
        self.0
    }
}

// Classify a reference to a scalar
impl<'a, T: Scalar> ClassifyRef for &'a T {
    type ClassifiedRef = &'a Secret<T>;
    fn classify_ref(self) -> &'a Secret<T> {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret values is the same
        unsafe { &*ptr::from_ref(self).cast::<Secret<T>>() }
    }
}

// Declassify a reference to a scalar
impl<'a, T: Scalar> DeclassifyRef for &'a Secret<T> {
    type DeclassifiedRef = &'a T;
    fn declassify_ref(self) -> &'a T {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret values is the same
        unsafe { &*ptr::from_ref(self).cast::<T>() }
    }
}

// Arrays of scalars can be classified
impl<T: Scalar, const N: usize> Classify for [T; N] {
    type Classified = [Secret<T>; N];
    fn classify(self) -> [Secret<T>; N] {
        self.map(|x| x.into())
    }
}

// Arrays of scalars can be declassified
impl<T: Scalar, const N: usize> Declassify for [Secret<T>; N] {
    type Declassified = [T; N];
    fn declassify(self) -> [T; N] {
        ct_declassify(&self);
        self.map(|x| x.0)
    }
}

// Matrices of scalars can be classified
impl<T: Scalar, const M: usize, const N: usize> Classify for [[T; N]; M] {
    type Classified = [[Secret<T>; N]; M];
    fn classify(self) -> [[Secret<T>; N]; M] {
        self.map(|x| x.map(|y| y.into()))
    }
}

// Matrices of scalars can be declassified
impl<T: Scalar, const N: usize, const M: usize> Declassify for [[Secret<T>; N]; M] {
    type Declassified = [[T; N]; M];
    fn declassify(self) -> [[T; N]; M] {
        ct_declassify(&self);
        self.map(|x| x.map(|y| y.0))
    }
}

// Mutable references to scalars can be classified
impl<'a, T: Scalar> ClassifyRefMut for &'a mut T {
    type ClassifiedRefMut = &'a mut Secret<T>;
    fn classify_ref_mut(self) -> &'a mut Secret<T> {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret values is the same
        unsafe { &mut *ptr::from_mut(self).cast::<Secret<T>>() }
    }
}

// Mutable references to scalars can be declassified
impl<'a, T: Scalar> DeclassifyRefMut for &'a mut Secret<T> {
    type DeclassifiedRefMut = &'a mut T;
    fn declassify_ref_mut(self) -> &'a mut T {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret values is the same
        unsafe { &mut *ptr::from_mut(self).cast::<T>() }
    }
}

// Immutable references to slices can be classified
impl<'a, T: Scalar> ClassifyRef for &'a [T] {
    type ClassifiedRef = &'a [Secret<T>];
    fn classify_ref(self) -> &'a [Secret<T>] {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret slices is the same
        unsafe { core::slice::from_raw_parts(self.as_ptr().cast::<Secret<T>>(), self.len()) }
    }
}

// Immutable references to slices can be declassified
impl<'a, T: Scalar> DeclassifyRef for &'a [Secret<T>] {
    type DeclassifiedRef = &'a [T];
    fn declassify_ref(self) -> &'a [T] {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret slices is the same
        unsafe { core::slice::from_raw_parts(self.as_ptr().cast::<T>(), self.len()) }
    }
}

// Mutable references to slices can be classified
impl<'a, T: Scalar> ClassifyRefMut for &'a mut [T] {
    type ClassifiedRefMut = &'a mut [Secret<T>];
    fn classify_ref_mut(self) -> &'a mut [Secret<T>] {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret slices is the same
        unsafe {
            core::slice::from_raw_parts_mut(self.as_mut_ptr().cast::<Secret<T>>(), self.len())
        }
    }
}

// Mutable references to slices can be declassified
impl<'a, T: Scalar> DeclassifyRefMut for &'a mut [Secret<T>] {
    type DeclassifiedRefMut = &'a mut [T];
    fn declassify_ref_mut(self) -> &'a mut [T] {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret slices is the same
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr().cast::<T>(), self.len()) }
    }
}

// Immutable references to arrays can be classified
impl<'a, T: Scalar, const N: usize> ClassifyRef for &'a [T; N] {
    type ClassifiedRef = &'a [Secret<T>; N];
    fn classify_ref(self) -> &'a [Secret<T>; N] {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret arrays is the same
        unsafe { &*self.as_ptr().cast::<[Secret<T>; N]>() }
    }
}

// Immutable references to arrays can be classified
impl<'a, T: Scalar, const N: usize> DeclassifyRef for &'a [Secret<T>; N] {
    type DeclassifiedRef = &'a [T; N];
    fn declassify_ref(self) -> &'a [T; N] {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret arrays is the same
        unsafe { &*self.as_ptr().cast::<[T; N]>() }
    }
}

// Mutable references to arrays can be classified
impl<'a, T: Scalar, const N: usize> ClassifyRefMut for &'a mut [T; N] {
    type ClassifiedRefMut = &'a mut [Secret<T>; N];
    fn classify_ref_mut(self) -> &'a mut [Secret<T>; N] {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret arrays is the same
        unsafe { &mut *self.as_mut_ptr().cast::<[Secret<T>; N]>() }
    }
}

// Mutable references to arrays can be declassified
impl<'a, T: Scalar, const N: usize> DeclassifyRefMut for &'a mut [Secret<T>; N] {
    type DeclassifiedRefMut = &'a mut [T; N];
    fn declassify_ref_mut(self) -> &'a mut [T; N] {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret arrays is the same
        unsafe { &mut *self.as_mut_ptr().cast::<[T; N]>() }
    }
}

/// Classify a mutable reference to a slice
pub fn classify_mut_slice<T: Scalar>(x: &mut [T]) -> &mut [Secret<T>] {
    ct_classify(x);
    // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
    //       the memory representation of the public and secret slices is the same
    unsafe { core::slice::from_raw_parts_mut(x.as_mut_ptr().cast::<Secret<T>>(), x.len()) }
}

/// Declassify a mutable reference to a slice
pub fn declassify_mut_slice<T: Scalar>(x: &mut [Secret<T>]) -> &mut [T] {
    ct_declassify(x);
    // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
    //       the memory representation of the public and secret slices is the same
    unsafe { core::slice::from_raw_parts_mut(x.as_mut_ptr().cast::<T>(), x.len()) }
}
