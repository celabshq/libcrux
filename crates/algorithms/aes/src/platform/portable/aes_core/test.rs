use super::*;

#[allow(non_snake_case)]
fn sub_bytes_inv_state(st: &mut State) {
    let U0 = st[7];
    let U1 = st[6];
    let U2 = st[5];
    let U3 = st[4];
    let U4 = st[3];
    let U5 = st[2];
    let U6 = st[1];
    let U7 = st[0];

    let T23 = U0 ^ U3;
    let T22 = xnor(U1, U3);
    let T2 = xnor(U0, U1);
    let T1 = U3 ^ U4;
    let T24 = xnor(U4, U7);
    let R5 = U6 ^ U7;
    let T8 = xnor(U1, T23);
    let T19 = T22 ^ R5;
    let T9 = xnor(U7, T1);
    let T10 = T2 ^ T24;
    let T13 = T2 ^ R5;
    let T3 = T1 ^ R5;
    let T25 = xnor(U2, T1);
    let R13 = U1 ^ U6;
    let T17 = xnor(U2, T19);
    let T20 = T24 ^ R13;
    let T4 = U4 ^ T8;
    let R17 = xnor(U2, U5);
    let R18 = xnor(U5, U6);
    let R19 = xnor(U2, U4);
    let Y5 = U0 ^ R17;
    let T6 = T22 ^ R17;
    let T16 = R13 ^ R19;
    let T27 = T1 ^ R18;
    let T15 = T10 ^ T27;
    let T14 = T10 ^ R18;
    let T26 = T3 ^ T16;
    let M1 = T13 & T6;
    let M2 = T23 & T8;
    let M3 = T14 ^ M1;
    let M4 = T19 & Y5;
    let M5 = M4 ^ M1;
    let M6 = T3 & T16;
    let M7 = T22 & T9;
    let M8 = T26 ^ M6;
    let M9 = T20 & T17;
    let M10 = M9 ^ M6;
    let M11 = T1 & T15;
    let M12 = T4 & T27;
    let M13 = M12 ^ M11;
    let M14 = T2 & T10;
    let M15 = M14 ^ M11;
    let M16 = M3 ^ M2;
    let M17 = M5 ^ T24;
    let M18 = M8 ^ M7;
    let M19 = M10 ^ M15;
    let M20 = M16 ^ M13;
    let M21 = M17 ^ M15;
    let M22 = M18 ^ M13;
    let M23 = M19 ^ T25;
    let M24 = M22 ^ M23;
    let M25 = M22 & M20;
    let M26 = M21 ^ M25;
    let M27 = M20 ^ M21;
    let M28 = M23 ^ M25;
    let M29 = M28 & M27;
    let M30 = M26 & M24;
    let M31 = M20 & M23;
    let M32 = M27 & M31;
    let M33 = M27 ^ M25;
    let M34 = M21 & M22;
    let M35 = M24 & M34;
    let M36 = M24 ^ M25;
    let M37 = M21 ^ M29;
    let M38 = M32 ^ M33;
    let M39 = M23 ^ M30;
    let M40 = M35 ^ M36;
    let M41 = M38 ^ M40;
    let M42 = M37 ^ M39;
    let M43 = M37 ^ M38;
    let M44 = M39 ^ M40;
    let M45 = M42 ^ M41;
    let M46 = M44 & T6;
    let M47 = M40 & T8;
    let M48 = M39 & Y5;
    let M49 = M43 & T16;
    let M50 = M38 & T9;
    let M51 = M37 & T17;
    let M52 = M42 & T15;
    let M53 = M45 & T27;
    let M54 = M41 & T10;
    let M55 = M44 & T13;
    let M56 = M40 & T23;
    let M57 = M39 & T19;
    let M58 = M43 & T3;
    let M59 = M38 & T22;
    let M60 = M37 & T20;
    let M61 = M42 & T1;
    let M62 = M45 & T4;
    let M63 = M41 & T2;
    let P0 = M52 ^ M61;
    let P1 = M58 ^ M59;
    let P2 = M54 ^ M62;
    let P3 = M47 ^ M50;
    let P4 = M48 ^ M56;
    let P5 = M46 ^ M51;
    let P6 = M49 ^ M60;
    let P7 = P0 ^ P1;
    let P8 = M50 ^ M53;
    let P9 = M55 ^ M63;
    let P10 = M57 ^ P4;
    let P11 = P0 ^ P3;
    let P12 = M46 ^ M48;
    let P13 = M49 ^ M51;
    let P14 = M49 ^ M62;
    let P15 = M54 ^ M59;
    let P16 = M57 ^ M61;
    let P17 = M58 ^ P2;
    let P18 = M63 ^ P5;
    let P19 = P2 ^ P3;
    let P20 = P4 ^ P6;
    let P22 = P2 ^ P7;
    let P23 = P7 ^ P8;
    let P24 = P5 ^ P7;
    let P25 = P6 ^ P10;
    let P26 = P9 ^ P11;
    let P27 = P10 ^ P18;
    let P28 = P11 ^ P25;
    let P29 = P15 ^ P20;
    let W0 = P13 ^ P22;
    let W1 = P26 ^ P29;
    let W2 = P17 ^ P28;
    let W3 = P12 ^ P22;
    let W4 = P23 ^ P27;
    let W5 = P19 ^ P24;
    let W6 = P14 ^ P23;
    let W7 = P9 ^ P16;

    st[0] = W7;
    st[1] = W6;
    st[2] = W5;
    st[3] = W4;
    st[4] = W3;
    st[5] = W2;
    st[6] = W1;
    st[7] = W0;
}

