// Copyright 2021 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::pb;
use super::Error;

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

    use bitbox02::keystore::lock;
    use bitbox02::testing::mock_unlocked_using_mnemonic;

    #[test]
    fn test_process() {
        lock();
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
