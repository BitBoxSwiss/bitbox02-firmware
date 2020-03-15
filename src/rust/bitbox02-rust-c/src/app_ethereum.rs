use super::util::{Bytes, CStrMut};

#[no_mangle]
pub unsafe extern "C" fn rust_ethereum_keypath_is_valid_xpub(
    keypath: *const u32,
    keypath_len: usize,
    expected_coin: u32,
) -> bool {
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    ethereum::keypath::is_valid_keypath_xpub(keypath, expected_coin)
}

#[no_mangle]
pub unsafe extern "C" fn rust_ethereum_keypath_is_valid_address(
    keypath: *const u32,
    keypath_len: usize,
    expected_coin: u32,
) -> bool {
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    ethereum::keypath::is_valid_keypath_address(keypath, expected_coin)
}

#[no_mangle]
pub extern "C" fn rust_ethereum_address_from_pubkey_hash(recipient: Bytes, mut out: CStrMut) {
    use core::convert::TryFrom;
    let recipient = <[u8; 20]>::try_from(recipient.as_ref()).unwrap();
    ethereum::address::from_pubkey_hash(&recipient, &mut out).unwrap();
}

#[no_mangle]
pub extern "C" fn rust_ethereum_address_from_pubkey(pubkey: Bytes, mut out: CStrMut) {
    assert!(pubkey.as_ref().len() == 65);
    let ptr = pubkey.as_ref().as_ptr() as *const [u8; 65];
    let pubkey = unsafe { &*ptr };
    ethereum::address::from_pubkey(pubkey, &mut out).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{rust_util_bytes, rust_util_cstr_mut};
    use std::prelude::v1::*;

    #[test]
    fn test_rust_ethereum_address_from_pubkey_hash() {
        let recipient =
            b"\xf4\xc2\x17\x10\xef\x8b\x5a\x5e\xc4\xbd\x37\x80\xa6\x87\xfe\x08\x34\x46\xe6\x7b";
        let recipient = rust_util_bytes(recipient.as_ptr(), recipient.len());
        let mut result = vec![0; 43];
        rust_ethereum_address_from_pubkey_hash(recipient, unsafe {
            rust_util_cstr_mut(result.as_mut_ptr(), result.len())
        });
        assert_eq!(
            b"0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B\0".to_vec(),
            result
        );
    }

    #[test]
    fn test_rust_ethereum_address_from_pubkey() {
        let pubkey = &[
            0x04, 0xd8, 0xae, 0xa8, 0x0d, 0x2d, 0xbc, 0xeb, 0xbe, 0x10, 0xfd, 0xfa, 0xc2, 0xd2,
            0xdb, 0x19, 0x64, 0x15, 0x5b, 0xa9, 0x9e, 0x0d, 0xd7, 0xbf, 0xd5, 0xcf, 0xfe, 0xd9,
            0x7a, 0x1c, 0xae, 0xf7, 0xd0, 0xb9, 0x07, 0x2d, 0x9c, 0x0f, 0x50, 0x49, 0x30, 0xef,
            0x59, 0xb7, 0x52, 0xd4, 0xfe, 0xa0, 0xcb, 0xde, 0x3e, 0x27, 0x3e, 0xe9, 0x54, 0xd8,
            0xda, 0xc8, 0xee, 0x03, 0x1a, 0x4e, 0xd1, 0x71, 0xfd,
        ];
        let pubkey = rust_util_bytes(pubkey.as_ptr(), pubkey.len());
        let mut result = vec![0; 43];
        rust_ethereum_address_from_pubkey(pubkey, unsafe {
            rust_util_cstr_mut(result.as_mut_ptr(), result.len())
        });
        assert_eq!(
            b"0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B\0".to_vec(),
            result
        );
    }
}
