#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    ier: IER,
    icr: ICR,
    isr: ISR,
    iohcr: IOHCR,
    _reserved5: [u8; 0x04],
    ioascr: IOASCR,
    _reserved6: [u8; 0x04],
    ioscr: IOSCR,
    _reserved7: [u8; 0x04],
    ioccr: IOCCR,
    _reserved8: [u8; 0x04],
    iogcsr: IOGCSR,
    iogcr: [IOGCR; 8],
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x08 - interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x0c - interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10 - I/O hysteresis control register
    #[inline(always)]
    pub const fn iohcr(&self) -> &IOHCR {
        &self.iohcr
    }
    ///0x18 - I/O analog switch control register
    #[inline(always)]
    pub const fn ioascr(&self) -> &IOASCR {
        &self.ioascr
    }
    ///0x20 - I/O sampling control register
    #[inline(always)]
    pub const fn ioscr(&self) -> &IOSCR {
        &self.ioscr
    }
    ///0x28 - I/O channel control register
    #[inline(always)]
    pub const fn ioccr(&self) -> &IOCCR {
        &self.ioccr
    }
    ///0x30 - I/O group control status register
    #[inline(always)]
    pub const fn iogcsr(&self) -> &IOGCSR {
        &self.iogcsr
    }
    ///0x34..0x54 - I/O group x counter register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `IOG1CR` register.</div>
    #[inline(always)]
    pub const fn iogcr(&self, n: usize) -> &IOGCR {
        &self.iogcr[n]
    }
    ///Iterator for array of:
    ///0x34..0x54 - I/O group x counter register
    #[inline(always)]
    pub fn iogcr_iter(&self) -> impl Iterator<Item = &IOGCR> {
        self.iogcr.iter()
    }
    ///0x34 - I/O group x counter register
    #[inline(always)]
    pub const fn iog1cr(&self) -> &IOGCR {
        self.iogcr(0)
    }
    ///0x38 - I/O group x counter register
    #[inline(always)]
    pub const fn iog2cr(&self) -> &IOGCR {
        self.iogcr(1)
    }
    ///0x3c - I/O group x counter register
    #[inline(always)]
    pub const fn iog3cr(&self) -> &IOGCR {
        self.iogcr(2)
    }
    ///0x40 - I/O group x counter register
    #[inline(always)]
    pub const fn iog4cr(&self) -> &IOGCR {
        self.iogcr(3)
    }
    ///0x44 - I/O group x counter register
    #[inline(always)]
    pub const fn iog5cr(&self) -> &IOGCR {
        self.iogcr(4)
    }
    ///0x48 - I/O group x counter register
    #[inline(always)]
    pub const fn iog6cr(&self) -> &IOGCR {
        self.iogcr(5)
    }
    ///0x4c - I/O group x counter register
    #[inline(always)]
    pub const fn iog7cr(&self) -> &IOGCR {
        self.iogcr(6)
    }
    ///0x50 - I/O group x counter register
    #[inline(always)]
    pub const fn iog8cr(&self) -> &IOGCR {
        self.iogcr(7)
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**IER (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///interrupt enable register
pub mod ier;
/**ICR (rw) register accessor: interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///interrupt clear register
pub mod icr;
/**ISR (r) register accessor: interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///interrupt status register
pub mod isr;
/**IOHCR (rw) register accessor: I/O hysteresis control register

You can [`read`](crate::Reg::read) this register and get [`iohcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iohcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOHCR)

For information about available fields see [`mod@iohcr`] module*/
pub type IOHCR = crate::Reg<iohcr::IOHCRrs>;
///I/O hysteresis control register
pub mod iohcr;
/**IOASCR (rw) register accessor: I/O analog switch control register

You can [`read`](crate::Reg::read) this register and get [`ioascr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioascr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOASCR)

For information about available fields see [`mod@ioascr`] module*/
pub type IOASCR = crate::Reg<ioascr::IOASCRrs>;
///I/O analog switch control register
pub mod ioascr;
/**IOSCR (rw) register accessor: I/O sampling control register

You can [`read`](crate::Reg::read) this register and get [`ioscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOSCR)

For information about available fields see [`mod@ioscr`] module*/
pub type IOSCR = crate::Reg<ioscr::IOSCRrs>;
///I/O sampling control register
pub mod ioscr;
/**IOCCR (rw) register accessor: I/O channel control register

You can [`read`](crate::Reg::read) this register and get [`ioccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOCCR)

For information about available fields see [`mod@ioccr`] module*/
pub type IOCCR = crate::Reg<ioccr::IOCCRrs>;
///I/O channel control register
pub mod ioccr;
/**IOGCSR (rw) register accessor: I/O group control status register

You can [`read`](crate::Reg::read) this register and get [`iogcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iogcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOGCSR)

For information about available fields see [`mod@iogcsr`] module*/
pub type IOGCSR = crate::Reg<iogcsr::IOGCSRrs>;
///I/O group control status register
pub mod iogcsr;
/**IOGCR (r) register accessor: I/O group x counter register

You can [`read`](crate::Reg::read) this register and get [`iogcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOG[1]CR)

For information about available fields see [`mod@iogcr`] module*/
pub type IOGCR = crate::Reg<iogcr::IOGCRrs>;
///I/O group x counter register
pub mod iogcr;
