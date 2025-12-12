// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "testing"))]
pub fn mcu_32_bytes(out: &mut [u8; 32]) {
    unsafe { bitbox02_sys::random_32_bytes_mcu(out.as_mut_ptr()) }
}

#[cfg(feature = "testing")]
pub fn mcu_32_bytes(out: &mut [u8; 32]) {
    unsafe extern "C" {
        fn rand() -> core::ffi::c_int;
    }

    for elem in out.iter_mut() {
        // Not uniform, but it's only for tests...
        *elem = unsafe { rand() as _ };
    }
}

pub fn random_32_bytes() -> alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>> {
    let mut out = alloc::boxed::Box::new(zeroize::Zeroizing::new([0u8; 32]));
    unsafe { bitbox02_sys::random_32_bytes(out.as_mut_ptr()) }
    out
}

#[cfg(feature = "testing")]
pub fn fake_reset() {
    unsafe {
        bitbox02_sys::random_fake_reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcu_32_bytes() {
        let mut result = [0; 32];
        mcu_32_bytes(&mut result);
        assert!([0; 32] != result);
    }
}
