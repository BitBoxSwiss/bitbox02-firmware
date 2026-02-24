// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::hal::Memory;
use sha2::Digest;
use zeroize::Zeroizing;

/// Creates `SHA256(salt_root || purpose || data)`, where `salt_root` is a persisted value that
/// remains unchanged until the device is reset. The `purpose` string namespaces individual uses of
/// the salt, and the provided `data` slice is hashed alongside it.
///
/// Returns `Err(())` if the salt root cannot be retrieved from persistent storage.
pub fn hash_data(
    memory: &mut impl Memory,
    data: &[u8],
    purpose: &str,
) -> Result<Zeroizing<Vec<u8>>, ()> {
    let salt_root = memory.get_salt_root()?;

    let mut hasher = sha2::Sha256::new();
    hasher.update(salt_root.as_slice());
    hasher.update(purpose.as_bytes());
    hasher.update(data);

    Ok(Zeroizing::new(hasher.finalize().to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use core::convert::TryInto;
    use hex_lit::hex;

    const MOCK_SALT_ROOT: [u8; 32] =
        hex!("0000000000000000111111111111111122222222222222223333333333333333");

    #[test]
    fn test_hash_data() {
        let mut mock_hal = TestingHal::new();
        mock_hal.memory.set_salt_root(&MOCK_SALT_ROOT);

        let data = hex!("001122334455667788");
        let expected = hex!("62db8dcd47ddf8e81809c377ed96643855d3052bb73237100ca81f0f5a7611e6");

        let hash = hash_data(&mut mock_hal.memory, &data, "test purpose").unwrap();
        assert_eq!(hash.as_slice(), &expected);
    }

    #[test]
    fn test_hash_data_empty_inputs() {
        let mut mock_hal = TestingHal::new();
        mock_hal.memory.set_salt_root(&MOCK_SALT_ROOT);

        let expected = hex!("2dbb05dd73d94edba6946611aaca367f76c809e96f20499ad674e596050f9833");

        let hash = hash_data(&mut mock_hal.memory, &[], "").unwrap();
        assert_eq!(hash.as_slice(), &expected);
    }
}
