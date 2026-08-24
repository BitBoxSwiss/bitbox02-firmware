#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    oar1: OAR1,
    oar2: OAR2,
    timingr: TIMINGR,
    timeoutr: TIMEOUTR,
    isr: ISR,
    icr: ICR,
    pecr: PECR,
    rxdr: RXDR,
    txdr: TXDR,
    autocr: AUTOCR,
}
impl RegisterBlock {
    ///0x00 - Control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - Control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - Own address register 1
    #[inline(always)]
    pub const fn oar1(&self) -> &OAR1 {
        &self.oar1
    }
    ///0x0c - Own address register 2
    #[inline(always)]
    pub const fn oar2(&self) -> &OAR2 {
        &self.oar2
    }
    ///0x10 - Timing register
    #[inline(always)]
    pub const fn timingr(&self) -> &TIMINGR {
        &self.timingr
    }
    ///0x14 - Status register 1
    #[inline(always)]
    pub const fn timeoutr(&self) -> &TIMEOUTR {
        &self.timeoutr
    }
    ///0x18 - Interrupt and Status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x1c - Interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x20 - PEC register
    #[inline(always)]
    pub const fn pecr(&self) -> &PECR {
        &self.pecr
    }
    ///0x24 - Receive data register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    ///0x28 - Transmit data register
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    ///0x2c - I2C Autonomous mode control register
    #[inline(always)]
    pub const fn autocr(&self) -> &AUTOCR {
        &self.autocr
    }
}
/**CR1 (rw) register accessor: Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///Control register 1
pub mod cr1;
/**CR2 (rw) register accessor: Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Control register 2
pub mod cr2;
/**OAR1 (rw) register accessor: Own address register 1

You can [`read`](crate::Reg::read) this register and get [`oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:OAR1)

For information about available fields see [`mod@oar1`] module*/
pub type OAR1 = crate::Reg<oar1::OAR1rs>;
///Own address register 1
pub mod oar1;
/**OAR2 (rw) register accessor: Own address register 2

You can [`read`](crate::Reg::read) this register and get [`oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:OAR2)

For information about available fields see [`mod@oar2`] module*/
pub type OAR2 = crate::Reg<oar2::OAR2rs>;
///Own address register 2
pub mod oar2;
/**TIMINGR (rw) register accessor: Timing register

You can [`read`](crate::Reg::read) this register and get [`timingr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:TIMINGR)

For information about available fields see [`mod@timingr`] module*/
pub type TIMINGR = crate::Reg<timingr::TIMINGRrs>;
///Timing register
pub mod timingr;
/**TIMEOUTR (rw) register accessor: Status register 1

You can [`read`](crate::Reg::read) this register and get [`timeoutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:TIMEOUTR)

For information about available fields see [`mod@timeoutr`] module*/
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTRrs>;
///Status register 1
pub mod timeoutr;
/**ISR (rw) register accessor: Interrupt and Status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Interrupt and Status register
pub mod isr;
/**ICR (w) register accessor: Interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Interrupt clear register
pub mod icr;
/**PECR (r) register accessor: PEC register

You can [`read`](crate::Reg::read) this register and get [`pecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:PECR)

For information about available fields see [`mod@pecr`] module*/
pub type PECR = crate::Reg<pecr::PECRrs>;
///PEC register
pub mod pecr;
/**RXDR (r) register accessor: Receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:RXDR)

For information about available fields see [`mod@rxdr`] module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///Receive data register
pub mod rxdr;
/**TXDR (rw) register accessor: Transmit data register

You can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:TXDR)

For information about available fields see [`mod@txdr`] module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///Transmit data register
pub mod txdr;
/**AUTOCR (rw) register accessor: I2C Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:AUTOCR)

For information about available fields see [`mod@autocr`] module*/
pub type AUTOCR = crate::Reg<autocr::AUTOCRrs>;
///I2C Autonomous mode control register
pub mod autocr;
