#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dinr: DINR,
    doutr: DOUTR,
    keyr0: KEYR0,
    keyr1: KEYR1,
    keyr2: KEYR2,
    keyr3: KEYR3,
    ivr0: IVR0,
    ivr1: IVR1,
    ivr2: IVR2,
    ivr3: IVR3,
    keyr4: KEYR4,
    keyr5: KEYR5,
    keyr6: KEYR6,
    keyr7: KEYR7,
    _reserved16: [u8; 0xc0],
    dpacfgr: DPACFGR,
    _reserved17: [u8; 0x01fc],
    ier: IER,
    isr: ISR,
    icr: ICR,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - data input register
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    ///0x0c - data output register
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    ///0x10 - key register 0
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    ///0x14 - key register 1
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    ///0x18 - key register 2
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    ///0x1c - key register 3
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    ///0x20 - initialization vector register 0
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    ///0x24 - initialization vector register 1
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    ///0x28 - initialization vector register 2
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    ///0x2c - initialization vector register 3
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
    ///0x30 - key register 4
    #[inline(always)]
    pub const fn keyr4(&self) -> &KEYR4 {
        &self.keyr4
    }
    ///0x34 - key register 5
    #[inline(always)]
    pub const fn keyr5(&self) -> &KEYR5 {
        &self.keyr5
    }
    ///0x38 - key register 6
    #[inline(always)]
    pub const fn keyr6(&self) -> &KEYR6 {
        &self.keyr6
    }
    ///0x3c - key register 7
    #[inline(always)]
    pub const fn keyr7(&self) -> &KEYR7 {
        &self.keyr7
    }
    ///0x100 - configuration register
    #[inline(always)]
    pub const fn dpacfgr(&self) -> &DPACFGR {
        &self.dpacfgr
    }
    ///0x300 - interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x304 - interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x308 - interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**DINR (w) register accessor: data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:DINR)

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///data input register
pub mod dinr;
/**DOUTR (r) register accessor: data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:DOUTR)

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///data output register
pub mod doutr;
/**KEYR0 (w) register accessor: key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR0)

For information about available fields see [`mod@keyr0`] module*/
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
///key register 0
pub mod keyr0;
/**KEYR1 (w) register accessor: key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR1)

For information about available fields see [`mod@keyr1`] module*/
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
///key register 1
pub mod keyr1;
/**KEYR2 (w) register accessor: key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR2)

For information about available fields see [`mod@keyr2`] module*/
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
///key register 2
pub mod keyr2;
/**KEYR3 (w) register accessor: key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR3)

For information about available fields see [`mod@keyr3`] module*/
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
///key register 3
pub mod keyr3;
/**IVR0 (rw) register accessor: initialization vector register 0

You can [`read`](crate::Reg::read) this register and get [`ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:IVR0)

For information about available fields see [`mod@ivr0`] module*/
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
///initialization vector register 0
pub mod ivr0;
/**IVR1 (rw) register accessor: initialization vector register 1

You can [`read`](crate::Reg::read) this register and get [`ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:IVR1)

For information about available fields see [`mod@ivr1`] module*/
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
///initialization vector register 1
pub mod ivr1;
/**IVR2 (rw) register accessor: initialization vector register 2

You can [`read`](crate::Reg::read) this register and get [`ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:IVR2)

For information about available fields see [`mod@ivr2`] module*/
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
///initialization vector register 2
pub mod ivr2;
/**IVR3 (rw) register accessor: initialization vector register 3

You can [`read`](crate::Reg::read) this register and get [`ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:IVR3)

For information about available fields see [`mod@ivr3`] module*/
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
///initialization vector register 3
pub mod ivr3;
/**KEYR4 (w) register accessor: key register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR4)

For information about available fields see [`mod@keyr4`] module*/
pub type KEYR4 = crate::Reg<keyr4::KEYR4rs>;
///key register 4
pub mod keyr4;
/**KEYR5 (w) register accessor: key register 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR5)

For information about available fields see [`mod@keyr5`] module*/
pub type KEYR5 = crate::Reg<keyr5::KEYR5rs>;
///key register 5
pub mod keyr5;
/**KEYR6 (w) register accessor: key register 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR6)

For information about available fields see [`mod@keyr6`] module*/
pub type KEYR6 = crate::Reg<keyr6::KEYR6rs>;
///key register 6
pub mod keyr6;
/**KEYR7 (w) register accessor: key register 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR7)

For information about available fields see [`mod@keyr7`] module*/
pub type KEYR7 = crate::Reg<keyr7::KEYR7rs>;
///key register 7
pub mod keyr7;
/**DPACFGR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`dpacfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpacfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:DPACFGR)

For information about available fields see [`mod@dpacfgr`] module*/
pub type DPACFGR = crate::Reg<dpacfgr::DPACFGRrs>;
///configuration register
pub mod dpacfgr;
/**IER (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///interrupt enable register
pub mod ier;
/**ISR (r) register accessor: interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///interrupt status register
pub mod isr;
/**ICR (w) register accessor: interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///interrupt clear register
pub mod icr;
