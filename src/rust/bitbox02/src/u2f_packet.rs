// SPDX-License-Identifier: Apache-2.0

pub use bitbox02_sys::USB_FRAME;

pub fn init() {
    unsafe {
        bitbox02_sys::u2f_packet_init();
    }
}

pub fn timeout_get(cid: &mut u32) -> bool {
    unsafe { bitbox02_sys::u2f_packet_timeout_get(cid as *mut _) }
}

pub fn timeout(cid: u32) {
    unsafe { bitbox02_sys::u2f_packet_timeout(cid) }
}

pub fn process(frame: &USB_FRAME) -> bool {
    unsafe { bitbox02_sys::u2f_packet_process(frame as *const _) }
}
