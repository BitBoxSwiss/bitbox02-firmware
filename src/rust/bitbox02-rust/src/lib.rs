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
#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::time::Duration;

mod bitboxbase;
mod error;
mod general;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_debug!(0, "Internal error: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn bitboxbase_workflow_display_base32(bytes: *const u8, bytes_len: usize) -> bool {
    assert!(!bytes.is_null());
    assert!(bytes_len > 0 && bytes_len <= 32);
    let bytes = unsafe { core::slice::from_raw_parts(bytes, bytes_len) };
    bitboxbase::workflow::display_base32(bytes)
}

static mut CONFIG: bitboxbase::config::Config = bitboxbase::config::Config::new();

// A trick to convince cbindgen that this is a char.
#[allow(non_camel_case_types)]
type c_char = u8;
/// This function is not multithread safe since it modifies a static global.
#[no_mangle]
pub extern "C" fn bitboxbase_config_set(
    status_led_mode: u8,
    status_screen_mode: u8,
    hostname: *const c_char,
    hostname_len: usize,
    default_display_duration: u64,
) -> bool {
    assert!(!hostname.is_null());
    let hostname = unsafe { core::slice::from_raw_parts(hostname, hostname_len) };
    let hostname = core::str::from_utf8(hostname).expect("Invalid utf-8");
    // It is not safe to call any functions that also touch CONFIG at the same time
    let config = unsafe { &mut CONFIG };
    match config.set_hostname(hostname) {
        Err(_) => return false,
        _ => (),
    }
    let status_led_mode = match status_led_mode {
        0 => bitboxbase::config::StatusLedMode::Always,
        1 => bitboxbase::config::StatusLedMode::OnWarning,
        2 => bitboxbase::config::StatusLedMode::OnError,
        _ => return false,
    };
    config.set_status_led_mode(status_led_mode);
    let status_screen_mode = match status_screen_mode {
        0 => bitboxbase::config::StatusScreenMode::OnWarning,
        1 => bitboxbase::config::StatusScreenMode::OnError,
        _ => return false,
    };
    config.set_status_screen_mode(status_screen_mode);
    match config.set_default_display_duration(Duration::from_millis(default_display_duration)) {
        Err(_) => return false,
        _ => (),
    }
    true
}
