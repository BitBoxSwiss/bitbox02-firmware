#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
    cfgr: CFGR,
    cr: CR,
    ccr1: CCR1,
    arr: ARR,
    cnt: CNT,
    _reserved8: [u8; 0x04],
    cfgr2: CFGR2,
    rcr: RCR,
    ccmr1: CCMR1,
    _reserved11: [u8; 0x04],
    ccr2: CCR2,
}
impl RegisterBlock {
    ///0x00 - Interrupt and Status Register (intput mode)
    #[inline(always)]
    pub const fn isr_input(&self) -> &ISR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Interrupt and Status Register (output mode)
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - Interrupt Clear Register (intput mode)
    #[inline(always)]
    pub const fn icr_input(&self) -> &ICR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Interrupt Clear Register (output mode)
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM interrupt Enable Register (intput mode)
    #[inline(always)]
    pub const fn dier_input(&self) -> &DIER_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM interrupt Enable Register (output mode)
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - Configuration Register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x14 - Compare Register
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    ///0x18 - Autoreload Register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x1c - Counter Register
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x28 - LPTIM repetition register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    ///0x2c - LPTIM capture/compare mode register 1
    #[inline(always)]
    pub const fn ccmr1(&self) -> &CCMR1 {
        &self.ccmr1
    }
    ///0x34 - LPTIM Compare Register 2
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
}
/**ISR_output (r) register accessor: Interrupt and Status Register (output mode)

You can [`read`](crate::Reg::read) this register and get [`isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:ISR_output)

For information about available fields see [`mod@isr_output`] module*/
#[doc(alias = "ISR_output")]
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUTrs>;
///Interrupt and Status Register (output mode)
pub mod isr_output;
/**ISR_input (r) register accessor: Interrupt and Status Register (intput mode)

You can [`read`](crate::Reg::read) this register and get [`isr_input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:ISR_input)

For information about available fields see [`mod@isr_input`] module*/
#[doc(alias = "ISR_input")]
pub type ISR_INPUT = crate::Reg<isr_input::ISR_INPUTrs>;
///Interrupt and Status Register (intput mode)
pub mod isr_input;
/**ICR_output (w) register accessor: Interrupt Clear Register (output mode)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:ICR_output)

For information about available fields see [`mod@icr_output`] module*/
#[doc(alias = "ICR_output")]
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUTrs>;
///Interrupt Clear Register (output mode)
pub mod icr_output;
/**ICR_input (w) register accessor: Interrupt Clear Register (intput mode)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:ICR_input)

For information about available fields see [`mod@icr_input`] module*/
#[doc(alias = "ICR_input")]
pub type ICR_INPUT = crate::Reg<icr_input::ICR_INPUTrs>;
///Interrupt Clear Register (intput mode)
pub mod icr_input;
/**DIER_output (rw) register accessor: LPTIM interrupt Enable Register (output mode)

You can [`read`](crate::Reg::read) this register and get [`dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:DIER_output)

For information about available fields see [`mod@dier_output`] module*/
#[doc(alias = "DIER_output")]
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUTrs>;
///LPTIM interrupt Enable Register (output mode)
pub mod dier_output;
/**DIER_input (rw) register accessor: LPTIM interrupt Enable Register (intput mode)

You can [`read`](crate::Reg::read) this register and get [`dier_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:DIER_input)

For information about available fields see [`mod@dier_input`] module*/
#[doc(alias = "DIER_input")]
pub type DIER_INPUT = crate::Reg<dier_input::DIER_INPUTrs>;
///LPTIM interrupt Enable Register (intput mode)
pub mod dier_input;
/**CFGR (rw) register accessor: Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Configuration Register
pub mod cfgr;
/**CR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Control Register
pub mod cr;
/**CCR1 (rw) register accessor: Compare Register

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CCR1)

For information about available fields see [`mod@ccr1`] module*/
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
///Compare Register
pub mod ccr1;
/**ARR (rw) register accessor: Autoreload Register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///Autoreload Register
pub mod arr;
/**CNT (r) register accessor: Counter Register

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///Counter Register
pub mod cnt;
/**CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///LPTIM configuration register 2
pub mod cfgr2;
/**RCR (rw) register accessor: LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///LPTIM repetition register
pub mod rcr;
/**CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CCMR1)

For information about available fields see [`mod@ccmr1`] module*/
pub type CCMR1 = crate::Reg<ccmr1::CCMR1rs>;
///LPTIM capture/compare mode register 1
pub mod ccmr1;
/**CCR2 (rw) register accessor: LPTIM Compare Register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPTIM1:CCR2)

For information about available fields see [`mod@ccr2`] module*/
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
///LPTIM Compare Register 2
pub mod ccr2;
