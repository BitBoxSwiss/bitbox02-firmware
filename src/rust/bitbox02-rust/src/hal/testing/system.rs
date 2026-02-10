// SPDX-License-Identifier: Apache-2.0

pub struct TestingSystem;

impl TestingSystem {
    pub fn new() -> Self {
        Self
    }
}

impl crate::hal::System for TestingSystem {
    fn reboot_to_bootloader(&mut self) -> ! {
        panic!("reboot_to_bootloader called")
    }
}
