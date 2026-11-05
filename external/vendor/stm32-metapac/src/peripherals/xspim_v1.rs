#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "XSPIM1 register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xspim {
    ptr: *mut u8,
}
unsafe impl Send for Xspim {}
unsafe impl Sync for Xspim {}
impl Xspim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "XSPIM control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "XSPIM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Multiplexed mode enable This bit enables the multiplexing of the two XSPIs."]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multiplexed mode enable This bit enables the multiplexing of the two XSPIs."]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "XSPI multiplexing mode."]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI multiplexing mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Chip select selector override enable."]
        #[inline(always)]
        pub const fn cssel_ovr_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Chip select selector override enable."]
        #[inline(always)]
        pub fn set_cssel_ovr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Chip select selector override setting for XSPI1."]
        #[inline(always)]
        pub const fn cssel_ovr_o1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Chip select selector override setting for XSPI1."]
        #[inline(always)]
        pub fn set_cssel_ovr_o1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Chip select selector override setting for XSPI2."]
        #[inline(always)]
        pub const fn cssel_ovr_o2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Chip select selector override setting for XSPI2."]
        #[inline(always)]
        pub fn set_cssel_ovr_o2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "REQ to ACK time In Multiplexed mode (MUXEN = 1), this field defines the time between two transactions."]
        #[inline(always)]
        pub const fn req2ack_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "REQ to ACK time In Multiplexed mode (MUXEN = 1), this field defines the time between two transactions."]
        #[inline(always)]
        pub fn set_req2ack_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("muxen", &self.muxen())
                .field("mode", &self.mode())
                .field("cssel_ovr_en", &self.cssel_ovr_en())
                .field("cssel_ovr_o1", &self.cssel_ovr_o1())
                .field("cssel_ovr_o2", &self.cssel_ovr_o2())
                .field("req2ack_time", &self.req2ack_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ muxen: {=bool:?}, mode: {=bool:?}, cssel_ovr_en: {=bool:?}, cssel_ovr_o1: {=bool:?}, cssel_ovr_o2: {=bool:?}, req2ack_time: {=u8:?} }}" , self . muxen () , self . mode () , self . cssel_ovr_en () , self . cssel_ovr_o1 () , self . cssel_ovr_o2 () , self . req2ack_time ())
        }
    }
}
