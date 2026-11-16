#[cfg(feature = "simd256")]
pub(crate) mod avx2;

pub(crate) mod portable;
pub(crate) mod traits;

#[cfg(test)]
pub(crate) mod tests;
