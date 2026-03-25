// SPDX-License-Identifier: Apache-2.0

pub fn init_mcu() {
    unsafe { bitbox02_sys::_init_chip() }
}

pub fn init() {
    unsafe { bitbox02_sys::system_init() }
}

pub fn common_stack_chk_guard() -> u32 {
    unsafe { bitbox02_sys::common_stack_chk_guard() }
}

pub fn common_main() {
    unsafe { bitbox02_sys::common_main() }
}
