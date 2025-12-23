// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;

use pb::response::Response;

use crate::keystore;

/// Returns the keystore's root fingerprint, which is the first 32
/// bits of the hash160 of the pubkey at the keypath m/.
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#key-identifiers
pub fn process() -> Result<Response, Error> {
    let fingerprint = keystore::root_fingerprint()?;
    Ok(Response::Fingerprint(pb::RootFingerprintResponse {
        fingerprint,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::keystore::testing::mock_unlocked_using_mnemonic;

    #[test]
    fn test_process() {
        keystore::lock();
        assert_eq!(process(), Err(Error::Generic));

        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );
        assert_eq!(
            process(),
            Ok(Response::Fingerprint(pb::RootFingerprintResponse {
                fingerprint: vec![0x02, 0x40, 0xe9, 0x2a],
            }))
        );

        mock_unlocked_using_mnemonic(
            "small agent wife animal marine cloth exit thank stool idea steel frame",
            "",
        );
        assert_eq!(
            process(),
            Ok(Response::Fingerprint(pb::RootFingerprintResponse {
                fingerprint: vec![0xf4, 0x0b, 0x46, 0x9a],
            }))
        );
    }
}
