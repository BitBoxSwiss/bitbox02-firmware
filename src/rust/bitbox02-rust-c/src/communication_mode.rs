// SPDX-License-Identifier: Apache-2.0

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_enabled() -> bool {
    bitbox02_rust::communication_mode::ble_enabled(&mut crate::HalImpl::new())
}

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_disable() {
    bitbox02_rust::communication_mode::ble_disable();
}
