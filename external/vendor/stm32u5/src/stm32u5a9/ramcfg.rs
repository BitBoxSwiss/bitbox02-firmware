#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    m1cr: M1CR,
    _reserved1: [u8; 0x04],
    m1isr: M1ISR,
    _reserved2: [u8; 0x1c],
    ram1erkeyr: RAM1ERKEYR,
    _reserved3: [u8; 0x14],
    m2cr: M2CR,
    m2ier: M2IER,
    m2isr: M2ISR,
    m2sear: M2SEAR,
    m2dear: M2DEAR,
    m2icr: M2ICR,
    m2wpr1: M2WPR1,
    m2wpr2: M2WPR2,
    _reserved11: [u8; 0x04],
    m2ecckeyr: M2ECCKEYR,
    m2erkeyr: M2ERKEYR,
    _reserved13: [u8; 0x14],
    m3cr: M3CR,
    m3ier: M3IER,
    m3isr: M3ISR,
    m3sear: M3SEAR,
    m3dear: M3DEAR,
    m3icr: M3ICR,
    _reserved19: [u8; 0x0c],
    m3ecckeyr: M3ECCKEYR,
    m3erkeyr: M3ERKEYR,
    _reserved21: [u8; 0x14],
    m4cr: M4CR,
    _reserved22: [u8; 0x04],
    m4isr: M4ISR,
    _reserved23: [u8; 0x1c],
    m4erkeyr: M4ERKEYR,
    _reserved24: [u8; 0x14],
    m5cr: M5CR,
    m5ier: M5IER,
    m5isr: M5ISR,
    m5sear: M5SEAR,
    m5dear: M5DEAR,
    m5icr: M5ICR,
    _reserved30: [u8; 0x0c],
    m5ecckeyr: M5ECCKEYR,
    m5erkeyr: M5ERKEYR,
    _reserved32: [u8; 0x14],
    m6cr: M6CR,
    _reserved33: [u8; 0x04],
    m6isr: M6ISR,
    _reserved34: [u8; 0x1c],
    m6erkeyr: M6ERKEYR,
}
impl RegisterBlock {
    ///0x00 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1CR {
        &self.m1cr
    }
    ///0x08 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn m1isr(&self) -> &M1ISR {
        &self.m1isr
    }
    ///0x28 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn ram1erkeyr(&self) -> &RAM1ERKEYR {
        &self.ram1erkeyr
    }
    ///0x40 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2CR {
        &self.m2cr
    }
    ///0x44 - RAMCFG SRAM x interrupt enable register
    #[inline(always)]
    pub const fn m2ier(&self) -> &M2IER {
        &self.m2ier
    }
    ///0x48 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn m2isr(&self) -> &M2ISR {
        &self.m2isr
    }
    ///0x4c - RAMCFG RAM x ECC single error address register
    #[inline(always)]
    pub const fn m2sear(&self) -> &M2SEAR {
        &self.m2sear
    }
    ///0x50 - RAMCFG RAM x ECC double error address register
    #[inline(always)]
    pub const fn m2dear(&self) -> &M2DEAR {
        &self.m2dear
    }
    ///0x54 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn m2icr(&self) -> &M2ICR {
        &self.m2icr
    }
    ///0x58 - RAMCFG SRAM2 write protection register 1
    #[inline(always)]
    pub const fn m2wpr1(&self) -> &M2WPR1 {
        &self.m2wpr1
    }
    ///0x5c - RAMCFG SRAM2 write protection register 2
    #[inline(always)]
    pub const fn m2wpr2(&self) -> &M2WPR2 {
        &self.m2wpr2
    }
    ///0x64 - RAMCFG SRAM x ECC key register
    #[inline(always)]
    pub const fn m2ecckeyr(&self) -> &M2ECCKEYR {
        &self.m2ecckeyr
    }
    ///0x68 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn m2erkeyr(&self) -> &M2ERKEYR {
        &self.m2erkeyr
    }
    ///0x80 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn m3cr(&self) -> &M3CR {
        &self.m3cr
    }
    ///0x84 - RAMCFG SRAM x interrupt enable register
    #[inline(always)]
    pub const fn m3ier(&self) -> &M3IER {
        &self.m3ier
    }
    ///0x88 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn m3isr(&self) -> &M3ISR {
        &self.m3isr
    }
    ///0x8c - RAMCFG RAM x ECC single error address register
    #[inline(always)]
    pub const fn m3sear(&self) -> &M3SEAR {
        &self.m3sear
    }
    ///0x90 - RAMCFG RAM x ECC double error address register
    #[inline(always)]
    pub const fn m3dear(&self) -> &M3DEAR {
        &self.m3dear
    }
    ///0x94 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn m3icr(&self) -> &M3ICR {
        &self.m3icr
    }
    ///0xa4 - RAMCFG SRAM x ECC key register
    #[inline(always)]
    pub const fn m3ecckeyr(&self) -> &M3ECCKEYR {
        &self.m3ecckeyr
    }
    ///0xa8 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn m3erkeyr(&self) -> &M3ERKEYR {
        &self.m3erkeyr
    }
    ///0xc0 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn m4cr(&self) -> &M4CR {
        &self.m4cr
    }
    ///0xc8 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn m4isr(&self) -> &M4ISR {
        &self.m4isr
    }
    ///0xe8 - RAMCFG SRAM x erase key register
    #[inline(always)]
    pub const fn m4erkeyr(&self) -> &M4ERKEYR {
        &self.m4erkeyr
    }
    ///0x100 - RAMCFG SRAM x control register
    #[inline(always)]
    pub const fn m5cr(&self) -> &M5CR {
        &self.m5cr
    }
    ///0x104 - RAMCFG SRAM x interrupt enable register
    #[inline(always)]
    pub const fn m5ier(&self) -> &M5IER {
        &self.m5ier
    }
    ///0x108 - RAMCFG RAMx interrupt status register
    #[inline(always)]
    pub const fn m5isr(&self) -> &M5ISR {
        &self.m5isr
    }
    ///0x10c - RAMCFG RAM x ECC single error address register
    #[inline(always)]
    pub const fn m5sear(&self) -> &M5SEAR {
        &self.m5sear
    }
    ///0x110 - RAMCFG RAM x ECC double error address register
    #[inline(always)]
    pub const fn m5dear(&self) -> &M5DEAR {
        &self.m5dear
    }
    ///0x114 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn m5icr(&self) -> &M5ICR {
        &self.m5icr
    }
    ///0x124 - RAMCFG RAM x interrupt clear register x
    #[inline(always)]
    pub const fn m5ecckeyr(&self) -> &M5ECCKEYR {
        &self.m5ecckeyr
    }
    ///0x128 -
    #[inline(always)]
    pub const fn m5erkeyr(&self) -> &M5ERKEYR {
        &self.m5erkeyr
    }
    ///0x140 - memory x control register
    #[inline(always)]
    pub const fn m6cr(&self) -> &M6CR {
        &self.m6cr
    }
    ///0x148 -
    #[inline(always)]
    pub const fn m6isr(&self) -> &M6ISR {
        &self.m6isr
    }
    ///0x168 -
    #[inline(always)]
    pub const fn m6erkeyr(&self) -> &M6ERKEYR {
        &self.m6erkeyr
    }
}
/**M1CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M1CR)

For information about available fields see [`mod@m1cr`] module*/
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
///RAMCFG SRAM x control register
pub mod m1cr;
/**M1ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M1ISR)

For information about available fields see [`mod@m1isr`] module*/
pub type M1ISR = crate::Reg<m1isr::M1ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod m1isr;
/**RAM1ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:RAM1ERKEYR)

For information about available fields see [`mod@ram1erkeyr`] module*/
pub type RAM1ERKEYR = crate::Reg<ram1erkeyr::RAM1ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod ram1erkeyr;
/**M2CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2CR)

For information about available fields see [`mod@m2cr`] module*/
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
///RAMCFG SRAM x control register
pub mod m2cr;
/**M2IER (rw) register accessor: RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2IER)

For information about available fields see [`mod@m2ier`] module*/
pub type M2IER = crate::Reg<m2ier::M2IERrs>;
///RAMCFG SRAM x interrupt enable register
pub mod m2ier;
/**M2ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2ISR)

For information about available fields see [`mod@m2isr`] module*/
pub type M2ISR = crate::Reg<m2isr::M2ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod m2isr;
/**M2SEAR (r) register accessor: RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m2sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2SEAR)

For information about available fields see [`mod@m2sear`] module*/
pub type M2SEAR = crate::Reg<m2sear::M2SEARrs>;
///RAMCFG RAM x ECC single error address register
pub mod m2sear;
/**M2DEAR (r) register accessor: RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m2dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2DEAR)

For information about available fields see [`mod@m2dear`] module*/
pub type M2DEAR = crate::Reg<m2dear::M2DEARrs>;
///RAMCFG RAM x ECC double error address register
pub mod m2dear;
/**M2ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`m2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2ICR)

For information about available fields see [`mod@m2icr`] module*/
pub type M2ICR = crate::Reg<m2icr::M2ICRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod m2icr;
/**M2WPR1 (rw) register accessor: RAMCFG SRAM2 write protection register 1

You can [`read`](crate::Reg::read) this register and get [`m2wpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2WPR1)

For information about available fields see [`mod@m2wpr1`] module*/
pub type M2WPR1 = crate::Reg<m2wpr1::M2WPR1rs>;
///RAMCFG SRAM2 write protection register 1
pub mod m2wpr1;
/**M2WPR2 (rw) register accessor: RAMCFG SRAM2 write protection register 2

You can [`read`](crate::Reg::read) this register and get [`m2wpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2WPR2)

For information about available fields see [`mod@m2wpr2`] module*/
pub type M2WPR2 = crate::Reg<m2wpr2::M2WPR2rs>;
///RAMCFG SRAM2 write protection register 2
pub mod m2wpr2;
/**M2ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2ECCKEYR)

For information about available fields see [`mod@m2ecckeyr`] module*/
pub type M2ECCKEYR = crate::Reg<m2ecckeyr::M2ECCKEYRrs>;
///RAMCFG SRAM x ECC key register
pub mod m2ecckeyr;
/**M2ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M2ERKEYR)

For information about available fields see [`mod@m2erkeyr`] module*/
pub type M2ERKEYR = crate::Reg<m2erkeyr::M2ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod m2erkeyr;
/**M3CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`m3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3CR)

For information about available fields see [`mod@m3cr`] module*/
pub type M3CR = crate::Reg<m3cr::M3CRrs>;
///RAMCFG SRAM x control register
pub mod m3cr;
/**M3IER (rw) register accessor: RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3IER)

For information about available fields see [`mod@m3ier`] module*/
pub type M3IER = crate::Reg<m3ier::M3IERrs>;
///RAMCFG SRAM x interrupt enable register
pub mod m3ier;
/**M3ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3ISR)

For information about available fields see [`mod@m3isr`] module*/
pub type M3ISR = crate::Reg<m3isr::M3ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod m3isr;
/**M3SEAR (r) register accessor: RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m3sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3SEAR)

For information about available fields see [`mod@m3sear`] module*/
pub type M3SEAR = crate::Reg<m3sear::M3SEARrs>;
///RAMCFG RAM x ECC single error address register
pub mod m3sear;
/**M3DEAR (r) register accessor: RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m3dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3DEAR)

For information about available fields see [`mod@m3dear`] module*/
pub type M3DEAR = crate::Reg<m3dear::M3DEARrs>;
///RAMCFG RAM x ECC double error address register
pub mod m3dear;
/**M3ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`m3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3ICR)

For information about available fields see [`mod@m3icr`] module*/
pub type M3ICR = crate::Reg<m3icr::M3ICRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod m3icr;
/**M3ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3ECCKEYR)

For information about available fields see [`mod@m3ecckeyr`] module*/
pub type M3ECCKEYR = crate::Reg<m3ecckeyr::M3ECCKEYRrs>;
///RAMCFG SRAM x ECC key register
pub mod m3ecckeyr;
/**M3ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3ERKEYR)

For information about available fields see [`mod@m3erkeyr`] module*/
pub type M3ERKEYR = crate::Reg<m3erkeyr::M3ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod m3erkeyr;
/**M4CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`m4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M4CR)

For information about available fields see [`mod@m4cr`] module*/
pub type M4CR = crate::Reg<m4cr::M4CRrs>;
///RAMCFG SRAM x control register
pub mod m4cr;
/**M4ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M4ISR)

For information about available fields see [`mod@m4isr`] module*/
pub type M4ISR = crate::Reg<m4isr::M4ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod m4isr;
/**M4ERKEYR (w) register accessor: RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M4ERKEYR)

For information about available fields see [`mod@m4erkeyr`] module*/
pub type M4ERKEYR = crate::Reg<m4erkeyr::M4ERKEYRrs>;
///RAMCFG SRAM x erase key register
pub mod m4erkeyr;
/**M5CR (rw) register accessor: RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`m5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5CR)

For information about available fields see [`mod@m5cr`] module*/
pub type M5CR = crate::Reg<m5cr::M5CRrs>;
///RAMCFG SRAM x control register
pub mod m5cr;
/**M5IER (rw) register accessor: RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5IER)

For information about available fields see [`mod@m5ier`] module*/
pub type M5IER = crate::Reg<m5ier::M5IERrs>;
///RAMCFG SRAM x interrupt enable register
pub mod m5ier;
/**M5ISR (r) register accessor: RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5ISR)

For information about available fields see [`mod@m5isr`] module*/
pub type M5ISR = crate::Reg<m5isr::M5ISRrs>;
///RAMCFG RAMx interrupt status register
pub mod m5isr;
/**M5SEAR (r) register accessor: RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m5sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5SEAR)

For information about available fields see [`mod@m5sear`] module*/
pub type M5SEAR = crate::Reg<m5sear::M5SEARrs>;
///RAMCFG RAM x ECC single error address register
pub mod m5sear;
/**M5DEAR (r) register accessor: RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m5dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5DEAR)

For information about available fields see [`mod@m5dear`] module*/
pub type M5DEAR = crate::Reg<m5dear::M5DEARrs>;
///RAMCFG RAM x ECC double error address register
pub mod m5dear;
/**M5ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`m5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5ICR)

For information about available fields see [`mod@m5icr`] module*/
pub type M5ICR = crate::Reg<m5icr::M5ICRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod m5icr;
/**M5ECCKEYR (rw) register accessor: RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`m5ecckeyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ecckeyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5ECCKEYR)

For information about available fields see [`mod@m5ecckeyr`] module*/
pub type M5ECCKEYR = crate::Reg<m5ecckeyr::M5ECCKEYRrs>;
///RAMCFG RAM x interrupt clear register x
pub mod m5ecckeyr;
/**M5ERKEYR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`m5erkeyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5erkeyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M5ERKEYR)

For information about available fields see [`mod@m5erkeyr`] module*/
pub type M5ERKEYR = crate::Reg<m5erkeyr::M5ERKEYRrs>;
///
pub mod m5erkeyr;
/**M6CR (rw) register accessor: memory x control register

You can [`read`](crate::Reg::read) this register and get [`m6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M6CR)

For information about available fields see [`mod@m6cr`] module*/
pub type M6CR = crate::Reg<m6cr::M6CRrs>;
///memory x control register
pub mod m6cr;
/**M6ISR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`m6isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M6ISR)

For information about available fields see [`mod@m6isr`] module*/
pub type M6ISR = crate::Reg<m6isr::M6ISRrs>;
///
pub mod m6isr;
/**M6ERKEYR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`m6erkeyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6erkeyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M6ERKEYR)

For information about available fields see [`mod@m6erkeyr`] module*/
pub type M6ERKEYR = crate::Reg<m6erkeyr::M6ERKEYRrs>;
///
pub mod m6erkeyr;
