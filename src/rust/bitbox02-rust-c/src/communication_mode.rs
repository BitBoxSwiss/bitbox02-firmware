// SPDX-License-Identifier: Apache-2.0

pub fn ble_enabled() -> bool {
    bitbox02_rust::communication_mode::ble_enabled(&mut crate::HalImpl::new())
}

pub fn ble_disable() {
    bitbox02_rust::communication_mode::ble_disable();
}

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_enabled() -> bool {
    ble_enabled()
}

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_disable() {
    ble_disable();
}
