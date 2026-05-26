#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ier: IER,
    cr: CR,
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    smpr: SMPR,
    _reserved6: [u8; 0x08],
    awd1tr: AWD1TR,
    awd2tr: AWD2TR,
    _reserved_8_chselr0: [u8; 0x04],
    awd3tr: AWD3TR,
    _reserved10: [u8; 0x10],
    dr: DR,
    pwrr: PWRR,
    _reserved12: [u8; 0x58],
    awd2cr: AWD2CR,
    awd3cr: AWD3CR,
    _reserved14: [u8; 0x0c],
    calfact: CALFACT,
    _reserved15: [u8; 0x18],
    or: OR,
    _reserved16: [u8; 0x0234],
    ccr: CCR,
}
impl RegisterBlock {
    ///0x00 - ADC interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - ADC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x08 - ADC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - ADC configuration register
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x10 - ADC configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x14 - ADC sample time register
    #[inline(always)]
    pub const fn smpr(&self) -> &SMPR {
        &self.smpr
    }
    ///0x20 - ADC watchdog threshold register
    #[inline(always)]
    pub const fn awd1tr(&self) -> &AWD1TR {
        &self.awd1tr
    }
    ///0x24 - ADC watchdog threshold register
    #[inline(always)]
    pub const fn awd2tr(&self) -> &AWD2TR {
        &self.awd2tr
    }
    ///0x28 - ADC channel selection register \[alternate\]
    #[inline(always)]
    pub const fn chselr1(&self) -> &CHSELR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - ADC channel selection register \[alternate\]
    #[inline(always)]
    pub const fn chselr0(&self) -> &CHSELR0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - ADC watchdog threshold register
    #[inline(always)]
    pub const fn awd3tr(&self) -> &AWD3TR {
        &self.awd3tr
    }
    ///0x40 - ADC data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x44 - ADC data register
    #[inline(always)]
    pub const fn pwrr(&self) -> &PWRR {
        &self.pwrr
    }
    ///0xa0 - ADC Analog Watchdog 2 Configuration register
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    ///0xa4 - ADC Analog Watchdog 3 Configuration register
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    ///0xb4 - ADC Calibration factor
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
    ///0xd0 - ADC option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
    ///0x308 - ADC common configuration register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
}
/**ISR (rw) register accessor: ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///ADC interrupt and status register
pub mod isr;
/**IER (rw) register accessor: ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///ADC interrupt enable register
pub mod ier;
/**CR (rw) register accessor: ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///ADC control register
pub mod cr;
/**CFGR1 (rw) register accessor: ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///ADC configuration register
pub mod cfgr1;
/**CFGR2 (rw) register accessor: ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///ADC configuration register 2
pub mod cfgr2;
/**SMPR (rw) register accessor: ADC sample time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:SMPR)

For information about available fields see [`mod@smpr`] module*/
pub type SMPR = crate::Reg<smpr::SMPRrs>;
///ADC sample time register
pub mod smpr;
/**AWD1TR (rw) register accessor: ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:AWD1TR)

For information about available fields see [`mod@awd1tr`] module*/
pub type AWD1TR = crate::Reg<awd1tr::AWD1TRrs>;
///ADC watchdog threshold register
pub mod awd1tr;
/**AWD2TR (rw) register accessor: ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd2tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:AWD2TR)

For information about available fields see [`mod@awd2tr`] module*/
pub type AWD2TR = crate::Reg<awd2tr::AWD2TRrs>;
///ADC watchdog threshold register
pub mod awd2tr;
/**CHSELR0 (rw) register accessor: ADC channel selection register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`chselr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CHSELR0)

For information about available fields see [`mod@chselr0`] module*/
pub type CHSELR0 = crate::Reg<chselr0::CHSELR0rs>;
///ADC channel selection register \[alternate\]
pub mod chselr0;
/**CHSELR1 (rw) register accessor: ADC channel selection register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`chselr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CHSELR1)

For information about available fields see [`mod@chselr1`] module*/
pub type CHSELR1 = crate::Reg<chselr1::CHSELR1rs>;
///ADC channel selection register \[alternate\]
pub mod chselr1;
/**AWD3TR (rw) register accessor: ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd3tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:AWD3TR)

For information about available fields see [`mod@awd3tr`] module*/
pub type AWD3TR = crate::Reg<awd3tr::AWD3TRrs>;
///ADC watchdog threshold register
pub mod awd3tr;
/**DR (r) register accessor: ADC data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///ADC data register
pub mod dr;
/**PWRR (rw) register accessor: ADC data register

You can [`read`](crate::Reg::read) this register and get [`pwrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:PWRR)

For information about available fields see [`mod@pwrr`] module*/
pub type PWRR = crate::Reg<pwrr::PWRRrs>;
///ADC data register
pub mod pwrr;
/**AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:AWD2CR)

For information about available fields see [`mod@awd2cr`] module*/
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
///ADC Analog Watchdog 2 Configuration register
pub mod awd2cr;
/**AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:AWD3CR)

For information about available fields see [`mod@awd3cr`] module*/
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
///ADC Analog Watchdog 3 Configuration register
pub mod awd3cr;
/**CALFACT (rw) register accessor: ADC Calibration factor

You can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CALFACT)

For information about available fields see [`mod@calfact`] module*/
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
///ADC Calibration factor
pub mod calfact;
/**OR (rw) register accessor: ADC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///ADC option register
pub mod or;
/**CCR (rw) register accessor: ADC common configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common configuration register
pub mod ccr;
