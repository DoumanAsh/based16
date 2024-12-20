use based16::{Encoder, Decoder, CharPair};
use based16::{hex_lower, hex_upper, unhex};
#[cfg(feature = "alloc")]
use based16::{hex_lower_to_vec, hex_upper_to_vec, unhex_to_vec};
use based16::{const_hex_lower, const_hex_upper};

use core::mem;

const ALL: [u8; 256] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F,
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F,
    0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF,
    0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF,
    0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF,
    0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xDC, 0xDD, 0xDE, 0xDF,
    0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF,
    0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
];

const ALL_LOWER: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";
const ALL_UPPER: &str = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";

#[test]
fn should_convert_hex_and_back_upper() {
    let encoder = Encoder::upper(&ALL);
    let mut all_upper = ALL_UPPER.as_bytes().chunks(2);

    let mut output = encoder.to_string();
    assert_eq!(ALL_UPPER, output);
    output = format!("{:?}", encoder);
    assert_eq!(output.len(), ALL_UPPER.len() + 2);
    assert_eq!(ALL_UPPER, output.trim_matches('"'));

    output = const_hex_upper(ALL).iter().map(CharPair::as_str).collect::<String>();
    assert_eq!(ALL_UPPER, output);
    assert_eq!(CharPair::array_as_str(&const_hex_upper(ALL)), ALL_UPPER);

    let mut encoded_hex = [mem::MaybeUninit::uninit(); ALL.len() * 2];
    for len in 0..=ALL.len() {
        let encoded_len =  hex_upper(&ALL, &mut encoded_hex[..len * 2]);
        assert_eq!(encoded_len, len * 2);
        let encoded_hex = unsafe {
            core::str::from_utf8(
                core::slice::from_raw_parts(encoded_hex.as_ptr() as _, encoded_len)
            ).expect("valid utf-8")
        };
        assert_eq!(&ALL_UPPER[..len * 2], encoded_hex);
    }

    let mut encoded_hex = [mem::MaybeUninit::uninit(); 255];
    let encoded_len =  hex_upper(&ALL, &mut encoded_hex);
    assert_eq!(encoded_len, 254);
    let encoded_hex = unsafe {
        core::str::from_utf8(
            core::slice::from_raw_parts(encoded_hex.as_ptr() as _, encoded_len)
        ).expect("valid utf-8")
    };
    assert_eq!(&ALL_UPPER[..254], encoded_hex);

    let mut decoded_hex = [mem::MaybeUninit::uninit(); ALL.len()];
    for len in 0..=ALL.len() {
        let decode_len = unhex(ALL_UPPER.as_bytes(), &mut decoded_hex[..len]).expect("Success");
        assert_eq!(decode_len, len);
        let decoded_hex = unsafe {
            core::slice::from_raw_parts(decoded_hex.as_ptr() as _, decode_len)
        };
        assert_eq!(&ALL[..len], decoded_hex);
    }

    #[cfg(feature = "alloc")]
    {
        let mut buff = Vec::new();
        let encoded_len =  hex_upper_to_vec(&ALL, &mut buff);
        assert_eq!(encoded_len, ALL_UPPER.len());

        let encoded_hex = core::str::from_utf8(buff.as_slice()).expect("valid utf-8");
        assert_eq!(ALL_UPPER, encoded_hex);

        buff.clear();
        let decode_len = unhex_to_vec(ALL_UPPER.as_bytes(), &mut buff).expect("To decode");
        assert_eq!(decode_len, ALL.len());
        assert_eq!(buff, ALL);
    }

    for pair in encoder {
        let expected = all_upper.next().expect("to have more expected data");
        assert_eq!(expected, pair.as_bytes());
    }

    let decoder = Decoder::new(&output).expect("To have valid output");
    for (idx, byte) in decoder.enumerate() {
        assert_eq!(byte.expect("to decode"), ALL[idx]);
    }
}

