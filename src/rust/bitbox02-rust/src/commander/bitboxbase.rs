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
use crate::platform::bitboxbase::display::display_status;
use crate::platform::bitboxbase::state::State;
use crate::util::Ipv4Addr;
use crate::workflow::pairing;

#[no_mangle]
pub extern "C" fn bitboxbase_workflow_confirm_pairing(bytes: *const u8, bytes_len: usize) -> bool {
    assert!(!bytes.is_null());
    assert!(bytes_len > 0 && bytes_len <= 32);
    let bytes = unsafe { core::slice::from_raw_parts(bytes, bytes_len) };
    bitbox02::leds_turn_small_led(0, true);
    bitbox02::leds_turn_small_led(4, true);
    bitbox02::leds_turn_big_led(0, None);
    bitbox02::leds_turn_big_led(1, None);

    let res = pairing::extra_hash_create(bytes);
    bitbox02::leds_turn_small_led(0, false);
    bitbox02::leds_turn_small_led(4, false);
    res
}

#[no_mangle]
pub extern "C" fn bitboxbase_heartbeat(
    request: *const bitbox02::BitBoxBaseHeartbeatRequest,
) -> bool {
    // Accessing a mutable static is unsafe
    let state = unsafe { State::get_singleton_mut() };
    // Accessing raw pointers are unsafe
    let request = match unsafe { request.as_ref() } {
        Some(request) => request,
        None => return false,
    };
    // Transmute uint32 to BitBoxBaseBackgroundState
    // This is safe for values 0 to 3.
    if request.state_code > 3 {
        return false;
    }
    state.state = unsafe { core::mem::transmute(request.state_code + 2) };
    if request.description_code > 3 {
        return false;
    }
    state.description_code = unsafe { core::mem::transmute(request.description_code) };
    bitbox02::bitboxbase_watchdog_reset();

    true
}

#[no_mangle]
pub extern "C" fn bitboxbase_config_set(
    request: *const bitbox02::BitBoxBaseSetConfigRequest,
) -> bool {
    // Accessing a mutable static is unsafe
    let config = unsafe { Config::get_singleton_mut() };
    // Accessing raw pointers are unsafe
    let request = match unsafe { request.as_ref() } {
        Some(request) => request,
        None => return false,
    };
    let hostname =
        bitbox02::util::str_from_null_terminated(&request.hostname[..]).expect("Invalid utf-8");
    match &hostname {
        &"" => (),
        hostname => match config.set_hostname(hostname) {
            Err(_) => return false,
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
        return false;
    }
    let status_led_mode = unsafe { core::mem::transmute(request.status_led_mode) };
    config.set_status_led_mode(status_led_mode);

    if request.status_screen_mode > 3 {
        return false;
    }
    let status_screen_mode = unsafe { core::mem::transmute(request.status_screen_mode) };
    config.set_status_screen_mode(status_screen_mode);

    true
}

#[no_mangle]
pub extern "C" fn bitboxbase_display_status(duration: u64) {
    let duration = if duration > 0 {
        Some(Duration::from_millis(duration))
    } else {
        None
    };
    // Accessing a mutable static is unsafe
    display_status(unsafe { Config::get_singleton() }, duration);
}
