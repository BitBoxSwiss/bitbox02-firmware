// SPDX-License-Identifier: Apache-2.0

pub fn pull_hww() -> Option<[u8; 64]> {
    let mut data = [0; 64];
    let ok = unsafe {
        bitbox02_sys::rust_usb_report_queue_pull(
            bitbox02_sys::rust_usb_report_queue_hww(),
            data.as_mut_ptr(),
        )
    };
    if !ok {
        return None;
    }
    Some(data)
}

#[cfg(feature = "app-u2f")]
pub fn pull_u2f() -> Option<[u8; 64]> {
    let mut data = [0; 64];
    let ok = unsafe {
        bitbox02_sys::rust_usb_report_queue_pull(
            bitbox02_sys::rust_usb_report_queue_u2f(),
            data.as_mut_ptr(),
        )
    };
    if !ok {
        return None;
    }
    Some(data)
}
