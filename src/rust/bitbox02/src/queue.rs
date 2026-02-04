// SPDX-License-Identifier: Apache-2.0

pub fn pull_hww() -> Option<[u8; 64]> {
    let hww_data = unsafe { bitbox02_sys::queue_pull(bitbox02_sys::queue_hww_queue()) };
    if hww_data.is_null() {
        return None;
    }
    let mut data: [u8; 64] = [0; 64];
    unsafe { core::ptr::copy_nonoverlapping(hww_data, data.as_mut_ptr(), 64) }
    Some(data)
}

#[cfg(feature = "app-u2f")]
pub fn pull_u2f() -> Option<[u8; 64]> {
    let u2f_data = unsafe { bitbox02_sys::queue_pull(bitbox02_sys::queue_u2f_queue()) };
    if u2f_data.is_null() {
        return None;
    }
    let mut data: [u8; 64] = [0; 64];
    unsafe { core::ptr::copy_nonoverlapping(u2f_data, data.as_mut_ptr(), 64) }
    Some(data)
}
