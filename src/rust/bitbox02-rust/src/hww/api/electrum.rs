// Copyright 2020 Shift Crypto AG
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

use super::error::{Error, ErrorKind};
use super::pb;

use pb::response::Response;

use bitbox02::keystore;

const HARDENED: u32 = 0x80000000;
const ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE: u32 = 4541509 + HARDENED;
const ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_TWO: u32 = 1112098098 + HARDENED;

/// Returns the electrum wallet encryption xpub.
/// `keypath` currently needs to be m/4541509'/1112098098'
/// Note: the result of this is only meant to be used for encryption by Electrum.
/// The resulting xpub must not be used to derive addresses or to receive coins.
pub async fn process(
    pb::ElectrumEncryptionKeyRequest { keypath }: &pb::ElectrumEncryptionKeyRequest,
) -> Result<Response, Error> {
    let expected_keypath = [
        ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE,
        ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_TWO,
    ];
    if *keypath != expected_keypath {
        return Err(Error {
            msg: Some("invalid keypath".into()),
            kind: ErrorKind::InvalidInput,
        });
    }
    let xpub = keystore::encode_xpub_at_keypath(keypath, keystore::xpub_type_t::XPUB)
        .map_err(Error::err_invalid_input)?;

    Ok(Response::ElectrumEncryptionKey(
        pb::ElectrumEncryptionKeyResponse { key: xpub },
    ))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock_unlocked, MUTEX};
    use std::boxed::Box;

    #[test]
    pub fn test_process() {
        let _guard = MUTEX.lock().unwrap();

        mock_unlocked();

        // All good.
        assert_eq!(
            block_on(process(&pb::ElectrumEncryptionKeyRequest {
                keypath: vec![
                    ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE,
                    ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_TWO
                ]
            })),
            Ok(Response::ElectrumEncryptionKey(
                pb::ElectrumEncryptionKeyResponse {
                    key: "xpub6AWqZzUWTTxAzVFXAavh7oX2apTkQAnjX9FU5pUMMjHiFzHLGLVWx9tAVvocV8c2WeoL7sUj2gZmdp3rDWaqmugZdSCYQVHCxCsVajQP7Cx".into()
                },
            ))
        );

        // Invalid keypath.
        assert_eq!(
            block_on(process(&pb::ElectrumEncryptionKeyRequest {
                keypath: vec![ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE, 0]
            }))
            .unwrap_err()
            .kind,
            ErrorKind::InvalidInput,
        );

        // Invalid keypath (wrong length).
        assert_eq!(
            block_on(process(&pb::ElectrumEncryptionKeyRequest {
                keypath: vec![
                    ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE,
                    ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_TWO,
                    0
                ]
            }))
            .unwrap_err()
            .kind,
            ErrorKind::InvalidInput,
        );
    }
}
