macro_rules! impl_hmac {
    ($name:ident,$fun:path,$tag_len:literal) => {
        /// Compute HMAC.
        ///
        /// Note that this function panics if `key` or `data` is larger than 2**32 bytes.
        /// This ensures that all values are in the range valid to be consumed by hacl-rs.
        #[inline(always)]
        pub fn $name(dst: &mut [u8; $tag_len], key: &[u8], data: &[u8]) {
            $fun(
                dst,
                key,
                key.len().try_into().unwrap(),
                data,
                data.len().try_into().unwrap(),
            )
        }
    };
}

impl_hmac!(hmac_sha2_256, crate::hacl::hmac::compute_sha2_256, 32);
impl_hmac!(hmac_sha2_384, crate::hacl::hmac::compute_sha2_384, 48);
impl_hmac!(hmac_sha2_512, crate::hacl::hmac::compute_sha2_512, 64);
