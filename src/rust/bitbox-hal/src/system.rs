// SPDX-License-Identifier: Apache-2.0

pub trait System {
    fn reboot_to_bootloader(&mut self) -> !;
}
