// SPDX-License-Identifier: Apache-2.0

pub fn init() {
    unsafe { bitbox02_sys::bootloader_init() }
}

pub fn jump() {
    unsafe { bitbox02_sys::bootloader_jump() }
}

pub fn mpu_regions_init() {
    unsafe { bitbox02_sys::mpu_regions_bootloader_init() }
}

pub fn render_default_screen() {
    unsafe { bitbox02_sys::bootloader_render_default_screen() }
}

pub fn render_ble_confirm_screen(confirmed: bool) {
    unsafe { bitbox02_sys::bootloader_render_ble_confirm_screen(confirmed) }
}
