pub fn simple<
    const OUTPUT_LEN: usize,
    HashImplementation: super::arrayref::DigestIncremental<OUTPUT_LEN> + super::arrayref::Hash<OUTPUT_LEN>,
>() {
    let payload = &[1, 2, 3, 4, 5];

    // oneshot API
    let mut digest_oneshot = [0u8; OUTPUT_LEN];
    HashImplementation::hash(&mut digest_oneshot, payload).unwrap();

    // incremental API
    let mut digest_incremental = [0u8; OUTPUT_LEN];
    let mut hasher = super::Hasher::<OUTPUT_LEN, HashImplementation>::new();
    hasher.update(payload).unwrap();
    hasher.finish(&mut digest_incremental);

    // ensure same
    assert_eq!(digest_oneshot, digest_incremental);
}
