#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "IPCC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcc {
    ptr: *mut u8,
}
unsafe impl Send for Ipcc {}
unsafe impl Sync for Ipcc {}
impl Ipcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CPU specific registers"]
    #[inline(always)]
    pub const fn cpu(self, n: usize) -> IpccCpu {
        assert!(n < 2usize);
        unsafe { IpccCpu::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
}
#[doc = "IPCC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpccCpu {
    ptr: *mut u8,
}
unsafe impl Send for IpccCpu {}
unsafe impl Sync for IpccCpu {}
impl IpccCpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register CPUx"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::CxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Mask register CPUx"]
    #[inline(always)]
    pub const fn mr(self) -> crate::common::Reg<regs::CxMr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status Set or Clear register CPUx"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::CxScr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CPUx to CPUy status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::CxToySr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register CPUx"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxCr(pub u32);
    impl CxCr {
        #[doc = "processor x Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "processor x Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "processor x Transmit channel free interrupt enable"]
        #[inline(always)]
        pub const fn txfie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "processor x Transmit channel free interrupt enable"]
        #[inline(always)]
        pub fn set_txfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for CxCr {
        #[inline(always)]
        fn default() -> CxCr {
            CxCr(0)
        }
    }
    impl core::fmt::Debug for CxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CxCr")
                .field("rxoie", &self.rxoie())
                .field("txfie", &self.txfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CxCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CxCr {{ rxoie: {=bool:?}, txfie: {=bool:?} }}",
                self.rxoie(),
                self.txfie()
            )
        }
    }
    #[doc = "Mask register CPUx"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxMr(pub u32);
    impl CxMr {
        #[doc = "processor x Receive channel y occupied interrupt enable"]
        #[inline(always)]
        pub const fn chom(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Receive channel y occupied interrupt enable"]
        #[inline(always)]
        pub fn set_chom(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor x Transmit channel y free interrupt mask"]
        #[inline(always)]
        pub const fn chfm(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Transmit channel y free interrupt mask"]
        #[inline(always)]
        pub fn set_chfm(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CxMr {
        #[inline(always)]
        fn default() -> CxMr {
            CxMr(0)
        }
    }
    impl core::fmt::Debug for CxMr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CxMr")
                .field("chom[0]", &self.chom(0usize))
                .field("chom[1]", &self.chom(1usize))
                .field("chom[2]", &self.chom(2usize))
                .field("chom[3]", &self.chom(3usize))
                .field("chom[4]", &self.chom(4usize))
                .field("chom[5]", &self.chom(5usize))
                .field("chfm[0]", &self.chfm(0usize))
                .field("chfm[1]", &self.chfm(1usize))
                .field("chfm[2]", &self.chfm(2usize))
                .field("chfm[3]", &self.chfm(3usize))
                .field("chfm[4]", &self.chfm(4usize))
                .field("chfm[5]", &self.chfm(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CxMr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CxMr {{ chom[0]: {=bool:?}, chom[1]: {=bool:?}, chom[2]: {=bool:?}, chom[3]: {=bool:?}, chom[4]: {=bool:?}, chom[5]: {=bool:?}, chfm[0]: {=bool:?}, chfm[1]: {=bool:?}, chfm[2]: {=bool:?}, chfm[3]: {=bool:?}, chfm[4]: {=bool:?}, chfm[5]: {=bool:?} }}" , self . chom (0usize) , self . chom (1usize) , self . chom (2usize) , self . chom (3usize) , self . chom (4usize) , self . chom (5usize) , self . chfm (0usize) , self . chfm (1usize) , self . chfm (2usize) , self . chfm (3usize) , self . chfm (4usize) , self . chfm (5usize))
        }
    }
    #[doc = "Status Set or Clear register CPUx"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxScr(pub u32);
    impl CxScr {
        #[doc = "processor x Receive channel y status clear"]
        #[inline(always)]
        pub const fn chc(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Receive channel y status clear"]
        #[inline(always)]
        pub fn set_chc(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor x Transmit channel y status set"]
        #[inline(always)]
        pub const fn chs(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Transmit channel y status set"]
        #[inline(always)]
        pub fn set_chs(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CxScr {
        #[inline(always)]
        fn default() -> CxScr {
            CxScr(0)
        }
    }
    impl core::fmt::Debug for CxScr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CxScr")
                .field("chc[0]", &self.chc(0usize))
                .field("chc[1]", &self.chc(1usize))
                .field("chc[2]", &self.chc(2usize))
                .field("chc[3]", &self.chc(3usize))
                .field("chc[4]", &self.chc(4usize))
                .field("chc[5]", &self.chc(5usize))
                .field("chs[0]", &self.chs(0usize))
                .field("chs[1]", &self.chs(1usize))
                .field("chs[2]", &self.chs(2usize))
                .field("chs[3]", &self.chs(3usize))
                .field("chs[4]", &self.chs(4usize))
                .field("chs[5]", &self.chs(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CxScr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CxScr {{ chc[0]: {=bool:?}, chc[1]: {=bool:?}, chc[2]: {=bool:?}, chc[3]: {=bool:?}, chc[4]: {=bool:?}, chc[5]: {=bool:?}, chs[0]: {=bool:?}, chs[1]: {=bool:?}, chs[2]: {=bool:?}, chs[3]: {=bool:?}, chs[4]: {=bool:?}, chs[5]: {=bool:?} }}" , self . chc (0usize) , self . chc (1usize) , self . chc (2usize) , self . chc (3usize) , self . chc (4usize) , self . chc (5usize) , self . chs (0usize) , self . chs (1usize) , self . chs (2usize) , self . chs (3usize) , self . chs (4usize) , self . chs (5usize))
        }
    }
    #[doc = "CPUx to CPUy status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxToySr(pub u32);
    impl CxToySr {
        #[doc = "processor x transmit to process y Receive channel z status flag"]
        #[inline(always)]
        pub const fn chf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x transmit to process y Receive channel z status flag"]
        #[inline(always)]
        pub fn set_chf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CxToySr {
        #[inline(always)]
        fn default() -> CxToySr {
            CxToySr(0)
        }
    }
    impl core::fmt::Debug for CxToySr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CxToySr")
                .field("chf[0]", &self.chf(0usize))
                .field("chf[1]", &self.chf(1usize))
                .field("chf[2]", &self.chf(2usize))
                .field("chf[3]", &self.chf(3usize))
                .field("chf[4]", &self.chf(4usize))
                .field("chf[5]", &self.chf(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CxToySr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CxToySr {{ chf[0]: {=bool:?}, chf[1]: {=bool:?}, chf[2]: {=bool:?}, chf[3]: {=bool:?}, chf[4]: {=bool:?}, chf[5]: {=bool:?} }}" , self . chf (0usize) , self . chf (1usize) , self . chf (2usize) , self . chf (3usize) , self . chf (4usize) , self . chf (5usize))
        }
    }
}
