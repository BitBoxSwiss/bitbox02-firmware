#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB Endpoint memory"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbram {
    ptr: *mut u8,
}
unsafe impl Send for Usbram {}
unsafe impl Sync for Usbram {}
impl Usbram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB Endpoint memory"]
    #[inline(always)]
    pub const fn mem(self, n: usize) -> crate::common::Reg<u16, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
