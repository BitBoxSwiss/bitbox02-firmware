// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::{Hal, System, Ui};
use bitbox_platform_host::timer::HostTimer;
use std::cell::Cell;

type DeviceHal = bitbox02::hal::BitBox02Hal<HostTimer>;
type DeviceSystem = bitbox02::hal::system::BitBox02System<HostTimer>;

pub type GraphicalHal = SimulatorHal<bitbox02::hal::ui::BitBox02Ui<HostTimer>, true>;

thread_local! {
    static RESET_REQUESTED: Cell<bool> = const { Cell::new(false) };
}

pub fn reset_requested() -> bool {
    RESET_REQUESTED.get()
}

pub fn take_reset_request() -> bool {
    RESET_REQUESTED.replace(false)
}

#[derive(Default)]
pub struct SimulatorSystem<const GUI: bool> {
    inner: DeviceSystem,
}

impl<const GUI: bool> System for SimulatorSystem<GUI> {
    async fn startup() {
        if GUI {
            DeviceSystem::startup().await;
        }
    }

    fn communication_timeout_reset(&mut self, value: i16) {
        self.inner.communication_timeout_reset(value);
    }

    fn is_btconly(&mut self) -> bool {
        self.inner.is_btconly()
    }

    fn reboot(&mut self) -> ! {
        self.inner.reboot()
    }

    fn reboot_to_bootloader(&mut self) -> ! {
        self.inner.reboot_to_bootloader()
    }

    fn reboot_after_reset(&mut self) {
        // The core must cancel the task and reset its UI only after this poll returns.
        RESET_REQUESTED.set(true);
    }

    fn reset_ble(&mut self) {
        self.inner.reset_ble();
    }
}

#[derive(Default)]
pub struct SimulatorHal<U, const GUI: bool> {
    inner: DeviceHal,
    ui: U,
    system: SimulatorSystem<GUI>,
}

impl<U: Ui, const GUI: bool> Hal for SimulatorHal<U, GUI> {
    type Ui = U;
    type Random = <DeviceHal as Hal>::Random;
    type Sd = <DeviceHal as Hal>::Sd;
    type SecureChip = <DeviceHal as Hal>::SecureChip;
    type Memory = <DeviceHal as Hal>::Memory;
    type Eeprom = <DeviceHal as Hal>::Eeprom;
    type System = SimulatorSystem<GUI>;

    fn as_mut(
        &mut self,
    ) -> bitbox_hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::Eeprom,
        Self::System,
    > {
        let inner = self.inner.as_mut();
        bitbox_hal::HalSubsystems {
            ui: &mut self.ui,
            random: inner.random,
            sd: inner.sd,
            securechip: inner.securechip,
            memory: inner.memory,
            eeprom: inner.eeprom,
            system: &mut self.system,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reboot_after_reset() {
        assert!(!reset_requested());
        SimulatorSystem::<true>::default().reboot_after_reset();
        assert!(take_reset_request());
        assert!(!take_reset_request());
        SimulatorSystem::<false>::default().reboot_after_reset();
        assert!(take_reset_request());
        assert!(!take_reset_request());
    }
}
