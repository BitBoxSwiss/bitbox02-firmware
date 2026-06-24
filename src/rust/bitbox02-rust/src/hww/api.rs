// SPDX-License-Identifier: Apache-2.0

use crate::pb;

pub(super) mod error;

#[cfg(feature = "app-ethereum")]
pub mod ethereum;

#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
pub mod bitcoin;
#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
pub(crate) mod payment_request;

#[cfg(feature = "app-cardano")]
mod cardano;

mod backup;
mod bip85;
mod bluetooth;
mod change_password;
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

use alloc::{boxed::Box, vec::Vec};
use core::{future::Future, pin::Pin};

use error::{Error, make_error};
use pb::request::Request;
use pb::response::Response;
use prost::Message;

use crate::hal::{Memory, Sd};

type ResponseFuture<'a> = Pin<Box<dyn Future<Output = Result<Response, Error>> + 'a>>;

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
fn process_api_btc<'a, H>(hal: &'a mut H, request: &'a Request) -> ResponseFuture<'a>
where
    H: crate::hal::Hal + 'a,
{
    match request {
        Request::BtcPub(request) => Box::pin(bitcoin::process_pub(hal, request)),
        Request::BtcSignInit(request) => Box::pin(bitcoin::signtx::process(hal, request)),
        Request::Btc(pb::BtcRequest {
            request: Some(request),
        }) => Box::pin(async move {
            bitcoin::process_api(hal, request)
                .await
                .map(|r| Response::Btc(pb::BtcResponse { response: Some(r) }))
        }),
        _ => Box::pin(async { Err(Error::Generic) }),
    }
}

#[cfg(not(any(feature = "app-bitcoin", feature = "app-litecoin")))]
fn process_api_btc<'a, H>(_hal: &'a mut H, _request: &'a Request) -> ResponseFuture<'a>
where
    H: crate::hal::Hal + 'a,
{
    Box::pin(async { Err(Error::Disabled) })
}

/// Checks if the device is ready to accept/handle an api endpoint.
fn can_call(hal: &mut impl crate::hal::Hal, request: &Request) -> bool {
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
    let state: State = if hal.memory().is_initialized() {
        if crate::keystore::is_locked() {
            State::InitializedAndLocked
        } else {
            State::InitializedAndUnlocked
        }
    } else if hal.memory().is_seeded() {
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
        | Request::ListBackups(_)
        | Request::Bluetooth(_) => matches!(
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
        | Request::Bip85(_)
        | Request::ChangePassword(_) => {
            matches!(state, State::InitializedAndUnlocked)
        }
        // These are streamed asynchronously using the `next_request()` primitive in
        // bitcoin/signtx.rs and are not handled directly.
        Request::BtcSignInput(_) | Request::BtcSignOutput(_) => false,
    }
}

/// Handle a protobuf api call.
fn process_api<'a, H>(hal: &'a mut H, request: &'a Request) -> ResponseFuture<'a>
where
    H: crate::hal::Hal + 'a,
{
    match request {
        Request::Reboot(request) => Box::pin(system::reboot_to_bootloader(hal, request)),
        Request::DeviceInfo(_) => Box::pin(device_info::process(hal)),
        Request::DeviceName(request) => Box::pin(set_device_name::process(hal, request)),
        Request::SetPassword(request) => Box::pin(set_password::process(hal, request)),
        Request::ChangePassword(_) => Box::pin(change_password::process(hal)),
        Request::Reset(_) => Box::pin(reset::process(hal)),
        Request::SetMnemonicPassphraseEnabled(request) => {
            Box::pin(set_mnemonic_passphrase_enabled::process(hal, request))
        }
        Request::InsertRemoveSdcard(request) => Box::pin(sdcard::process(hal, request)),
        Request::ListBackups(_) => Box::pin(backup::list(hal)),
        Request::CheckSdcard(_) => Box::pin(async move {
            Ok(Response::CheckSdcard(pb::CheckSdCardResponse {
                inserted: hal.sd().sdcard_inserted().await,
            }))
        }),
        Request::CheckBackup(request) => Box::pin(backup::check(hal, request)),
        Request::CreateBackup(request) => Box::pin(backup::create(hal, request)),
        Request::RestoreBackup(request) => Box::pin(restore::from_file(hal, request)),
        Request::ShowMnemonic(_) => Box::pin(show_mnemonic::process(hal)),
        Request::RestoreFromMnemonic(request) => Box::pin(restore::from_mnemonic(hal, request)),
        Request::ElectrumEncryptionKey(request) => Box::pin(electrum::process(hal, request)),

        #[cfg(feature = "app-ethereum")]
        Request::Eth(pb::EthRequest {
            request: Some(request),
        }) => Box::pin(async move {
            ethereum::process_api(hal, request)
                .await
                .map(|r| Response::Eth(pb::EthResponse { response: Some(r) }))
        }),
        #[cfg(not(feature = "app-ethereum"))]
        Request::Eth(_) => Box::pin(async { Err(Error::Disabled) }),

        Request::Fingerprint(pb::RootFingerprintRequest {}) => {
            Box::pin(async { rootfingerprint::process() })
        }
        request @ Request::BtcPub(_)
        | request @ Request::Btc(_)
        | request @ Request::BtcSignInit(_) => process_api_btc(hal, request),

        #[cfg(feature = "app-cardano")]
        Request::Cardano(pb::CardanoRequest {
            request: Some(request),
        }) => Box::pin(async move {
            cardano::process_api(hal, request)
                .await
                .map(|r| Response::Cardano(pb::CardanoResponse { response: Some(r) }))
        }),
        #[cfg(not(feature = "app-cardano"))]
        Request::Cardano(_) => Box::pin(async { Err(Error::Disabled) }),
        Request::Bip85(request) => Box::pin(bip85::process(hal, request)),
        Request::Bluetooth(pb::BluetoothRequest {
            request: Some(request),
        }) => Box::pin(async move {
            bluetooth::process_api(hal, request)
                .await
                .map(|r| Response::Bluetooth(pb::BluetoothResponse { response: Some(r) }))
        }),
        _ => Box::pin(async { Err(Error::InvalidInput) }),
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
    if !can_call(hal, &request) {
        return encode(make_error(Error::InvalidState));
    }

    match process_api(hal, &request).await {
        Ok(response) => encode(response),
        Err(error) => encode(make_error(error)),
    }
}
