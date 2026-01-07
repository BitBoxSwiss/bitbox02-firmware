// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ffi::c_char;

use crate::hal::Memory;
use sha2::Digest;
use util::bytes::{Bytes, BytesMut};
use zeroize::Zeroizing;

/// Creates `SHA256(salt_root || purpose || data)`, where `salt_root` is a persisted value that
/// remains unchanged until the device is reset. The `purpose` string namespaces individual uses of
/// the salt, and the provided `data` slice is hashed alongside it.
///
/// Returns `Err(())` if the salt root cannot be retrieved from persistent storage.
pub fn hash_data(
    hal: &mut impl crate::hal::Hal,
    data: &[u8],
    purpose: &str,
) -> Result<Zeroizing<Vec<u8>>, ()> {
    let salt_root = hal.memory().get_salt_root()?;

    let mut hasher = sha2::Sha256::new();
    hasher.update(salt_root.as_slice());
    hasher.update(purpose.as_bytes());
    hasher.update(data);

    Ok(Zeroizing::new(hasher.finalize().to_vec()))
}

/// # Safety
///
/// `purpose` must be a valid, null-terminated UTF-8 string pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_salt_hash_data(
    data: Bytes,
    purpose: *const c_char,
    mut hash_out: BytesMut,
) -> bool {
    let purpose_str = match unsafe { util::strings::str_from_null_terminated_ptr(purpose) } {
        Ok(purpose) => purpose,
        Err(()) => return false,
    };
    let mut hal = crate::hal::BitBox02Hal::new();
    match hash_data(&mut hal, data.as_ref(), purpose_str) {
        Ok(hash) => {
            hash_out.as_mut()[..32].copy_from_slice(&hash);
            true
        }
        Err(()) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use bitbox02::{memory, testing::mock_memory};
    use core::convert::TryInto;
    use core::ptr;
    use hex_lit::hex;

    const MOCK_SALT_ROOT: [u8; 32] =
        hex!("0000000000000000111111111111111122222222222222223333333333333333");

    #[test]
    fn test_hash_data() {
        mock_memory();
        let mut mock_hal = TestingHal::new();
        mock_hal.memory.set_salt_root(&MOCK_SALT_ROOT);

        let data = hex!("001122334455667788");
        let expected = hex!("62db8dcd47ddf8e81809c377ed96643855d3052bb73237100ca81f0f5a7611e6");

        let hash = hash_data(&mut mock_hal, &data, "test purpose").unwrap();
        assert_eq!(hash.as_slice(), &expected);
    }

    #[test]
    fn test_hash_data_empty_inputs() {
        mock_memory();
        let mut mock_hal = TestingHal::new();
        mock_hal.memory.set_salt_root(&MOCK_SALT_ROOT);

        let expected = hex!("2dbb05dd73d94edba6946611aaca367f76c809e96f20499ad674e596050f9833");

        let hash = hash_data(&mut mock_hal, &[], "").unwrap();
        assert_eq!(hash.as_slice(), &expected);
    }

    #[test]
    fn test_rust_salt_hash_data() {
        mock_memory();
        memory::set_salt_root(&MOCK_SALT_ROOT).unwrap();

        let data = hex!("001122334455667788");
        let expected = hex!("62db8dcd47ddf8e81809c377ed96643855d3052bb73237100ca81f0f5a7611e6");

        let mut hash_out = [0u8; 32];
        let purpose = c"test purpose";
        assert!(unsafe {
            rust_salt_hash_data(
                util::bytes::rust_util_bytes(data.as_ptr(), data.len()),
                purpose.as_ptr(),
                util::bytes::rust_util_bytes_mut(hash_out.as_mut_ptr(), hash_out.len()),
            )
        });
        assert_eq!(hash_out, expected);
    }

    #[test]
    fn test_rust_salt_hash_data_empty_inputs() {
        mock_memory();
        memory::set_salt_root(&MOCK_SALT_ROOT).unwrap();

        let expected = hex!("2dbb05dd73d94edba6946611aaca367f76c809e96f20499ad674e596050f9833");
        let mut hash_out = [0u8; 32];
        assert!(unsafe {
            rust_salt_hash_data(
                util::bytes::rust_util_bytes(ptr::null(), 0),
                c"".as_ptr(),
                util::bytes::rust_util_bytes_mut(hash_out.as_mut_ptr(), hash_out.len()),
            )
        });
        assert_eq!(hash_out, expected);
    }
}
