// SPDX-License-Identifier: Apache-2.0

use crate::usb_packet::USB_FRAME;

pub fn write_poll(buf: &[u8; 64]) -> bool {
    unsafe { bitbox02_sys::hid_u2f_write_poll(buf.as_ptr() as *const _) }
}

pub fn read(frame: &mut USB_FRAME) -> bool {
    unsafe { bitbox02_sys::hid_u2f_read(frame as *mut USB_FRAME as *mut u8) }
}
