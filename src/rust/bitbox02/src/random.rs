// SPDX-License-Identifier: Apache-2.0

/// Supplies the randomness source to the noise crate.
pub enum BB02Random32 {}

impl bitbox02_noise::Random32 for BB02Random32 {
    fn mcu_32_bytes(out: &mut [u8; 32]) {
        mcu_32_bytes(out);
    }
}

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

/// `private_key_out` must be 32 bytes.
#[unsafe(no_mangle)]
pub extern "C" fn rust_noise_generate_static_private_key(
    mut private_key_out: util::bytes::BytesMut,
) {
    let key = bitbox02_noise::generate_static_private_key::<BB02Random32>();
    private_key_out.as_mut().copy_from_slice(&key[..]);
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

    #[test]
    fn test_generate_static_private_key() {
        let key = bitbox02_noise::generate_static_private_key::<BB02Random32>();
        assert_eq!(key[0] & 0b111, 0);
        assert_eq!(key[31] & 0b1000_0000, 0);
        assert_eq!(key[31] & 0b0100_0000, 0b0100_0000);
    }
}
