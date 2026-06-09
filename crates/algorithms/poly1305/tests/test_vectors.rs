#[test]
fn poly1305_test_vectors() {
    let vectors = libcrux_kats::poly1305::load();
    assert!(!vectors.is_empty());

    for (i, tv) in vectors.iter().enumerate() {
        let mut tag = [0u8; 16];
        libcrux_poly1305::mac(&tv.key, &tv.input, &mut tag)
            .unwrap_or_else(|_| panic!("vector {i}: mac failed"));
        assert_eq!(&tag, &tv.mac, "vector {i}: MAC mismatch");
    }
    println!("Ran {} Poly1305 test vectors", vectors.len());
}
