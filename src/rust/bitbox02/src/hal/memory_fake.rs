// SPDX-License-Identifier: Apache-2.0

pub use fake_hardware::memory::FakeMemory as BitBox02Memory;

#[cfg(test)]
mod tests {
    use bitbox_hal::{Hal, Memory};

    #[test]
    fn test_set_initialized_uses_shared_fake_hardware_state() {
        fake_hardware::memory::reset();

        let mut first = crate::hal::BitBox02Hal::new();
        let mut second = crate::hal::BitBox02Hal::new();

        assert!(!first.memory().is_initialized());
        assert!(!second.memory().is_initialized());

        first.memory().set_initialized().unwrap();

        assert!(first.memory().is_initialized());
        assert!(second.memory().is_initialized());

        fake_hardware::memory::reset();
    }
}
