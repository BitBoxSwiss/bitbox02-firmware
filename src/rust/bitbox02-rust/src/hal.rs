// SPDX-License-Identifier: Apache-2.0

pub mod bitbox02;
pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

#[cfg(feature = "testing")]
pub mod testing;

pub use bitbox02::BitBox02Hal;
pub use memory::Memory;
pub use random::Random;
pub use sd::Sd;
pub use securechip::SecureChip;
pub use system::System;
pub use ui::Ui;

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    fn ui(&mut self) -> &mut impl Ui;
    fn sd(&mut self) -> &mut impl Sd;
    fn random(&mut self) -> &mut impl Random;
    fn securechip(&mut self) -> &mut impl SecureChip;
    fn memory(&mut self) -> &mut impl Memory;
    fn system(&mut self) -> &mut impl System;
}
