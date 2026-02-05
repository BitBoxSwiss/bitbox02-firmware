// SPDX-License-Identifier: Apache-2.0

pub fn product() -> &'static str {
    unsafe {
        let mut len = 0;
        let s = bitbox02_sys::platform_product(&mut len as *mut _) as *const u8;
        let s = core::slice::from_raw_parts(s, len);
        str::from_utf8_unchecked(s)
    }
}
