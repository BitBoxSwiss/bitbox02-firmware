#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "1-channel timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1ch {
    ptr: *mut u8,
}
unsafe impl Send for Tim1ch {}
unsafe impl Sync for Tim1ch {}
impl Tim1ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr1ch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1) (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::Tisel1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
}
#[doc = "1-channel with one complementary output timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1chCmp {
    ptr: *mut u8,
}
unsafe impl Send for Tim1chCmp {}
unsafe impl Sync for Tim1chCmp {}
impl Tim1chCmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr1chCmp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "capture/compare register x (x=1) (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn dtr2(self) -> crate::common::Reg<regs::Dtr21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::Tisel1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "alternate function register 1"]
    #[inline(always)]
    pub const fn af1(self) -> crate::common::Reg<regs::Af11chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "alternate function register 2"]
    #[inline(always)]
    pub const fn af2(self) -> crate::common::Reg<regs::Af21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
}
#[doc = "2-channel timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim2ch {
    ptr: *mut u8,
}
unsafe impl Send for Tim2ch {}
unsafe impl Sync for Tim2ch {}
impl Tim2ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr22ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr2ch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput2ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2) (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::Tisel2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
}
#[doc = "2-channel with one complementary output timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim2chCmp {
    ptr: *mut u8,
}
unsafe impl Send for Tim2chCmp {}
unsafe impl Sync for Tim2chCmp {}
impl Tim2chCmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr22chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr2chCmp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn dtr2(self) -> crate::common::Reg<regs::Dtr21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::Tisel2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "alternate function register 1"]
    #[inline(always)]
    pub const fn af1(self) -> crate::common::Reg<regs::Af11chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "alternate function register 2"]
    #[inline(always)]
    pub const fn af2(self) -> crate::common::Reg<regs::Af21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
}
#[doc = "Advanced Control timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimAdv {
    ptr: *mut u8,
}
unsafe impl Send for TimAdv {}
unsafe impl Sync for TimAdv {}
impl TimAdv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::SmcrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrAdv, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutputGp16, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::RcrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::BdtrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "capture/compare register 5 (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr5(self) -> crate::common::Reg<regs::Ccr5Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "capture/compare register 5 (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr5_dither(self) -> crate::common::Reg<regs::Ccr5DitherAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "capture/compare register 6 (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr6(self) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "capture/compare register 6 (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr6_dither(self) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "capture/compare mode register 3"]
    #[inline(always)]
    pub const fn ccmr3(self) -> crate::common::Reg<regs::Ccmr3Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn dtr2(self) -> crate::common::Reg<regs::Dtr21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "encoder control register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::EcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::TiselGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "alternate function register 1"]
    #[inline(always)]
    pub const fn af1(self) -> crate::common::Reg<regs::Af1Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "alternate function register 2"]
    #[inline(always)]
    pub const fn af2(self) -> crate::common::Reg<regs::Af2Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
}
#[doc = "Basic timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasic {
    ptr: *mut u8,
}
unsafe impl Send for TimBasic {}
unsafe impl Sync for TimBasic {}
impl TimBasic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Basic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasicNoCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "Virtual Basic timers without CR2 register for common part of TIM_BASIC and TIM_1CH_CMP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasicNoCr2 {
    ptr: *mut u8,
}
unsafe impl Send for TimBasicNoCr2 {}
unsafe impl Sync for TimBasicNoCr2 {}
impl TimBasicNoCr2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasicNoCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "Virtual timer for common part of TIM_BASIC and TIM_1CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimCore {
    ptr: *mut u8,
}
unsafe impl Send for TimCore {}
unsafe impl Sync for TimCore {}
impl TimCore {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "General purpose 16-bit timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimGp16 {
    ptr: *mut u8,
}
unsafe impl Send for TimGp16 {}
unsafe impl Sync for TimGp16 {}
impl TimGp16 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::SmcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrGp16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutputGp16, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4) (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDither1ch, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "encoder control register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::EcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::TiselGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "alternate function register 1"]
    #[inline(always)]
    pub const fn af1(self) -> crate::common::Reg<regs::Af1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "alternate function register 2"]
    #[inline(always)]
    pub const fn af2(self) -> crate::common::Reg<regs::Af21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
}
#[doc = "General purpose 32-bit timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimGp32 {
    ptr: *mut u8,
}
unsafe impl Send for TimGp32 {}
unsafe impl Sync for TimGp32 {}
impl TimGp32 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::SmcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrGp16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutputGp16, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter (Dither mode disabled)"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "counter (Dither mode enbled)"]
    #[inline(always)]
    pub const fn cnt_dither(self) -> crate::common::Reg<regs::CntDitherGp32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[inline(always)]
    pub const fn arr_dither(self) -> crate::common::Reg<regs::ArrDitherGp32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4) (Dither mode disabled)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4) (Dither mode enabled)"]
    #[inline(always)]
    pub const fn ccr_dither(self, n: usize) -> crate::common::Reg<regs::CcrDitherGp32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "encoder control register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::EcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "input selection register"]
    #[inline(always)]
    pub const fn tisel(self) -> crate::common::Reg<regs::TiselGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "alternate function register 1"]
    #[inline(always)]
    pub const fn af1(self) -> crate::common::Reg<regs::Af1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "alternate function register 2"]
    #[inline(always)]
    pub const fn af2(self) -> crate::common::Reg<regs::Af21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
}
pub mod regs {
    #[doc = "alternate function register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af11chCmp(pub u32);
    impl Af11chCmp {
        #[doc = "TIMx_BKIN input enable"]
        #[inline(always)]
        pub const fn bkine(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMx_BKIN input enable"]
        #[inline(always)]
        pub fn set_bkine(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM_BRK_CMPx (x=1-8) enable"]
        #[inline(always)]
        pub const fn bkcmpe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TIM_BRK_CMPx (x=1-8) enable"]
        #[inline(always)]
        pub fn set_bkcmpe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TIMx_BKIN input polarity"]
        #[inline(always)]
        pub const fn bkinp(&self) -> super::vals::Bkinp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Bkinp::from_bits(val as u8)
        }
        #[doc = "TIMx_BKIN input polarity"]
        #[inline(always)]
        pub fn set_bkinp(&mut self, val: super::vals::Bkinp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM_BRK_CMPx (x=1-4) input polarity"]
        #[inline(always)]
        pub const fn bkcmpp(&self, n: usize) -> super::vals::Bkinp {
            assert!(n < 4usize);
            let offs = 10usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkinp::from_bits(val as u8)
        }
        #[doc = "TIM_BRK_CMPx (x=1-4) input polarity"]
        #[inline(always)]
        pub fn set_bkcmpp(&mut self, n: usize, val: super::vals::Bkinp) {
            assert!(n < 4usize);
            let offs = 10usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Af11chCmp {
        #[inline(always)]
        fn default() -> Af11chCmp {
            Af11chCmp(0)
        }
    }
    impl core::fmt::Debug for Af11chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af11chCmp")
                .field("bkine", &self.bkine())
                .field("bkcmpe[0]", &self.bkcmpe(0usize))
                .field("bkcmpe[1]", &self.bkcmpe(1usize))
                .field("bkcmpe[2]", &self.bkcmpe(2usize))
                .field("bkcmpe[3]", &self.bkcmpe(3usize))
                .field("bkcmpe[4]", &self.bkcmpe(4usize))
                .field("bkcmpe[5]", &self.bkcmpe(5usize))
                .field("bkcmpe[6]", &self.bkcmpe(6usize))
                .field("bkcmpe[7]", &self.bkcmpe(7usize))
                .field("bkinp", &self.bkinp())
                .field("bkcmpp[0]", &self.bkcmpp(0usize))
                .field("bkcmpp[1]", &self.bkcmpp(1usize))
                .field("bkcmpp[2]", &self.bkcmpp(2usize))
                .field("bkcmpp[3]", &self.bkcmpp(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af11chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Af11chCmp {{ bkine: {=bool:?}, bkcmpe[0]: {=bool:?}, bkcmpe[1]: {=bool:?}, bkcmpe[2]: {=bool:?}, bkcmpe[3]: {=bool:?}, bkcmpe[4]: {=bool:?}, bkcmpe[5]: {=bool:?}, bkcmpe[6]: {=bool:?}, bkcmpe[7]: {=bool:?}, bkinp: {:?}, bkcmpp[0]: {:?}, bkcmpp[1]: {:?}, bkcmpp[2]: {:?}, bkcmpp[3]: {:?} }}" , self . bkine () , self . bkcmpe (0usize) , self . bkcmpe (1usize) , self . bkcmpe (2usize) , self . bkcmpe (3usize) , self . bkcmpe (4usize) , self . bkcmpe (5usize) , self . bkcmpe (6usize) , self . bkcmpe (7usize) , self . bkinp () , self . bkcmpp (0usize) , self . bkcmpp (1usize) , self . bkcmpp (2usize) , self . bkcmpp (3usize))
        }
    }
    #[doc = "alternate function register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af1Adv(pub u32);
    impl Af1Adv {
        #[doc = "TIMx_BKIN input enable"]
        #[inline(always)]
        pub const fn bkine(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMx_BKIN input enable"]
        #[inline(always)]
        pub fn set_bkine(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM_BRK_CMPx (x=1-8) enable"]
        #[inline(always)]
        pub const fn bkcmpe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TIM_BRK_CMPx (x=1-8) enable"]
        #[inline(always)]
        pub fn set_bkcmpe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TIMx_BKIN input polarity"]
        #[inline(always)]
        pub const fn bkinp(&self) -> super::vals::Bkinp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Bkinp::from_bits(val as u8)
        }
        #[doc = "TIMx_BKIN input polarity"]
        #[inline(always)]
        pub fn set_bkinp(&mut self, val: super::vals::Bkinp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM_BRK_CMPx (x=1-4) input polarity"]
        #[inline(always)]
        pub const fn bkcmpp(&self, n: usize) -> super::vals::Bkinp {
            assert!(n < 4usize);
            let offs = 10usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkinp::from_bits(val as u8)
        }
        #[doc = "TIM_BRK_CMPx (x=1-4) input polarity"]
        #[inline(always)]
        pub fn set_bkcmpp(&mut self, n: usize, val: super::vals::Bkinp) {
            assert!(n < 4usize);
            let offs = 10usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "etr_in source selection"]
        #[inline(always)]
        pub const fn etrsel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "etr_in source selection"]
        #[inline(always)]
        pub fn set_etrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
    }
    impl Default for Af1Adv {
        #[inline(always)]
        fn default() -> Af1Adv {
            Af1Adv(0)
        }
    }
    impl core::fmt::Debug for Af1Adv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af1Adv")
                .field("bkine", &self.bkine())
                .field("bkcmpe[0]", &self.bkcmpe(0usize))
                .field("bkcmpe[1]", &self.bkcmpe(1usize))
                .field("bkcmpe[2]", &self.bkcmpe(2usize))
                .field("bkcmpe[3]", &self.bkcmpe(3usize))
                .field("bkcmpe[4]", &self.bkcmpe(4usize))
                .field("bkcmpe[5]", &self.bkcmpe(5usize))
                .field("bkcmpe[6]", &self.bkcmpe(6usize))
                .field("bkcmpe[7]", &self.bkcmpe(7usize))
                .field("bkinp", &self.bkinp())
                .field("bkcmpp[0]", &self.bkcmpp(0usize))
                .field("bkcmpp[1]", &self.bkcmpp(1usize))
                .field("bkcmpp[2]", &self.bkcmpp(2usize))
                .field("bkcmpp[3]", &self.bkcmpp(3usize))
                .field("etrsel", &self.etrsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af1Adv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Af1Adv {{ bkine: {=bool:?}, bkcmpe[0]: {=bool:?}, bkcmpe[1]: {=bool:?}, bkcmpe[2]: {=bool:?}, bkcmpe[3]: {=bool:?}, bkcmpe[4]: {=bool:?}, bkcmpe[5]: {=bool:?}, bkcmpe[6]: {=bool:?}, bkcmpe[7]: {=bool:?}, bkinp: {:?}, bkcmpp[0]: {:?}, bkcmpp[1]: {:?}, bkcmpp[2]: {:?}, bkcmpp[3]: {:?}, etrsel: {=u8:?} }}" , self . bkine () , self . bkcmpe (0usize) , self . bkcmpe (1usize) , self . bkcmpe (2usize) , self . bkcmpe (3usize) , self . bkcmpe (4usize) , self . bkcmpe (5usize) , self . bkcmpe (6usize) , self . bkcmpe (7usize) , self . bkinp () , self . bkcmpp (0usize) , self . bkcmpp (1usize) , self . bkcmpp (2usize) , self . bkcmpp (3usize) , self . etrsel ())
        }
    }
    #[doc = "alternate function register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af1Gp16(pub u32);
    impl Af1Gp16 {
        #[doc = "etr_in source selection"]
        #[inline(always)]
        pub const fn etrsel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "etr_in source selection"]
        #[inline(always)]
        pub fn set_etrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
    }
    impl Default for Af1Gp16 {
        #[inline(always)]
        fn default() -> Af1Gp16 {
            Af1Gp16(0)
        }
    }
    impl core::fmt::Debug for Af1Gp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af1Gp16").field("etrsel", &self.etrsel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af1Gp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Af1Gp16 {{ etrsel: {=u8:?} }}", self.etrsel())
        }
    }
    #[doc = "alternate function register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af21chCmp(pub u32);
    impl Af21chCmp {
        #[doc = "ocref_clr source selection"]
        #[inline(always)]
        pub const fn ocrsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "ocref_clr source selection"]
        #[inline(always)]
        pub fn set_ocrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Af21chCmp {
        #[inline(always)]
        fn default() -> Af21chCmp {
            Af21chCmp(0)
        }
    }
    impl core::fmt::Debug for Af21chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af21chCmp").field("ocrsel", &self.ocrsel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af21chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Af21chCmp {{ ocrsel: {=u8:?} }}", self.ocrsel())
        }
    }
    #[doc = "alternate function register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af2Adv(pub u32);
    impl Af2Adv {
        #[doc = "TIMx_BKIN2 input enable"]
        #[inline(always)]
        pub const fn bk2ine(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMx_BKIN2 input enable"]
        #[inline(always)]
        pub fn set_bk2ine(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM_BRK2_CMPx (x=1-8) enable"]
        #[inline(always)]
        pub const fn bk2cmpe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TIM_BRK2_CMPx (x=1-8) enable"]
        #[inline(always)]
        pub fn set_bk2cmpe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TIMx_BK2IN input polarity"]
        #[inline(always)]
        pub const fn bk2inp(&self) -> super::vals::Bkinp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Bkinp::from_bits(val as u8)
        }
        #[doc = "TIMx_BK2IN input polarity"]
        #[inline(always)]
        pub fn set_bk2inp(&mut self, val: super::vals::Bkinp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM_BRK2_CMPx (x=1-4) input polarity"]
        #[inline(always)]
        pub const fn bk2cmpp(&self, n: usize) -> super::vals::Bkinp {
            assert!(n < 4usize);
            let offs = 10usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkinp::from_bits(val as u8)
        }
        #[doc = "TIM_BRK2_CMPx (x=1-4) input polarity"]
        #[inline(always)]
        pub fn set_bk2cmpp(&mut self, n: usize, val: super::vals::Bkinp) {
            assert!(n < 4usize);
            let offs = 10usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "ocref_clr source selection"]
        #[inline(always)]
        pub const fn ocrsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "ocref_clr source selection"]
        #[inline(always)]
        pub fn set_ocrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Af2Adv {
        #[inline(always)]
        fn default() -> Af2Adv {
            Af2Adv(0)
        }
    }
    impl core::fmt::Debug for Af2Adv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af2Adv")
                .field("bk2ine", &self.bk2ine())
                .field("bk2cmpe[0]", &self.bk2cmpe(0usize))
                .field("bk2cmpe[1]", &self.bk2cmpe(1usize))
                .field("bk2cmpe[2]", &self.bk2cmpe(2usize))
                .field("bk2cmpe[3]", &self.bk2cmpe(3usize))
                .field("bk2cmpe[4]", &self.bk2cmpe(4usize))
                .field("bk2cmpe[5]", &self.bk2cmpe(5usize))
                .field("bk2cmpe[6]", &self.bk2cmpe(6usize))
                .field("bk2cmpe[7]", &self.bk2cmpe(7usize))
                .field("bk2inp", &self.bk2inp())
                .field("bk2cmpp[0]", &self.bk2cmpp(0usize))
                .field("bk2cmpp[1]", &self.bk2cmpp(1usize))
                .field("bk2cmpp[2]", &self.bk2cmpp(2usize))
                .field("bk2cmpp[3]", &self.bk2cmpp(3usize))
                .field("ocrsel", &self.ocrsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af2Adv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Af2Adv {{ bk2ine: {=bool:?}, bk2cmpe[0]: {=bool:?}, bk2cmpe[1]: {=bool:?}, bk2cmpe[2]: {=bool:?}, bk2cmpe[3]: {=bool:?}, bk2cmpe[4]: {=bool:?}, bk2cmpe[5]: {=bool:?}, bk2cmpe[6]: {=bool:?}, bk2cmpe[7]: {=bool:?}, bk2inp: {:?}, bk2cmpp[0]: {:?}, bk2cmpp[1]: {:?}, bk2cmpp[2]: {:?}, bk2cmpp[3]: {:?}, ocrsel: {=u8:?} }}" , self . bk2ine () , self . bk2cmpe (0usize) , self . bk2cmpe (1usize) , self . bk2cmpe (2usize) , self . bk2cmpe (3usize) , self . bk2cmpe (4usize) , self . bk2cmpe (5usize) , self . bk2cmpe (6usize) , self . bk2cmpe (7usize) , self . bk2inp () , self . bk2cmpp (0usize) , self . bk2cmpp (1usize) , self . bk2cmpp (2usize) , self . bk2cmpp (3usize) , self . ocrsel ())
        }
    }
    #[doc = "auto-reload register (Dither mode disabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ArrCore(pub u32);
    impl ArrCore {
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ArrCore {
        #[inline(always)]
        fn default() -> ArrCore {
            ArrCore(0)
        }
    }
    impl core::fmt::Debug for ArrCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ArrCore").field("arr", &self.arr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ArrCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ArrCore {{ arr: {=u16:?} }}", self.arr())
        }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ArrDitherCore(pub u32);
    impl ArrDitherCore {
        #[doc = "Dither value"]
        #[inline(always)]
        pub const fn dither(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Dither value"]
        #[inline(always)]
        pub fn set_dither(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 4usize)) | (((val as u32) & 0xffff) << 4usize);
        }
    }
    impl Default for ArrDitherCore {
        #[inline(always)]
        fn default() -> ArrDitherCore {
            ArrDitherCore(0)
        }
    }
    impl core::fmt::Debug for ArrDitherCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ArrDitherCore")
                .field("dither", &self.dither())
                .field("arr", &self.arr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ArrDitherCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ArrDitherCore {{ dither: {=u8:?}, arr: {=u16:?} }}",
                self.dither(),
                self.arr()
            )
        }
    }
    #[doc = "auto-reload register (Dither mode enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ArrDitherGp32(pub u32);
    impl ArrDitherGp32 {
        #[doc = "Dither value"]
        #[inline(always)]
        pub const fn dither(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Dither value"]
        #[inline(always)]
        pub fn set_dither(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn arr(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for ArrDitherGp32 {
        #[inline(always)]
        fn default() -> ArrDitherGp32 {
            ArrDitherGp32(0)
        }
    }
    impl core::fmt::Debug for ArrDitherGp32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ArrDitherGp32")
                .field("dither", &self.dither())
                .field("arr", &self.arr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ArrDitherGp32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ArrDitherGp32 {{ dither: {=u8:?}, arr: {=u32:?} }}",
                self.dither(),
                self.arr()
            )
        }
    }
    #[doc = "break and dead-time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdtr1chCmp(pub u32);
    impl Bdtr1chCmp {
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::Lock {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lock::from_bits(val as u8)
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: super::vals::Lock) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub const fn ossi(&self) -> super::vals::Ossi {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Ossi::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub fn set_ossi(&mut self, val: super::vals::Ossi) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub const fn ossr(&self) -> super::vals::Ossr {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ossr::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub fn set_ossr(&mut self, val: super::vals::Ossr) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Break x (x=1) enable"]
        #[inline(always)]
        pub const fn bke(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 12usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) enable"]
        #[inline(always)]
        pub fn set_bke(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 12usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Break x (x=1) polarity"]
        #[inline(always)]
        pub const fn bkp(&self, n: usize) -> super::vals::Bkp {
            assert!(n < 1usize);
            let offs = 13usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkp::from_bits(val as u8)
        }
        #[doc = "Break x (x=1) polarity"]
        #[inline(always)]
        pub fn set_bkp(&mut self, n: usize, val: super::vals::Bkp) {
            assert!(n < 1usize);
            let offs = 13usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Break x (x=1) filter"]
        #[inline(always)]
        pub const fn bkf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 1usize);
            let offs = 16usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Break x (x=1) filter"]
        #[inline(always)]
        pub fn set_bkf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 1usize);
            let offs = 16usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
        #[doc = "Break x (x=1) Disarm"]
        #[inline(always)]
        pub const fn bkdsrm(&self, n: usize) -> super::vals::Bkdsrm {
            assert!(n < 1usize);
            let offs = 26usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkdsrm::from_bits(val as u8)
        }
        #[doc = "Break x (x=1) Disarm"]
        #[inline(always)]
        pub fn set_bkdsrm(&mut self, n: usize, val: super::vals::Bkdsrm) {
            assert!(n < 1usize);
            let offs = 26usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Break x (x=1) bidirectional"]
        #[inline(always)]
        pub const fn bkbid(&self, n: usize) -> super::vals::Bkbid {
            assert!(n < 1usize);
            let offs = 28usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkbid::from_bits(val as u8)
        }
        #[doc = "Break x (x=1) bidirectional"]
        #[inline(always)]
        pub fn set_bkbid(&mut self, n: usize, val: super::vals::Bkbid) {
            assert!(n < 1usize);
            let offs = 28usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Bdtr1chCmp {
        #[inline(always)]
        fn default() -> Bdtr1chCmp {
            Bdtr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Bdtr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdtr1chCmp")
                .field("dtg", &self.dtg())
                .field("lock", &self.lock())
                .field("ossi", &self.ossi())
                .field("ossr", &self.ossr())
                .field("bke[0]", &self.bke(0usize))
                .field("bkp[0]", &self.bkp(0usize))
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .field("bkf[0]", &self.bkf(0usize))
                .field("bkdsrm[0]", &self.bkdsrm(0usize))
                .field("bkbid[0]", &self.bkbid(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdtr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bdtr1chCmp {{ dtg: {=u8:?}, lock: {:?}, ossi: {:?}, ossr: {:?}, bke[0]: {=bool:?}, bkp[0]: {:?}, aoe: {=bool:?}, moe: {=bool:?}, bkf[0]: {:?}, bkdsrm[0]: {:?}, bkbid[0]: {:?} }}" , self . dtg () , self . lock () , self . ossi () , self . ossr () , self . bke (0usize) , self . bkp (0usize) , self . aoe () , self . moe () , self . bkf (0usize) , self . bkdsrm (0usize) , self . bkbid (0usize))
        }
    }
    #[doc = "break and dead-time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BdtrAdv(pub u32);
    impl BdtrAdv {
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::Lock {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lock::from_bits(val as u8)
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: super::vals::Lock) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub const fn ossi(&self) -> super::vals::Ossi {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Ossi::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub fn set_ossi(&mut self, val: super::vals::Ossi) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub const fn ossr(&self) -> super::vals::Ossr {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ossr::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub fn set_ossr(&mut self, val: super::vals::Ossr) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Break x (x=1,2) enable"]
        #[inline(always)]
        pub const fn bke(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1,2) enable"]
        #[inline(always)]
        pub fn set_bke(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Break x (x=1,2) polarity"]
        #[inline(always)]
        pub const fn bkp(&self, n: usize) -> super::vals::Bkp {
            assert!(n < 2usize);
            let offs = 13usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkp::from_bits(val as u8)
        }
        #[doc = "Break x (x=1,2) polarity"]
        #[inline(always)]
        pub fn set_bkp(&mut self, n: usize, val: super::vals::Bkp) {
            assert!(n < 2usize);
            let offs = 13usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Break x (x=1,2) filter"]
        #[inline(always)]
        pub const fn bkf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Break x (x=1,2) filter"]
        #[inline(always)]
        pub fn set_bkf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
        #[doc = "Break x (x=1,2) Disarm"]
        #[inline(always)]
        pub const fn bkdsrm(&self, n: usize) -> super::vals::Bkdsrm {
            assert!(n < 2usize);
            let offs = 26usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkdsrm::from_bits(val as u8)
        }
        #[doc = "Break x (x=1,2) Disarm"]
        #[inline(always)]
        pub fn set_bkdsrm(&mut self, n: usize, val: super::vals::Bkdsrm) {
            assert!(n < 2usize);
            let offs = 26usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Break x (x=1,2) bidirectional"]
        #[inline(always)]
        pub const fn bkbid(&self, n: usize) -> super::vals::Bkbid {
            assert!(n < 2usize);
            let offs = 28usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkbid::from_bits(val as u8)
        }
        #[doc = "Break x (x=1,2) bidirectional"]
        #[inline(always)]
        pub fn set_bkbid(&mut self, n: usize, val: super::vals::Bkbid) {
            assert!(n < 2usize);
            let offs = 28usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for BdtrAdv {
        #[inline(always)]
        fn default() -> BdtrAdv {
            BdtrAdv(0)
        }
    }
    impl core::fmt::Debug for BdtrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BdtrAdv")
                .field("dtg", &self.dtg())
                .field("lock", &self.lock())
                .field("ossi", &self.ossi())
                .field("ossr", &self.ossr())
                .field("bke[0]", &self.bke(0usize))
                .field("bke[1]", &self.bke(1usize))
                .field("bkp[0]", &self.bkp(0usize))
                .field("bkp[1]", &self.bkp(1usize))
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .field("bkf[0]", &self.bkf(0usize))
                .field("bkf[1]", &self.bkf(1usize))
                .field("bkdsrm[0]", &self.bkdsrm(0usize))
                .field("bkdsrm[1]", &self.bkdsrm(1usize))
                .field("bkbid[0]", &self.bkbid(0usize))
                .field("bkbid[1]", &self.bkbid(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BdtrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BdtrAdv {{ dtg: {=u8:?}, lock: {:?}, ossi: {:?}, ossr: {:?}, bke[0]: {=bool:?}, bke[1]: {=bool:?}, bkp[0]: {:?}, bkp[1]: {:?}, aoe: {=bool:?}, moe: {=bool:?}, bkf[0]: {:?}, bkf[1]: {:?}, bkdsrm[0]: {:?}, bkdsrm[1]: {:?}, bkbid[0]: {:?}, bkbid[1]: {:?} }}" , self . dtg () , self . lock () , self . ossi () , self . ossr () , self . bke (0usize) , self . bke (1usize) , self . bkp (0usize) , self . bkp (1usize) , self . aoe () , self . moe () , self . bkf (0usize) , self . bkf (1usize) , self . bkdsrm (0usize) , self . bkdsrm (1usize) , self . bkbid (0usize) , self . bkbid (1usize))
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer1ch(pub u32);
    impl Ccer1ch {
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer1ch {
        #[inline(always)]
        fn default() -> Ccer1ch {
            Ccer1ch(0)
        }
    }
    impl core::fmt::Debug for Ccer1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer1ch")
                .field("cce[0]", &self.cce(0usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer1ch {{ cce[0]: {=bool:?}, ccp[0]: {=bool:?}, ccnp[0]: {=bool:?} }}",
                self.cce(0usize),
                self.ccp(0usize),
                self.ccnp(0usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer1chCmp(pub u32);
    impl Ccer1chCmp {
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[inline(always)]
        pub fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer1chCmp {
        #[inline(always)]
        fn default() -> Ccer1chCmp {
            Ccer1chCmp(0)
        }
    }
    impl core::fmt::Debug for Ccer1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer1chCmp")
                .field("cce[0]", &self.cce(0usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccne[0]", &self.ccne(0usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer1chCmp {{ cce[0]: {=bool:?}, ccp[0]: {=bool:?}, ccne[0]: {=bool:?}, ccnp[0]: {=bool:?} }}",
                self.cce(0usize),
                self.ccp(0usize),
                self.ccne(0usize),
                self.ccnp(0usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer2ch(pub u32);
    impl Ccer2ch {
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer2ch {
        #[inline(always)]
        fn default() -> Ccer2ch {
            Ccer2ch(0)
        }
    }
    impl core::fmt::Debug for Ccer2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer2ch")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccer2ch {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?} }}" , self . cce (0usize) , self . cce (1usize) , self . ccp (0usize) , self . ccp (1usize) , self . ccnp (0usize) , self . ccnp (1usize))
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer2chCmp(pub u32);
    impl Ccer2chCmp {
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[inline(always)]
        pub fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer2chCmp {
        #[inline(always)]
        fn default() -> Ccer2chCmp {
            Ccer2chCmp(0)
        }
    }
    impl core::fmt::Debug for Ccer2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer2chCmp")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccne[0]", &self.ccne(0usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccer2chCmp {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccne[0]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?} }}" , self . cce (0usize) , self . cce (1usize) , self . ccp (0usize) , self . ccp (1usize) , self . ccne (0usize) , self . ccnp (0usize) , self . ccnp (1usize))
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerAdv(pub u32);
    impl CcerAdv {
        #[doc = "Capture/Compare x (x=1-6) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-6) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-6) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-6) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) complementary output enable"]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) complementary output enable"]
        #[inline(always)]
        pub fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerAdv {
        #[inline(always)]
        fn default() -> CcerAdv {
            CcerAdv(0)
        }
    }
    impl core::fmt::Debug for CcerAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcerAdv")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("cce[2]", &self.cce(2usize))
                .field("cce[3]", &self.cce(3usize))
                .field("cce[4]", &self.cce(4usize))
                .field("cce[5]", &self.cce(5usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccp[2]", &self.ccp(2usize))
                .field("ccp[3]", &self.ccp(3usize))
                .field("ccp[4]", &self.ccp(4usize))
                .field("ccp[5]", &self.ccp(5usize))
                .field("ccne[0]", &self.ccne(0usize))
                .field("ccne[1]", &self.ccne(1usize))
                .field("ccne[2]", &self.ccne(2usize))
                .field("ccne[3]", &self.ccne(3usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .field("ccnp[2]", &self.ccnp(2usize))
                .field("ccnp[3]", &self.ccnp(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcerAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CcerAdv {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, cce[2]: {=bool:?}, cce[3]: {=bool:?}, cce[4]: {=bool:?}, cce[5]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccp[2]: {=bool:?}, ccp[3]: {=bool:?}, ccp[4]: {=bool:?}, ccp[5]: {=bool:?}, ccne[0]: {=bool:?}, ccne[1]: {=bool:?}, ccne[2]: {=bool:?}, ccne[3]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?}, ccnp[2]: {=bool:?}, ccnp[3]: {=bool:?} }}" , self . cce (0usize) , self . cce (1usize) , self . cce (2usize) , self . cce (3usize) , self . cce (4usize) , self . cce (5usize) , self . ccp (0usize) , self . ccp (1usize) , self . ccp (2usize) , self . ccp (3usize) , self . ccp (4usize) , self . ccp (5usize) , self . ccne (0usize) , self . ccne (1usize) , self . ccne (2usize) , self . ccne (3usize) , self . ccnp (0usize) , self . ccnp (1usize) , self . ccnp (2usize) , self . ccnp (3usize))
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerGp16(pub u32);
    impl CcerGp16 {
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerGp16 {
        #[inline(always)]
        fn default() -> CcerGp16 {
            CcerGp16(0)
        }
    }
    impl core::fmt::Debug for CcerGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcerGp16")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("cce[2]", &self.cce(2usize))
                .field("cce[3]", &self.cce(3usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccp[2]", &self.ccp(2usize))
                .field("ccp[3]", &self.ccp(3usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .field("ccnp[2]", &self.ccnp(2usize))
                .field("ccnp[3]", &self.ccnp(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcerGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CcerGp16 {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, cce[2]: {=bool:?}, cce[3]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccp[2]: {=bool:?}, ccp[3]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?}, ccnp[2]: {=bool:?}, ccnp[3]: {=bool:?} }}" , self . cce (0usize) , self . cce (1usize) , self . cce (2usize) , self . cce (3usize) , self . ccp (0usize) , self . ccp (1usize) , self . ccp (2usize) , self . ccp (3usize) , self . ccnp (0usize) , self . ccnp (1usize) , self . ccnp (2usize) , self . ccnp (3usize))
        }
    }
    #[doc = "capture/compare mode register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr3Adv(pub u32);
    impl Ccmr3Adv {
        #[doc = "Output compare x (x=5,6) fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare x (x=5,6) fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare x (x=5,6) preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare x (x=5,6) preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare x (x=5,6) mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let mut val = 0;
            let offs = 4usize + n * 8usize;
            val += (((self.0 >> offs) & 0x07) << 0usize);
            let offs = 16usize + n * 8usize;
            val += (((self.0 >> offs) & 0x01) << 3usize);
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare x (x=5,6) mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << offs);
            let offs = 16usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << offs);
        }
        #[doc = "Output compare x (x=5,6) clear enable"]
        #[inline(always)]
        pub const fn occe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare x (x=5,6) clear enable"]
        #[inline(always)]
        pub fn set_occe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccmr3Adv {
        #[inline(always)]
        fn default() -> Ccmr3Adv {
            Ccmr3Adv(0)
        }
    }
    impl core::fmt::Debug for Ccmr3Adv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr3Adv")
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocfe[1]", &self.ocfe(1usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocpe[1]", &self.ocpe(1usize))
                .field("ocm[0]", &self.ocm(0usize))
                .field("ocm[1]", &self.ocm(1usize))
                .field("occe[0]", &self.occe(0usize))
                .field("occe[1]", &self.occe(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr3Adv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccmr3Adv {{ ocfe[0]: {=bool:?}, ocfe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {:?}, ocm[1]: {:?}, occe[0]: {=bool:?}, occe[1]: {=bool:?} }}" , self . ocfe (0usize) , self . ocfe (1usize) , self . ocpe (0usize) , self . ocpe (1usize) , self . ocm (0usize) , self . ocm (1usize) , self . occe (0usize) , self . occe (1usize))
        }
    }
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput1ch(pub u32);
    impl CcmrInput1ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput1ch {
        #[inline(always)]
        fn default() -> CcmrInput1ch {
            CcmrInput1ch(0)
        }
    }
    impl core::fmt::Debug for CcmrInput1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrInput1ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("icpsc[0]", &self.icpsc(0usize))
                .field("icf[0]", &self.icf(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrInput1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrInput1ch {{ ccs[0]: {:?}, icpsc[0]: {=u8:?}, icf[0]: {:?} }}",
                self.ccs(0usize),
                self.icpsc(0usize),
                self.icf(0usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput2ch(pub u32);
    impl CcmrInput2ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput2ch {
        #[inline(always)]
        fn default() -> CcmrInput2ch {
            CcmrInput2ch(0)
        }
    }
    impl core::fmt::Debug for CcmrInput2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrInput2ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ccs[1]", &self.ccs(1usize))
                .field("icpsc[0]", &self.icpsc(0usize))
                .field("icpsc[1]", &self.icpsc(1usize))
                .field("icf[0]", &self.icf(0usize))
                .field("icf[1]", &self.icf(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrInput2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CcmrInput2ch {{ ccs[0]: {:?}, ccs[1]: {:?}, icpsc[0]: {=u8:?}, icpsc[1]: {=u8:?}, icf[0]: {:?}, icf[1]: {:?} }}" , self . ccs (0usize) , self . ccs (1usize) , self . icpsc (0usize) , self . icpsc (1usize) , self . icf (0usize) , self . icf (1usize))
        }
    }
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput1ch(pub u32);
    impl CcmrOutput1ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 1usize);
            let mut val = 0;
            let offs = 4usize + n * 8usize;
            val += (((self.0 >> offs) & 0x07) << 0usize);
            let offs = 16usize + n * 8usize;
            val += (((self.0 >> offs) & 0x01) << 3usize);
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << offs);
            let offs = 16usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << offs);
        }
    }
    impl Default for CcmrOutput1ch {
        #[inline(always)]
        fn default() -> CcmrOutput1ch {
            CcmrOutput1ch(0)
        }
    }
    impl core::fmt::Debug for CcmrOutput1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrOutput1ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocm[0]", &self.ocm(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrOutput1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrOutput1ch {{ ccs[0]: {:?}, ocfe[0]: {=bool:?}, ocpe[0]: {=bool:?}, ocm[0]: {:?} }}",
                self.ccs(0usize),
                self.ocfe(0usize),
                self.ocpe(0usize),
                self.ocm(0usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput2ch(pub u32);
    impl CcmrOutput2ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let mut val = 0;
            let offs = 4usize + n * 8usize;
            val += (((self.0 >> offs) & 0x07) << 0usize);
            let offs = 16usize + n * 8usize;
            val += (((self.0 >> offs) & 0x01) << 3usize);
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << offs);
            let offs = 16usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << offs);
        }
    }
    impl Default for CcmrOutput2ch {
        #[inline(always)]
        fn default() -> CcmrOutput2ch {
            CcmrOutput2ch(0)
        }
    }
    impl core::fmt::Debug for CcmrOutput2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrOutput2ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ccs[1]", &self.ccs(1usize))
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocfe[1]", &self.ocfe(1usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocpe[1]", &self.ocpe(1usize))
                .field("ocm[0]", &self.ocm(0usize))
                .field("ocm[1]", &self.ocm(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrOutput2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CcmrOutput2ch {{ ccs[0]: {:?}, ccs[1]: {:?}, ocfe[0]: {=bool:?}, ocfe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {:?}, ocm[1]: {:?} }}" , self . ccs (0usize) , self . ccs (1usize) , self . ocfe (0usize) , self . ocfe (1usize) , self . ocpe (0usize) , self . ocpe (1usize) , self . ocm (0usize) , self . ocm (1usize))
        }
    }
    #[doc = "capture/compare mode register x (x=1-2) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutputGp16(pub u32);
    impl CcmrOutputGp16 {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let mut val = 0;
            let offs = 4usize + n * 8usize;
            val += (((self.0 >> offs) & 0x07) << 0usize);
            let offs = 16usize + n * 8usize;
            val += (((self.0 >> offs) & 0x01) << 3usize);
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << offs);
            let offs = 16usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << offs);
        }
        #[doc = "Output compare y clear enable"]
        #[inline(always)]
        pub const fn occe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y clear enable"]
        #[inline(always)]
        pub fn set_occe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcmrOutputGp16 {
        #[inline(always)]
        fn default() -> CcmrOutputGp16 {
            CcmrOutputGp16(0)
        }
    }
    impl core::fmt::Debug for CcmrOutputGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrOutputGp16")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ccs[1]", &self.ccs(1usize))
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocfe[1]", &self.ocfe(1usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocpe[1]", &self.ocpe(1usize))
                .field("ocm[0]", &self.ocm(0usize))
                .field("ocm[1]", &self.ocm(1usize))
                .field("occe[0]", &self.occe(0usize))
                .field("occe[1]", &self.occe(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrOutputGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CcmrOutputGp16 {{ ccs[0]: {:?}, ccs[1]: {:?}, ocfe[0]: {=bool:?}, ocfe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {:?}, ocm[1]: {:?}, occe[0]: {=bool:?}, occe[1]: {=bool:?} }}" , self . ccs (0usize) , self . ccs (1usize) , self . ocfe (0usize) , self . ocfe (1usize) , self . ocpe (0usize) , self . ocpe (1usize) , self . ocm (0usize) , self . ocm (1usize) , self . occe (0usize) , self . occe (1usize))
        }
    }
    #[doc = "capture/compare register x (x=1-4,6) (Dither mode disabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr1ch(pub u32);
    impl Ccr1ch {
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr1ch {
        #[inline(always)]
        fn default() -> Ccr1ch {
            Ccr1ch(0)
        }
    }
    impl core::fmt::Debug for Ccr1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr1ch").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr1ch {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "capture/compare register 5 (Dither mode disabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr5Adv(pub u32);
    impl Ccr5Adv {
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Group channel 5 and channel x (x=1-3)"]
        #[inline(always)]
        pub const fn gc5c(&self, n: usize) -> super::vals::Gc5c {
            assert!(n < 3usize);
            let offs = 29usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Gc5c::from_bits(val as u8)
        }
        #[doc = "Group channel 5 and channel x (x=1-3)"]
        #[inline(always)]
        pub fn set_gc5c(&mut self, n: usize, val: super::vals::Gc5c) {
            assert!(n < 3usize);
            let offs = 29usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccr5Adv {
        #[inline(always)]
        fn default() -> Ccr5Adv {
            Ccr5Adv(0)
        }
    }
    impl core::fmt::Debug for Ccr5Adv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr5Adv")
                .field("ccr", &self.ccr())
                .field("gc5c[0]", &self.gc5c(0usize))
                .field("gc5c[1]", &self.gc5c(1usize))
                .field("gc5c[2]", &self.gc5c(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr5Adv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr5Adv {{ ccr: {=u16:?}, gc5c[0]: {:?}, gc5c[1]: {:?}, gc5c[2]: {:?} }}",
                self.ccr(),
                self.gc5c(0usize),
                self.gc5c(1usize),
                self.gc5c(2usize)
            )
        }
    }
    #[doc = "capture/compare register 5 (Dither mode enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr5DitherAdv(pub u32);
    impl Ccr5DitherAdv {
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn dither(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_dither(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0xffff;
            val as u16
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 4usize)) | (((val as u32) & 0xffff) << 4usize);
        }
        #[doc = "Group channel 5 and channel x (x=1-3)"]
        #[inline(always)]
        pub const fn gc5c(&self, n: usize) -> super::vals::Gc5c {
            assert!(n < 3usize);
            let offs = 29usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Gc5c::from_bits(val as u8)
        }
        #[doc = "Group channel 5 and channel x (x=1-3)"]
        #[inline(always)]
        pub fn set_gc5c(&mut self, n: usize, val: super::vals::Gc5c) {
            assert!(n < 3usize);
            let offs = 29usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccr5DitherAdv {
        #[inline(always)]
        fn default() -> Ccr5DitherAdv {
            Ccr5DitherAdv(0)
        }
    }
    impl core::fmt::Debug for Ccr5DitherAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr5DitherAdv")
                .field("dither", &self.dither())
                .field("ccr", &self.ccr())
                .field("gc5c[0]", &self.gc5c(0usize))
                .field("gc5c[1]", &self.gc5c(1usize))
                .field("gc5c[2]", &self.gc5c(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr5DitherAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr5DitherAdv {{ dither: {=u8:?}, ccr: {=u16:?}, gc5c[0]: {:?}, gc5c[1]: {:?}, gc5c[2]: {:?} }}",
                self.dither(),
                self.ccr(),
                self.gc5c(0usize),
                self.gc5c(1usize),
                self.gc5c(2usize)
            )
        }
    }
    #[doc = "capture/compare register x (x=1-4,6) (Dither mode enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcrDither1ch(pub u32);
    impl CcrDither1ch {
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn dither(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_dither(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0xffff;
            val as u16
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 4usize)) | (((val as u32) & 0xffff) << 4usize);
        }
    }
    impl Default for CcrDither1ch {
        #[inline(always)]
        fn default() -> CcrDither1ch {
            CcrDither1ch(0)
        }
    }
    impl core::fmt::Debug for CcrDither1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcrDither1ch")
                .field("dither", &self.dither())
                .field("ccr", &self.ccr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcrDither1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcrDither1ch {{ dither: {=u8:?}, ccr: {=u16:?} }}",
                self.dither(),
                self.ccr()
            )
        }
    }
    #[doc = "capture/compare register x (x=1-4,6) (Dither mode enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcrDitherGp32(pub u32);
    impl CcrDitherGp32 {
        #[doc = "Dither value"]
        #[inline(always)]
        pub const fn dither(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Dither value"]
        #[inline(always)]
        pub fn set_dither(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for CcrDitherGp32 {
        #[inline(always)]
        fn default() -> CcrDitherGp32 {
            CcrDitherGp32(0)
        }
    }
    impl core::fmt::Debug for CcrDitherGp32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcrDitherGp32")
                .field("dither", &self.dither())
                .field("ccr", &self.ccr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcrDitherGp32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcrDitherGp32 {{ dither: {=u8:?}, ccr: {=u32:?} }}",
                self.dither(),
                self.ccr()
            )
        }
    }
    #[doc = "counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntCore(pub u32);
    impl CntCore {
        #[doc = "counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "UIF copy"]
        #[inline(always)]
        pub const fn uifcpy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UIF copy"]
        #[inline(always)]
        pub fn set_uifcpy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CntCore {
        #[inline(always)]
        fn default() -> CntCore {
            CntCore(0)
        }
    }
    impl core::fmt::Debug for CntCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CntCore")
                .field("cnt", &self.cnt())
                .field("uifcpy", &self.uifcpy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CntCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CntCore {{ cnt: {=u16:?}, uifcpy: {=bool:?} }}",
                self.cnt(),
                self.uifcpy()
            )
        }
    }
    #[doc = "counter (Dither mode enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntDitherGp32(pub u32);
    impl CntDitherGp32 {
        #[doc = "counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "UIF copy"]
        #[inline(always)]
        pub const fn uifcpy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UIF copy"]
        #[inline(always)]
        pub fn set_uifcpy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CntDitherGp32 {
        #[inline(always)]
        fn default() -> CntDitherGp32 {
            CntDitherGp32(0)
        }
    }
    impl core::fmt::Debug for CntDitherGp32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CntDitherGp32")
                .field("cnt", &self.cnt())
                .field("uifcpy", &self.uifcpy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CntDitherGp32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CntDitherGp32 {{ cnt: {=u32:?}, uifcpy: {=bool:?} }}",
                self.cnt(),
                self.uifcpy()
            )
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr11ch(pub u32);
    impl Cr11ch {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Dithering enable"]
        #[inline(always)]
        pub const fn dithen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering enable"]
        #[inline(always)]
        pub fn set_dithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cr11ch {
        #[inline(always)]
        fn default() -> Cr11ch {
            Cr11ch(0)
        }
    }
    impl core::fmt::Debug for Cr11ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr11ch")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .field("uifremap", &self.uifremap())
                .field("dithen", &self.dithen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr11ch {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr11ch {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, arpe: {=bool:?}, ckd: {:?}, uifremap: {=bool:?}, dithen: {=bool:?} }}" , self . cen () , self . udis () , self . urs () , self . opm () , self . arpe () , self . ckd () , self . uifremap () , self . dithen ())
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Core(pub u32);
    impl Cr1Core {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Dithering enable"]
        #[inline(always)]
        pub const fn dithen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering enable"]
        #[inline(always)]
        pub fn set_dithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cr1Core {
        #[inline(always)]
        fn default() -> Cr1Core {
            Cr1Core(0)
        }
    }
    impl core::fmt::Debug for Cr1Core {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1Core")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("arpe", &self.arpe())
                .field("uifremap", &self.uifremap())
                .field("dithen", &self.dithen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1Core {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1Core {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, arpe: {=bool:?}, uifremap: {=bool:?}, dithen: {=bool:?} }}" , self . cen () , self . udis () , self . urs () , self . opm () , self . arpe () , self . uifremap () , self . dithen ())
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Gp16(pub u32);
    impl Cr1Gp16 {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub const fn cms(&self) -> super::vals::Cms {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cms::from_bits(val as u8)
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub fn set_cms(&mut self, val: super::vals::Cms) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Dithering enable"]
        #[inline(always)]
        pub const fn dithen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering enable"]
        #[inline(always)]
        pub fn set_dithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cr1Gp16 {
        #[inline(always)]
        fn default() -> Cr1Gp16 {
            Cr1Gp16(0)
        }
    }
    impl core::fmt::Debug for Cr1Gp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1Gp16")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("dir", &self.dir())
                .field("cms", &self.cms())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .field("uifremap", &self.uifremap())
                .field("dithen", &self.dithen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1Gp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1Gp16 {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, dir: {:?}, cms: {:?}, arpe: {=bool:?}, ckd: {:?}, uifremap: {=bool:?}, dithen: {=bool:?} }}" , self . cen () , self . udis () , self . urs () , self . opm () , self . dir () , self . cms () , self . arpe () , self . ckd () , self . uifremap () , self . dithen ())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr21chCmp(pub u32);
    impl Cr21chCmp {
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr21chCmp {
        #[inline(always)]
        fn default() -> Cr21chCmp {
            Cr21chCmp(0)
        }
    }
    impl core::fmt::Debug for Cr21chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr21chCmp")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("ois[0]", &self.ois(0usize))
                .field("oisn[0]", &self.oisn(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr21chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr21chCmp {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, ois[0]: {=bool:?}, oisn[0]: {=bool:?} }}",
                self.ccpc(),
                self.ccus(),
                self.ccds(),
                self.ois(0usize),
                self.oisn(0usize)
            )
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr22ch(pub u32);
    impl Cr22ch {
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 25usize) & 0x01) << 3usize);
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 25usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr22ch {
        #[inline(always)]
        fn default() -> Cr22ch {
            Cr22ch(0)
        }
    }
    impl core::fmt::Debug for Cr22ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr22ch")
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr22ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr22ch {{ mms: {:?}, ti1s: {:?} }}", self.mms(), self.ti1s())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr22chCmp(pub u32);
    impl Cr22chCmp {
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 25usize) & 0x01) << 3usize);
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 25usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state x (x=1,2)"]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1,2)"]
        #[inline(always)]
        pub fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr22chCmp {
        #[inline(always)]
        fn default() -> Cr22chCmp {
            Cr22chCmp(0)
        }
    }
    impl core::fmt::Debug for Cr22chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr22chCmp")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .field("ois[0]", &self.ois(0usize))
                .field("ois[1]", &self.ois(1usize))
                .field("oisn[0]", &self.oisn(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr22chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr22chCmp {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, mms: {:?}, ti1s: {:?}, ois[0]: {=bool:?}, ois[1]: {=bool:?}, oisn[0]: {=bool:?} }}" , self . ccpc () , self . ccus () , self . ccds () , self . mms () , self . ti1s () , self . ois (0usize) , self . ois (1usize) , self . oisn (0usize))
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Adv(pub u32);
    impl Cr2Adv {
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 25usize) & 0x01) << 3usize);
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 25usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state x (x=1-6)"]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1-6)"]
        #[inline(always)]
        pub fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state x N x (x=1-4)"]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x N x (x=1-4)"]
        #[inline(always)]
        pub fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master mode selection 2"]
        #[inline(always)]
        pub const fn mms2(&self) -> super::vals::Mms2 {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::Mms2::from_bits(val as u8)
        }
        #[doc = "Master mode selection 2"]
        #[inline(always)]
        pub fn set_mms2(&mut self, val: super::vals::Mms2) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Cr2Adv {
        #[inline(always)]
        fn default() -> Cr2Adv {
            Cr2Adv(0)
        }
    }
    impl core::fmt::Debug for Cr2Adv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Adv")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .field("ois[0]", &self.ois(0usize))
                .field("ois[1]", &self.ois(1usize))
                .field("ois[2]", &self.ois(2usize))
                .field("ois[3]", &self.ois(3usize))
                .field("ois[4]", &self.ois(4usize))
                .field("ois[5]", &self.ois(5usize))
                .field("oisn[0]", &self.oisn(0usize))
                .field("oisn[1]", &self.oisn(1usize))
                .field("oisn[2]", &self.oisn(2usize))
                .field("oisn[3]", &self.oisn(3usize))
                .field("mms2", &self.mms2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Adv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2Adv {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, mms: {:?}, ti1s: {:?}, ois[0]: {=bool:?}, ois[1]: {=bool:?}, ois[2]: {=bool:?}, ois[3]: {=bool:?}, ois[4]: {=bool:?}, ois[5]: {=bool:?}, oisn[0]: {=bool:?}, oisn[1]: {=bool:?}, oisn[2]: {=bool:?}, oisn[3]: {=bool:?}, mms2: {:?} }}" , self . ccpc () , self . ccus () , self . ccds () , self . mms () , self . ti1s () , self . ois (0usize) , self . ois (1usize) , self . ois (2usize) , self . ois (3usize) , self . ois (4usize) , self . ois (5usize) , self . oisn (0usize) , self . oisn (1usize) , self . oisn (2usize) , self . oisn (3usize) , self . mms2 ())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Basic(pub u32);
    impl Cr2Basic {
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 25usize) & 0x01) << 3usize);
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 25usize);
        }
    }
    impl Default for Cr2Basic {
        #[inline(always)]
        fn default() -> Cr2Basic {
            Cr2Basic(0)
        }
    }
    impl core::fmt::Debug for Cr2Basic {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Basic").field("mms", &self.mms()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Basic {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr2Basic {{ mms: {:?} }}", self.mms())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Gp16(pub u32);
    impl Cr2Gp16 {
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 25usize) & 0x01) << 3usize);
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 25usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr2Gp16 {
        #[inline(always)]
        fn default() -> Cr2Gp16 {
            Cr2Gp16(0)
        }
    }
    impl core::fmt::Debug for Cr2Gp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Gp16")
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Gp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2Gp16 {{ ccds: {:?}, mms: {:?}, ti1s: {:?} }}",
                self.ccds(),
                self.mms(),
                self.ti1s()
            )
        }
    }
    #[doc = "DMA control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr1chCmp(pub u32);
    impl Dcr1chCmp {
        #[doc = "DMA base address"]
        #[inline(always)]
        pub const fn dba(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA base address"]
        #[inline(always)]
        pub fn set_dba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub const fn dbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub fn set_dbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "DMA burst source selection"]
        #[inline(always)]
        pub const fn dbss(&self) -> super::vals::Dbss {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Dbss::from_bits(val as u8)
        }
        #[doc = "DMA burst source selection"]
        #[inline(always)]
        pub fn set_dbss(&mut self, val: super::vals::Dbss) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Dcr1chCmp {
        #[inline(always)]
        fn default() -> Dcr1chCmp {
            Dcr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Dcr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr1chCmp")
                .field("dba", &self.dba())
                .field("dbl", &self.dbl())
                .field("dbss", &self.dbss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dcr1chCmp {{ dba: {=u8:?}, dbl: {=u8:?}, dbss: {:?} }}",
                self.dba(),
                self.dbl(),
                self.dbss()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier1ch(pub u32);
    impl Dier1ch {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Dier1ch {
        #[inline(always)]
        fn default() -> Dier1ch {
            Dier1ch(0)
        }
    }
    impl core::fmt::Debug for Dier1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier1ch")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier1ch {{ uie: {=bool:?}, ccie[0]: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize)
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier1chCmp(pub u32);
    impl Dier1chCmp {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Dier1chCmp {
        #[inline(always)]
        fn default() -> Dier1chCmp {
            Dier1chCmp(0)
        }
    }
    impl core::fmt::Debug for Dier1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier1chCmp")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("comie", &self.comie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dier1chCmp {{ uie: {=bool:?}, ccie[0]: {=bool:?}, comie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?} }}" , self . uie () , self . ccie (0usize) , self . comie () , self . bie () , self . ude () , self . ccde (0usize))
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier2ch(pub u32);
    impl Dier2ch {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Dier2ch {
        #[inline(always)]
        fn default() -> Dier2ch {
            Dier2ch(0)
        }
    }
    impl core::fmt::Debug for Dier2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier2ch")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("tie", &self.tie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier2ch {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, tie: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize),
                self.ccie(1usize),
                self.tie()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier2chCmp(pub u32);
    impl Dier2chCmp {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Dier2chCmp {
        #[inline(always)]
        fn default() -> Dier2chCmp {
            Dier2chCmp(0)
        }
    }
    impl core::fmt::Debug for Dier2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier2chCmp")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("comie", &self.comie())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .field("comde", &self.comde())
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dier2chCmp {{ uie: {=bool:?}, ccie[0]: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, comde: {=bool:?}, tde: {=bool:?} }}" , self . uie () , self . ccie (0usize) , self . comie () , self . tie () , self . bie () , self . ude () , self . ccde (0usize) , self . comde () , self . tde ())
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierAdv(pub u32);
    impl DierAdv {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Index interrupt enable"]
        #[inline(always)]
        pub const fn idxie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Index interrupt enable"]
        #[inline(always)]
        pub fn set_idxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Direction change interrupt enable"]
        #[inline(always)]
        pub const fn dirie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change interrupt enable"]
        #[inline(always)]
        pub fn set_dirie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Index error interrupt enable"]
        #[inline(always)]
        pub const fn ierrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Index error interrupt enable"]
        #[inline(always)]
        pub fn set_ierrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Transition error interrupt enable"]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transition error interrupt enable"]
        #[inline(always)]
        pub fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for DierAdv {
        #[inline(always)]
        fn default() -> DierAdv {
            DierAdv(0)
        }
    }
    impl core::fmt::Debug for DierAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierAdv")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("ccie[2]", &self.ccie(2usize))
                .field("ccie[3]", &self.ccie(3usize))
                .field("comie", &self.comie())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .field("ccde[1]", &self.ccde(1usize))
                .field("ccde[2]", &self.ccde(2usize))
                .field("ccde[3]", &self.ccde(3usize))
                .field("comde", &self.comde())
                .field("tde", &self.tde())
                .field("idxie", &self.idxie())
                .field("dirie", &self.dirie())
                .field("ierrie", &self.ierrie())
                .field("terrie", &self.terrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DierAdv {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, ccie[2]: {=bool:?}, ccie[3]: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, ccde[1]: {=bool:?}, ccde[2]: {=bool:?}, ccde[3]: {=bool:?}, comde: {=bool:?}, tde: {=bool:?}, idxie: {=bool:?}, dirie: {=bool:?}, ierrie: {=bool:?}, terrie: {=bool:?} }}" , self . uie () , self . ccie (0usize) , self . ccie (1usize) , self . ccie (2usize) , self . ccie (3usize) , self . comie () , self . tie () , self . bie () , self . ude () , self . ccde (0usize) , self . ccde (1usize) , self . ccde (2usize) , self . ccde (3usize) , self . comde () , self . tde () , self . idxie () , self . dirie () , self . ierrie () , self . terrie ())
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierBasicNoCr2(pub u32);
    impl DierBasicNoCr2 {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DierBasicNoCr2 {
        #[inline(always)]
        fn default() -> DierBasicNoCr2 {
            DierBasicNoCr2(0)
        }
    }
    impl core::fmt::Debug for DierBasicNoCr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierBasicNoCr2")
                .field("uie", &self.uie())
                .field("ude", &self.ude())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierBasicNoCr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DierBasicNoCr2 {{ uie: {=bool:?}, ude: {=bool:?} }}",
                self.uie(),
                self.ude()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierCore(pub u32);
    impl DierCore {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DierCore {
        #[inline(always)]
        fn default() -> DierCore {
            DierCore(0)
        }
    }
    impl core::fmt::Debug for DierCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierCore").field("uie", &self.uie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DierCore {{ uie: {=bool:?} }}", self.uie())
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierGp16(pub u32);
    impl DierGp16 {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Index interrupt enable"]
        #[inline(always)]
        pub const fn idxie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Index interrupt enable"]
        #[inline(always)]
        pub fn set_idxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Direction change interrupt enable"]
        #[inline(always)]
        pub const fn dirie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change interrupt enable"]
        #[inline(always)]
        pub fn set_dirie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Index error interrupt enable"]
        #[inline(always)]
        pub const fn ierrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Index error interrupt enable"]
        #[inline(always)]
        pub fn set_ierrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Transition error interrupt enable"]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transition error interrupt enable"]
        #[inline(always)]
        pub fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for DierGp16 {
        #[inline(always)]
        fn default() -> DierGp16 {
            DierGp16(0)
        }
    }
    impl core::fmt::Debug for DierGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierGp16")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("ccie[2]", &self.ccie(2usize))
                .field("ccie[3]", &self.ccie(3usize))
                .field("tie", &self.tie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .field("ccde[1]", &self.ccde(1usize))
                .field("ccde[2]", &self.ccde(2usize))
                .field("ccde[3]", &self.ccde(3usize))
                .field("tde", &self.tde())
                .field("idxie", &self.idxie())
                .field("dirie", &self.dirie())
                .field("ierrie", &self.ierrie())
                .field("terrie", &self.terrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DierGp16 {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, ccie[2]: {=bool:?}, ccie[3]: {=bool:?}, tie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, ccde[1]: {=bool:?}, ccde[2]: {=bool:?}, ccde[3]: {=bool:?}, tde: {=bool:?}, idxie: {=bool:?}, dirie: {=bool:?}, ierrie: {=bool:?}, terrie: {=bool:?} }}" , self . uie () , self . ccie (0usize) , self . ccie (1usize) , self . ccie (2usize) , self . ccie (3usize) , self . tie () , self . ude () , self . ccde (0usize) , self . ccde (1usize) , self . ccde (2usize) , self . ccde (3usize) , self . tde () , self . idxie () , self . dirie () , self . ierrie () , self . terrie ())
        }
    }
    #[doc = "deadtime register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr21chCmp(pub u32);
    impl Dtr21chCmp {
        #[doc = "Dead-time falling edge generator setup"]
        #[inline(always)]
        pub const fn dtgf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time falling edge generator setup"]
        #[inline(always)]
        pub fn set_dtgf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Deadtime asymmetric enable"]
        #[inline(always)]
        pub const fn dtae(&self) -> super::vals::Dtae {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Dtae::from_bits(val as u8)
        }
        #[doc = "Deadtime asymmetric enable"]
        #[inline(always)]
        pub fn set_dtae(&mut self, val: super::vals::Dtae) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Deadtime preload enable"]
        #[inline(always)]
        pub const fn dtpe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime preload enable"]
        #[inline(always)]
        pub fn set_dtpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Dtr21chCmp {
        #[inline(always)]
        fn default() -> Dtr21chCmp {
            Dtr21chCmp(0)
        }
    }
    impl core::fmt::Debug for Dtr21chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr21chCmp")
                .field("dtgf", &self.dtgf())
                .field("dtae", &self.dtae())
                .field("dtpe", &self.dtpe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtr21chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dtr21chCmp {{ dtgf: {=u8:?}, dtae: {:?}, dtpe: {=bool:?} }}",
                self.dtgf(),
                self.dtae(),
                self.dtpe()
            )
        }
    }
    #[doc = "encoder control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcrGp16(pub u32);
    impl EcrGp16 {
        #[doc = "Index enable"]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Index enable"]
        #[inline(always)]
        pub fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Index direction"]
        #[inline(always)]
        pub const fn idir(&self) -> super::vals::Idir {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Idir::from_bits(val as u8)
        }
        #[doc = "Index direction"]
        #[inline(always)]
        pub fn set_idir(&mut self, val: super::vals::Idir) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Index blanking"]
        #[inline(always)]
        pub const fn iblk(&self) -> super::vals::Iblk {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Iblk::from_bits(val as u8)
        }
        #[doc = "Index blanking"]
        #[inline(always)]
        pub fn set_iblk(&mut self, val: super::vals::Iblk) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "First index"]
        #[inline(always)]
        pub const fn fidx(&self) -> super::vals::Fidx {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Fidx::from_bits(val as u8)
        }
        #[doc = "First index"]
        #[inline(always)]
        pub fn set_fidx(&mut self, val: super::vals::Fidx) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Index positioning"]
        #[inline(always)]
        pub const fn ipos(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Index positioning"]
        #[inline(always)]
        pub fn set_ipos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Pulse width"]
        #[inline(always)]
        pub const fn pw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Pulse width"]
        #[inline(always)]
        pub fn set_pw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Pulse width prescaler"]
        #[inline(always)]
        pub const fn pwprsc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Pulse width prescaler"]
        #[inline(always)]
        pub fn set_pwprsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for EcrGp16 {
        #[inline(always)]
        fn default() -> EcrGp16 {
            EcrGp16(0)
        }
    }
    impl core::fmt::Debug for EcrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EcrGp16")
                .field("ie", &self.ie())
                .field("idir", &self.idir())
                .field("iblk", &self.iblk())
                .field("fidx", &self.fidx())
                .field("ipos", &self.ipos())
                .field("pw", &self.pw())
                .field("pwprsc", &self.pwprsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EcrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EcrGp16 {{ ie: {=bool:?}, idir: {:?}, iblk: {:?}, fidx: {:?}, ipos: {=u8:?}, pw: {=u8:?}, pwprsc: {=u8:?} }}" , self . ie () , self . idir () , self . iblk () , self . fidx () , self . ipos () , self . pw () , self . pwprsc ())
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr1ch(pub u32);
    impl Egr1ch {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr1ch {
        #[inline(always)]
        fn default() -> Egr1ch {
            Egr1ch(0)
        }
    }
    impl core::fmt::Debug for Egr1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr1ch")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr1ch {{ ug: {=bool:?}, ccg[0]: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize)
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr1chCmp(pub u32);
    impl Egr1chCmp {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break x (x=1) generation"]
        #[inline(always)]
        pub const fn bg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) generation"]
        #[inline(always)]
        pub fn set_bg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr1chCmp {
        #[inline(always)]
        fn default() -> Egr1chCmp {
            Egr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Egr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr1chCmp")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("comg", &self.comg())
                .field("bg[0]", &self.bg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr1chCmp {{ ug: {=bool:?}, ccg[0]: {=bool:?}, comg: {=bool:?}, bg[0]: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.comg(),
                self.bg(0usize)
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr2ch(pub u32);
    impl Egr2ch {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-2) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-2) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Egr2ch {
        #[inline(always)]
        fn default() -> Egr2ch {
            Egr2ch(0)
        }
    }
    impl core::fmt::Debug for Egr2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr2ch")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("tg", &self.tg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr2ch {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, tg: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.ccg(1usize),
                self.tg()
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr2chCmp(pub u32);
    impl Egr2chCmp {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1,2) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1,2) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1) generation"]
        #[inline(always)]
        pub const fn bg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) generation"]
        #[inline(always)]
        pub fn set_bg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr2chCmp {
        #[inline(always)]
        fn default() -> Egr2chCmp {
            Egr2chCmp(0)
        }
    }
    impl core::fmt::Debug for Egr2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr2chCmp")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("comg", &self.comg())
                .field("tg", &self.tg())
                .field("bg[0]", &self.bg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Egr2chCmp {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg[0]: {=bool:?} }}" , self . ug () , self . ccg (0usize) , self . ccg (1usize) , self . comg () , self . tg () , self . bg (0usize))
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrAdv(pub u32);
    impl EgrAdv {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1-2) generation"]
        #[inline(always)]
        pub const fn bg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1-2) generation"]
        #[inline(always)]
        pub fn set_bg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for EgrAdv {
        #[inline(always)]
        fn default() -> EgrAdv {
            EgrAdv(0)
        }
    }
    impl core::fmt::Debug for EgrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EgrAdv")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("ccg[2]", &self.ccg(2usize))
                .field("ccg[3]", &self.ccg(3usize))
                .field("comg", &self.comg())
                .field("tg", &self.tg())
                .field("bg[0]", &self.bg(0usize))
                .field("bg[1]", &self.bg(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EgrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EgrAdv {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, ccg[2]: {=bool:?}, ccg[3]: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg[0]: {=bool:?}, bg[1]: {=bool:?} }}" , self . ug () , self . ccg (0usize) , self . ccg (1usize) , self . ccg (2usize) , self . ccg (3usize) , self . comg () , self . tg () , self . bg (0usize) , self . bg (1usize))
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrCore(pub u32);
    impl EgrCore {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EgrCore {
        #[inline(always)]
        fn default() -> EgrCore {
            EgrCore(0)
        }
    }
    impl core::fmt::Debug for EgrCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EgrCore").field("ug", &self.ug()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EgrCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EgrCore {{ ug: {=bool:?} }}", self.ug())
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrGp16(pub u32);
    impl EgrGp16 {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for EgrGp16 {
        #[inline(always)]
        fn default() -> EgrGp16 {
            EgrGp16(0)
        }
    }
    impl core::fmt::Debug for EgrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EgrGp16")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("ccg[2]", &self.ccg(2usize))
                .field("ccg[3]", &self.ccg(3usize))
                .field("tg", &self.tg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EgrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EgrGp16 {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, ccg[2]: {=bool:?}, ccg[3]: {=bool:?}, tg: {=bool:?} }}" , self . ug () , self . ccg (0usize) , self . ccg (1usize) , self . ccg (2usize) , self . ccg (3usize) , self . tg ())
        }
    }
    #[doc = "repetition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr1chCmp(pub u32);
    impl Rcr1chCmp {
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rcr1chCmp {
        #[inline(always)]
        fn default() -> Rcr1chCmp {
            Rcr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Rcr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr1chCmp").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rcr1chCmp {{ rep: {=u8:?} }}", self.rep())
        }
    }
    #[doc = "repetition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcrAdv(pub u32);
    impl RcrAdv {
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub const fn rep(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RcrAdv {
        #[inline(always)]
        fn default() -> RcrAdv {
            RcrAdv(0)
        }
    }
    impl core::fmt::Debug for RcrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RcrAdv").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RcrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RcrAdv {{ rep: {=u16:?} }}", self.rep())
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr2ch(pub u32);
    impl Smcr2ch {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let mut val = 0;
            val += (((self.0 >> 0usize) & 0x07) << 0usize);
            val += (((self.0 >> 16usize) & 0x01) << 3usize);
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 0usize);
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 16usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 20usize) & 0x03) << 3usize);
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32 >> 3usize) & 0x03) << 20usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Smcr2ch {
        #[inline(always)]
        fn default() -> Smcr2ch {
            Smcr2ch(0)
        }
    }
    impl core::fmt::Debug for Smcr2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcr2ch")
                .field("sms", &self.sms())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smcr2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Smcr2ch {{ sms: {:?}, ts: {:?}, msm: {:?} }}",
                self.sms(),
                self.ts(),
                self.msm()
            )
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr2chCmp(pub u32);
    impl Smcr2chCmp {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let mut val = 0;
            val += (((self.0 >> 0usize) & 0x07) << 0usize);
            val += (((self.0 >> 16usize) & 0x01) << 3usize);
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 0usize);
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 16usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 20usize) & 0x03) << 3usize);
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32 >> 3usize) & 0x03) << 20usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "SMS preload enable"]
        #[inline(always)]
        pub const fn smspe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SMS preload enable"]
        #[inline(always)]
        pub fn set_smspe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Smcr2chCmp {
        #[inline(always)]
        fn default() -> Smcr2chCmp {
            Smcr2chCmp(0)
        }
    }
    impl core::fmt::Debug for Smcr2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcr2chCmp")
                .field("sms", &self.sms())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .field("smspe", &self.smspe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smcr2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Smcr2chCmp {{ sms: {:?}, ts: {:?}, msm: {:?}, smspe: {=bool:?} }}",
                self.sms(),
                self.ts(),
                self.msm(),
                self.smspe()
            )
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmcrAdv(pub u32);
    impl SmcrAdv {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let mut val = 0;
            val += (((self.0 >> 0usize) & 0x07) << 0usize);
            val += (((self.0 >> 16usize) & 0x01) << 3usize);
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 0usize);
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 16usize);
        }
        #[doc = "OCREF clear selection"]
        #[inline(always)]
        pub const fn occs(&self) -> super::vals::Occs {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Occs::from_bits(val as u8)
        }
        #[doc = "OCREF clear selection"]
        #[inline(always)]
        pub fn set_occs(&mut self, val: super::vals::Occs) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 20usize) & 0x03) << 3usize);
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32 >> 3usize) & 0x03) << 20usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub const fn etf(&self) -> super::vals::FilterValue {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub fn set_etf(&mut self, val: super::vals::FilterValue) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub const fn etp(&self) -> super::vals::Etp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Etp::from_bits(val as u8)
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub fn set_etp(&mut self, val: super::vals::Etp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "SMS preload enable"]
        #[inline(always)]
        pub const fn smspe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SMS preload enable"]
        #[inline(always)]
        pub fn set_smspe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SMS preload source"]
        #[inline(always)]
        pub const fn smsps(&self) -> super::vals::Smsps {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Smsps::from_bits(val as u8)
        }
        #[doc = "SMS preload source"]
        #[inline(always)]
        pub fn set_smsps(&mut self, val: super::vals::Smsps) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for SmcrAdv {
        #[inline(always)]
        fn default() -> SmcrAdv {
            SmcrAdv(0)
        }
    }
    impl core::fmt::Debug for SmcrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SmcrAdv")
                .field("sms", &self.sms())
                .field("occs", &self.occs())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .field("etf", &self.etf())
                .field("etps", &self.etps())
                .field("ece", &self.ece())
                .field("etp", &self.etp())
                .field("smspe", &self.smspe())
                .field("smsps", &self.smsps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SmcrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SmcrAdv {{ sms: {:?}, occs: {:?}, ts: {:?}, msm: {:?}, etf: {:?}, etps: {:?}, ece: {=bool:?}, etp: {:?}, smspe: {=bool:?}, smsps: {:?} }}" , self . sms () , self . occs () , self . ts () , self . msm () , self . etf () , self . etps () , self . ece () , self . etp () , self . smspe () , self . smsps ())
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmcrGp16(pub u32);
    impl SmcrGp16 {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let mut val = 0;
            val += (((self.0 >> 0usize) & 0x07) << 0usize);
            val += (((self.0 >> 16usize) & 0x01) << 3usize);
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 0usize);
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32 >> 3usize) & 0x01) << 16usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 20usize) & 0x03) << 3usize);
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32 >> 3usize) & 0x03) << 20usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub const fn etf(&self) -> super::vals::FilterValue {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub fn set_etf(&mut self, val: super::vals::FilterValue) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub const fn etp(&self) -> super::vals::Etp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Etp::from_bits(val as u8)
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub fn set_etp(&mut self, val: super::vals::Etp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "SMS preload enable"]
        #[inline(always)]
        pub const fn smspe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SMS preload enable"]
        #[inline(always)]
        pub fn set_smspe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SMS preload source"]
        #[inline(always)]
        pub const fn smsps(&self) -> super::vals::Smsps {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Smsps::from_bits(val as u8)
        }
        #[doc = "SMS preload source"]
        #[inline(always)]
        pub fn set_smsps(&mut self, val: super::vals::Smsps) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for SmcrGp16 {
        #[inline(always)]
        fn default() -> SmcrGp16 {
            SmcrGp16(0)
        }
    }
    impl core::fmt::Debug for SmcrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SmcrGp16")
                .field("sms", &self.sms())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .field("etf", &self.etf())
                .field("etps", &self.etps())
                .field("ece", &self.ece())
                .field("etp", &self.etp())
                .field("smspe", &self.smspe())
                .field("smsps", &self.smsps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SmcrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SmcrGp16 {{ sms: {:?}, ts: {:?}, msm: {:?}, etf: {:?}, etps: {:?}, ece: {=bool:?}, etp: {:?}, smspe: {=bool:?}, smsps: {:?} }}" , self . sms () , self . ts () , self . msm () , self . etf () , self . etps () , self . ece () , self . etp () , self . smspe () , self . smsps ())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1ch(pub u32);
    impl Sr1ch {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr1ch {
        #[inline(always)]
        fn default() -> Sr1ch {
            Sr1ch(0)
        }
    }
    impl core::fmt::Debug for Sr1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1ch")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr1ch {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccof[0]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.ccof(0usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1chCmp(pub u32);
    impl Sr1chCmp {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn bif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[inline(always)]
        pub fn set_bif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr1chCmp {
        #[inline(always)]
        fn default() -> Sr1chCmp {
            Sr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Sr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1chCmp")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("comif", &self.comif())
                .field("bif[0]", &self.bif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1chCmp {{ uif: {=bool:?}, ccif[0]: {=bool:?}, comif: {=bool:?}, bif[0]: {=bool:?}, ccof[0]: {=bool:?} }}" , self . uif () , self . ccif (0usize) , self . comif () , self . bif (0usize) , self . ccof (0usize))
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2ch(pub u32);
    impl Sr2ch {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr2ch {
        #[inline(always)]
        fn default() -> Sr2ch {
            Sr2ch(0)
        }
    }
    impl core::fmt::Debug for Sr2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2ch")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("tif", &self.tif())
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr2ch {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, tif: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?} }}" , self . uif () , self . ccif (0usize) , self . ccif (1usize) , self . tif () , self . ccof (0usize) , self . ccof (1usize))
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2chCmp(pub u32);
    impl Sr2chCmp {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1,2) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1,2) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn bif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[inline(always)]
        pub fn set_bif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1,2) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1,2) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr2chCmp {
        #[inline(always)]
        fn default() -> Sr2chCmp {
            Sr2chCmp(0)
        }
    }
    impl core::fmt::Debug for Sr2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2chCmp")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif[0]", &self.bif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr2chCmp {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif[0]: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?} }}" , self . uif () , self . ccif (0usize) , self . ccif (1usize) , self . comif () , self . tif () , self . bif (0usize) , self . ccof (0usize) , self . ccof (1usize))
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrAdv(pub u32);
    impl SrAdv {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1,2) interrupt flag"]
        #[inline(always)]
        pub const fn bif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1,2) interrupt flag"]
        #[inline(always)]
        pub fn set_bif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "System break interrupt flag"]
        #[inline(always)]
        pub const fn sbif(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "System break interrupt flag"]
        #[inline(always)]
        pub fn set_sbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Capture/compare 5 interrupt flag"]
        #[inline(always)]
        pub const fn ccif5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 5 interrupt flag"]
        #[inline(always)]
        pub fn set_ccif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Capture/compare 6 interrupt flag"]
        #[inline(always)]
        pub const fn ccif6(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 6 interrupt flag"]
        #[inline(always)]
        pub fn set_ccif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Index interrupt flag"]
        #[inline(always)]
        pub const fn idxif(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Index interrupt flag"]
        #[inline(always)]
        pub fn set_idxif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Direction change interrupt flag"]
        #[inline(always)]
        pub const fn dirif(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change interrupt flag"]
        #[inline(always)]
        pub fn set_dirif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Index error interrupt flag"]
        #[inline(always)]
        pub const fn ierrif(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Index error interrupt flag"]
        #[inline(always)]
        pub fn set_ierrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Transition error interrupt flag"]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transition error interrupt flag"]
        #[inline(always)]
        pub fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for SrAdv {
        #[inline(always)]
        fn default() -> SrAdv {
            SrAdv(0)
        }
    }
    impl core::fmt::Debug for SrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrAdv")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("ccif[2]", &self.ccif(2usize))
                .field("ccif[3]", &self.ccif(3usize))
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif[0]", &self.bif(0usize))
                .field("bif[1]", &self.bif(1usize))
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .field("ccof[2]", &self.ccof(2usize))
                .field("ccof[3]", &self.ccof(3usize))
                .field("sbif", &self.sbif())
                .field("ccif5", &self.ccif5())
                .field("ccif6", &self.ccif6())
                .field("idxif", &self.idxif())
                .field("dirif", &self.dirif())
                .field("ierrif", &self.ierrif())
                .field("terrif", &self.terrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SrAdv {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, ccif[2]: {=bool:?}, ccif[3]: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif[0]: {=bool:?}, bif[1]: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?}, ccof[2]: {=bool:?}, ccof[3]: {=bool:?}, sbif: {=bool:?}, ccif5: {=bool:?}, ccif6: {=bool:?}, idxif: {=bool:?}, dirif: {=bool:?}, ierrif: {=bool:?}, terrif: {=bool:?} }}" , self . uif () , self . ccif (0usize) , self . ccif (1usize) , self . ccif (2usize) , self . ccif (3usize) , self . comif () , self . tif () , self . bif (0usize) , self . bif (1usize) , self . ccof (0usize) , self . ccof (1usize) , self . ccof (2usize) , self . ccof (3usize) , self . sbif () , self . ccif5 () , self . ccif6 () , self . idxif () , self . dirif () , self . ierrif () , self . terrif ())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrCore(pub u32);
    impl SrCore {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SrCore {
        #[inline(always)]
        fn default() -> SrCore {
            SrCore(0)
        }
    }
    impl core::fmt::Debug for SrCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrCore").field("uif", &self.uif()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SrCore {{ uif: {=bool:?} }}", self.uif())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrGp16(pub u32);
    impl SrGp16 {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Index interrupt flag"]
        #[inline(always)]
        pub const fn idxif(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Index interrupt flag"]
        #[inline(always)]
        pub fn set_idxif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Direction change interrupt flag"]
        #[inline(always)]
        pub const fn dirif(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change interrupt flag"]
        #[inline(always)]
        pub fn set_dirif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Index error interrupt flag"]
        #[inline(always)]
        pub const fn ierrif(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Index error interrupt flag"]
        #[inline(always)]
        pub fn set_ierrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Transition error interrupt flag"]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transition error interrupt flag"]
        #[inline(always)]
        pub fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for SrGp16 {
        #[inline(always)]
        fn default() -> SrGp16 {
            SrGp16(0)
        }
    }
    impl core::fmt::Debug for SrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrGp16")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("ccif[2]", &self.ccif(2usize))
                .field("ccif[3]", &self.ccif(3usize))
                .field("tif", &self.tif())
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .field("ccof[2]", &self.ccof(2usize))
                .field("ccof[3]", &self.ccof(3usize))
                .field("idxif", &self.idxif())
                .field("dirif", &self.dirif())
                .field("ierrif", &self.ierrif())
                .field("terrif", &self.terrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SrGp16 {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, ccif[2]: {=bool:?}, ccif[3]: {=bool:?}, tif: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?}, ccof[2]: {=bool:?}, ccof[3]: {=bool:?}, idxif: {=bool:?}, dirif: {=bool:?}, ierrif: {=bool:?}, terrif: {=bool:?} }}" , self . uif () , self . ccif (0usize) , self . ccif (1usize) , self . ccif (2usize) , self . ccif (3usize) , self . tif () , self . ccof (0usize) , self . ccof (1usize) , self . ccof (2usize) , self . ccof (3usize) , self . idxif () , self . dirif () , self . ierrif () , self . terrif ())
        }
    }
    #[doc = "input selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tisel1ch(pub u32);
    impl Tisel1ch {
        #[doc = "Selects TIM_TIx (x=1) input"]
        #[inline(always)]
        pub const fn tisel(&self, n: usize) -> u8 {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Selects TIM_TIx (x=1) input"]
        #[inline(always)]
        pub fn set_tisel(&mut self, n: usize, val: u8) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Tisel1ch {
        #[inline(always)]
        fn default() -> Tisel1ch {
            Tisel1ch(0)
        }
    }
    impl core::fmt::Debug for Tisel1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tisel1ch")
                .field("tisel[0]", &self.tisel(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tisel1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tisel1ch {{ tisel[0]: {=u8:?} }}", self.tisel(0usize))
        }
    }
    #[doc = "input selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tisel2ch(pub u32);
    impl Tisel2ch {
        #[doc = "Selects TIM_TIx (x=1-2) input"]
        #[inline(always)]
        pub const fn tisel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Selects TIM_TIx (x=1-2) input"]
        #[inline(always)]
        pub fn set_tisel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Tisel2ch {
        #[inline(always)]
        fn default() -> Tisel2ch {
            Tisel2ch(0)
        }
    }
    impl core::fmt::Debug for Tisel2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tisel2ch")
                .field("tisel[0]", &self.tisel(0usize))
                .field("tisel[1]", &self.tisel(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tisel2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tisel2ch {{ tisel[0]: {=u8:?}, tisel[1]: {=u8:?} }}",
                self.tisel(0usize),
                self.tisel(1usize)
            )
        }
    }
    #[doc = "input selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TiselGp16(pub u32);
    impl TiselGp16 {
        #[doc = "Selects TIM_TIx (x=1-4) input"]
        #[inline(always)]
        pub const fn tisel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Selects TIM_TIx (x=1-4) input"]
        #[inline(always)]
        pub fn set_tisel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for TiselGp16 {
        #[inline(always)]
        fn default() -> TiselGp16 {
            TiselGp16(0)
        }
    }
    impl core::fmt::Debug for TiselGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TiselGp16")
                .field("tisel[0]", &self.tisel(0usize))
                .field("tisel[1]", &self.tisel(1usize))
                .field("tisel[2]", &self.tisel(2usize))
                .field("tisel[3]", &self.tisel(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TiselGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TiselGp16 {{ tisel[0]: {=u8:?}, tisel[1]: {=u8:?}, tisel[2]: {=u8:?}, tisel[3]: {=u8:?} }}",
                self.tisel(0usize),
                self.tisel(1usize),
                self.tisel(2usize),
                self.tisel(3usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkbid {
        #[doc = "Break input tim_brk in input mode"]
        INPUT = 0x0,
        #[doc = "Break input tim_brk in bidirectional mode"]
        BIDIRECTIONAL = 0x01,
    }
    impl Bkbid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkbid {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkbid {
        #[inline(always)]
        fn from(val: u8) -> Bkbid {
            Bkbid::from_bits(val)
        }
    }
    impl From<Bkbid> for u8 {
        #[inline(always)]
        fn from(val: Bkbid) -> u8 {
            Bkbid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkdsrm {
        #[doc = "Break input tim_brk is armed"]
        ARMED = 0x0,
        #[doc = "Break input tim_brk is disarmed"]
        DISARMED = 0x01,
    }
    impl Bkdsrm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkdsrm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkdsrm {
        #[inline(always)]
        fn from(val: u8) -> Bkdsrm {
            Bkdsrm::from_bits(val)
        }
    }
    impl From<Bkdsrm> for u8 {
        #[inline(always)]
        fn from(val: Bkdsrm) -> u8 {
            Bkdsrm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkinp {
        #[doc = "input polarity is not inverted (active low if BKxP = 0, active high if BKxP = 1)"]
        NOT_INVERTED = 0x0,
        #[doc = "input polarity is inverted (active high if BKxP = 0, active low if BKxP = 1)"]
        INVERTED = 0x01,
    }
    impl Bkinp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkinp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkinp {
        #[inline(always)]
        fn from(val: u8) -> Bkinp {
            Bkinp::from_bits(val)
        }
    }
    impl From<Bkinp> for u8 {
        #[inline(always)]
        fn from(val: Bkinp) -> u8 {
            Bkinp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkp {
        #[doc = "Break input tim_brk is active low"]
        ACTIVE_LOW = 0x0,
        #[doc = "Break input tim_brk is active high"]
        ACTIVE_HIGH = 0x01,
    }
    impl Bkp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkp {
        #[inline(always)]
        fn from(val: u8) -> Bkp {
            Bkp::from_bits(val)
        }
    }
    impl From<Bkp> for u8 {
        #[inline(always)]
        fn from(val: Bkp) -> u8 {
            Bkp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ccds {
        #[doc = "CCx DMA request sent when CCx event occurs"]
        ON_COMPARE = 0x0,
        #[doc = "CCx DMA request sent when update event occurs"]
        ON_UPDATE = 0x01,
    }
    impl Ccds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccds {
        #[inline(always)]
        fn from(val: u8) -> Ccds {
            Ccds::from_bits(val)
        }
    }
    impl From<Ccds> for u8 {
        #[inline(always)]
        fn from(val: Ccds) -> u8 {
            Ccds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcmrInputCcs {
        _RESERVED_0 = 0x0,
        #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
        TI4 = 0x01,
        #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
        TI3 = 0x02,
        #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
        TRC = 0x03,
    }
    impl CcmrInputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrInputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrInputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrInputCcs {
            CcmrInputCcs::from_bits(val)
        }
    }
    impl From<CcmrInputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrInputCcs) -> u8 {
            CcmrInputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcmrOutputCcs {
        #[doc = "CCx channel is configured as output"]
        OUTPUT = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcmrOutputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrOutputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrOutputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrOutputCcs {
            CcmrOutputCcs::from_bits(val)
        }
    }
    impl From<CcmrOutputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrOutputCcs) -> u8 {
            CcmrOutputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckd {
        #[doc = "t_DTS = t_CK_INT"]
        DIV1 = 0x0,
        #[doc = "t_DTS = 2 × t_CK_INT"]
        DIV2 = 0x01,
        #[doc = "t_DTS = 4 × t_CK_INT"]
        DIV4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckd {
        #[inline(always)]
        fn from(val: u8) -> Ckd {
            Ckd::from_bits(val)
        }
    }
    impl From<Ckd> for u8 {
        #[inline(always)]
        fn from(val: Ckd) -> u8 {
            Ckd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cms {
        #[doc = "The counter counts up or down depending on the direction bit"]
        EDGE_ALIGNED = 0x0,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
        CENTER_ALIGNED1 = 0x01,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
        CENTER_ALIGNED2 = 0x02,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
        CENTER_ALIGNED3 = 0x03,
    }
    impl Cms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cms {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cms {
        #[inline(always)]
        fn from(val: u8) -> Cms {
            Cms::from_bits(val)
        }
    }
    impl From<Cms> for u8 {
        #[inline(always)]
        fn from(val: Cms) -> u8 {
            Cms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dbss {
        _RESERVED_0 = 0x0,
        #[doc = "Update"]
        UPDATE = 0x01,
        #[doc = "CC1"]
        CC1 = 0x02,
        #[doc = "CC2"]
        CC2 = 0x03,
        #[doc = "CC3"]
        CC3 = 0x04,
        #[doc = "CC4"]
        CC4 = 0x05,
        #[doc = "COM"]
        COM = 0x06,
        #[doc = "Trigger"]
        TRIGGER = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Dbss {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbss {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbss {
        #[inline(always)]
        fn from(val: u8) -> Dbss {
            Dbss::from_bits(val)
        }
    }
    impl From<Dbss> for u8 {
        #[inline(always)]
        fn from(val: Dbss) -> u8 {
            Dbss::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "Counter used as upcounter"]
        UP = 0x0,
        #[doc = "Counter used as downcounter"]
        DOWN = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dtae {
        #[doc = "Deadtime on rising and falling edges are identical, and defined with DTG\\[7:0\\]
register"]
        IDENTICAL = 0x0,
        #[doc = "Deadtime on rising edge is defined with DTG\\[7:0\\]
register and deadtime on falling edge is defined with DTGF\\[7:0\\]
bits."]
        DISTINCT = 0x01,
    }
    impl Dtae {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dtae {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dtae {
        #[inline(always)]
        fn from(val: u8) -> Dtae {
            Dtae::from_bits(val)
        }
    }
    impl From<Dtae> for u8 {
        #[inline(always)]
        fn from(val: Dtae) -> u8 {
            Dtae::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Etp {
        #[doc = "ETR is noninverted, active at high level or rising edge"]
        NOT_INVERTED = 0x0,
        #[doc = "ETR is inverted, active at low level or falling edge"]
        INVERTED = 0x01,
    }
    impl Etp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etp {
        #[inline(always)]
        fn from(val: u8) -> Etp {
            Etp::from_bits(val)
        }
    }
    impl From<Etp> for u8 {
        #[inline(always)]
        fn from(val: Etp) -> u8 {
            Etp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Etps {
        #[doc = "Prescaler OFF"]
        DIV1 = 0x0,
        #[doc = "ETRP frequency divided by 2"]
        DIV2 = 0x01,
        #[doc = "ETRP frequency divided by 4"]
        DIV4 = 0x02,
        #[doc = "ETRP frequency divided by 8"]
        DIV8 = 0x03,
    }
    impl Etps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etps {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etps {
        #[inline(always)]
        fn from(val: u8) -> Etps {
            Etps::from_bits(val)
        }
    }
    impl From<Etps> for u8 {
        #[inline(always)]
        fn from(val: Etps) -> u8 {
            Etps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fidx {
        #[doc = "Index is always active"]
        ALWAYS_ACTIVE = 0x0,
        #[doc = "the first Index only resets the counter"]
        FIRST_ONLY = 0x01,
    }
    impl Fidx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fidx {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fidx {
        #[inline(always)]
        fn from(val: u8) -> Fidx {
            Fidx::from_bits(val)
        }
    }
    impl From<Fidx> for u8 {
        #[inline(always)]
        fn from(val: Fidx) -> u8 {
            Fidx::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FilterValue {
        #[doc = "No filter, sampling is done at fDTS"]
        NO_FILTER = 0x0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FCK_INT_N2 = 0x01,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FCK_INT_N4 = 0x02,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FCK_INT_N8 = 0x03,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FDTS_DIV2_N6 = 0x04,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FDTS_DIV2_N8 = 0x05,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FDTS_DIV4_N6 = 0x06,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FDTS_DIV4_N8 = 0x07,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FDTS_DIV8_N6 = 0x08,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FDTS_DIV8_N8 = 0x09,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FDTS_DIV16_N5 = 0x0a,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FDTS_DIV16_N6 = 0x0b,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FDTS_DIV16_N8 = 0x0c,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FDTS_DIV32_N5 = 0x0d,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FDTS_DIV32_N6 = 0x0e,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FDTS_DIV32_N8 = 0x0f,
    }
    impl FilterValue {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterValue {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterValue {
        #[inline(always)]
        fn from(val: u8) -> FilterValue {
            FilterValue::from_bits(val)
        }
    }
    impl From<FilterValue> for u8 {
        #[inline(always)]
        fn from(val: FilterValue) -> u8 {
            FilterValue::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Gc5c {
        #[doc = "No effect of TIM_OC5REF on TIM_OCxREFC (x=1-3)"]
        NO_EFFECT = 0x0,
        #[doc = "TIM_OCxREFC is the logical AND of TIM_OCxREF and TIM_OC5REF"]
        LOGICAL_AND = 0x01,
    }
    impl Gc5c {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gc5c {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gc5c {
        #[inline(always)]
        fn from(val: u8) -> Gc5c {
            Gc5c::from_bits(val)
        }
    }
    impl From<Gc5c> for u8 {
        #[inline(always)]
        fn from(val: Gc5c) -> u8 {
            Gc5c::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iblk {
        #[doc = "Index always active"]
        ALWAYS_ACTIVE = 0x0,
        #[doc = "Index disabled when tim_ti3 input is active, as per CC3P bitfield"]
        CC3P = 0x01,
        #[doc = "Index disabled when tim_ti4 input is active, as per CC4P bitfield"]
        CC4P = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Iblk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iblk {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iblk {
        #[inline(always)]
        fn from(val: u8) -> Iblk {
            Iblk::from_bits(val)
        }
    }
    impl From<Iblk> for u8 {
        #[inline(always)]
        fn from(val: Iblk) -> u8 {
            Iblk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Idir {
        #[doc = "Index resets the counter whatever the direction"]
        BOTH = 0x0,
        #[doc = "Index resets the counter when up-counting only"]
        UP = 0x01,
        #[doc = "Index resets the counter when down-counting only"]
        DOWN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Idir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Idir {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Idir {
        #[inline(always)]
        fn from(val: u8) -> Idir {
            Idir::from_bits(val)
        }
    }
    impl From<Idir> for u8 {
        #[inline(always)]
        fn from(val: Idir) -> u8 {
            Idir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lock {
        #[doc = "No bit is write protected"]
        DISABLED = 0x0,
        #[doc = "DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written"]
        LEVEL1 = 0x01,
        #[doc = "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
        LEVEL2 = 0x02,
        #[doc = "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
        LEVEL3 = 0x03,
    }
    impl Lock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lock {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lock {
        #[inline(always)]
        fn from(val: u8) -> Lock {
            Lock::from_bits(val)
        }
    }
    impl From<Lock> for u8 {
        #[inline(always)]
        fn from(val: Lock) -> u8 {
            Lock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mms {
        #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
        RESET = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
        ENABLE = 0x01,
        #[doc = "The update event is selected as trigger output"]
        UPDATE = 0x02,
        #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        COMPARE_PULSE = 0x03,
        #[doc = "OC1REF signal is used as trigger output"]
        COMPARE_OC1 = 0x04,
        #[doc = "OC2REF signal is used as trigger output"]
        COMPARE_OC2 = 0x05,
        #[doc = "OC3REF signal is used as trigger output"]
        COMPARE_OC3 = 0x06,
        #[doc = "OC4REF signal is used as trigger output"]
        COMPARE_OC4 = 0x07,
        #[doc = "The encoder clock signal is used as trigger output"]
        ENCODER_CLOCK_OUTPUT = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mms {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mms {
        #[inline(always)]
        fn from(val: u8) -> Mms {
            Mms::from_bits(val)
        }
    }
    impl From<Mms> for u8 {
        #[inline(always)]
        fn from(val: Mms) -> u8 {
            Mms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mms2 {
        #[doc = "The UG bit from the TIMx_EGR register is used as TRGO2"]
        RESET = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as TRGO2"]
        ENABLE = 0x01,
        #[doc = "The update event is selected as TRGO2"]
        UPDATE = 0x02,
        #[doc = "TRGO2 send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        COMPARE_PULSE = 0x03,
        #[doc = "OC1REF signal is used as TRGO2"]
        COMPARE_OC1 = 0x04,
        #[doc = "OC2REF signal is used as TRGO2"]
        COMPARE_OC2 = 0x05,
        #[doc = "OC3REF signal is used as TRGO2"]
        COMPARE_OC3 = 0x06,
        #[doc = "OC4REF signal is used as TRGO2"]
        COMPARE_OC4 = 0x07,
        #[doc = "OC5REF signal is used as TRGO2"]
        COMPARE_OC5 = 0x08,
        #[doc = "OC6REF signal is used as TRGO2"]
        COMPARE_OC6 = 0x09,
        #[doc = "OC4REF rising or falling edges generate pulses on TRGO2"]
        COMPARE_PULSE_OC4 = 0x0a,
        #[doc = "OC6REF rising or falling edges generate pulses on TRGO2"]
        COMPARE_PULSE_OC6 = 0x0b,
        #[doc = "OC4REF or OC6REF rising edges generate pulses on TRGO2"]
        COMPARE_PULSE_OC4_OR_OC6_RISING = 0x0c,
        #[doc = "OC4REF rising or OC6REF falling edges generate pulses on TRGO2"]
        COMPARE_PULSE_OC4_RISING_OR_OC6_FALLING = 0x0d,
        #[doc = "OC5REF or OC6REF rising edges generate pulses on TRGO2"]
        COMPARE_PULSE_OC5_OR_OC6_RISING = 0x0e,
        #[doc = "OC5REF rising or OC6REF falling edges generate pulses on TRGO2"]
        COMPARE_PULSE_OC5_RISING_OR_OC6_FALLING = 0x0f,
    }
    impl Mms2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mms2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mms2 {
        #[inline(always)]
        fn from(val: u8) -> Mms2 {
            Mms2::from_bits(val)
        }
    }
    impl From<Mms2> for u8 {
        #[inline(always)]
        fn from(val: Mms2) -> u8 {
            Mms2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msm {
        #[doc = "No action"]
        NO_SYNC = 0x0,
        #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
        SYNC = 0x01,
    }
    impl Msm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msm {
        #[inline(always)]
        fn from(val: u8) -> Msm {
            Msm::from_bits(val)
        }
    }
    impl From<Msm> for u8 {
        #[inline(always)]
        fn from(val: Msm) -> u8 {
            Msm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Occs {
        #[doc = "tim_ocref_clr_int is connected to the tim_ocref_clr input"]
        INPUT = 0x0,
        #[doc = "tim_ocref_clr_int is connected to tim_etrf"]
        ETRF = 0x01,
    }
    impl Occs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Occs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Occs {
        #[inline(always)]
        fn from(val: u8) -> Occs {
            Occs::from_bits(val)
        }
    }
    impl From<Occs> for u8 {
        #[inline(always)]
        fn from(val: Occs) -> u8 {
            Occs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ocm {
        #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
        FROZEN = 0x0,
        #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
        ACTIVE_ON_MATCH = 0x01,
        #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
        INACTIVE_ON_MATCH = 0x02,
        #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
        TOGGLE = 0x03,
        #[doc = "OCyREF is forced low"]
        FORCE_INACTIVE = 0x04,
        #[doc = "OCyREF is forced high"]
        FORCE_ACTIVE = 0x05,
        #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
        PWM_MODE1 = 0x06,
        #[doc = "Inversely to PwmMode1"]
        PWM_MODE2 = 0x07,
        #[doc = "In up-counting mode, the channel is active until a trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update."]
        RETRIGERRABLE_OPM_MODE_1 = 0x08,
        #[doc = "In up-counting mode, the channel is inactive until a trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down- counting mode, the channel is active until a trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update."]
        RETRIGERRABLE_OPM_MODE_2 = 0x09,
        #[doc = "_reserved1"]
        _RESERVED1 = 0x0a,
        #[doc = "_reserved2"]
        _RESERVED2 = 0x0b,
        #[doc = "tim_oc1ref has the same behavior as in PWM mode 1. tim_oc1refc is the logical OR between tim_oc1ref and tim_oc2ref."]
        COMBINED_PWM_MODE_1 = 0x0c,
        #[doc = "tim_oc1ref has the same behavior as in PWM mode 2. tim_oc1refc is the logical AND between tim_oc1ref and tim_oc2ref."]
        COMBINED_PWM_MODE_2 = 0x0d,
        #[doc = "tim_oc1ref has the same behavior as in PWM mode 1. tim_oc1refc outputs tim_oc1ref when the counter is counting up, tim_oc2ref when it is counting down."]
        ASYMMETRIC_PWM_MODE_1 = 0x0e,
        #[doc = "tim_oc1ref has the same behavior as in PWM mode 2. tim_oc1refc outputs tim_oc1ref when the counter is counting up, tim_oc2ref when it is counting down."]
        ASYMMETRIC_PWM_MODE_2 = 0x0f,
    }
    impl Ocm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocm {
        #[inline(always)]
        fn from(val: u8) -> Ocm {
            Ocm::from_bits(val)
        }
    }
    impl From<Ocm> for u8 {
        #[inline(always)]
        fn from(val: Ocm) -> u8 {
            Ocm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ossi {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        DISABLED = 0x0,
        #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
        IDLE_LEVEL = 0x01,
    }
    impl Ossi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ossi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ossi {
        #[inline(always)]
        fn from(val: u8) -> Ossi {
            Ossi::from_bits(val)
        }
    }
    impl From<Ossi> for u8 {
        #[inline(always)]
        fn from(val: Ossi) -> u8 {
            Ossi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ossr {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        DISABLED = 0x0,
        #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
        IDLE_LEVEL = 0x01,
    }
    impl Ossr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ossr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ossr {
        #[inline(always)]
        fn from(val: u8) -> Ossr {
            Ossr::from_bits(val)
        }
    }
    impl From<Ossr> for u8 {
        #[inline(always)]
        fn from(val: Ossr) -> u8 {
            Ossr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sms {
        #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
        DISABLED = 0x0,
        #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
        ENCODER_MODE_1 = 0x01,
        #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
        ENCODER_MODE_2 = 0x02,
        #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
        ENCODER_MODE_3 = 0x03,
        #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
        RESET_MODE = 0x04,
        #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
        GATED_MODE = 0x05,
        #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
        TRIGGER_MODE = 0x06,
        #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
        EXT_CLOCK_MODE = 0x07,
        #[doc = "Rising edge of the selected trigger input (tim_trgi) reinitializes the counter, generates an update of the registers and starts the counter."]
        COMBINED_RESET_TRIGGER = 0x08,
        #[doc = "The counter clock is enabled when the trigger input (tim_trgi) is high. The counter stops and is reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
        COMBINED_GATED_TRIGGER = 0x09,
        #[doc = "Encoder mode, Clock plus direction, x2 mode."]
        ENCODER_UP_X2 = 0x0a,
        #[doc = "Encoder mode, Clock plus direction, x1 mode, tim_ti2fp2 edge sensitivity is set by CC2P"]
        ENCODER_UP_X1 = 0x0b,
        #[doc = "Encoder mode, Directional Clock, x2 mode."]
        ENCODER_DIR_X2 = 0x0c,
        #[doc = "Encoder mode, Directional Clock, x1 mode, tim_ti1fp1 and tim_ti2fp2 edge sensitivity is set by CC1P and CC2P."]
        ENCODER_DIR_X1 = 0x0d,
        #[doc = "Quadrature encoder mode, x1 mode, counting on tim_ti1fp1 edges only, edge sensitivity is set by CC1P."]
        QUADRATURE_ENCODER_MODE_X1_TI1PF1 = 0x0e,
        #[doc = "Quadrature encoder mode, x1 mode, counting on tim_ti2fp2 edges only, edge sensitivity is set by CC2P."]
        QUADRATURE_ENCODER_MODE_X1_TI2PF2 = 0x0f,
    }
    impl Sms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sms {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sms {
        #[inline(always)]
        fn from(val: u8) -> Sms {
            Sms::from_bits(val)
        }
    }
    impl From<Sms> for u8 {
        #[inline(always)]
        fn from(val: Sms) -> u8 {
            Sms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smsps {
        #[doc = "The transfer is triggered by the Timer’s Update event"]
        UPDATE = 0x0,
        #[doc = "The transfer is triggered by the Index event"]
        INDEX = 0x01,
    }
    impl Smsps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smsps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smsps {
        #[inline(always)]
        fn from(val: u8) -> Smsps {
            Smsps::from_bits(val)
        }
    }
    impl From<Smsps> for u8 {
        #[inline(always)]
        fn from(val: Smsps) -> u8 {
            Smsps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ti1s {
        #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
        NORMAL = 0x0,
        #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
        XOR = 0x01,
    }
    impl Ti1s {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ti1s {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ti1s {
        #[inline(always)]
        fn from(val: u8) -> Ti1s {
            Ti1s::from_bits(val)
        }
    }
    impl From<Ti1s> for u8 {
        #[inline(always)]
        fn from(val: Ti1s) -> u8 {
            Ti1s::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ts {
        #[doc = "Internal Trigger 0"]
        ITR0 = 0x0,
        #[doc = "Internal Trigger 1"]
        ITR1 = 0x01,
        #[doc = "Internal Trigger 2"]
        ITR2 = 0x02,
        #[doc = "Internal Trigger 3"]
        ITR3 = 0x03,
        #[doc = "TI1 Edge Detector"]
        TI1F_ED = 0x04,
        #[doc = "Filtered Timer Input 1"]
        TI1FP1 = 0x05,
        #[doc = "Filtered Timer Input 2"]
        TI2FP2 = 0x06,
        #[doc = "External Trigger input"]
        ETRF = 0x07,
        #[doc = "Internal Trigger 4"]
        ITR4 = 0x08,
        #[doc = "Internal Trigger 5"]
        ITR5 = 0x09,
        #[doc = "Internal Trigger 6"]
        ITR6 = 0x0a,
        #[doc = "Internal Trigger 7"]
        ITR7 = 0x0b,
        #[doc = "Internal Trigger 8"]
        ITR8 = 0x0c,
        #[doc = "Internal Trigger 9"]
        ITR9 = 0x0d,
        #[doc = "Internal Trigger 10"]
        ITR10 = 0x0e,
        #[doc = "Internal Trigger 11"]
        ITR11 = 0x0f,
        #[doc = "Internal Trigger 12"]
        ITR12 = 0x10,
        #[doc = "Internal Trigger 13"]
        ITR13 = 0x11,
        #[doc = "Internal Trigger 14"]
        ITR14 = 0x12,
        #[doc = "Internal Trigger 15"]
        ITR15 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Ts {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ts {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ts {
        #[inline(always)]
        fn from(val: u8) -> Ts {
            Ts::from_bits(val)
        }
    }
    impl From<Ts> for u8 {
        #[inline(always)]
        fn from(val: Ts) -> u8 {
            Ts::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Urs {
        #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
        ANY_EVENT = 0x0,
        #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
        COUNTER_ONLY = 0x01,
    }
    impl Urs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Urs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Urs {
        #[inline(always)]
        fn from(val: u8) -> Urs {
            Urs::from_bits(val)
        }
    }
    impl From<Urs> for u8 {
        #[inline(always)]
        fn from(val: Urs) -> u8 {
            Urs::to_bits(val)
        }
    }
}
