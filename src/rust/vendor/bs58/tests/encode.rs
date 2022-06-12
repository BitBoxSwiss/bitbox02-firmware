mod cases;

const FILLER: [u8; 512] = [b'~'; 512];

#[test]
fn test_encode() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(s, bs58::encode(val).into_string());

        assert_eq!(s.as_bytes(), &*bs58::encode(val).into_vec());

        {
            let mut bytes = FILLER;
            assert_eq!(Ok(s.len()), bs58::encode(val).into(&mut bytes[..]));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
        }

        {
            let mut bytes = FILLER;
            if !s.is_empty() {
                bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
            }
            let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
            assert_eq!(Ok(s.len()), bs58::encode(val).into(string));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            if !s.is_empty() {
                assert_eq!(0, bytes[s.len()]);
            }
            assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
        }
    }
}

#[test]
#[cfg(feature = "check")]
fn test_encode_check() {
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        assert_eq!(s, bs58::encode(val).with_check().into_string());

        assert_eq!(s.as_bytes(), &*bs58::encode(val).with_check().into_vec());

        {
            let mut bytes = FILLER;
            assert_eq!(
                Ok(s.len()),
                bs58::encode(val).with_check().into(&mut bytes[..])
            );
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);

            if !val.is_empty() {
                assert_eq!(
                    Ok(s.len()),
                    bs58::encode(&val[1..])
                        .with_check_version(val[0])
                        .into(&mut bytes[..])
                );
                assert_eq!(s.as_bytes(), &bytes[..s.len()]);
                assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
            }
        }

        {
            let mut bytes = FILLER;
            if !s.is_empty() {
                bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
            }
            let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
            assert_eq!(Ok(s.len()), bs58::encode(val).with_check().into(string));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            if !s.is_empty() {
                assert_eq!(0, bytes[s.len()]);
            }
            assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
        }
    }
}
