// SPDX-License-Identifier: Apache-2.0

//! Small mocking infrastructure for testing.

unsafe extern "C" fn c_mock_random_32_bytes(buf_out: *mut u8) {
    let s = unsafe { core::slice::from_raw_parts_mut(buf_out, 32) };
    s.copy_from_slice(b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
}

/// This sets up memory in RAM for use in unit tests. As there is only one RAM volume, access only serially.
/// The memory is initialized to be like after factory setup, i.e. 0xFF everywhere followed by `memory_setup()`.
pub fn mock_memory() {
    unsafe {
        bitbox02_sys::fake_memory_factoryreset();

        assert!(crate::memory::memory_setup(c_mock_random_32_bytes));

        if bitbox02_sys::smarteeprom_is_enabled() {
            bitbox02_sys::smarteeprom_disable();
        }
        bitbox02_sys::smarteeprom_bb02_config();
        bitbox02_sys::bitbox02_smarteeprom_init();
        bitbox02_sys::spi_mem_full_erase();
    }
}

/// A wrapper that adds the Sync trait to RefCell. We can use this in testing as our unit tests run
/// single-threaded.
pub struct UnsafeSyncRefCell<T>(core::cell::RefCell<T>);
impl<T> UnsafeSyncRefCell<T> {
    pub const fn new(value: T) -> Self {
        Self(core::cell::RefCell::new(value))
    }
}

unsafe impl<T> Sync for UnsafeSyncRefCell<T> {}

impl<T> core::ops::Deref for UnsafeSyncRefCell<T> {
    type Target = core::cell::RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
