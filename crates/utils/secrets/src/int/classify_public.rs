//! This module defines functions for classifying and declassifying various types.
//! We give definitions for all conversions so that they can be tested.
//! We define no-ops here and force inlining, to ensure that these are free.

use crate::traits::*;

// TODO: Remove hax exemptions once this is supported.
//       See https://github.com/cryspen/hax/issues/1674.

// IMPORTANT NOTE: These impls must be kept in sync with those in `classify_secret.rs`.
// Not keeping these synchronized can lead to confusing method resolution errors and differences
// when the check-secret-independence feature is enabled or not.
// These impls reflect the secret ones, where every `Secret<T>` has been replaced by `T`, the functions
// just return self, and are annotated with #[inline(always)] to make sure they're optimized out.

// Classify a scalar
impl<T: Scalar> Classify for T {
    type Classified = T;

    #[inline(always)]
    fn classify(self) -> T {
        self
    }
}

// Declassify a scalar
impl<T: Scalar> Declassify for T {
    type Declassified = T;

    #[inline(always)]
    fn declassify(self) -> T {
        self
    }
}

// Classify a reference to a scalar
impl<'a, T: Scalar> ClassifyRef for &'a T {
    type ClassifiedRef = &'a T;

    #[inline(always)]
    fn classify_ref(self) -> &'a T {
        self
    }
}

// Declassify a reference to a scalar
impl<'a, T: Scalar> DeclassifyRef for &'a T {
    type DeclassifiedRef = &'a T;

    #[inline(always)]
    fn declassify_ref(self) -> &'a T {
        self
    }
}

// Arrays of scalars can be classified
impl<T: Scalar, const N: usize> Classify for [T; N] {
    type Classified = [T; N];

    #[inline(always)]
    fn classify(self) -> [T; N] {
        self
    }
}

// Arrays of scalars can be declassified
impl<T: Scalar, const N: usize> Declassify for [T; N] {
    type Declassified = [T; N];

    #[inline(always)]
    fn declassify(self) -> [T; N] {
        self
    }
}

// Matrices of scalars can be classified
impl<T: Scalar, const M: usize, const N: usize> Classify for [[T; N]; M] {
    type Classified = [[T; N]; M];

    #[inline(always)]
    fn classify(self) -> [[T; N]; M] {
        self
    }
}

// Matrices of scalars can be declassified
impl<T: Scalar, const N: usize, const M: usize> Declassify for [[T; N]; M] {
    type Declassified = [[T; N]; M];

    #[inline(always)]
    fn declassify(self) -> [[T; N]; M] {
        self
    }
}

// Mutable references to scalars can be classified
#[hax_lib::exclude]
impl<'a, T: Scalar> ClassifyRefMut for &'a mut T {
    type ClassifiedRefMut = &'a mut T;

    #[inline(always)]
    fn classify_ref_mut(self) -> &'a mut T {
        self
    }
}

// Mutable references to scalars can be declassified
#[hax_lib::exclude]
impl<'a, T: Scalar> DeclassifyRefMut for &'a mut T {
    type DeclassifiedRefMut = &'a mut T;

    #[inline(always)]
    fn declassify_ref_mut(self) -> &'a mut T {
        self
    }
}

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

// Mutable references to slices can be classified
#[hax_lib::exclude]
impl<'a, T: Scalar> ClassifyRefMut for &'a mut [T] {
    type ClassifiedRefMut = &'a mut [T];

    #[inline(always)]
    fn classify_ref_mut(self) -> &'a mut [T] {
        self
    }
}

// Mutable references to slices can be declassified
#[hax_lib::exclude]
impl<'a, T: Scalar> DeclassifyRefMut for &'a mut [T] {
    type DeclassifiedRefMut = &'a mut [T];

    #[inline(always)]
    fn declassify_ref_mut(self) -> &'a mut [T] {
        self
    }
}

// Immutable references to arrays can be classified
impl<'a, T: Scalar, const N: usize> ClassifyRef for &'a [T; N] {
    type ClassifiedRef = &'a [T; N];

    #[inline(always)]
    fn classify_ref(self) -> &'a [T; N] {
        self
    }
}

// Immutable references to arrays can be classified
impl<'a, T: Scalar, const N: usize> DeclassifyRef for &'a [T; N] {
    type DeclassifiedRef = &'a [T; N];

    #[inline(always)]
    fn declassify_ref(self) -> &'a [T; N] {
        self
    }
}

// Mutable references to arrays can be classified
#[hax_lib::exclude]
impl<'a, T: Scalar, const N: usize> ClassifyRefMut for &'a mut [T; N] {
    type ClassifiedRefMut = &'a mut [T; N];

    #[inline(always)]
    fn classify_ref_mut(self) -> &'a mut [T; N] {
        self
    }
}

// Mutable references to arrays can be declassified
#[hax_lib::exclude]
impl<'a, T: Scalar, const N: usize> DeclassifyRefMut for &'a mut [T; N] {
    type DeclassifiedRefMut = &'a mut [T; N];

    #[inline(always)]
    fn declassify_ref_mut(self) -> &'a mut [T; N] {
        self
    }
}

/// Classify a mutable reference to a slice
/// We define a separate function for this because hax has limited support for &mut-returning functions
///
/// Note that this function has a different signature than the corresponding `check-secret-independence` one.
/// Every call to the secret version of this function compiles with this one, but the reverse is not true.
#[inline(always)]
pub fn classify_mut_slice<T>(x: T) -> T {
    x
}

/// Declassify a mutable reference to a slice
/// We define a separate function for this because hax has limited support for &mut-returning functions
///
/// Note that this function has a different signature than the corresponding `check-secret-independence` one.
/// Every call to the secret version of this function compiles with this one, but the reverse is not true.
#[inline(always)]
pub fn declassify_mut_slice<T>(x: T) -> T {
    x
}
