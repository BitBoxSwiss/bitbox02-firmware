// Copyright 2020 Shift Cryptosecurity AG
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

mod api;
pub mod noise;

extern crate alloc;
use alloc::vec::Vec;

const OP_UNLOCK: u8 = b'u';
const OP_ATTESTATION: u8 = b'a';

const OP_STATUS_SUCCESS: u8 = 0;
const OP_STATUS_FAILURE: u8 = 1;
const OP_STATUS_FAILURE_UNINITIALIZED: u8 = 2;

/// Must be called during the execution of a usb task. This sends out the response to the host and
/// awaits the next request. If the request is not a valid noise encrypted protofbuf api request
/// message, `Err(Error::InvalidInput)` is returned.
#[cfg(not(feature = "testing"))]
pub async fn next_request(
    response: crate::pb::response::Response,
) -> Result<crate::pb::request::Request, api::error::Error> {
    let mut out = [OP_STATUS_SUCCESS].to_vec();
    noise::encrypt(&api::encode(response), &mut out).or(Err(api::error::Error::NoiseEncrypt))?;
    let request = crate::async_usb::next_request(out).await;
    match request.split_first() {
        Some((&noise::OP_NOISE_MSG, encrypted_request)) => {
            let decrypted_request =
                noise::decrypt(encrypted_request).or(Err(api::error::Error::NoiseDecrypt))?;
            api::decode(&decrypted_request[..])
        }
        _ => Err(api::error::Error::InvalidInput),
    }
}

#[cfg(feature = "testing")]
pub struct SafeData<T>(T);
// Safety: must not be accessed concurrently.
#[cfg(feature = "testing")]
unsafe impl<T> Sync for SafeData<T> {}

#[cfg(feature = "testing")]
lazy_static! {
    pub static ref MOCK_NEXT_REQUEST: SafeData<
        core::cell::RefCell<
            Option<
                alloc::boxed::Box<
                    dyn Fn(
                        crate::pb::response::Response,
                    )
                        -> Result<crate::pb::request::Request, api::error::Error>,
                >,
            >,
        >,
    > = SafeData(core::cell::RefCell::new(None));
}

/// Set `MOCK_NEXT_REQUEST` to mock requests from the host.
#[cfg(feature = "testing")]
pub async fn next_request(
    response: crate::pb::response::Response,
) -> Result<crate::pb::request::Request, api::error::Error> {
    let func = MOCK_NEXT_REQUEST.0.borrow();
    func.as_ref().unwrap()(response)
}

/// Process OP_UNLOCK.
async fn api_unlock() -> Vec<u8> {
    match crate::workflow::unlock::unlock().await {
        Ok(()) => [OP_STATUS_SUCCESS].to_vec(),
        Err(()) => [OP_STATUS_FAILURE_UNINITIALIZED].to_vec(),
    }
}

/// Process OP_ATTESTATION.
///
/// On failure, returns < 1 >.
///
/// On success, returns < 0 | bootloader_hash 32 | device_pubkey 64 |
/// certificate 64 | root_pubkey_identifier 32 | challenge_signature 64>
fn api_attestation(usb_in: &[u8]) -> Vec<u8> {
    use core::convert::TryInto;

    let usb_in: [u8; 32] = match usb_in.try_into() {
        Ok(usb_in) => usb_in,
        Err(_) => return [OP_STATUS_FAILURE].to_vec(),
    };

    let result = match crate::attestation::perform(usb_in) {
        Ok(result) => result,
        Err(()) => return [OP_STATUS_FAILURE].to_vec(),
    };

    let mut out = Vec::with_capacity(257);
    out.push(OP_STATUS_SUCCESS);
    out.extend_from_slice(&result.bootloader_hash[..]);
    out.extend_from_slice(&result.device_pubkey[..]);
    out.extend_from_slice(&result.certificate[..]);
    out.extend_from_slice(&result.root_pubkey_identifier[..]);
    out.extend_from_slice(&result.challenge_signature[..]);
    out
}

/// Async HWW api processing main entry point.
/// `usb_in` - api request bytes.
/// Returns the usb response bytes.
pub async fn process_packet(usb_in: Vec<u8>) -> Vec<u8> {
    match usb_in.split_first() {
        Some((&OP_UNLOCK, b"")) => return api_unlock().await,
        Some((&OP_ATTESTATION, rest)) => return api_attestation(rest),
        _ => (),
    }

    // No other message than the attestation and unlock calls shall pass until the device is
    // unlocked or ready to be initialized.
    if bitbox02::memory::is_initialized() && bitbox02::keystore::is_locked() {
        return Vec::new();
    }

    let mut out = [OP_STATUS_SUCCESS].to_vec();
    match noise::process(usb_in, &mut out).await {
        Ok(()) => out,
        Err(noise::Error) => [OP_STATUS_FAILURE].to_vec(),
    }
}
