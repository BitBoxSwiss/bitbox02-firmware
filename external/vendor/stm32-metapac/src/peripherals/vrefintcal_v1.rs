#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "VREFINT Factory Calibration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrefintcal {
    ptr: *mut u8,
}
unsafe impl Send for Vrefintcal {}
unsafe impl Sync for Vrefintcal {}
impl Vrefintcal {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Factory calibration"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
