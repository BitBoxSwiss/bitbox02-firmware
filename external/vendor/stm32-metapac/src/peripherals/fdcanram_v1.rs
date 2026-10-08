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
    #[doc = "11-bit filter"]
    #[inline(always)]
    pub const fn flssa(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 28usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "29-bit filter"]
    #[inline(always)]
    pub const fn flesa(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
    #[doc = "Rx FIFO 0"]
    #[inline(always)]
    pub const fn rxfifo0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 54usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize + n * 4usize) as _) }
    }
    #[doc = "Rx FIFO 1"]
    #[inline(always)]
    pub const fn rxfifo1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 54usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize + n * 4usize) as _) }
    }
    #[doc = "Tx event FIFO"]
    #[inline(always)]
    pub const fn txefifo(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize + n * 4usize) as _) }
    }
    #[doc = "Tx buffer"]
    #[inline(always)]
    pub const fn txbuf(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 54usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize + n * 4usize) as _) }
    }
}
