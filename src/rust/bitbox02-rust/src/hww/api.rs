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

mod pb {
    include!("./api/shiftcrypto.bitbox02.rs");
}
mod error;
mod set_device_name;
mod set_password;

use alloc::vec::Vec;

use error::{make_error, Error};
use pb::request::Request;
use pb::response::Response;
use prost::Message;

/// Encodes a protobuf Response message.
fn encode(response: Response) -> Vec<u8> {
    let response = pb::Response {
        response: Some(response),
    };
    let mut out = Vec::<u8>::new();
    response.encode(&mut out).unwrap();
    out
}

/// Returns the field tag number of the request as defined in the .proto file.  This is needed for
/// compatibility with commander_states.c, and needed as long as API calls processed in C use
/// `commmander_states_force_next()`.
fn request_tag(request: &Request) -> u32 {
    use Request::*;
    match request {
        RandomNumber(_) => bitbox02::Request_random_number_tag,
        DeviceName(_) => bitbox02::Request_device_name_tag,
        DeviceLanguage(_) => bitbox02::Request_device_language_tag,
        DeviceInfo(_) => bitbox02::Request_device_info_tag,
        SetPassword(_) => bitbox02::Request_set_password_tag,
        CreateBackup(_) => bitbox02::Request_create_backup_tag,
        ShowMnemonic(_) => bitbox02::Request_show_mnemonic_tag,
        BtcPub(_) => bitbox02::Request_btc_pub_tag,
        BtcSignInit(_) => bitbox02::Request_btc_sign_init_tag,
        BtcSignInput(_) => bitbox02::Request_btc_sign_input_tag,
        BtcSignOutput(_) => bitbox02::Request_btc_sign_output_tag,
        InsertRemoveSdcard(_) => bitbox02::Request_insert_remove_sdcard_tag,
        CheckSdcard(_) => bitbox02::Request_check_sdcard_tag,
        SetMnemonicPassphraseEnabled(_) => bitbox02::Request_set_mnemonic_passphrase_enabled_tag,
        ListBackups(_) => bitbox02::Request_list_backups_tag,
        RestoreBackup(_) => bitbox02::Request_restore_backup_tag,
        PerformAttestation(_) => bitbox02::Request_perform_attestation_tag,
        Reboot(_) => bitbox02::Request_reboot_tag,
        CheckBackup(_) => bitbox02::Request_check_backup_tag,
        Eth(_) => bitbox02::Request_eth_tag,
        Reset(_) => bitbox02::Request_reset_tag,
        RestoreFromMnemonic(_) => bitbox02::Request_restore_from_mnemonic_tag,
        Bitboxbase(_) => bitbox02::Request_bitboxbase_tag,
        Fingerprint(_) => bitbox02::Request_fingerprint_tag,
        Btc(_) => bitbox02::Request_btc_tag,
        ElectrumEncryptionKey(_) => bitbox02::Request_electrum_encryption_key_tag,
    }
}

/// Handle a protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case it should be handled by
/// the C commander.
async fn process_api(request: &Request) -> Option<Result<Response, Error>> {
    match request {
        Request::DeviceName(ref request) => Some(set_device_name::process(request).await),
        Request::SetPassword(ref request) => Some(set_password::process(request).await),
        _ => None,
    }
}

/// Handle a protobuf api call.  API calls not handled by Rust are
/// handled by the C commander, which allows us to use Rust for new
/// api calls and port the old calls step by step.
///
/// `input` is a hww.proto Request message, protobuf encoded.
/// Returns a protobuf encoded hww.proto Response message.
pub async fn process(input: Vec<u8>) -> Vec<u8> {
    let request = match pb::Request::decode(&input[..]) {
        Ok(pb::Request {
            request: Some(request),
        }) => request,
        _ => return encode(make_error(Error::COMMANDER_ERR_INVALID_INPUT)),
    };
    if !bitbox02::commander::states_can_call(request_tag(&request) as u16) {
        return encode(make_error(Error::COMMANDER_ERR_INVALID_STATE));
    }

    // Since we will process the call now, so can clear the 'force next' info.
    // We do this before processing as the api call can potentially define the next api call
    // to be forced.
    bitbox02::commander::states_clear_force_next();

    match process_api(&request).await {
        Some(Ok(response)) => encode(response),
        Some(Err(error)) => encode(make_error(error)),
        // Api call not handled in Rust -> handle it in C.
        None => bitbox02::commander::commander(input),
    }
}
