/// Test vectors from RFC 8439 Section 2.5 and Appendix A.3.
///
/// Tests both the public wrapper API (mac()) and the underlying HACL
/// implementation directly.

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

/// Test the public wrapper API with the RFC 7539 Section 2.5.2 test vector.
#[test]
fn wrapper_rfc7539_section_2_5_2() {
    let key: [u8; 32] =
        hex_to_bytes("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b")
            .try_into()
            .unwrap();
    let msg = b"Cryptographic Forum Research Group";
    let expected_tag = hex_to_bytes("a8061dc1305136c6c22b8baf0c0127a9");

    let mut tag = [0u8; 16];
    assert!(libcrux_poly1305::mac(&key, msg, &mut tag).is_ok());

    assert_eq!(&tag[..], &expected_tag[..]);
}

/// Test the HACL Poly1305 implementation directly with 32-byte keys.
/// RFC 7539 Section 2.5.2 test vector.
#[test]
fn rfc7539_section_2_5_2_via_hacl() {
    let key = hex_to_bytes("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
    let msg = b"Cryptographic Forum Research Group";
    let expected_tag = hex_to_bytes("a8061dc1305136c6c22b8baf0c0127a9");

    let mut tag = [0u8; 16];
    libcrux_poly1305::hacl::mac_poly1305::mac(&mut tag, msg, msg.len() as u32, &key);

    assert_eq!(&tag[..], &expected_tag[..]);
}

/// RFC 7539 Section 2.5.2 — additional test vector with empty message
#[test]
fn poly1305_empty_message() {
    // Key: all zeros (r=0, s=0) => tag should be 0
    let key = [0u8; 32];
    let msg: &[u8] = &[];
    let expected_tag = [0u8; 16]; // With r=0 and s=0, tag = s = 0

    let mut tag = [0u8; 16];
    libcrux_poly1305::hacl::mac_poly1305::mac(&mut tag, msg, 0, &key);

    assert_eq!(&tag[..], &expected_tag[..]);
}

/// RFC 8439 (was 7539) Appendix A.3 — Test Vector #1
#[test]
fn rfc8439_appendix_a3_test1() {
    let key = hex_to_bytes("0000000000000000000000000000000000000000000000000000000000000000");
    let msg = hex_to_bytes(
        "00000000000000000000000000000000\
         00000000000000000000000000000000\
         00000000000000000000000000000000\
         00000000000000000000000000000000",
    );
    let expected_tag = hex_to_bytes("00000000000000000000000000000000");

    let mut tag = [0u8; 16];
    libcrux_poly1305::hacl::mac_poly1305::mac(&mut tag, &msg, msg.len() as u32, &key);

    assert_eq!(&tag[..], &expected_tag[..]);
}

/// RFC 8439 Appendix A.3 — Test Vector #2
#[test]
fn rfc8439_appendix_a3_test2() {
    // "Any submission to the IETF intended by the Contributor..."
    let key = hex_to_bytes("0000000000000000000000000000000036e5f6b5c5e06070f0efca96227a863e");
    let msg = b"Any submission to the IETF intended by the Contributor for \
publication as all or part of an IETF Internet-Draft or RFC and any statement \
made within the context of an IETF activity is considered an \"IETF \
Contribution\". Such statements include oral statements in IETF sessions, as \
well as written and electronic communications made at any time or place, which \
are addressed to";
    let expected_tag = hex_to_bytes("36e5f6b5c5e06070f0efca96227a863e");

    let mut tag = [0u8; 16];
    libcrux_poly1305::hacl::mac_poly1305::mac(&mut tag, msg, msg.len() as u32, &key);

    assert_eq!(&tag[..], &expected_tag[..]);
}

/// RFC 8439 Appendix A.3 — Test Vector #3
#[test]
fn rfc8439_appendix_a3_test3() {
    let key = hex_to_bytes("36e5f6b5c5e06070f0efca96227a863e00000000000000000000000000000000");
    let msg = b"Any submission to the IETF intended by the Contributor for \
publication as all or part of an IETF Internet-Draft or RFC and any statement \
made within the context of an IETF activity is considered an \"IETF \
Contribution\". Such statements include oral statements in IETF sessions, as \
well as written and electronic communications made at any time or place, which \
are addressed to";
    let expected_tag = hex_to_bytes("f3477e7cd95417af89a6b8794c310cf0");

    let mut tag = [0u8; 16];
    libcrux_poly1305::hacl::mac_poly1305::mac(&mut tag, msg, msg.len() as u32, &key);

    assert_eq!(&tag[..], &expected_tag[..]);
}

/// RFC 8439 Appendix A.3 — Test Vector #4 (from Section 2.5.2)
#[test]
fn rfc8439_appendix_a3_test4() {
    let key = hex_to_bytes("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0");
    let msg = hex_to_bytes(
        "2754776173206272696c6c69672c2061\
         6e642074686520736c6974687920746f\
         7665730a446964206779726520616e64\
         2067696d626c6520696e207468652077\
         6162653a0a416c6c206d696d73792077\
         6572652074686520626f726f676f7665\
         732c0a416e6420746865206d6f6d6520\
         7261746873206f757467726162652e",
    );
    let expected_tag = hex_to_bytes("4541669a7eaaee61e708dc7cbcc5eb62");

    let mut tag = [0u8; 16];
    libcrux_poly1305::hacl::mac_poly1305::mac(&mut tag, &msg, msg.len() as u32, &key);

    assert_eq!(&tag[..], &expected_tag[..]);
}
