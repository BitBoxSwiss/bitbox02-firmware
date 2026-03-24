// SPDX-License-Identifier: Apache-2.0

/// C interface.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_communication_mode_ble_enabled(hal: *mut crate::BitBox02HAL) -> bool {
    bitbox02_rust::communication_mode::ble_enabled(unsafe { crate::bitbox02hal_mut(hal) })
}

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_disable() {
    bitbox02_rust::communication_mode::ble_disable();
}
