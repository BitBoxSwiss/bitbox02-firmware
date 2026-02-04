// SPDX-License-Identifier: Apache-2.0

pub use bitbox02_sys::USB_FRAME;

// Compile-time assertion: a single USB report is exactly one `USB_FRAME`.
// This is relevant because some code paths receive raw `[u8; 64]` reports and copy them into an
// aligned `USB_FRAME` before passing a `USB_FRAME*` into C.
const _: [u8; bitbox02_sys::USB_REPORT_SIZE as usize] = [0u8; core::mem::size_of::<USB_FRAME>()];

pub fn process(frame: &USB_FRAME) -> bool {
    unsafe { bitbox02_sys::usb_packet_process(frame as *const _) }
}

#[cfg(feature = "simulator-graphical")]
pub fn process_from_report(packet: &[u8; 64]) -> bool {
    let mut frame = core::mem::MaybeUninit::<USB_FRAME>::uninit();
    unsafe {
        core::ptr::copy_nonoverlapping(
            packet.as_ptr(),
            frame.as_mut_ptr().cast::<u8>(),
            packet.len(),
        );
        bitbox02_sys::usb_packet_process(frame.as_ptr())
    }
}
