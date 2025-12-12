// SPDX-License-Identifier: Apache-2.0

// Emit a warning if FIRMWARE_VERSION_SHORT isn't set. We don't want this to be a hard error during
// development so that rust tools are happy.
fn main() {
    let version = option_env!("FIRMWARE_VERSION_SHORT");
    if let Some(version) = version {
        if version.is_empty() {
            println!("cargo::warning=FIRMWARE_VERSION_SHORT is empty");
        }
    } else {
        println!("cargo::warning=FIRMWARE_VERSION_SHORT is not set");
    }
}
