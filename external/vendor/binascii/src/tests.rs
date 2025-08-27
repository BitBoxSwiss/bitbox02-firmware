use super::*;
const SAMPLE_DATA: &str = "This is a short sentence the we'll use for testing.";

#[test]
fn test_hex2bin() {
    let mut output_buffer = [0u8; 100];
    let input = "1f2F3d4d".as_bytes();

    // check good case
    assert_eq!(hex2bin(&input, &mut output_buffer).ok().unwrap(), &[0x1f, 0x2f, 0x3d, 0x4d]);

    // check bad input
    assert!(hex2bin("z1".as_bytes(), &mut output_buffer).is_err());

    // check short output buffer
    assert!(hex2bin("a1a2a3a4".as_bytes(), &mut output_buffer[0..2]).is_err());

    // check odd input
    assert!(hex2bin("a".as_bytes(), &mut output_buffer).is_err());
}

#[test]
fn test_bin2hex() {
    let mut buffer = [0u8; 200];

    // normal use
    assert_eq!(bin2hex(&[0x1f, 0xf2], &mut buffer).ok().unwrap(), "1ff2".as_bytes());

    // short buffer
    assert!(bin2hex(&[0x1f, 0xf2], &mut buffer[0..2]).is_err());
}

#[test]
fn base32_sanity() {
    for length in 0..30 {
        let mut output = [0u8; 500];
        let mut dec_out = [0u8; 200];
        let input = &SAMPLE_DATA[..length].as_bytes();

        let encoded_output = b32encode(&input, &mut output).ok().unwrap();

        let decoded_output = b32decode(&encoded_output, &mut dec_out).ok().unwrap();

        assert_eq!(input, &decoded_output);
    }
}

#[test]
fn base64_sanity() {
    for length in 0..30 {
        let mut output = [0u8; 500];
        let mut dec_out = [0u8; 200];
        let input = &SAMPLE_DATA[..length].as_bytes();

        let encoded_output = b64encode(&input, &mut output).ok().unwrap();

        let decoded_output = b64decode(&encoded_output, &mut dec_out).ok().unwrap();

        assert_eq!(input, &decoded_output);
    }
}

#[test]
fn base64_padding_checks() {
    {
        let invalid = "00==";
        let mut decoded = [0u8; 3];
        assert_eq!(b64decode(invalid.as_bytes(), &mut decoded[..]).err().expect("Accepted invalid input"), ConvertError::InvalidInput);
    }

    {
        let invalid = "001=";
        let mut decoded = [0u8; 3];
        assert_eq!(b64decode(invalid.as_bytes(), &mut decoded[..]).err().expect("Accepted invalid input"), ConvertError::InvalidInput);
    }

    {
        let valid = "0A==";
        let mut decoded = [0u8; 3];
        b64decode(valid.as_bytes(), &mut decoded[..]).ok().unwrap();
    }

    {
        let valid = "0AA=";
        let mut decoded = [0u8; 3];
        b64decode(valid.as_bytes(), &mut decoded[..]).ok().unwrap();
    }

    {
        let valid = "0A00";
        let mut decoded = [0u8; 3];
        b64decode(valid.as_bytes(), &mut decoded[..]).ok().unwrap();
    }
}

// Check if round tripping an encoded text produces an example of Base64 encoding malleability (bad)
fn malleability_check(encoded: &[u8]) {
    let mut buffer = [0u8; 1000];
    if let Ok(result1) = b64decode(encoded, &mut buffer) {
        let mut buffer2 = [0u8; 1000];
        if let Ok(result2) = b64encode(result1, &mut buffer2) {
            assert_eq!(encoded, result2, "two different encodings of same payload were found, base64 encoding is malleable!");
        }
    }
}

// Some of these patterns are valid encodings and some are invalid, ensure that
// either round-tripping them produces an error, or produces the same encoded
// string, so that there are no two different valid encodings of the same payload.
#[test]
fn base64_malleability_checks() {
    malleability_check(b"00==");
    malleability_check(b"000=");
    malleability_check(b"00A=");
    malleability_check(b"001=");
    malleability_check(b"0A==");
    malleability_check(b"0AA=");
    malleability_check(b"A00=");
}

fn decode_tester<F>(f: F) where for <'a> F: Fn(&[u8], &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    const DATA: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    for slice_len in 0..DATA.len() {
        let test_data = DATA[0..slice_len].as_bytes();

        for output_size in 0..100 {
            let mut output = [0u8; 100];
            let _ = f(test_data, &mut output[0..output_size]);
        }
    }
}

#[test]
fn b16_len_test() {
    decode_tester(hex2bin);
}

#[test]
fn b32_len_test() {
    decode_tester(b32decode);
}
#[test]

fn b64_len_test() {
    decode_tester(b64decode);
}