fn sbox_fwd(s: u8) -> u8 {
    match s {
        0 => 0x63,
        1 => 0x7c,
        2 => 0x77,
        3 => 0x7b,
        4 => 0xf2,
        5 => 0x6b,
        6 => 0x6f,
        7 => 0xc5,
        8 => 0x30,
        9 => 0x01,
        10 => 0x67,
        11 => 0x2b,
        12 => 0xfe,
        13 => 0xd7,
        14 => 0xab,
        15 => 0x76,
        16 => 0xca,
        17 => 0x82,
        18 => 0xc9,
        19 => 0x7d,
        20 => 0xfa,
        21 => 0x59,
        22 => 0x47,
        23 => 0xf0,
        24 => 0xad,
        25 => 0xd4,
        26 => 0xa2,
        27 => 0xaf,
        28 => 0x9c,
        29 => 0xa4,
        30 => 0x72,
        31 => 0xc0,
        32 => 0xb7,
        33 => 0xfd,
        34 => 0x93,
        35 => 0x26,
        36 => 0x36,
        37 => 0x3f,
        38 => 0xf7,
        39 => 0xcc,
        40 => 0x34,
        41 => 0xa5,
        42 => 0xe5,
        43 => 0xf1,
        44 => 0x71,
        45 => 0xd8,
        46 => 0x31,
        47 => 0x15,
        48 => 0x04,
        49 => 0xc7,
        50 => 0x23,
        51 => 0xc3,
        52 => 0x18,
        53 => 0x96,
        54 => 0x05,
        55 => 0x9a,
        56 => 0x07,
        57 => 0x12,
        58 => 0x80,
        59 => 0xe2,
        60 => 0xeb,
        61 => 0x27,
        62 => 0xb2,
        63 => 0x75,
        64 => 0x09,
        65 => 0x83,
        66 => 0x2c,
        67 => 0x1a,
        68 => 0x1b,
        69 => 0x6e,
        70 => 0x5a,
        71 => 0xa0,
        72 => 0x52,
        73 => 0x3b,
        74 => 0xd6,
        75 => 0xb3,
        76 => 0x29,
        77 => 0xe3,
        78 => 0x2f,
        79 => 0x84,
        80 => 0x53,
        81 => 0xd1,
        82 => 0x00,
        83 => 0xed,
        84 => 0x20,
        85 => 0xfc,
        86 => 0xb1,
        87 => 0x5b,
        88 => 0x6a,
        89 => 0xcb,
        90 => 0xbe,
        91 => 0x39,
        92 => 0x4a,
        93 => 0x4c,
        94 => 0x58,
        95 => 0xcf,
        96 => 0xd0,
        97 => 0xef,
        98 => 0xaa,
        99 => 0xfb,
        100 => 0x43,
        101 => 0x4d,
        102 => 0x33,
        103 => 0x85,
        104 => 0x45,
        105 => 0xf9,
        106 => 0x02,
        107 => 0x7f,
        108 => 0x50,
        109 => 0x3c,
        110 => 0x9f,
        111 => 0xa8,
        112 => 0x51,
        113 => 0xa3,
        114 => 0x40,
        115 => 0x8f,
        116 => 0x92,
        117 => 0x9d,
        118 => 0x38,
        119 => 0xf5,
        120 => 0xbc,
        121 => 0xb6,
        122 => 0xda,
        123 => 0x21,
        124 => 0x10,
        125 => 0xff,
        126 => 0xf3,
        127 => 0xd2,
        128 => 0xcd,
        129 => 0x0c,
        130 => 0x13,
        131 => 0xec,
        132 => 0x5f,
        133 => 0x97,
        134 => 0x44,
        135 => 0x17,
        136 => 0xc4,
        137 => 0xa7,
        138 => 0x7e,
        139 => 0x3d,
        140 => 0x64,
        141 => 0x5d,
        142 => 0x19,
        143 => 0x73,
        144 => 0x60,
        145 => 0x81,
        146 => 0x4f,
        147 => 0xdc,
        148 => 0x22,
        149 => 0x2a,
        150 => 0x90,
        151 => 0x88,
        152 => 0x46,
        153 => 0xee,
        154 => 0xb8,
        155 => 0x14,
        156 => 0xde,
        157 => 0x5e,
        158 => 0x0b,
        159 => 0xdb,
        160 => 0xe0,
        161 => 0x32,
        162 => 0x3a,
        163 => 0x0a,
        164 => 0x49,
        165 => 0x06,
        166 => 0x24,
        167 => 0x5c,
        168 => 0xc2,
        169 => 0xd3,
        170 => 0xac,
        171 => 0x62,
        172 => 0x91,
        173 => 0x95,
        174 => 0xe4,
        175 => 0x79,
        176 => 0xe7,
        177 => 0xc8,
        178 => 0x37,
        179 => 0x6d,
        180 => 0x8d,
        181 => 0xd5,
        182 => 0x4e,
        183 => 0xa9,
        184 => 0x6c,
        185 => 0x56,
        186 => 0xf4,
        187 => 0xea,
        188 => 0x65,
        189 => 0x7a,
        190 => 0xae,
        191 => 0x08,
        192 => 0xba,
        193 => 0x78,
        194 => 0x25,
        195 => 0x2e,
        196 => 0x1c,
        197 => 0xa6,
        198 => 0xb4,
        199 => 0xc6,
        200 => 0xe8,
        201 => 0xdd,
        202 => 0x74,
        203 => 0x1f,
        204 => 0x4b,
        205 => 0xbd,
        206 => 0x8b,
        207 => 0x8a,
        208 => 0x70,
        209 => 0x3e,
        210 => 0xb5,
        211 => 0x66,
        212 => 0x48,
        213 => 0x03,
        214 => 0xf6,
        215 => 0x0e,
        216 => 0x61,
        217 => 0x35,
        218 => 0x57,
        219 => 0xb9,
        220 => 0x86,
        221 => 0xc1,
        222 => 0x1d,
        223 => 0x9e,
        224 => 0xe1,
        225 => 0xf8,
        226 => 0x98,
        227 => 0x11,
        228 => 0x69,
        229 => 0xd9,
        230 => 0x8e,
        231 => 0x94,
        232 => 0x9b,
        233 => 0x1e,
        234 => 0x87,
        235 => 0xe9,
        236 => 0xce,
        237 => 0x55,
        238 => 0x28,
        239 => 0xdf,
        240 => 0x8c,
        241 => 0xa1,
        242 => 0x89,
        243 => 0x0d,
        244 => 0xbf,
        245 => 0xe6,
        246 => 0x42,
        247 => 0x68,
        248 => 0x41,
        249 => 0x99,
        250 => 0x2d,
        251 => 0x0f,
        252 => 0xb0,
        253 => 0x54,
        254 => 0xbb,
        255 => 0x16,
    }
}

