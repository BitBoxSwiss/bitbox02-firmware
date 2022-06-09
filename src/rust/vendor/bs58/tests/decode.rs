mod cases;

#[cfg(feature = "check")]
use assert_matches::assert_matches;

#[test]
fn test_decode() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(val.to_vec(), bs58::decode(s).into_vec().unwrap());
    }
}

#[test]
fn test_decode_small_buffer_err() {
    let mut output = [0; 2];
    assert_eq!(
        bs58::decode("a3gV").into(&mut output),
        Err(bs58::decode::Error::BufferTooSmall)
    );
}

#[test]
fn test_decode_invalid_char() {
    let sample = "123456789abcd!efghij";
    assert_eq!(
        bs58::decode(sample).into_vec().unwrap_err(),
        bs58::decode::Error::InvalidCharacter {
            character: '!',
            index: 13
        }
    );
}

#[test]
#[cfg(feature = "check")]
fn test_decode_check() {
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        assert_eq!(
            val.to_vec(),
            bs58::decode(s).with_check(None).into_vec().unwrap()
        );
    }

    for &(val, s) in cases::CHECK_TEST_CASES[1..].iter() {
        assert_eq!(
            val.to_vec(),
            bs58::decode(s).with_check(Some(val[0])).into_vec().unwrap()
        );
    }
}

#[test]
#[cfg(feature = "check")]
fn test_check_ver_failed() {
    let d = bs58::decode("K5zqBMZZTzUbAZQgrt4")
        .with_check(Some(0x01))
        .into_vec();

    assert!(d.is_err());
    assert_matches!(d.unwrap_err(), bs58::decode::Error::InvalidVersion { .. });
}
