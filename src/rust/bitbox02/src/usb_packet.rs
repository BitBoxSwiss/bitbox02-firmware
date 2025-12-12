// SPDX-License-Identifier: Apache-2.0

pub use bitbox02_sys::USB_FRAME;

pub fn process(packet: &[u8; 64]) -> bool {
    unsafe { bitbox02_sys::usb_packet_process(packet.as_ptr() as *const _) }
}
