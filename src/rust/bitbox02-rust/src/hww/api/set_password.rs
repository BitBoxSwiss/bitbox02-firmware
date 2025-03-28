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

use super::Error;
use crate::pb;

use crate::workflow::{password, status, unlock};
use bitbox02::keystore;
use pb::response::Response;

/// Handles the SetPassword api call. This has the user enter a password twice and creates the
/// seed/keystore. After this call is finished, the keystore is fully unlocked.
///
/// `entropy` must be exactly 16 or 32 bytes and provides additional entropy used when creating the
/// seed. If 16 bytes are provided, the seed will also be 16 bytes long, corresponding to 12 BIP39
/// recovery words. If 32 bytes are provided, the seed will also be 32 bytes long, corresponding to
/// 24 BIP39 recovery words.
pub async fn process(
    pb::SetPasswordRequest { entropy }: &pb::SetPasswordRequest,
) -> Result<Response, Error> {
    if entropy.len() != 16 && entropy.len() != 32 {
        return Err(Error::InvalidInput);
    }
    let password = password::enter_twice().await?;
    if let Err(err) = keystore::create_and_store_seed(&password, entropy) {
        status::status(&format!("Error\n{:?}", err), false).await;
        return Err(Error::Generic);
    }
    if keystore::unlock(&password).is_err() {
        panic!("Unexpected error during restore: unlock failed.");
    }
    unlock::unlock_bip39(unlock::enter_mnemonic_passphrase).await?;
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, mock_memory, Data};

    use alloc::boxed::Box;

    #[test]
    fn test_process() {
        mock_memory();
        mock(Data {
            ui_trinary_input_string_create: Some(Box::new(|_params| "password".into())),
            ..Default::default()
        });
        assert!(keystore::is_locked());
        assert_eq!(
            block_on(process(&pb::SetPasswordRequest {
                entropy: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
            })),
            Ok(Response::Success(pb::Success {}))
        );
        assert!(!keystore::is_locked());
        assert!(keystore::copy_seed().unwrap().len() == 32);
    }

    /// Shorter host entropy results in shorter seed.
    #[test]
    fn test_process_16_bytes() {
        mock_memory();
        mock(Data {
            ui_trinary_input_string_create: Some(Box::new(|_params| "password".into())),
            ..Default::default()
        });
        assert!(keystore::is_locked());
        assert_eq!(
            block_on(process(&pb::SetPasswordRequest {
                entropy: b"aaaaaaaaaaaaaaaa".to_vec(),
            })),
            Ok(Response::Success(pb::Success {}))
        );
        assert!(!keystore::is_locked());
        assert!(keystore::copy_seed().unwrap().len() == 16);
    }

    /// Invalid host entropy size.
    #[test]
    fn test_process_invalid_host_entropy() {
        mock_memory();
        mock(Data {
            ui_trinary_input_string_create: Some(Box::new(|_params| "password".into())),
            ..Default::default()
        });
        assert!(keystore::is_locked());
        assert_eq!(
            block_on(process(&pb::SetPasswordRequest {
                entropy: b"aaaaaaaaaaaaaaaaa".to_vec(),
            })),
            Err(Error::InvalidInput),
        );
        assert!(keystore::is_locked());
    }

    #[test]
    fn test_process_2nd_password_doesnt_match() {
        mock_memory();
        static mut COUNTER: u32 = 0;
        mock(Data {
            ui_trinary_input_string_create: Some(Box::new(|_params| {
                match unsafe {
                    COUNTER += 1;
                    COUNTER
                } {
                    1 => "password".into(),
                    2 => "wrong".into(),
                    _ => panic!("too many user inputs"),
                }
            })),
            ..Default::default()
        });
        assert!(keystore::is_locked());
        assert_eq!(
            block_on(process(&pb::SetPasswordRequest {
                entropy: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
            })),
            Err(Error::Generic),
        );
        assert!(keystore::is_locked());
    }
}
