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

use core::fmt::Write;
use core::time::Duration;

use arrayvec::ArrayString;
use bitbox02::{delay, ug_clear_buffer, ug_font_select, ug_put_string, ug_send_buffer};

use super::config::Config;
use crate::c_char;
use crate::util::FixedCString;

fn write_status<W: Write>(w: &mut W, config: &Config) {
    let _ = write!(w, "hostname: ");
    if let Some(hostname) = &config.hostname {
        let _ = write!(w, "{}", hostname);
    } else {
        let _ = write!(w, "<unnamed>");
    }
    let _ = write!(w, "\n");
    let _ = write!(w, "ip: ");
    if let Some(ip) = &config.ip {
        let _ = write!(w, "{}", ip);
    } else {
        let _ = write!(w, "<unassigned>");
    }
    let _ = write!(w, "\n");
    let _ = write!(w, "status: OK\n");
    let _ = write!(w, "mode: {:?}\n", config.status_led_mode);
}

pub fn display_status(config: &Config, duration: Option<Duration>) {
    ug_clear_buffer();
    ug_font_select();
    let mut buf = ArrayString::<[_; 256]>::new();
    write_status(&mut buf, config);
    ug_put_string(10, 10, &buf, false);
    ug_send_buffer();
    if let Some(duration) = duration {
        delay(duration)
    } else {
        delay(config.default_display_duration)
    }
}

#[no_mangle]
pub extern "C" fn bitboxbase_status_get(ptr: *mut c_char, ptr_len: usize) {
    let buf = unsafe { core::slice::from_raw_parts_mut(ptr, ptr_len) };
    let mut wrapper = FixedCString::new(buf);
    write_status(&mut wrapper, unsafe { Config::get_singleton() });
}
