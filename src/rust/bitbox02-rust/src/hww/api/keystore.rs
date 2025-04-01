// Copyright 2025 Shift Crypto AG
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
use pb::unlock_request_host_info_response::InfoType;

use crate::workflow::{
    status,
    trinary_choice::{choose, TrinaryChoice},
    unlock,
};

use alloc::string::String;

// We check that the passphrase only contains characters that can be entered on the device too, to
// avoid a situation where the user would not be able to type their passphrase into the device,
// e.g. when using a third party wallet app that does not support entering the passphrase on the
// host.
//
// It's also good to not allow all UTF-8 chars for compatiblity with the wider ecosystem in general.
fn is_host_passphrase_valid(passphrase: &str) -> bool {
    let allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 !\"#$%&'()*+,-./:;<=>?^[\\]@_{|}";
    passphrase.len() <= bitbox02::ui::INPUT_STRING_MAX_SIZE
        && passphrase.chars().all(|c| allowed.contains(c))
}

async fn get_mnemonic_passphrase_from_host() -> Result<Option<zeroize::Zeroizing<String>>, Error> {
    let response = crate::hww::next_request(Response::UnlockHostInfo(
        pb::UnlockRequestHostInfoResponse {
            r#type: InfoType::Passphrase as _,
        },
    ))
    .await?;

    let info: &pb::UnlockHostInfoRequest = match &response {
        pb::request::Request::UnlockHostInfo(info) => info,
        _ => return Err(Error::InvalidState),
    };

    Ok(match info.passphrase.as_deref() {
        Some(passphrase) if is_host_passphrase_valid(passphrase) => {
            Some(zeroize::Zeroizing::new(passphrase.into()))
        }
        Some(_) => {
            status::status("Invalid\npassphrase", false).await;
            None
        }
        None => None,
    })
}

async fn get_mnemonic_passphrase(
    supports_host_passphrase: bool,
) -> Result<Option<zeroize::Zeroizing<String>>, Error> {
    if supports_host_passphrase {
        let choice = choose(
            "Where to enter\npassphrase?",
            Some("Device"),
            None,
            Some("Host"),
        )
        .await;

        if choice == TrinaryChoice::TRINARY_CHOICE_RIGHT {
            let mut waiting_component = bitbox02::ui::info_centered_create("Waiting on host...");
            waiting_component.screen_stack_push();
            return get_mnemonic_passphrase_from_host().await;
        }
    }
    Ok(unlock::enter_mnemonic_passphrase().await?)
}

pub async fn process_unlock(request: &pb::UnlockRequest) -> Result<Response, Error> {
    unlock::unlock(|| get_mnemonic_passphrase(request.supports_host_passphrase)).await?;
    Ok(Response::Success(pb::Success {}))
}
