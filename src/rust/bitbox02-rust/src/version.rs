// SPDX-License-Identifier: Apache-2.0

/// Firmware version, short format, e.g. "v9.12.0".
// We don't want this to be a hard error during development so that rust tools are happy.
pub static FIRMWARE_VERSION_SHORT: &str = {
    let version = option_env!("FIRMWARE_VERSION_SHORT");
    if let Some(version) = version {
        version
    } else {
        ""
    }
};
