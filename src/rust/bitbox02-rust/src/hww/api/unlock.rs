// SPDX-License-Identifier: Apache-2.0

use super::error::Error;
use crate::hal::ui::ConfirmParams;
use crate::hal::{Memory, Ui};
use crate::pb;
use crate::pb::request::Request;
use crate::pb::response::Response;
use crate::pb::unlock_response::State;
use crate::workflow::unlock;
use alloc::string::String;
use core::pin::pin;
use zeroize::Zeroizing;

// Match the BitBox02 keyboard buffer, excluding its terminating NUL.
// Keep in sync with INPUT_STRING_MAX_SIZE - 1 in src/ui/components/trinary_input_string.h.
const MAX_PASSPHRASE_LEN: usize = 149;

// Keep in sync with _special_chars in src/ui/components/trinary_input_string.c.
const SPECIAL: &[u8] = b" !\"#$%&'()*+,-./:;<=>?^[\\]@_{|}";

fn response(state: State) -> Response {
    Response::Unlock(pb::UnlockResponse { state: state as _ })
}

/// Returns None when the host requested entry. The caller can then borrow the UI again
/// after the pinned device-entry future has been dropped. Confirmation starts only after
/// device entry completes and host entry has been withdrawn.
async fn device_entry(hal: &mut impl crate::hal::Hal) -> Result<Option<Zeroizing<String>>, Error> {
    enum Completed {
        Device(Zeroizing<String>),
        Request(Result<pb::UnlockContinueRequest, Error>),
    }

    // Keep the same UI future across continuations; restarting it per poll would erase partial input.
    let mut entry = pin!(unlock::enter_mnemonic_passphrase(hal));
    loop {
        let mut next = pin!(async {
            match crate::hww::next_request(response(State::PassphrasePending)).await? {
                Request::UnlockContinue(request) => Ok(request),
                _ => Err(Error::InvalidState),
            }
        });
        // Device input completion wins a simultaneous click. Borrow both futures so the
        // unfinished one stays alive after the race.
        let completed =
            futures_lite::future::or(async { Completed::Device(entry.as_mut().await) }, async {
                Completed::Request(next.as_mut().await)
            })
            .await;
        match completed {
            Completed::Device(passphrase) => {
                // Never cancel next_request when the UI finishes: its intermediate response must
                // be consumed and the next request received before sending the final response.
                next.await?;
                return Ok(Some(passphrase));
            }
            Completed::Request(request) => {
                if request?.request_host_entry {
                    return Ok(None);
                }
            }
        }
    }
}

fn validate_host_passphrase(passphrase: &str) -> Result<(), &'static str> {
    // Restrict host input to values that can also be entered again on the device.
    if passphrase.len() > MAX_PASSPHRASE_LEN {
        return Err("Passphrase too\nlong");
    }
    if !passphrase
        .bytes()
        .all(|c| c.is_ascii_alphanumeric() || SPECIAL.contains(&c))
    {
        return Err("Unsupported\ncharacters");
    }
    Ok(())
}

async fn passphrase(hal: &mut impl crate::hal::Hal) -> Result<Zeroizing<String>, Error> {
    loop {
        let (passphrase, from_host) = match device_entry(hal).await? {
            Some(passphrase) => {
                // Withdraw host entry before showing either confirmation screen. A queued host
                // click loses to completed input, including on this continuation; it must not
                // interrupt confirmation or reopen the host-entry consent flow.
                let request = crate::hww::next_request(response(State::PassphraseEntered)).await?;
                if !matches!(request, Request::UnlockContinue(_)) {
                    return Err(Error::InvalidState);
                }
                (passphrase, false)
            }
            None => {
                // Entry was interrupted by the host button. Rejection/cancellation always starts a
                // fresh device-entry screen, without preserving a partially entered passphrase.
                if hal
                    .ui()
                    .confirm(&ConfirmParams {
                        title: "",
                        body: "Enter passphrase\non host?",
                        ..Default::default()
                    })
                    .await
                    .is_err()
                {
                    continue;
                }

                let request = {
                    let waiting = async {
                        hal.ui().waiting("Enter passphrase\non host").await;
                        unreachable!()
                    };
                    // Poll the waiting screen first so it is visible before HOST_ENTRY_READY. This
                    // race only cancels the screen: next_request is always consumed to completion.
                    futures_lite::future::or(
                        waiting,
                        crate::hww::next_request(response(State::HostEntryReady)),
                    )
                    .await?
                };
                let Request::UnlockHostInfo(mut request) = request else {
                    return Err(Error::InvalidState);
                };
                let Some(passphrase) = request.passphrase.take().map(Zeroizing::new) else {
                    continue;
                };
                if let Err(message) = validate_host_passphrase(&passphrase) {
                    // Invalid user input restarts entry just like rejection; malformed protocol
                    // messages still fail in next_request.
                    hal.ui().status(message, false).await;
                    continue;
                }
                (passphrase, true)
            }
        };
        if unlock::confirm_mnemonic_passphrase(hal, &passphrase, from_host)
            .await
            .is_ok()
        {
            return Ok(passphrase);
        }
        hal.ui().status("Please try again", false).await;
    }
}

pub async fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    // Let clients call unlock after pairing without first checking initialization. Return DONE
    // without touching setup in progress.
    if !hal.memory().is_initialized() {
        return Ok(response(State::Done));
    }
    // Switch the waiting screen from "See the BitBoxApp" to the logo immediately
    // before unlock, avoiding a logo flash before entry or the waiting text
    // reappearing afterward. Also do this if U2F already unlocked the device.
    hal.ui().switch_to_logo();
    unlock::unlock_with_passphrase(hal, passphrase).await?;
    Ok(response(State::Done))
}

#[cfg(test)]
mod tests;
