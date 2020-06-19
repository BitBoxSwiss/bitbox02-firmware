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

use core::time::Duration;

use arrayvec::ArrayString;

use crate::error::Error;
use crate::util::Ipv4Addr;

/// This determines if the LED should be turned off in some states.
#[derive(Debug, Clone)]
#[repr(u8)]
#[allow(dead_code)] // We are setting this with transmute
pub enum StatusLedMode {
    /// Always show the LED
    Always,
    /// Only show the led in the Working, Warning and Error states
    OnWorking,
    /// Only show the led in the Warning and Error states
    OnWarning,
    /// Only show the led in the Error state
    OnError,
}

#[derive(Debug, Clone)]
#[repr(u8)]
#[allow(dead_code)] // We are setting this with transmute
pub enum StatusScreenMode {
    /// Always show the screen
    Always,
    /// Only show the screen in the Working, Warning and Error states
    OnWorking,
    /// Only show the screen in the Warning and Error states
    OnWarning,
    /// Only show the screen in the Error state
    OnError,
}

pub struct Config {
    pub status_led_mode: StatusLedMode,
    pub status_screen_mode: StatusScreenMode,
    pub default_display_duration: Duration,
    pub hostname: Option<ArrayString<[u8; 64]>>,
    pub ip: Option<Ipv4Addr>,
}

impl Config {
    pub const fn new() -> Config {
        Config {
            status_led_mode: StatusLedMode::Always,
            status_screen_mode: StatusScreenMode::OnWarning,
            default_display_duration: Duration::from_secs(10),
            hostname: None,
            ip: None,
        }
    }

    pub fn set_status_led_mode(&mut self, mode: StatusLedMode) {
        self.status_led_mode = mode;
    }

    pub fn set_status_screen_mode(&mut self, mode: StatusScreenMode) {
        self.status_screen_mode = mode;
    }

    pub fn set_hostname(&mut self, hostname: &str) -> Result<(), Error> {
        if !hostname.is_ascii() {
            return Err(Error::InvalidHostname);
        }
        if hostname.is_empty() || hostname.len() > 63 {
            return Err(Error::InvalidHostname);
        }
        if hostname.starts_with('-') || hostname.ends_with('-') {
            return Err(Error::InvalidHostname);
        }
        if hostname.chars().any(|c| !c.is_alphanumeric() && c != '-') {
            return Err(Error::InvalidHostname);
        }
        self.hostname = Some(ArrayString::from(hostname).expect("Buffer to short"));
        Ok(())
    }

    pub fn set_ip(&mut self, ip: Ipv4Addr) {
        self.ip = Some(ip);
    }
}
