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

pub struct State {
    pub state: BitBoxBaseBackgroundState,
    pub description_code: BitBoxBaseBackgroundDescription,
}

/// Global state of bitboxbase background. As long as we export this to C code it has to be repr(C)
/// and have a long name.
/// TODO: Shorten name when not used in C anymore.
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BitBoxBaseBackgroundState {
    /// Waiting is the initial state before any heartbeat after power on.
    BBBWaiting,
    /// NotAlive is the error state when heartbeats stop coming
    BBBNotAlive,
    /// State given by the MW
    BBBIdle,
    /// State given by the MW
    BBBWorking,
    /// State given by the MW
    BBBWarning,
    /// State given by the MW
    BBBError,
}

/// Strings that the MW can set
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BitBoxBaseBackgroundDescription {
    Empty,
    InitialBlockDownload,
    UpdateDownload,
    OutOfDiskSpace,
    RedisError,
    Reboot,
    Shutdown,
    UpdateFailed,
    NoNetworkConnection,
}

pub const DESCRIPTIONS: &[&str] = &[
    "",
    "Initial block download",
    "Downloading update",
    "Out of disk space",
    "Redis",
    "Reboot",
    "Shutdown",
    "Update failed",
    "No network connection",
];
