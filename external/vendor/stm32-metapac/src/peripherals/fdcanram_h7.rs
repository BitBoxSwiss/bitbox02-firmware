#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FDCAN Message RAM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcanram {
    ptr: *mut u8,
}
unsafe impl Send for Fdcanram {}
unsafe impl Sync for Fdcanram {}
impl Fdcanram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FDCAN Message RAM"]
    #[inline(always)]
    pub const fn ram(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2560usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