#[test]
fn should_convert_hex_and_back_lower() {
    let encoder = Encoder::lower(&ALL);
    let mut all_lower = ALL_LOWER.as_bytes().chunks(2);

    let mut output = encoder.to_string();
    assert_eq!(ALL_LOWER, output);
    output = const_hex_lower(ALL).iter().map(CharPair::as_str).collect::<String>();
    assert_eq!(ALL_LOWER, output);
    assert_eq!(CharPair::array_as_str(&const_hex_lower(ALL)), ALL_LOWER);

    let mut encoded_hex = [mem::MaybeUninit::uninit(); ALL.len() * 2];
    for len in 0..=ALL.len() {
        let encoded_len =  hex_lower(&ALL, &mut encoded_hex[..len * 2]);
        assert_eq!(encoded_len, len * 2);

        let encoded_hex = unsafe {
            core::str::from_utf8(
                core::slice::from_raw_parts(encoded_hex.as_ptr() as _, encoded_len)
            ).expect("valid utf-8")
        };
        assert_eq!(&ALL_LOWER[..len * 2], encoded_hex);

    }

    let mut encoded_hex = [mem::MaybeUninit::uninit(); 255];
    let encoded_len =  hex_lower(&ALL, &mut encoded_hex);
    assert_eq!(encoded_len, 254);
    let encoded_hex = unsafe {
        core::str::from_utf8(
            core::slice::from_raw_parts(encoded_hex.as_ptr() as _, encoded_len)
        ).expect("valid utf-8")
    };
    assert_eq!(&ALL_LOWER[..254], encoded_hex);

    let mut decoded_hex = [mem::MaybeUninit::uninit(); ALL.len()];
    for len in 0..=ALL.len() {
        let decode_len = unhex(ALL_LOWER.as_bytes(), &mut decoded_hex[..len]).expect("Success");
        assert_eq!(decode_len, len);
        let decoded_hex = unsafe {
            core::slice::from_raw_parts(decoded_hex.as_ptr() as _, decode_len)
        };
        assert_eq!(&ALL[..len], decoded_hex);
    }

    #[cfg(feature = "alloc")]
    {
        let mut buff = Vec::new();
        let encoded_len =  hex_lower_to_vec(&ALL, &mut buff);
        assert_eq!(encoded_len, ALL_LOWER.len());

        let encoded_hex = core::str::from_utf8(buff.as_slice()).expect("valid utf-8");
        assert_eq!(ALL_LOWER, encoded_hex);

        buff.clear();
        let decode_len = unhex_to_vec(ALL_LOWER.as_bytes(), &mut buff).expect("To decode");
        assert_eq!(decode_len, ALL.len());
        assert_eq!(buff, ALL);
    }

    for pair in encoder {
        let expected = all_lower.next().expect("to have more expected data");
        assert_eq!(expected, pair.as_bytes());
    }

    let decoder = Decoder::new(&output).expect("To have valid output");
    for (idx, byte) in decoder.enumerate() {
        assert_eq!(byte.expect("to decode"), ALL[idx]);
    }
}

#[test]
fn should_convert_min_hex() {
    const INPUT: &[&str] = &[
        "99",
        "99AA",
        "99AABB",
        "99AABBCC",
        "99AABBCCDD",
        "99AABBCCDDEE",
        "99AABBCCDDEEFF",
    ];

    let expected_output = [153u8, 170, 187, 204, 221, 238, 255];
    let mut decoded_hex = [mem::MaybeUninit::uninit(); 7];

    for idx in 0..INPUT.len() {
        let expected_output = &expected_output[..idx+1];
        let decode_len = unhex(INPUT[idx].as_bytes(), &mut decoded_hex).expect("Success");
        assert_eq!(decode_len, expected_output.len());
        let decoded_hex = unsafe {
            core::slice::from_raw_parts(decoded_hex.as_ptr() as *const u8, decode_len)
        };
        assert_eq!(expected_output, decoded_hex);
    }
}

#[test]
fn should_convert_min_binary() {
    const INPUT: &[&[u8]] = &[
        &[153],
        &[153, 170],
        &[153, 170, 187],
        &[153, 170, 187, 204],
        &[153, 170, 187, 204, 221],
        &[153, 170, 187, 204, 221, 238],
        &[153, 170, 187, 204, 221, 238, 255],
    ];

    let expected_output = "99AABBCCDDEEFF";
    let mut encoded_hex = [mem::MaybeUninit::uninit(); 14];

    for idx in 0..INPUT.len() {
        let input = INPUT[idx];
        let expected_output = &expected_output[..input.len() * 2];

        let encoded_len = hex_upper(input, &mut encoded_hex);
        assert_eq!(encoded_len, expected_output.len());
        let encoded_hex = unsafe {
            core::str::from_utf8_unchecked(
                core::slice::from_raw_parts(encoded_hex.as_ptr() as *const u8, encoded_len)
            )
        };
        assert_eq!(expected_output, encoded_hex);
    }
}
