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

use crate::pb;

pub(super) mod error;

#[cfg(feature = "app-ethereum")]
mod ethereum;

#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
pub mod bitcoin;

#[cfg(feature = "app-cardano")]
mod cardano;

mod backup;
mod bip85;
mod device_info;
mod electrum;
mod reset;
mod restore;
mod rootfingerprint;
mod sdcard;
mod set_device_name;
mod set_mnemonic_passphrase_enabled;
mod set_password;
mod show_mnemonic;
mod system;

use alloc::vec::Vec;

use error::{make_error, Error};
use pb::request::Request;
use pb::response::Response;
use prost::Message;

/// Encodes a protobuf Response message.
pub fn encode(response: Response) -> Vec<u8> {
    let response = pb::Response {
        response: Some(response),
    };
    response.encode_to_vec()
}

/// Decodes a protofbuf Request message.
pub fn decode(input: &[u8]) -> Result<Request, Error> {
    match pb::Request::decode(input) {
        Ok(pb::Request {
            request: Some(request),
        }) => Ok(request),
        _ => Err(Error::InvalidInput),
    }
}

#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
async fn process_api_btc(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    match request {
        Request::BtcPub(ref request) => bitcoin::process_pub(hal, request).await,
        Request::BtcSignInit(ref request) => bitcoin::signtx::process(hal, request).await,
        Request::Btc(pb::BtcRequest {
            request: Some(request),
        }) => bitcoin::process_api(hal, request)
            .await
            .map(|r| Response::Btc(pb::BtcResponse { response: Some(r) })),
        _ => Err(Error::Generic),
    }
}

#[cfg(not(any(feature = "app-bitcoin", feature = "app-litecoin")))]
async fn process_api_btc(
    _hal: &mut impl crate::hal::Hal,
    _request: &Request,
) -> Result<Response, Error> {
    Err(Error::Disabled)
}

/// Checks if the device is ready to accept/handle an api endpoint.
fn can_call(request: &Request) -> bool {
    // We have four main states:
    // Creating a wallet on an uninitialized device goes from Uninitialized to Seeded, and when the
    // backup is created to `Initialized*`.
    // Restoring a backup skips the seeded state and goes straight to `Initialized*`.
    // Each state has a set of valid api calls associated.
    enum State {
        // Uninitialized (reset).
        Uninitialized,
        // Seeded (password defined, seed created/loaded).
        Seeded,
        // InitializedAndLocked (seed backuped up on SD card, keystore locked).
        InitializedAndLocked,
        // InitializedAndUnlocked (seed backuped up on SD card, keystore unlocked).
        InitializedAndUnlocked,
    }
    let state: State = if bitbox02::memory::is_initialized() {
        if bitbox02::keystore::is_locked() {
            State::InitializedAndLocked
        } else {
            State::InitializedAndUnlocked
        }
    } else if bitbox02::memory::is_seeded() {
        State::Seeded
    } else {
        State::Uninitialized
    };

    match request {
        // Deprecated call, last used in v1.0.0.
        Request::PerformAttestation(_) => false,
        Request::DeviceInfo(_)
        | Request::Reboot(_)
        | Request::DeviceName(_)
        | Request::DeviceLanguage(_)
        | Request::CheckSdcard(_)
        | Request::InsertRemoveSdcard(_)
        | Request::ListBackups(_) => matches!(
            state,
            State::Uninitialized | State::Seeded | State::InitializedAndUnlocked
        ),
        Request::SetPassword(_) | Request::RestoreBackup(_) | Request::RestoreFromMnemonic(_) => {
            matches!(state, State::Uninitialized | State::Seeded)
        }
        Request::CreateBackup(_) | Request::ShowMnemonic(_) => {
            matches!(state, State::Seeded | State::InitializedAndUnlocked)
        }
        Request::Fingerprint(_)
        | Request::ElectrumEncryptionKey(_)
        | Request::BtcPub(_)
        | Request::Btc(_)
        | Request::BtcSignInit(_)
        | Request::CheckBackup(_)
        | Request::SetMnemonicPassphraseEnabled(_)
        | Request::Eth(_)
        | Request::Reset(_)
        | Request::Cardano(_)
        | Request::Bip85(_) => {
            matches!(state, State::InitializedAndUnlocked)
        }
        // These are streamed asynchronously using the `next_request()` primitive in
        // bitcoin/signtx.rs and are not handled directly.
        Request::BtcSignInput(_) | Request::BtcSignOutput(_) => false,
    }
}

/// Handle a protobuf api call.
async fn process_api(hal: &mut impl crate::hal::Hal, request: &Request) -> Result<Response, Error> {
    match request {
        Request::Reboot(ref request) => system::reboot(hal, request).await,
        Request::DeviceInfo(_) => device_info::process(),
        Request::DeviceName(ref request) => set_device_name::process(hal, request).await,
        Request::SetPassword(ref request) => set_password::process(hal, request).await,
        Request::Reset(_) => reset::process(hal).await,
        Request::SetMnemonicPassphraseEnabled(ref request) => {
            set_mnemonic_passphrase_enabled::process(hal, request).await
        }
        Request::InsertRemoveSdcard(ref request) => sdcard::process(hal, request).await,
        Request::ListBackups(_) => backup::list(),
        Request::CheckSdcard(_) => Ok(Response::CheckSdcard(pb::CheckSdCardResponse {
            inserted: bitbox02::sd::sdcard_inserted(),
        })),
        Request::CheckBackup(ref request) => backup::check(hal, request).await,
        Request::CreateBackup(ref request) => backup::create(hal, request).await,
        Request::RestoreBackup(ref request) => restore::from_file(hal, request).await,
        Request::ShowMnemonic(_) => show_mnemonic::process(hal).await,
        Request::RestoreFromMnemonic(ref request) => restore::from_mnemonic(hal, request).await,
        Request::ElectrumEncryptionKey(ref request) => electrum::process(request).await,

        #[cfg(feature = "app-ethereum")]
        Request::Eth(pb::EthRequest {
            request: Some(ref request),
        }) => ethereum::process_api(hal, request)
            .await
            .map(|r| Response::Eth(pb::EthResponse { response: Some(r) })),
        #[cfg(not(feature = "app-ethereum"))]
        Request::Eth(_) => Err(Error::Disabled),

        Request::Fingerprint(pb::RootFingerprintRequest {}) => rootfingerprint::process(),
        request @ Request::BtcPub(_)
        | request @ Request::Btc(_)
        | request @ Request::BtcSignInit(_) => process_api_btc(hal, request).await,

        #[cfg(feature = "app-cardano")]
        Request::Cardano(pb::CardanoRequest {
            request: Some(ref request),
        }) => cardano::process_api(hal, request)
            .await
            .map(|r| Response::Cardano(pb::CardanoResponse { response: Some(r) })),
        #[cfg(not(feature = "app-cardano"))]
        Request::Cardano(_) => Err(Error::Disabled),
        Request::Bip85(ref request) => bip85::process(hal, request).await,
        _ => Err(Error::InvalidInput),
    }
}

/// Handle a protobuf api call.
///
/// `input` is a hww.proto Request message, protobuf encoded.
/// Returns a protobuf encoded hww.proto Response message.
pub async fn process(hal: &mut impl crate::hal::Hal, input: Vec<u8>) -> Vec<u8> {
    let request = match decode(&input[..]) {
        Ok(request) => request,
        Err(err) => return encode(make_error(err)),
    };
    if !can_call(&request) {
        return encode(make_error(Error::InvalidState));
    }

    match process_api(hal, &request).await {
        Ok(response) => encode(response),
        Err(error) => encode(make_error(error)),
    }
}