fn sbox_inv(s: u8) -> u8 {
    match s {
        0 => 0x52,
        1 => 0x09,
        2 => 0x6a,
        3 => 0xd5,
        4 => 0x30,
        5 => 0x36,
        6 => 0xa5,
        7 => 0x38,
        8 => 0xbf,
        9 => 0x40,
        10 => 0xa3,
        11 => 0x9e,
        12 => 0x81,
        13 => 0xf3,
        14 => 0xd7,
        15 => 0xfb,
        16 => 0x7c,
        17 => 0xe3,
        18 => 0x39,
        19 => 0x82,
        20 => 0x9b,
        21 => 0x2f,
        22 => 0xff,
        23 => 0x87,
        24 => 0x34,
        25 => 0x8e,
        26 => 0x43,
        27 => 0x44,
        28 => 0xc4,
        29 => 0xde,
        30 => 0xe9,
        31 => 0xcb,
        32 => 0x54,
        33 => 0x7b,
        34 => 0x94,
        35 => 0x32,
        36 => 0xa6,
        37 => 0xc2,
        38 => 0x23,
        39 => 0x3d,
        40 => 0xee,
        41 => 0x4c,
        42 => 0x95,
        43 => 0x0b,
        44 => 0x42,
        45 => 0xfa,
        46 => 0xc3,
        47 => 0x4e,
        48 => 0x08,
        49 => 0x2e,
        50 => 0xa1,
        51 => 0x66,
        52 => 0x28,
        53 => 0xd9,
        54 => 0x24,
        55 => 0xb2,
        56 => 0x76,
        57 => 0x5b,
        58 => 0xa2,
        59 => 0x49,
        60 => 0x6d,
        61 => 0x8b,
        62 => 0xd1,
        63 => 0x25,
        64 => 0x72,
        65 => 0xf8,
        66 => 0xf6,
        67 => 0x64,
        68 => 0x86,
        69 => 0x68,
        70 => 0x98,
        71 => 0x16,
        72 => 0xd4,
        73 => 0xa4,
        74 => 0x5c,
        75 => 0xcc,
        76 => 0x5d,
        77 => 0x65,
        78 => 0xb6,
        79 => 0x92,
        80 => 0x6c,
        81 => 0x70,
        82 => 0x48,
        83 => 0x50,
        84 => 0xfd,
        85 => 0xed,
        86 => 0xb9,
        87 => 0xda,
        88 => 0x5e,
        89 => 0x15,
        90 => 0x46,
        91 => 0x57,
        92 => 0xa7,
        93 => 0x8d,
        94 => 0x9d,
        95 => 0x84,
        96 => 0x90,
        97 => 0xd8,
        98 => 0xab,
        99 => 0x00,
        100 => 0x8c,
        101 => 0xbc,
        102 => 0xd3,
        103 => 0x0a,
        104 => 0xf7,
        105 => 0xe4,
        106 => 0x58,
        107 => 0x05,
        108 => 0xb8,
        109 => 0xb3,
        110 => 0x45,
        111 => 0x06,
        112 => 0xd0,
        113 => 0x2c,
        114 => 0x1e,
        115 => 0x8f,
        116 => 0xca,
        117 => 0x3f,
        118 => 0x0f,
        119 => 0x02,
        120 => 0xc1,
        121 => 0xaf,
        122 => 0xbd,
        123 => 0x03,
        124 => 0x01,
        125 => 0x13,
        126 => 0x8a,
        127 => 0x6b,
        128 => 0x3a,
        129 => 0x91,
        130 => 0x11,
        131 => 0x41,
        132 => 0x4f,
        133 => 0x67,
        134 => 0xdc,
        135 => 0xea,
        136 => 0x97,
        137 => 0xf2,
        138 => 0xcf,
        139 => 0xce,
        140 => 0xf0,
        141 => 0xb4,
        142 => 0xe6,
        143 => 0x73,
        144 => 0x96,
        145 => 0xac,
        146 => 0x74,
        147 => 0x22,
        148 => 0xe7,
        149 => 0xad,
        150 => 0x35,
        151 => 0x85,
        152 => 0xe2,
        153 => 0xf9,
        154 => 0x37,
        155 => 0xe8,
        156 => 0x1c,
        157 => 0x75,
        158 => 0xdf,
        159 => 0x6e,
        160 => 0x47,
        161 => 0xf1,
        162 => 0x1a,
        163 => 0x71,
        164 => 0x1d,
        165 => 0x29,
        166 => 0xc5,
        167 => 0x89,
        168 => 0x6f,
        169 => 0xb7,
        170 => 0x62,
        171 => 0x0e,
        172 => 0xaa,
        173 => 0x18,
        174 => 0xbe,
        175 => 0x1b,
        176 => 0xfc,
        177 => 0x56,
        178 => 0x3e,
        179 => 0x4b,
        180 => 0xc6,
        181 => 0xd2,
        182 => 0x79,
        183 => 0x20,
        184 => 0x9a,
        185 => 0xdb,
        186 => 0xc0,
        187 => 0xfe,
        188 => 0x78,
        189 => 0xcd,
        190 => 0x5a,
        191 => 0xf4,
        192 => 0x1f,
        193 => 0xdd,
        194 => 0xa8,
        195 => 0x33,
        196 => 0x88,
        197 => 0x07,
        198 => 0xc7,
        199 => 0x31,
        200 => 0xb1,
        201 => 0x12,
        202 => 0x10,
        203 => 0x59,
        204 => 0x27,
        205 => 0x80,
        206 => 0xec,
        207 => 0x5f,
        208 => 0x60,
        209 => 0x51,
        210 => 0x7f,
        211 => 0xa9,
        212 => 0x19,
        213 => 0xb5,
        214 => 0x4a,
        215 => 0x0d,
        216 => 0x2d,
        217 => 0xe5,
        218 => 0x7a,
        219 => 0x9f,
        220 => 0x93,
        221 => 0xc9,
        222 => 0x9c,
        223 => 0xef,
        224 => 0xa0,
        225 => 0xe0,
        226 => 0x3b,
        227 => 0x4d,
        228 => 0xae,
        229 => 0x2a,
        230 => 0xf5,
        231 => 0xb0,
        232 => 0xc8,
        233 => 0xeb,
        234 => 0xbb,
        235 => 0x3c,
        236 => 0x83,
        237 => 0x53,
        238 => 0x99,
        239 => 0x61,
        240 => 0x17,
        241 => 0x2b,
        242 => 0x04,
        243 => 0x7e,
        244 => 0xba,
        245 => 0x77,
        246 => 0xd6,
        247 => 0x26,
        248 => 0xe1,
        249 => 0x69,
        250 => 0x14,
        251 => 0x63,
        252 => 0x55,
        253 => 0x21,
        254 => 0x0c,
        255 => 0x7d,
    }
}

