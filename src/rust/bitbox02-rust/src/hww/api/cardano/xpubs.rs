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

use alloc::vec::Vec;

use pb::cardano_response::Response;

use super::keypath::validate_account_shelley;

/// Return the xpub at the request keypath.
///
/// 64 bytes: 32 bytes public key + 32 bytes chain code.
pub fn process(request: &pb::CardanoXpubsRequest) -> Result<Response, Error> {
    let mut xpubs: Vec<Vec<u8>> = Vec::with_capacity(request.keypaths.len());
    for pb::Keypath { keypath } in &request.keypaths {
        validate_account_shelley(keypath)?;

        let xpub = crate::keystore::ed25519::get_xpub(keypath)?;
        let mut xpub_bytes = Vec::with_capacity(64);
        xpub_bytes.extend_from_slice(xpub.pubkey_bytes());
        xpub_bytes.extend_from_slice(xpub.chain_code());
        xpubs.push(xpub_bytes);
    }
    Ok(Response::Xpubs(pb::CardanoXpubsResponse { xpubs }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox02::testing::mock_unlocked;
    use util::bip32::HARDENED;

    #[test]
    fn test_process() {
        bitbox02::keystore::lock();
        assert_eq!(
            process(&pb::CardanoXpubsRequest { keypaths: vec![] }),
            Ok(Response::Xpubs(pb::CardanoXpubsResponse { xpubs: vec![] })),
        );

        // Locked.
        assert_eq!(
            process(&pb::CardanoXpubsRequest {
                keypaths: vec![pb::Keypath {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED]
                }],
            }),
            Err(Error::Generic),
        );

        mock_unlocked();
        assert_eq!(
            process(&pb::CardanoXpubsRequest {
                keypaths: vec![
                    pb::Keypath {
                        keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED]
                    },
                    pb::Keypath {
                        keypath: vec![1852 + HARDENED, 1815 + HARDENED, 1 + HARDENED]
                    }
                ],
            }),
            Ok(Response::Xpubs(pb::CardanoXpubsResponse {
                xpubs: vec![
                    vec![
                        135, 93, 21, 165, 177, 234, 235, 114, 75, 217, 61, 109, 54, 203, 75, 97,
                        188, 69, 219, 186, 120, 219, 156, 176, 139, 147, 231, 40, 146, 89, 211,
                        216, 174, 223, 100, 1, 197, 31, 45, 152, 27, 1, 127, 215, 4, 53, 226, 217,
                        223, 160, 215, 78, 124, 206, 75, 146, 6, 29, 251, 8, 139, 95, 8, 206
                    ],
                    vec![
                        205, 217, 152, 187, 63, 149, 35, 26, 115, 72, 234, 223, 192, 248, 151, 77,
                        20, 221, 211, 158, 71, 189, 60, 40, 26, 217, 150, 150, 122, 49, 129, 126,
                        93, 199, 240, 91, 226, 212, 218, 106, 29, 25, 36, 178, 129, 146, 0, 184,
                        113, 4, 22, 225, 46, 250, 1, 192, 77, 21, 220, 167, 234, 215, 191, 233
                    ]
                ],
            })),
        );

        // Invalid keypaths
        let invalid_keypaths: &[&[u32]] = &[
            // Invalid purpose
            &[1851 + HARDENED, 1815 + HARDENED, HARDENED],
            // Invalid coin
            &[1852 + HARDENED, 1814 + HARDENED, HARDENED],
            // Account too low
            &[1852 + HARDENED, 1815 + HARDENED, 0],
            &[1852 + HARDENED, 1815 + HARDENED, HARDENED - 1],
            // Account too high
            &[1852 + HARDENED, 1815 + HARDENED, 100 + HARDENED],
            // Wrong number of elements (too short)
            &[1852 + HARDENED, 1815 + HARDENED],
            // Wrong number of elements (too long)
            &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0],
        ];
        for invalid_keypath in invalid_keypaths {
            assert_eq!(
                process(&pb::CardanoXpubsRequest {
                    keypaths: vec![pb::Keypath {
                        keypath: invalid_keypath.to_vec(),
                    },],
                }),
                Err(Error::InvalidInput),
            );
        }
    }
}
