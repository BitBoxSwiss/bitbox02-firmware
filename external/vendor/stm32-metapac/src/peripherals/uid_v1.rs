#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Device Factory programmed 96-bit unique device identifier"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uid {
    ptr: *mut u8,
}
unsafe impl Send for Uid {}
unsafe impl Sync for Uid {}
impl Uid {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Factory programmed 96-bit unique device identifier word 0"]
    #[inline(always)]
    pub const fn uid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