use rand_core::RngCore as _;

use crate::platform::portable::aes_core::transpose_u8x16;

fn get_bit_u8(x: &[u8], i: usize, j: usize) -> u8 {
    (x[i] >> j) & 0x1
}

fn get_bit_u16(x: &[u16], i: usize, j: usize) -> u8 {
    ((x[j] >> i) & 0x1) as u8
}

#[test]
fn test_transpose() {
    let mut x = [0u8; 16];
    rand::rng().fill_bytes(&mut x);
    let mut y = [0u16; 8];
    transpose_u8x16(&x, &mut y);
    for i in 0..16 {
        for j in 0..8 {
            if get_bit_u8(&x, i, j) != get_bit_u16(&y, i, j) {
                #[cfg(feature = "std")]
                {
                    std::eprintln!("x[{},{}] = {}", i, j, get_bit_u8(&x, i, j));
                    std::eprintln!("y[{},{}] = {}", i, j, get_bit_u16(&y, i, j));
                }
                panic!();
            } else {
                #[cfg(feature = "std")]
                std::eprintln!("transpose ok: {},{}", i, j);
            }
        }
    }
    let mut z = [0u8; 16];
    transpose_u16x8(&y, &mut z);
    for i in 0..16 {
        for j in 0..8 {
            if get_bit_u8(&x, i, j) != get_bit_u8(&z, i, j) {
                #[cfg(feature = "std")]
                {
                    std::eprintln!("x[{},{}] = {}", i, j, get_bit_u8(&x, i, j));
                    std::eprintln!("z[{},{}] = {}", i, j, get_bit_u8(&z, i, j));
                }
                panic!();
            } else {
                #[cfg(feature = "std")]
                std::eprintln!("inv-transpose ok: {},{}", i, j);
            }
        }
    }
}

