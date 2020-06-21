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

//use bitbox02_rust::commander::bitboxbase::{
//    confirm_pairing, display_status, heartbeat, set_config,
//};

// A trick to convince cbindgen that an u8 is char.
// cbindgen will convert `u8` to `uint8_t` and `i8` to `int8_t` which are `unsigned char` and
// `signed char` respectively. `c_char` is converted to `char` without `signed` or `unsigned`.
#[allow(non_camel_case_types)]
type c_char = u8;

use core::time::Duration;

use bitbox02_rust::platform::bitboxbase::config::Config;
use bitbox02_rust::platform::bitboxbase::config::StatusLedMode;
use bitbox02_rust::platform::bitboxbase::display::write_status;
use bitbox02_rust::platform::bitboxbase::state::{
    BitBoxBaseBackgroundDescription, BitBoxBaseBackgroundState, State, DESCRIPTIONS,
};
use bitbox02_rust::util::FixedCString;
use bitbox02_rust::util::Ipv4Addr;
use bitbox02_rust::workflow::pairing;
use core::fmt::Write;

pub fn confirm_pairing(
    request: &bitbox02::BitBoxBaseConfirmPairingRequest,
) -> bitbox02::CommanderError {
    let bytes = &request.msg[..];
    bitbox02::leds_turn_small_led(0, true);
    bitbox02::leds_turn_small_led(4, true);
    bitbox02::leds_turn_big_led(0, None);
    bitbox02::leds_turn_big_led(1, None);

    bitbox02::bitboxbase_screensaver_reset();
    let res = {
        // Extra hash in pairing hash for the BitBoxBase.
        let mut buf = [0u8; 32];
        bitbox02::sha256(bytes, &mut buf).expect("sha256 failed");
        bitbox02_rust::bb02_async::block_on(pairing::confirm(&buf))
    };
    bitbox02::leds_turn_small_led(0, false);
    bitbox02::leds_turn_small_led(4, false);
    match res {
        true => bitbox02::COMMANDER_OK,
        false => bitbox02::COMMANDER_ERR_USER_ABORT,
    }
}

pub fn heartbeat(request: &bitbox02::BitBoxBaseHeartbeatRequest) -> bitbox02::CommanderError {
    // Accessing a mutable static is unsafe
    let state = unsafe { &mut STATE };
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

pub fn set_config(request: &bitbox02::BitBoxBaseSetConfigRequest) -> bitbox02::CommanderError {
    // Accessing a mutable static is unsafe
    let config = unsafe { &mut CONFIG };
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

pub fn display_status(
    request: &bitbox02::BitBoxBaseDisplayStatusRequest,
) -> bitbox02::CommanderError {
    let duration = request.duration as u64;
    let duration = if duration > 0 {
        Some(Duration::from_millis(duration))
    } else {
        None
    };
    // Accessing a mutable static is unsafe
    bitbox02::bitboxbase_screensaver_reset();
    bitbox02_rust::platform::bitboxbase::display::display_status(unsafe { &CONFIG }, duration);
    bitbox02::COMMANDER_OK
}

/// # Safety
/// request must be not NULL.
#[no_mangle]
pub unsafe extern "C" fn commander_bitboxbase(
    request: *const bitbox02::BitBoxBaseRequest,
) -> bitbox02::commander_error_t {
    // Accessing raw pointers are unsafe
    let request = match request.as_ref() {
        Some(request) => request,
        None => return bitbox02::COMMANDER_ERR_GENERIC,
    };
    // It is unsafe to access C style unions
    match request.which_request {
        bitbox02::BitBoxBaseRequest_confirm_pairing_tag => {
            confirm_pairing(&request.request.confirm_pairing)
        }
        bitbox02::BitBoxBaseRequest_heartbeat_tag => heartbeat(&request.request.heartbeat),
        bitbox02::BitBoxBaseRequest_display_status_tag => {
            display_status(&request.request.display_status)
        }
        bitbox02::BitBoxBaseRequest_set_config_tag => set_config(&request.request.set_config),
        _ => bitbox02::COMMANDER_ERR_GENERIC,
    }
}

// aaaah, global!
pub static mut CONFIG: Config = Config::new();

#[no_mangle]
pub extern "C" fn bitboxbase_config_led_mode_get() -> StatusLedMode {
    let config = unsafe { &CONFIG };
    config.status_led_mode.clone()
}

/// # Safety
/// `res` must be NOT NULL and of size at least `res_len`.
#[no_mangle]
pub unsafe extern "C" fn bitboxbase_config_ip_get(res: *mut c_char, res_len: usize) {
    // It is not safe to call any functions that also touch CONFIG at the same time
    let config = &CONFIG;
    let buf = core::slice::from_raw_parts_mut(res, res_len);
    let mut fcstring = FixedCString::new(buf);

    if let Some(ip) = &config.ip {
        let _ = write!(fcstring, "{}", ip);
    } else {
        let _ = write!(fcstring, "unknown");
    }
}

pub static mut STATE: State = State {
    state: BitBoxBaseBackgroundState::BBBWaiting,
    description_code: BitBoxBaseBackgroundDescription::Empty,
};

#[no_mangle]
pub extern "C" fn bitboxbase_state_set_not_alive() {
    let state = unsafe { &mut STATE };
    if state.state != BitBoxBaseBackgroundState::BBBNotAlive {
        (*state).state = BitBoxBaseBackgroundState::BBBNotAlive;
        bitbox02::bitboxbase_screensaver_reset();
    }
}

#[no_mangle]
pub extern "C" fn bitboxbase_state_get() -> BitBoxBaseBackgroundState {
    let state = unsafe { &STATE };
    state.state
}

/// # Safety
/// `buf` must be not NULL and be of size at least `buf_len`.
#[no_mangle]
pub unsafe extern "C" fn bitboxbase_state_get_description(buf: *mut c_char, buf_len: usize) {
    assert!(!buf.is_null());
    let state = &STATE;
    let buf = core::slice::from_raw_parts_mut(buf, buf_len);
    let mut buf = FixedCString::new(buf);
    let _ = write!(
        buf,
        "{}",
        DESCRIPTIONS
            .get(state.description_code as usize)
            .unwrap_or(&"<Unknown>")
    );
}

/// # Safety
/// `ptr` must be not NULL and of size at least `ptr_len`.
#[no_mangle]
pub unsafe extern "C" fn bitboxbase_status_get(ptr: *mut c_char, ptr_len: usize) {
    let buf = core::slice::from_raw_parts_mut(ptr, ptr_len);
    let mut wrapper = FixedCString::new(buf);
    write_status(&mut wrapper, &CONFIG);
}
