#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ram1cr: RAM1CR,
    _reserved1: [u8; 0x04],
    ram1isr: RAM1ISR,
    _reserved2: [u8; 0x1c],
    ram1erkeyr: RAM1ERKEYR,
    _reserved3: [u8; 0x14],
    ram2cr: RAM2CR,
    ram2ier: RAM2IER,
    ram2isr: RAM2ISR,
    ram2sear: RAM2SEAR,
    ram2dear: RAM2DEAR,
    ram2icr: RAM2ICR,
    ram2wpr1: RAM2WPR1,
    ram2wpr2: RAM2WPR2,
    _reserved11: [u8; 0x04],
    ram2ecckeyr: RAM2ECCKEYR,
    ram2erkeyr: RAM2ERKEYR,
    _reserved13: [u8; 0x14],
    ram3cr: RAM3CR,
    ram3ier: RAM3IER,
    ram3isr: RAM3ISR,
    ram3sear: RAM3SEAR,
    ram3dear: RAM3DEAR,
    ram3icr: RAM3ICR,
    _reserved19: [u8; 0x0c],
    ram3ecckeyr: RAM3ECCKEYR,
    ram3erkeyr: RAM3ERKEYR,
    _reserved21: [u8; 0x14],
    ram4cr: RAM4CR,
    _reserved22: [u8; 0x04],
    ram4isr: RAM4ISR,
    _reserved23: [u8; 0x1c],
    ram4erkeyr: RAM4ERKEYR,
    _reserved24: [u8; 0x14],
    ram5cr: RAM5CR,
    ram5ier: RAM5IER,
    ram5isr: RAM5ISR,
    ram5sear: RAM5SEAR,
    ram5dear: RAM5DEAR,
    ram5icr: RAM5ICR,
}
impl RegisterBlock {
    ///0x00 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn ram1cr(&self) -> &RAM1CR {
        &self.ram1cr
    }
    ///0x08 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn ram1isr(&self) -> &RAM1ISR {
        &self.ram1isr
    }
    ///0x28 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn ram1erkeyr(&self) -> &RAM1ERKEYR {
        &self.ram1erkeyr
    }
    ///0x40 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn ram2cr(&self) -> &RAM2CR {
        &self.ram2cr
    }
    ///0x44 - RAMCFG SRAM x interrupt enable register
    #[inline(always)]
    pub const fn ram2ier(&self) -> &RAM2IER {
        &self.ram2ier
    }
    ///0x48 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn ram2isr(&self) -> &RAM2ISR {
        &self.ram2isr
    }
    ///0x4c - RAMCFG RAM x ECC single error address register
    #[inline(always)]
    pub const fn ram2sear(&self) -> &RAM2SEAR {
        &self.ram2sear
    }
    ///0x50 - RAMCFG RAM x ECC double error address register
    #[inline(always)]
    pub const fn ram2dear(&self) -> &RAM2DEAR {
        &self.ram2dear
    }
    ///0x54 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn ram2icr(&self) -> &RAM2ICR {
        &self.ram2icr
    }
    ///0x58 - RAMCFG SRAM2 write protection register 1
    #[inline(always)]
    pub const fn ram2wpr1(&self) -> &RAM2WPR1 {
        &self.ram2wpr1
    }
    ///0x5c - RAMCFG SRAM2 write protection register 2
    #[inline(always)]
    pub const fn ram2wpr2(&self) -> &RAM2WPR2 {
        &self.ram2wpr2
    }
    ///0x64 - RAMCFG SRAM x ECC key register
    #[inline(always)]
    pub const fn ram2ecckeyr(&self) -> &RAM2ECCKEYR {
        &self.ram2ecckeyr
    }
    ///0x68 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn ram2erkeyr(&self) -> &RAM2ERKEYR {
        &self.ram2erkeyr
    }
    ///0x80 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn ram3cr(&self) -> &RAM3CR {
        &self.ram3cr
    }
    ///0x84 - RAMCFG SRAM x interrupt enable register
    #[inline(always)]
    pub const fn ram3ier(&self) -> &RAM3IER {
        &self.ram3ier
    }
    ///0x88 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn ram3isr(&self) -> &RAM3ISR {
        &self.ram3isr
    }
    ///0x8c - RAMCFG RAM x ECC single error address register
    #[inline(always)]
    pub const fn ram3sear(&self) -> &RAM3SEAR {
        &self.ram3sear
    }
    ///0x90 - RAMCFG RAM x ECC double error address register
    #[inline(always)]
    pub const fn ram3dear(&self) -> &RAM3DEAR {
        &self.ram3dear
    }
    ///0x94 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn ram3icr(&self) -> &RAM3ICR {
        &self.ram3icr
    }
    ///0xa4 - RAMCFG SRAM x ECC key register
    #[inline(always)]
    pub const fn ram3ecckeyr(&self) -> &RAM3ECCKEYR {
        &self.ram3ecckeyr
    }
    ///0xa8 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn ram3erkeyr(&self) -> &RAM3ERKEYR {
        &self.ram3erkeyr
    }
    ///0xc0 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn ram4cr(&self) -> &RAM4CR {
        &self.ram4cr
    }
    ///0xc8 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn ram4isr(&self) -> &RAM4ISR {
        &self.ram4isr
    }
    ///0xe8 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn ram4erkeyr(&self) -> &RAM4ERKEYR {
        &self.ram4erkeyr
    }
    ///0x100 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn ram5cr(&self) -> &RAM5CR {
        &self.ram5cr
    }
    ///0x104 - RAMCFG SRAM x interrupt enable register
    #[inline(always)]
    pub const fn ram5ier(&self) -> &RAM5IER {
        &self.ram5ier
    }
    ///0x108 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn ram5isr(&self) -> &RAM5ISR {
        &self.ram5isr
    }
    ///0x10c - RAMCFG RAM x ECC single error address register
    #[inline(always)]
    pub const fn ram5sear(&self) -> &RAM5SEAR {
        &self.ram5sear
    }
    ///0x110 - RAMCFG RAM x ECC double error address register
    #[inline(always)]
    pub const fn ram5dear(&self) -> &RAM5DEAR {
        &self.ram5dear
    }
    ///0x114 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn ram5icr(&self) -> &RAM5ICR {
        &self.ram5icr
    }
}
/**RAM1CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`ram1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM1CR)

For information about available fields see [`mod@ram1cr`] module*/
pub type RAM1CR = crate::Reg<ram1cr::RAM1CRrs>;
///RAMCFG SRAM x control register
pub mod ram1cr;
/**RAM1ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ram1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM1ISR)

For information about available fields see [`mod@ram1isr`] module*/
pub type RAM1ISR = crate::Reg<ram1isr::RAM1ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod ram1isr;
/**RAM1ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM1ERKEYR)

For information about available fields see [`mod@ram1erkeyr`] module*/
pub type RAM1ERKEYR = crate::Reg<ram1erkeyr::RAM1ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod ram1erkeyr;
/**RAM2CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`ram2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2CR)

For information about available fields see [`mod@ram2cr`] module*/
pub type RAM2CR = crate::Reg<ram2cr::RAM2CRrs>;
///RAMCFG SRAM x control register
pub mod ram2cr;
/**RAM2IER (rw) register accessor: RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ram2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2IER)

For information about available fields see [`mod@ram2ier`] module*/
pub type RAM2IER = crate::Reg<ram2ier::RAM2IERrs>;
///RAMCFG SRAM x interrupt enable register
pub mod ram2ier;
/**RAM2ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ram2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2ISR)

For information about available fields see [`mod@ram2isr`] module*/
pub type RAM2ISR = crate::Reg<ram2isr::RAM2ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod ram2isr;
/**RAM2SEAR (r) register accessor: RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`ram2sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2SEAR)

For information about available fields see [`mod@ram2sear`] module*/
pub type RAM2SEAR = crate::Reg<ram2sear::RAM2SEARrs>;
///RAMCFG RAM x ECC single error address register
pub mod ram2sear;
/**RAM2DEAR (r) register accessor: RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`ram2dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2DEAR)

For information about available fields see [`mod@ram2dear`] module*/
pub type RAM2DEAR = crate::Reg<ram2dear::RAM2DEARrs>;
///RAMCFG RAM x ECC double error address register
pub mod ram2dear;
/**RAM2ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`ram2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2ICR)

For information about available fields see [`mod@ram2icr`] module*/
pub type RAM2ICR = crate::Reg<ram2icr::RAM2ICRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod ram2icr;
/**RAM2WPR1 (rw) register accessor: RAMCFG SRAM2 write protection register 1

You can [`read`](crate::Reg::read) this register and get [`ram2wpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2wpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2WPR1)

For information about available fields see [`mod@ram2wpr1`] module*/
pub type RAM2WPR1 = crate::Reg<ram2wpr1::RAM2WPR1rs>;
///RAMCFG SRAM2 write protection register 1
pub mod ram2wpr1;
/**RAM2WPR2 (rw) register accessor: RAMCFG SRAM2 write protection register 2

You can [`read`](crate::Reg::read) this register and get [`ram2wpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2wpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2WPR2)

For information about available fields see [`mod@ram2wpr2`] module*/
pub type RAM2WPR2 = crate::Reg<ram2wpr2::RAM2WPR2rs>;
///RAMCFG SRAM2 write protection register 2
pub mod ram2wpr2;
/**RAM2ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2ECCKEYR)

For information about available fields see [`mod@ram2ecckeyr`] module*/
pub type RAM2ECCKEYR = crate::Reg<ram2ecckeyr::RAM2ECCKEYRrs>;
///RAMCFG SRAM x ECC key register
pub mod ram2ecckeyr;
/**RAM2ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2ERKEYR)

For information about available fields see [`mod@ram2erkeyr`] module*/
pub type RAM2ERKEYR = crate::Reg<ram2erkeyr::RAM2ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod ram2erkeyr;
/**RAM3CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`ram3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3CR)

For information about available fields see [`mod@ram3cr`] module*/
pub type RAM3CR = crate::Reg<ram3cr::RAM3CRrs>;
///RAMCFG SRAM x control register
pub mod ram3cr;
/**RAM3IER (rw) register accessor: RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ram3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3IER)

For information about available fields see [`mod@ram3ier`] module*/
pub type RAM3IER = crate::Reg<ram3ier::RAM3IERrs>;
///RAMCFG SRAM x interrupt enable register
pub mod ram3ier;
/**RAM3ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ram3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3ISR)

For information about available fields see [`mod@ram3isr`] module*/
pub type RAM3ISR = crate::Reg<ram3isr::RAM3ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod ram3isr;
/**RAM3SEAR (r) register accessor: RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`ram3sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3SEAR)

For information about available fields see [`mod@ram3sear`] module*/
pub type RAM3SEAR = crate::Reg<ram3sear::RAM3SEARrs>;
///RAMCFG RAM x ECC single error address register
pub mod ram3sear;
/**RAM3DEAR (r) register accessor: RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`ram3dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3DEAR)

For information about available fields see [`mod@ram3dear`] module*/
pub type RAM3DEAR = crate::Reg<ram3dear::RAM3DEARrs>;
///RAMCFG RAM x ECC double error address register
pub mod ram3dear;
/**RAM3ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`ram3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3ICR)

For information about available fields see [`mod@ram3icr`] module*/
pub type RAM3ICR = crate::Reg<ram3icr::RAM3ICRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod ram3icr;
/**RAM3ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3ECCKEYR)

For information about available fields see [`mod@ram3ecckeyr`] module*/
pub type RAM3ECCKEYR = crate::Reg<ram3ecckeyr::RAM3ECCKEYRrs>;
///RAMCFG SRAM x ECC key register
pub mod ram3ecckeyr;
/**RAM3ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3ERKEYR)

For information about available fields see [`mod@ram3erkeyr`] module*/
pub type RAM3ERKEYR = crate::Reg<ram3erkeyr::RAM3ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod ram3erkeyr;
/**RAM4CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`ram4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM4CR)

For information about available fields see [`mod@ram4cr`] module*/
pub type RAM4CR = crate::Reg<ram4cr::RAM4CRrs>;
///RAMCFG SRAM x control register
pub mod ram4cr;
/**RAM4ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ram4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM4ISR)

For information about available fields see [`mod@ram4isr`] module*/
pub type RAM4ISR = crate::Reg<ram4isr::RAM4ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod ram4isr;
/**RAM4ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM4ERKEYR)

For information about available fields see [`mod@ram4erkeyr`] module*/
pub type RAM4ERKEYR = crate::Reg<ram4erkeyr::RAM4ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod ram4erkeyr;
/**RAM5CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`ram5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM5CR)

For information about available fields see [`mod@ram5cr`] module*/
pub type RAM5CR = crate::Reg<ram5cr::RAM5CRrs>;
///RAMCFG SRAM x control register
pub mod ram5cr;
/**RAM5IER (rw) register accessor: RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ram5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM5IER)

For information about available fields see [`mod@ram5ier`] module*/
pub type RAM5IER = crate::Reg<ram5ier::RAM5IERrs>;
///RAMCFG SRAM x interrupt enable register
pub mod ram5ier;
/**RAM5ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ram5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM5ISR)

For information about available fields see [`mod@ram5isr`] module*/
pub type RAM5ISR = crate::Reg<ram5isr::RAM5ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod ram5isr;
/**RAM5SEAR (r) register accessor: RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`ram5sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM5SEAR)

For information about available fields see [`mod@ram5sear`] module*/
pub type RAM5SEAR = crate::Reg<ram5sear::RAM5SEARrs>;
///RAMCFG RAM x ECC single error address register
pub mod ram5sear;
/**RAM5DEAR (r) register accessor: RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`ram5dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM5DEAR)

For information about available fields see [`mod@ram5dear`] module*/
pub type RAM5DEAR = crate::Reg<ram5dear::RAM5DEARrs>;
///RAMCFG RAM x ECC double error address register
pub mod ram5dear;
/**RAM5ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`ram5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM5ICR)

For information about available fields see [`mod@ram5icr`] module*/
pub type RAM5ICR = crate::Reg<ram5icr::RAM5ICRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod ram5icr;
