// Copyright 2019 Shift Cryptosecurity AG
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

use bitbox02;
use core::time::Duration;

use crate::platform::bitboxbase::config::Config;
use crate::platform::bitboxbase::display;
use crate::platform::bitboxbase::state::State;
use crate::util::Ipv4Addr;
use crate::workflow::pairing;

fn confirm_pairing(
    request: &bitbox02::BitBoxBaseConfirmPairingRequest,
) -> bitbox02::CommanderError {
    let bytes = &request.msg[..];
    bitbox02::leds_turn_small_led(0, true);
    bitbox02::leds_turn_small_led(4, true);
    bitbox02::leds_turn_big_led(0, None);
    bitbox02::leds_turn_big_led(1, None);

    bitbox02::bitboxbase_screensaver_reset();
    let res = pairing::extra_hash_create(bytes);
    bitbox02::leds_turn_small_led(0, false);
    bitbox02::leds_turn_small_led(4, false);
    match res {
        true => bitbox02::COMMANDER_OK,
        false => bitbox02::COMMANDER_ERR_USER_ABORT,
    }
}

fn heartbeat(request: &bitbox02::BitBoxBaseHeartbeatRequest) -> bitbox02::CommanderError {
    // Accessing a mutable static is unsafe
    let state = unsafe { State::get_singleton_mut() };
    // Transmute uint32 to BitBoxBaseBackgroundState
    // This is safe for values 0 to 3.
    if request.state_code > 3 {
        return bitbox02::COMMANDER_ERR_GENERIC;
    }
    // Transmute uint32 to BitBoxBaseBackgroundDescription
    // This is safe for values 0 to 3.
    if request.description_code > 8 {
        return bitbox02::COMMANDER_ERR_GENERIC;
    }

    let new_state = unsafe { core::mem::transmute(request.state_code + 2) };
    if new_state != state.state {
        bitbox02::bitboxbase_screensaver_reset();
    }
    state.state = new_state;
    let new_description_code = unsafe { core::mem::transmute(request.description_code) };
    if new_description_code != state.description_code {
        bitbox02::bitboxbase_screensaver_reset();
    }
    state.description_code = new_description_code;
    bitbox02::bitboxbase_watchdog_reset();

    bitbox02::COMMANDER_OK
}

fn set_config(request: &bitbox02::BitBoxBaseSetConfigRequest) -> bitbox02::CommanderError {
    // Accessing a mutable static is unsafe
    let config = unsafe { Config::get_singleton_mut() };
    let hostname =
        bitbox02::util::str_from_null_terminated(&request.hostname[..]).expect("Invalid utf-8");
    match &hostname {
        &"" => (),
        hostname => match config.set_hostname(hostname) {
            Err(_) => return bitbox02::COMMANDER_ERR_GENERIC,
            _ => (),
        },
    }

    match request.which_ip_option {
        bitbox02::BitBoxBaseSetConfigRequest_ip_tag => {
            // Accessing c union types is unsafe
            let ip: Ipv4Addr = unsafe { request.ip_option.ip }.into();
            config.set_ip(ip);
        }
        _ => (), // ip not set
    }

    if request.status_led_mode > 3 {
        return bitbox02::COMMANDER_ERR_GENERIC;
    }
    let status_led_mode = unsafe { core::mem::transmute(request.status_led_mode) };
    config.set_status_led_mode(status_led_mode);

    if request.status_screen_mode > 3 {
        return bitbox02::COMMANDER_ERR_GENERIC;
    }
    let status_screen_mode = unsafe { core::mem::transmute(request.status_screen_mode) };
    config.set_status_screen_mode(status_screen_mode);

    bitbox02::COMMANDER_OK
}

fn display_status(request: &bitbox02::BitBoxBaseDisplayStatusRequest) -> bitbox02::CommanderError {
    let duration = request.duration as u64;
    let duration = if duration > 0 {
        Some(Duration::from_millis(duration))
    } else {
        None
    };
    // Accessing a mutable static is unsafe
    bitbox02::bitboxbase_screensaver_reset();
    display::display_status(unsafe { Config::get_singleton() }, duration);
    bitbox02::COMMANDER_OK
}

#[no_mangle]
pub extern "C" fn commander_bitboxbase(
    request: *const bitbox02::BitBoxBaseRequest,
) -> bitbox02::commander_error_t {
    // Accessing raw pointers are unsafe
    let request = match unsafe { request.as_ref() } {
        Some(request) => request,
        None => return bitbox02::COMMANDER_ERR_GENERIC,
    };
    // It is unsafe to access C style unions
    match request.which_request {
        bitbox02::BitBoxBaseRequest_confirm_pairing_tag => {
            confirm_pairing(unsafe { &request.request.confirm_pairing })
        }
        bitbox02::BitBoxBaseRequest_heartbeat_tag => {
            heartbeat(unsafe { &request.request.heartbeat })
        }
        bitbox02::BitBoxBaseRequest_display_status_tag => {
            display_status(unsafe { &request.request.display_status })
        }
        bitbox02::BitBoxBaseRequest_set_config_tag => {
            set_config(unsafe { &request.request.set_config })
        }
        _ => bitbox02::COMMANDER_ERR_GENERIC,
    }
}