#[test]
fn test_sbox() {
    let mut x = [0u8; 16];
    let mut y = [0u16; 8];
    let mut w = [0u8; 16];
    for i in 0..=255 {
        x[0] = i;
        x[9] = i;
        transpose_u8x16(&x, &mut y);
        sub_bytes_state(&mut y);
        transpose_u16x8(&y, &mut w);
        if w[0] != sbox_fwd(i) {
            #[cfg(feature = "std")]
            std::eprintln!("sbox[{}] = {}, should be {}", i, w[0], sbox_fwd(i));
            panic!();
        } else {
            #[cfg(feature = "std")]
            std::eprintln!("sbox ok {}", i)
        }
    }
}

#[test]
fn test_sbox_inv() {
    let mut x = [0u8; 16];
    let mut y = [0u16; 8];
    let mut w = [0u8; 16];
    for i in 0..=255 {
        x[0] = i;
        x[9] = i;
        transpose_u8x16(&x, &mut y);
        sub_bytes_inv_state(&mut y);
        transpose_u16x8(&y, &mut w);
        if w[0] != sbox_inv(i) {
            #[cfg(feature = "std")]
            std::eprintln!("sbox_inv[{}] = {}, should be {}", i, w[0], sbox_inv(i));
            panic!();
        } else {
            #[cfg(feature = "std")]
            std::eprintln!("sbox inv ok {}", i)
        }
    }
}
