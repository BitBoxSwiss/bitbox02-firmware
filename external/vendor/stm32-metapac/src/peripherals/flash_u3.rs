#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLASH access control register."]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FLASH nonsecure key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH secure key register."]
    #[inline(always)]
    pub const fn skeyr(self) -> crate::common::Reg<regs::Skeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH option key register."]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<regs::Optkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH bank 1 power-down key register."]
    #[inline(always)]
    pub const fn pdkey1r(self) -> crate::common::Reg<regs::Pdkey1r, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH bank 2 power-down key register."]
    #[inline(always)]
    pub const fn pdkey2r(self) -> crate::common::Reg<regs::Pdkey2r, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FLASH nonsecure status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH secure status register."]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH nonsecure control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH secure control register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FLASH ECC register."]
    #[inline(always)]
    pub const fn ecccorr(self) -> crate::common::Reg<regs::Ecccorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH ECC detection register."]
    #[inline(always)]
    pub const fn eccdetr(self) -> crate::common::Reg<regs::Eccdetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "FLASH operation status register."]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "FLASH option register."]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FLASH nonsecure boot address 0 register."]
    #[inline(always)]
    pub const fn boot0r(self) -> crate::common::Reg<regs::Boot0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FLASH nonsecure boot address 1 register."]
    #[inline(always)]
    pub const fn boot1r(self) -> crate::common::Reg<regs::Boot1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FLASH secure boot address 0 register."]
    #[inline(always)]
    pub const fn sboot0r(self) -> crate::common::Reg<regs::Sboot0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "FLASH secure watermark1 register 1."]
    #[inline(always)]
    pub const fn secwm1r1(self) -> crate::common::Reg<regs::Secwm1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FLASH secure watermark1 register 2."]
    #[inline(always)]
    pub const fn secwm1r2(self) -> crate::common::Reg<regs::Secwm1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FLASH WRP1 area A address register."]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "FLASH WRP1 area B address register."]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "FLASH secure watermark2 register 1."]
    #[inline(always)]
    pub const fn secwm2r1(self) -> crate::common::Reg<regs::Secwm2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "FLASH secure watermark2 register 2."]
    #[inline(always)]
    pub const fn secwm2r2(self) -> crate::common::Reg<regs::Secwm2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "FLASH WPR2 area A address register."]
    #[inline(always)]
    pub const fn wrp2ar(self) -> crate::common::Reg<regs::Wrp2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "FLASH WPR2 area B address register."]
    #[inline(always)]
    pub const fn wrp2br(self) -> crate::common::Reg<regs::Wrp2br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 1."]
    #[inline(always)]
    pub const fn secbb1r1(self) -> crate::common::Reg<regs::Secbb1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 2."]
    #[inline(always)]
    pub const fn secbb1r2(self) -> crate::common::Reg<regs::Secbb1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 3."]
    #[inline(always)]
    pub const fn secbb1r3(self) -> crate::common::Reg<regs::Secbb1r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 4."]
    #[inline(always)]
    pub const fn secbb1r4(self) -> crate::common::Reg<regs::Secbb1r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 1."]
    #[inline(always)]
    pub const fn secbb2r1(self) -> crate::common::Reg<regs::Secbb2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 2."]
    #[inline(always)]
    pub const fn secbb2r2(self) -> crate::common::Reg<regs::Secbb2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 3."]
    #[inline(always)]
    pub const fn secbb2r3(self) -> crate::common::Reg<regs::Secbb2r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 4."]
    #[inline(always)]
    pub const fn secbb2r4(self) -> crate::common::Reg<regs::Secbb2r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FLASH secure HDP control register."]
    #[inline(always)]
    pub const fn sechdpcr(self) -> crate::common::Reg<regs::Sechdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FLASH privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FLASH HDP extension register."]
    #[inline(always)]
    pub const fn sechdpex_tr(self) -> crate::common::Reg<regs::SechdpexTr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "FLASH privilege block-based bank 1 register 1."]
    #[inline(always)]
    pub const fn privbb1r1(self) -> crate::common::Reg<regs::Privbb1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "FLASH privilege block-based bank 1 register 2."]
    #[inline(always)]
    pub const fn privbb1r2(self) -> crate::common::Reg<regs::Privbb1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "FLASH privilege block-based bank 1 register 3."]
    #[inline(always)]
    pub const fn privbb1r3(self) -> crate::common::Reg<regs::Privbb1r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "FLASH privilege block-based bank 1 register 4."]
    #[inline(always)]
    pub const fn privbb1r4(self) -> crate::common::Reg<regs::Privbb1r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 1."]
    #[inline(always)]
    pub const fn privbb2r1(self) -> crate::common::Reg<regs::Privbb2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 2."]
    #[inline(always)]
    pub const fn privbb2r2(self) -> crate::common::Reg<regs::Privbb2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 3."]
    #[inline(always)]
    pub const fn privbb2r3(self) -> crate::common::Reg<regs::Privbb2r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 4."]
    #[inline(always)]
    pub const fn privbb2r4(self) -> crate::common::Reg<regs::Privbb2r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "FLASH OEM1 key register 1."]
    #[inline(always)]
    pub const fn oem1keyr1(self) -> crate::common::Reg<regs::Oem1keyr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "FLASH OEM1 key register 2."]
    #[inline(always)]
    pub const fn oem1keyr2(self) -> crate::common::Reg<regs::Oem1keyr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "FLASH OEM1 key register 3."]
    #[inline(always)]
    pub const fn oem1keyr3(self) -> crate::common::Reg<regs::Oem1keyr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "FLASH OEM1 key register 4."]
    #[inline(always)]
    pub const fn oem1keyr4(self) -> crate::common::Reg<regs::Oem1keyr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "FLASH OEM2 key register 1."]
    #[inline(always)]
    pub const fn oem2keyr1(self) -> crate::common::Reg<regs::Oem2keyr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 2."]
    #[inline(always)]
    pub const fn oem2keyr2(self) -> crate::common::Reg<regs::Oem2keyr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 3."]
    #[inline(always)]
    pub const fn oem2keyr3(self) -> crate::common::Reg<regs::Oem2keyr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 4."]
    #[inline(always)]
    pub const fn oem2keyr4(self) -> crate::common::Reg<regs::Oem2keyr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "FLASH OEM key status register."]
    #[inline(always)]
    pub const fn oemkeysr(self) -> crate::common::Reg<regs::Oemkeysr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Latency."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Prefetch enable."]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable."]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power read mode."]
        #[inline(always)]
        pub const fn lpm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power read mode."]
        #[inline(always)]
        pub fn set_lpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Bank 1 power-down mode request."]
        #[inline(always)]
        pub const fn pdreq1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 power-down mode request."]
        #[inline(always)]
        pub fn set_pdreq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Bank 2 power-down mode request."]
        #[inline(always)]
        pub const fn pdreq2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 power-down mode request."]
        #[inline(always)]
        pub fn set_pdreq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash memory power-down mode during Sleep mode."]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory power-down mode during Sleep mode."]
        #[inline(always)]
        pub fn set_sleep_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("prften", &self.prften())
                .field("lpm", &self.lpm())
                .field("pdreq1", &self.pdreq1())
                .field("pdreq2", &self.pdreq2())
                .field("sleep_pd", &self.sleep_pd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Acr {{ latency: {=u8:?}, prften: {=bool:?}, lpm: {=bool:?}, pdreq1: {=bool:?}, pdreq2: {=bool:?}, sleep_pd: {=bool:?} }}" , self . latency () , self . prften () , self . lpm () , self . pdreq1 () , self . pdreq2 () , self . sleep_pd ())
        }
    }
    #[doc = "FLASH nonsecure boot address 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Boot0r(pub u32);
    impl Boot0r {
        #[doc = "Nonsecure boot base address 0."]
        #[inline(always)]
        pub const fn add(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Nonsecure boot base address 0."]
        #[inline(always)]
        pub fn set_add(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Boot0r {
        #[inline(always)]
        fn default() -> Boot0r {
            Boot0r(0)
        }
    }
    impl core::fmt::Debug for Boot0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Boot0r").field("add", &self.add()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Boot0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Boot0r {{ add: {=u32:?} }}", self.add())
        }
    }
    #[doc = "FLASH nonsecure boot address 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Boot1r(pub u32);
    impl Boot1r {
        #[doc = "nonsecure boot address 1."]
        #[inline(always)]
        pub const fn add(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "nonsecure boot address 1."]
        #[inline(always)]
        pub fn set_add(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Boot1r {
        #[inline(always)]
        fn default() -> Boot1r {
            Boot1r(0)
        }
    }
    impl core::fmt::Debug for Boot1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Boot1r").field("add", &self.add()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Boot1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Boot1r {{ add: {=u32:?} }}", self.add())
        }
    }
    #[doc = "FLASH nonsecure control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Nonsecure programming."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure programming."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Nonsecure page erase."]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure page erase."]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Nonsecure bank 1 mass erase."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure bank 1 mass erase."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Nonsecure page number selection."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Nonsecure page number selection."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "Nonsecure bank selection for page erase."]
        #[inline(always)]
        pub const fn bker(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure bank selection for page erase."]
        #[inline(always)]
        pub fn set_bker(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Nonsecure burst write programming mode."]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure burst write programming mode."]
        #[inline(always)]
        pub fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Nonsecure bank 2 mass erase."]
        #[inline(always)]
        pub const fn mer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure bank 2 mass erase."]
        #[inline(always)]
        pub fn set_mer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Nonsecure start."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure start."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Option modification start."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Option modification start."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Nonsecure end of operation interrupt enable."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure end of operation interrupt enable."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Nonsecure error interrupt enable."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure error interrupt enable."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Option-byte loading forced."]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Option-byte loading forced."]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Option lock."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option lock."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Nonsecure lock."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure lock."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("mer1", &self.mer1())
                .field("pnb", &self.pnb())
                .field("bker", &self.bker())
                .field("bwr", &self.bwr())
                .field("mer2", &self.mer2())
                .field("strt", &self.strt())
                .field("optstrt", &self.optstrt())
                .field("eopie", &self.eopie())
                .field("errie", &self.errie())
                .field("obl_launch", &self.obl_launch())
                .field("optlock", &self.optlock())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ pg: {=bool:?}, per: {=bool:?}, mer1: {=bool:?}, pnb: {=u8:?}, bker: {=bool:?}, bwr: {=bool:?}, mer2: {=bool:?}, strt: {=bool:?}, optstrt: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, obl_launch: {=bool:?}, optlock: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . per () , self . mer1 () , self . pnb () , self . bker () , self . bwr () , self . mer2 () , self . strt () , self . optstrt () , self . eopie () , self . errie () , self . obl_launch () , self . optlock () , self . lock ())
        }
    }
    #[doc = "FLASH ECC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecccorr(pub u32);
    impl Ecccorr {
        #[doc = "ECC fail address."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ECC fail address."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "ECC fail bank."]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank."]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "System flash memory ECC fail."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "System flash memory ECC fail."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC correction interrupt enable."]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable."]
        #[inline(always)]
        pub fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC correction."]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction."]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ecccorr {
        #[inline(always)]
        fn default() -> Ecccorr {
            Ecccorr(0)
        }
    }
    impl core::fmt::Debug for Ecccorr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecccorr")
                .field("addr_ecc", &self.addr_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("eccie", &self.eccie())
                .field("eccc", &self.eccc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecccorr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ecccorr {{ addr_ecc: {=u32:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, eccie: {=bool:?}, eccc: {=bool:?} }}" , self . addr_ecc () , self . bk_ecc () , self . sysf_ecc () , self . eccie () , self . eccc ())
        }
    }
    #[doc = "FLASH ECC detection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdetr(pub u32);
    impl Eccdetr {
        #[doc = "ECC fail address."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ECC fail address."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "ECC fail bank."]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank."]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "System flash memory ECC fail."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "System flash memory ECC fail."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC detection."]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection."]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccdetr {
        #[inline(always)]
        fn default() -> Eccdetr {
            Eccdetr(0)
        }
    }
    impl core::fmt::Debug for Eccdetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdetr")
                .field("addr_ecc", &self.addr_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eccdetr {{ addr_ecc: {=u32:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, eccd: {=bool:?} }}",
                self.addr_ecc(),
                self.bk_ecc(),
                self.sysf_ecc(),
                self.eccd()
            )
        }
    }
    #[doc = "FLASH nonsecure key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Flash memory nonsecure key."]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash memory nonsecure key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Keyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "FLASH OEM1 key register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr1(pub u32);
    impl Oem1keyr1 {
        #[doc = "OEM1\\[31:0\\]
bytes key."]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1\\[31:0\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem1key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem1keyr1 {
        #[inline(always)]
        fn default() -> Oem1keyr1 {
            Oem1keyr1(0)
        }
    }
    impl core::fmt::Debug for Oem1keyr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem1keyr1").field("oem1key", &self.oem1key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem1keyr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem1keyr1 {{ oem1key: {=u32:?} }}", self.oem1key())
        }
    }
    #[doc = "FLASH OEM1 key register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr2(pub u32);
    impl Oem1keyr2 {
        #[doc = "OEM1\\[63:32\\]
bytes key."]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1\\[63:32\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem1key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem1keyr2 {
        #[inline(always)]
        fn default() -> Oem1keyr2 {
            Oem1keyr2(0)
        }
    }
    impl core::fmt::Debug for Oem1keyr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem1keyr2").field("oem1key", &self.oem1key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem1keyr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem1keyr2 {{ oem1key: {=u32:?} }}", self.oem1key())
        }
    }
    #[doc = "FLASH OEM1 key register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr3(pub u32);
    impl Oem1keyr3 {
        #[doc = "OEM1\\[95:64\\]
bytes key."]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1\\[95:64\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem1key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem1keyr3 {
        #[inline(always)]
        fn default() -> Oem1keyr3 {
            Oem1keyr3(0)
        }
    }
    impl core::fmt::Debug for Oem1keyr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem1keyr3").field("oem1key", &self.oem1key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem1keyr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem1keyr3 {{ oem1key: {=u32:?} }}", self.oem1key())
        }
    }
    #[doc = "FLASH OEM1 key register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr4(pub u32);
    impl Oem1keyr4 {
        #[doc = "OEM1\\[127:96\\]
bytes key."]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1\\[127:96\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem1key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem1keyr4 {
        #[inline(always)]
        fn default() -> Oem1keyr4 {
            Oem1keyr4(0)
        }
    }
    impl core::fmt::Debug for Oem1keyr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem1keyr4").field("oem1key", &self.oem1key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem1keyr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem1keyr4 {{ oem1key: {=u32:?} }}", self.oem1key())
        }
    }
    #[doc = "FLASH OEM2 key register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr1(pub u32);
    impl Oem2keyr1 {
        #[doc = "OEM2\\[31:0\\]
bytes key."]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2\\[31:0\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem2key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem2keyr1 {
        #[inline(always)]
        fn default() -> Oem2keyr1 {
            Oem2keyr1(0)
        }
    }
    impl core::fmt::Debug for Oem2keyr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem2keyr1").field("oem2key", &self.oem2key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem2keyr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem2keyr1 {{ oem2key: {=u32:?} }}", self.oem2key())
        }
    }
    #[doc = "FLASH OEM2 key register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr2(pub u32);
    impl Oem2keyr2 {
        #[doc = "OEM2\\[63:32\\]
bytes key."]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2\\[63:32\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem2key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem2keyr2 {
        #[inline(always)]
        fn default() -> Oem2keyr2 {
            Oem2keyr2(0)
        }
    }
    impl core::fmt::Debug for Oem2keyr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem2keyr2").field("oem2key", &self.oem2key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem2keyr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem2keyr2 {{ oem2key: {=u32:?} }}", self.oem2key())
        }
    }
    #[doc = "FLASH OEM2 key register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr3(pub u32);
    impl Oem2keyr3 {
        #[doc = "OEM2\\[95:64\\]
bytes key."]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2\\[95:64\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem2key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem2keyr3 {
        #[inline(always)]
        fn default() -> Oem2keyr3 {
            Oem2keyr3(0)
        }
    }
    impl core::fmt::Debug for Oem2keyr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem2keyr3").field("oem2key", &self.oem2key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem2keyr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem2keyr3 {{ oem2key: {=u32:?} }}", self.oem2key())
        }
    }
    #[doc = "FLASH OEM2 key register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr4(pub u32);
    impl Oem2keyr4 {
        #[doc = "OEM2\\[127:96\\]
bytes key."]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2\\[127:96\\]
bytes key."]
        #[inline(always)]
        pub fn set_oem2key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem2keyr4 {
        #[inline(always)]
        fn default() -> Oem2keyr4 {
            Oem2keyr4(0)
        }
    }
    impl core::fmt::Debug for Oem2keyr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oem2keyr4").field("oem2key", &self.oem2key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oem2keyr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oem2keyr4 {{ oem2key: {=u32:?} }}", self.oem2key())
        }
    }
    #[doc = "FLASH OEM key status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oemkeysr(pub u32);
    impl Oemkeysr {
        #[doc = "8-bit OEMKEY1 CRC."]
        #[inline(always)]
        pub const fn oem1keycrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit OEMKEY1 CRC."]
        #[inline(always)]
        pub fn set_oem1keycrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "8-bit OEM2KEY CRC."]
        #[inline(always)]
        pub const fn oem2keycrc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit OEM2KEY CRC."]
        #[inline(always)]
        pub fn set_oem2keycrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Oemkeysr {
        #[inline(always)]
        fn default() -> Oemkeysr {
            Oemkeysr(0)
        }
    }
    impl core::fmt::Debug for Oemkeysr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oemkeysr")
                .field("oem1keycrc", &self.oem1keycrc())
                .field("oem2keycrc", &self.oem2keycrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oemkeysr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Oemkeysr {{ oem1keycrc: {=u8:?}, oem2keycrc: {=u8:?} }}",
                self.oem1keycrc(),
                self.oem2keycrc()
            )
        }
    }
    #[doc = "FLASH operation status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address."]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address."]
        #[inline(always)]
        pub fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "Interrupted operation bank."]
        #[inline(always)]
        pub const fn bk_op(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupted operation bank."]
        #[inline(always)]
        pub fn set_bk_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Operation in system flash memory interrupted."]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system flash memory interrupted."]
        #[inline(always)]
        pub fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Flash memory operation code."]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "Flash memory operation code."]
        #[inline(always)]
        pub fn set_code_op(&mut self, val: super::vals::CodeOp) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Opsr {
        #[inline(always)]
        fn default() -> Opsr {
            Opsr(0)
        }
    }
    impl core::fmt::Debug for Opsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opsr")
                .field("addr_op", &self.addr_op())
                .field("bk_op", &self.bk_op())
                .field("sysf_op", &self.sysf_op())
                .field("code_op", &self.code_op())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opsr {{ addr_op: {=u32:?}, bk_op: {=bool:?}, sysf_op: {=bool:?}, code_op: {:?} }}",
                self.addr_op(),
                self.bk_op(),
                self.sysf_op(),
                self.code_op()
            )
        }
    }
    #[doc = "FLASH option key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optkeyr(pub u32);
    impl Optkeyr {
        #[doc = "Option-byte key."]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Option-byte key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Optkeyr {
        #[inline(always)]
        fn default() -> Optkeyr {
            Optkeyr(0)
        }
    }
    impl core::fmt::Debug for Optkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optkeyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Optkeyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "FLASH option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Readout protection level."]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Readout protection level."]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset level."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::BorLev {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::BorLev::from_bits(val as u8)
        }
        #[doc = "BOR reset level."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::BorLev) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Backup domain reset with power-on reset."]
        #[inline(always)]
        pub const fn bdrst_por(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain reset with power-on reset."]
        #[inline(always)]
        pub fn set_bdrst_por(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Reset generation in Stop mode."]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Stop mode."]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset generation in Standby mode."]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Standby mode."]
        #[inline(always)]
        pub fn set_nrst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset generation in Shutdown mode."]
        #[inline(always)]
        pub const fn nrst_shdw(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Shutdown mode."]
        #[inline(always)]
        pub fn set_nrst_shdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM1 erase upon system reset."]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 erase upon system reset."]
        #[inline(always)]
        pub fn set_sram1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Independent watchdog selection."]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog selection."]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode."]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode."]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode."]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Standby mode."]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection."]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog selection."]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Swap banks."]
        #[inline(always)]
        pub const fn swap_bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Swap banks."]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Dual-bank on 512-Kbyte flash memory devices."]
        #[inline(always)]
        pub const fn dualbank(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Dual-bank on 512-Kbyte flash memory devices."]
        #[inline(always)]
        pub fn set_dualbank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM2 parity check enable."]
        #[inline(always)]
        pub const fn sram2_pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity check enable."]
        #[inline(always)]
        pub fn set_sram2_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 erase when system reset."]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase when system reset."]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Software BOOT0."]
        #[inline(always)]
        pub const fn nswboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Software BOOT0."]
        #[inline(always)]
        pub fn set_nswboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "NBOOT0 option bit."]
        #[inline(always)]
        pub const fn nboot0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "NBOOT0 option bit."]
        #[inline(always)]
        pub fn set_nboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "High-speed I/O at low Vless thansub>DD less than/sub>voltage configuration bit."]
        #[inline(always)]
        pub const fn io_vdd_hslv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed I/O at low Vless thansub>DD less than/sub>voltage configuration bit."]
        #[inline(always)]
        pub fn set_io_vdd_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "High-speed I/O at low Vless thansub>DDIO2less than/sub> voltage configuration bit."]
        #[inline(always)]
        pub const fn io_vddio2_hslv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed I/O at low Vless thansub>DDIO2less than/sub> voltage configuration bit."]
        #[inline(always)]
        pub fn set_io_vddio2_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Global TrustZone security enable."]
        #[inline(always)]
        pub const fn tzen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Global TrustZone security enable."]
        #[inline(always)]
        pub fn set_tzen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optr {
        #[inline(always)]
        fn default() -> Optr {
            Optr(0)
        }
    }
    impl core::fmt::Debug for Optr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optr")
                .field("rdp", &self.rdp())
                .field("bor_lev", &self.bor_lev())
                .field("bdrst_por", &self.bdrst_por())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("nrst_shdw", &self.nrst_shdw())
                .field("sram1_rst", &self.sram1_rst())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("swap_bank", &self.swap_bank())
                .field("dualbank", &self.dualbank())
                .field("sram2_pe", &self.sram2_pe())
                .field("sram2_rst", &self.sram2_rst())
                .field("nswboot0", &self.nswboot0())
                .field("nboot0", &self.nboot0())
                .field("io_vdd_hslv", &self.io_vdd_hslv())
                .field("io_vddio2_hslv", &self.io_vddio2_hslv())
                .field("tzen", &self.tzen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optr {{ rdp: {:?}, bor_lev: {:?}, bdrst_por: {=bool:?}, nrst_stop: {=bool:?}, nrst_stdby: {=bool:?}, nrst_shdw: {=bool:?}, sram1_rst: {=bool:?}, iwdg_sw: {=bool:?}, iwdg_stop: {=bool:?}, iwdg_stdby: {=bool:?}, wwdg_sw: {=bool:?}, swap_bank: {=bool:?}, dualbank: {=bool:?}, sram2_pe: {=bool:?}, sram2_rst: {=bool:?}, nswboot0: {=bool:?}, nboot0: {=bool:?}, io_vdd_hslv: {=bool:?}, io_vddio2_hslv: {=bool:?}, tzen: {=bool:?} }}" , self . rdp () , self . bor_lev () , self . bdrst_por () , self . nrst_stop () , self . nrst_stdby () , self . nrst_shdw () , self . sram1_rst () , self . iwdg_sw () , self . iwdg_stop () , self . iwdg_stdby () , self . wwdg_sw () , self . swap_bank () , self . dualbank () , self . sram2_pe () , self . sram2_rst () , self . nswboot0 () , self . nboot0 () , self . io_vdd_hslv () , self . io_vddio2_hslv () , self . tzen ())
        }
    }
    #[doc = "FLASH bank 1 power-down key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdkey1r(pub u32);
    impl Pdkey1r {
        #[doc = "Bank 1 power-down key."]
        #[inline(always)]
        pub const fn key1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank 1 power-down key."]
        #[inline(always)]
        pub fn set_key1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pdkey1r {
        #[inline(always)]
        fn default() -> Pdkey1r {
            Pdkey1r(0)
        }
    }
    impl core::fmt::Debug for Pdkey1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdkey1r").field("key1", &self.key1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdkey1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdkey1r {{ key1: {=u32:?} }}", self.key1())
        }
    }
    #[doc = "FLASH bank 2 power-down key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdkey2r(pub u32);
    impl Pdkey2r {
        #[doc = "Bank 2 power-down key."]
        #[inline(always)]
        pub const fn key2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank 2 power-down key."]
        #[inline(always)]
        pub fn set_key2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pdkey2r {
        #[inline(always)]
        fn default() -> Pdkey2r {
            Pdkey2r(0)
        }
    }
    impl core::fmt::Debug for Pdkey2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdkey2r").field("key2", &self.key2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdkey2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdkey2r {{ key2: {=u32:?} }}", self.key2())
        }
    }
    #[doc = "FLASH privilege block-based bank 1 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb1r1(pub u32);
    impl Privbb1r1 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb1r1 {
        #[inline(always)]
        fn default() -> Privbb1r1 {
            Privbb1r1(0)
        }
    }
    impl core::fmt::Debug for Privbb1r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb1r1")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb1r1 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block-based bank 1 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb1r2(pub u32);
    impl Privbb1r2 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb1r2 {
        #[inline(always)]
        fn default() -> Privbb1r2 {
            Privbb1r2(0)
        }
    }
    impl core::fmt::Debug for Privbb1r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb1r2")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb1r2 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block-based bank 1 register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb1r3(pub u32);
    impl Privbb1r3 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb1r3 {
        #[inline(always)]
        fn default() -> Privbb1r3 {
            Privbb1r3(0)
        }
    }
    impl core::fmt::Debug for Privbb1r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb1r3")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb1r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb1r3 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block-based bank 1 register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb1r4(pub u32);
    impl Privbb1r4 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb1r4 {
        #[inline(always)]
        fn default() -> Privbb1r4 {
            Privbb1r4(0)
        }
    }
    impl core::fmt::Debug for Privbb1r4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb1r4")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb1r4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb1r4 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb2r1(pub u32);
    impl Privbb2r1 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb2r1 {
        #[inline(always)]
        fn default() -> Privbb2r1 {
            Privbb2r1(0)
        }
    }
    impl core::fmt::Debug for Privbb2r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb2r1")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb2r1 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb2r2(pub u32);
    impl Privbb2r2 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb2r2 {
        #[inline(always)]
        fn default() -> Privbb2r2 {
            Privbb2r2(0)
        }
    }
    impl core::fmt::Debug for Privbb2r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb2r2")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb2r2 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb2r3(pub u32);
    impl Privbb2r3 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb2r3 {
        #[inline(always)]
        fn default() -> Privbb2r3 {
            Privbb2r3(0)
        }
    }
    impl core::fmt::Debug for Privbb2r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb2r3")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb2r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb2r3 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb2r4(pub u32);
    impl Privbb2r4 {
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page privileged/unprivileged attribution."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privbb2r4 {
        #[inline(always)]
        fn default() -> Privbb2r4 {
            Privbb2r4(0)
        }
    }
    impl core::fmt::Debug for Privbb2r4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb2r4")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb2r4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privbb2r4 {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "FLASH privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Privileged protection for secure registers."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged protection for secure registers."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged protection for nonsecure registers."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged protection for nonsecure registers."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, priv_: {=bool:?} }}",
                self.spriv(),
                self.priv_()
            )
        }
    }
    #[doc = "FLASH secure boot address 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sboot0r(pub u32);
    impl Sboot0r {
        #[doc = "Boot lock."]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Boot lock."]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure boot base address 0."]
        #[inline(always)]
        pub const fn add(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Secure boot base address 0."]
        #[inline(always)]
        pub fn set_add(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Sboot0r {
        #[inline(always)]
        fn default() -> Sboot0r {
            Sboot0r(0)
        }
    }
    impl core::fmt::Debug for Sboot0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sboot0r")
                .field("boot_lock", &self.boot_lock())
                .field("add", &self.add())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sboot0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sboot0r {{ boot_lock: {=bool:?}, add: {=u32:?} }}",
                self.boot_lock(),
                self.add()
            )
        }
    }
    #[doc = "FLASH secure control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Secure programming."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure page erase."]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure page erase."]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure bank 1 mass erase."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank 1 mass erase."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure page number selection."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Secure page number selection."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "Secure bank selection for page erase."]
        #[inline(always)]
        pub const fn bker(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank selection for page erase."]
        #[inline(always)]
        pub fn set_bker(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Secure burst write programming mode."]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure burst write programming mode."]
        #[inline(always)]
        pub fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Secure bank 2 mass erase."]
        #[inline(always)]
        pub const fn mer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank 2 mass erase."]
        #[inline(always)]
        pub fn set_mer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Secure start."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure start."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure end of operation interrupt enable."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Secure end of operation interrupt enable."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Secure error interrupt enable."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Secure error interrupt enable."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Flash memory security state invert."]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory security state invert."]
        #[inline(always)]
        pub fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Secure lock."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Secure lock."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("mer1", &self.mer1())
                .field("pnb", &self.pnb())
                .field("bker", &self.bker())
                .field("bwr", &self.bwr())
                .field("mer2", &self.mer2())
                .field("strt", &self.strt())
                .field("eopie", &self.eopie())
                .field("errie", &self.errie())
                .field("inv", &self.inv())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ pg: {=bool:?}, per: {=bool:?}, mer1: {=bool:?}, pnb: {=u8:?}, bker: {=bool:?}, bwr: {=bool:?}, mer2: {=bool:?}, strt: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, inv: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . per () , self . mer1 () , self . pnb () , self . bker () , self . bwr () , self . mer2 () , self . strt () , self . eopie () , self . errie () , self . inv () , self . lock ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r1(pub u32);
    impl Secbb1r1 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb1r1 {
        #[inline(always)]
        fn default() -> Secbb1r1 {
            Secbb1r1(0)
        }
    }
    impl core::fmt::Debug for Secbb1r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r1")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb1r1 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r2(pub u32);
    impl Secbb1r2 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb1r2 {
        #[inline(always)]
        fn default() -> Secbb1r2 {
            Secbb1r2(0)
        }
    }
    impl core::fmt::Debug for Secbb1r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r2")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb1r2 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r3(pub u32);
    impl Secbb1r3 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb1r3 {
        #[inline(always)]
        fn default() -> Secbb1r3 {
            Secbb1r3(0)
        }
    }
    impl core::fmt::Debug for Secbb1r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r3")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb1r3 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r4(pub u32);
    impl Secbb1r4 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb1r4 {
        #[inline(always)]
        fn default() -> Secbb1r4 {
            Secbb1r4(0)
        }
    }
    impl core::fmt::Debug for Secbb1r4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r4")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb1r4 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r1(pub u32);
    impl Secbb2r1 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb2r1 {
        #[inline(always)]
        fn default() -> Secbb2r1 {
            Secbb2r1(0)
        }
    }
    impl core::fmt::Debug for Secbb2r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r1")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb2r1 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r2(pub u32);
    impl Secbb2r2 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb2r2 {
        #[inline(always)]
        fn default() -> Secbb2r2 {
            Secbb2r2(0)
        }
    }
    impl core::fmt::Debug for Secbb2r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r2")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb2r2 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r3(pub u32);
    impl Secbb2r3 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb2r3 {
        #[inline(always)]
        fn default() -> Secbb2r3 {
            Secbb2r3(0)
        }
    }
    impl core::fmt::Debug for Secbb2r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r3")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb2r3 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r4(pub u32);
    impl Secbb2r4 {
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub const fn sec31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Page secure/nonsecure attribution."]
        #[inline(always)]
        pub fn set_sec31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secbb2r4 {
        #[inline(always)]
        fn default() -> Secbb2r4 {
            Secbb2r4(0)
        }
    }
    impl core::fmt::Debug for Secbb2r4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r4")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("sec8", &self.sec8())
                .field("sec9", &self.sec9())
                .field("sec10", &self.sec10())
                .field("sec11", &self.sec11())
                .field("sec12", &self.sec12())
                .field("sec13", &self.sec13())
                .field("sec14", &self.sec14())
                .field("sec15", &self.sec15())
                .field("sec16", &self.sec16())
                .field("sec17", &self.sec17())
                .field("sec18", &self.sec18())
                .field("sec19", &self.sec19())
                .field("sec20", &self.sec20())
                .field("sec21", &self.sec21())
                .field("sec22", &self.sec22())
                .field("sec23", &self.sec23())
                .field("sec24", &self.sec24())
                .field("sec25", &self.sec25())
                .field("sec26", &self.sec26())
                .field("sec27", &self.sec27())
                .field("sec28", &self.sec28())
                .field("sec29", &self.sec29())
                .field("sec30", &self.sec30())
                .field("sec31", &self.sec31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secbb2r4 {{ sec0: {=bool:?}, sec1: {=bool:?}, sec2: {=bool:?}, sec3: {=bool:?}, sec4: {=bool:?}, sec5: {=bool:?}, sec6: {=bool:?}, sec7: {=bool:?}, sec8: {=bool:?}, sec9: {=bool:?}, sec10: {=bool:?}, sec11: {=bool:?}, sec12: {=bool:?}, sec13: {=bool:?}, sec14: {=bool:?}, sec15: {=bool:?}, sec16: {=bool:?}, sec17: {=bool:?}, sec18: {=bool:?}, sec19: {=bool:?}, sec20: {=bool:?}, sec21: {=bool:?}, sec22: {=bool:?}, sec23: {=bool:?}, sec24: {=bool:?}, sec25: {=bool:?}, sec26: {=bool:?}, sec27: {=bool:?}, sec28: {=bool:?}, sec29: {=bool:?}, sec30: {=bool:?}, sec31: {=bool:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . sec8 () , self . sec9 () , self . sec10 () , self . sec11 () , self . sec12 () , self . sec13 () , self . sec14 () , self . sec15 () , self . sec16 () , self . sec17 () , self . sec18 () , self . sec19 () , self . sec20 () , self . sec21 () , self . sec22 () , self . sec23 () , self . sec24 () , self . sec25 () , self . sec26 () , self . sec27 () , self . sec28 () , self . sec29 () , self . sec30 () , self . sec31 ())
        }
    }
    #[doc = "FLASH secure HDP control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sechdpcr(pub u32);
    impl Sechdpcr {
        #[doc = "HDP1 area access disable."]
        #[inline(always)]
        pub const fn hdp1_accdis(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "HDP1 area access disable."]
        #[inline(always)]
        pub fn set_hdp1_accdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "HDP2 area access disable."]
        #[inline(always)]
        pub const fn hdp2_accdis(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "HDP2 area access disable."]
        #[inline(always)]
        pub fn set_hdp2_accdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "HDP1 extension area access disable."]
        #[inline(always)]
        pub const fn hdp1ex_t_accdis(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "HDP1 extension area access disable."]
        #[inline(always)]
        pub fn set_hdp1ex_t_accdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "HDP2 extension area access disable."]
        #[inline(always)]
        pub const fn hdp2ex_t_accdis(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "HDP2 extension area access disable."]
        #[inline(always)]
        pub fn set_hdp2ex_t_accdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sechdpcr {
        #[inline(always)]
        fn default() -> Sechdpcr {
            Sechdpcr(0)
        }
    }
    impl core::fmt::Debug for Sechdpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sechdpcr")
                .field("hdp1_accdis", &self.hdp1_accdis())
                .field("hdp2_accdis", &self.hdp2_accdis())
                .field("hdp1ex_t_accdis", &self.hdp1ex_t_accdis())
                .field("hdp2ex_t_accdis", &self.hdp2ex_t_accdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sechdpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sechdpcr {{ hdp1_accdis: {=u8:?}, hdp2_accdis: {=u8:?}, hdp1ex_t_accdis: {=u8:?}, hdp2ex_t_accdis: {=u8:?} }}" , self . hdp1_accdis () , self . hdp2_accdis () , self . hdp1ex_t_accdis () , self . hdp2ex_t_accdis ())
        }
    }
    #[doc = "FLASH HDP extension register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SechdpexTr(pub u32);
    impl SechdpexTr {
        #[doc = "HDP area extension in 4-Kbyte pages in bank 1."]
        #[inline(always)]
        pub const fn hdp1_ex_t(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "HDP area extension in 4-Kbyte pages in bank 1."]
        #[inline(always)]
        pub fn set_hdp1_ex_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "HDP area extension in 4-Kbyte pages in bank 2."]
        #[inline(always)]
        pub const fn hdp2_ex_t(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "HDP area extension in 4-Kbyte pages in bank 2."]
        #[inline(always)]
        pub fn set_hdp2_ex_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for SechdpexTr {
        #[inline(always)]
        fn default() -> SechdpexTr {
            SechdpexTr(0)
        }
    }
    impl core::fmt::Debug for SechdpexTr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SechdpexTr")
                .field("hdp1_ex_t", &self.hdp1_ex_t())
                .field("hdp2_ex_t", &self.hdp2_ex_t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SechdpexTr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SechdpexTr {{ hdp1_ex_t: {=u8:?}, hdp2_ex_t: {=u8:?} }}",
                self.hdp1_ex_t(),
                self.hdp2_ex_t()
            )
        }
    }
    #[doc = "FLASH secure watermark1 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r1(pub u32);
    impl Secwm1r1 {
        #[doc = "Start page of first secure area."]
        #[inline(always)]
        pub const fn secwm1_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of first secure area."]
        #[inline(always)]
        pub fn set_secwm1_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of first secure area."]
        #[inline(always)]
        pub const fn secwm1_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of first secure area."]
        #[inline(always)]
        pub fn set_secwm1_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm1r1 {
        #[inline(always)]
        fn default() -> Secwm1r1 {
            Secwm1r1(0)
        }
    }
    impl core::fmt::Debug for Secwm1r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm1r1")
                .field("secwm1_strt", &self.secwm1_strt())
                .field("secwm1_end", &self.secwm1_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm1r1 {{ secwm1_strt: {=u8:?}, secwm1_end: {=u8:?} }}",
                self.secwm1_strt(),
                self.secwm1_end()
            )
        }
    }
    #[doc = "FLASH secure watermark1 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r2(pub u32);
    impl Secwm1r2 {
        #[doc = "End page of first hide protection area."]
        #[inline(always)]
        pub const fn hdp1_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of first hide protection area."]
        #[inline(always)]
        pub fn set_hdp1_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Hide protection first area enable."]
        #[inline(always)]
        pub const fn hdp1en(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Hide protection first area enable."]
        #[inline(always)]
        pub fn set_hdp1en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Secwm1r2 {
        #[inline(always)]
        fn default() -> Secwm1r2 {
            Secwm1r2(0)
        }
    }
    impl core::fmt::Debug for Secwm1r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm1r2")
                .field("hdp1_end", &self.hdp1_end())
                .field("hdp1en", &self.hdp1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm1r2 {{ hdp1_end: {=u8:?}, hdp1en: {=u8:?} }}",
                self.hdp1_end(),
                self.hdp1en()
            )
        }
    }
    #[doc = "FLASH secure watermark2 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r1(pub u32);
    impl Secwm2r1 {
        #[doc = "Start page of second secure area."]
        #[inline(always)]
        pub const fn secwm2_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of second secure area."]
        #[inline(always)]
        pub fn set_secwm2_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of second secure area."]
        #[inline(always)]
        pub const fn secwm2_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of second secure area."]
        #[inline(always)]
        pub fn set_secwm2_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm2r1 {
        #[inline(always)]
        fn default() -> Secwm2r1 {
            Secwm2r1(0)
        }
    }
    impl core::fmt::Debug for Secwm2r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm2r1")
                .field("secwm2_strt", &self.secwm2_strt())
                .field("secwm2_end", &self.secwm2_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm2r1 {{ secwm2_strt: {=u8:?}, secwm2_end: {=u8:?} }}",
                self.secwm2_strt(),
                self.secwm2_end()
            )
        }
    }
    #[doc = "FLASH secure watermark2 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r2(pub u32);
    impl Secwm2r2 {
        #[doc = "End page of hide protection second area."]
        #[inline(always)]
        pub const fn hdp2_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of hide protection second area."]
        #[inline(always)]
        pub fn set_hdp2_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Hide protection second area enable."]
        #[inline(always)]
        pub const fn hdp2en(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Hide protection second area enable."]
        #[inline(always)]
        pub fn set_hdp2en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Secwm2r2 {
        #[inline(always)]
        fn default() -> Secwm2r2 {
            Secwm2r2(0)
        }
    }
    impl core::fmt::Debug for Secwm2r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm2r2")
                .field("hdp2_end", &self.hdp2_end())
                .field("hdp2en", &self.hdp2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm2r2 {{ hdp2_end: {=u8:?}, hdp2en: {=u8:?} }}",
                self.hdp2_end(),
                self.hdp2en()
            )
        }
    }
    #[doc = "FLASH secure key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Skeyr(pub u32);
    impl Skeyr {
        #[doc = "Flash memory secure key."]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash memory secure key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Skeyr {
        #[inline(always)]
        fn default() -> Skeyr {
            Skeyr(0)
        }
    }
    impl core::fmt::Debug for Skeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Skeyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Skeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Skeyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "FLASH nonsecure status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Nonsecure end of operation."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure end of operation."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Nonsecure operation error."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure operation error."]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Nonsecure programming error."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure programming error."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Nonsecure write protection error."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure write protection error."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Nonsecure programming alignment error."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure programming alignment error."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Nonsecure size error."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure size error."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Nonsecure programming sequence error."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure programming sequence error."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option write error."]
        #[inline(always)]
        pub const fn optwerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Option write error."]
        #[inline(always)]
        pub fn set_optwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Nonsecure busy."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure busy."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Nonsecure wait data to write."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure wait data to write."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OEM1 lock."]
        #[inline(always)]
        pub const fn oem1lock(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OEM1 lock."]
        #[inline(always)]
        pub fn set_oem1lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OEM2 lock."]
        #[inline(always)]
        pub const fn oem2lock(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OEM2 lock."]
        #[inline(always)]
        pub fn set_oem2lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bank 1 in power-down mode."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 in power-down mode."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bank 2 in power-down mode."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 in power-down mode."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("eop", &self.eop())
                .field("operr", &self.operr())
                .field("progerr", &self.progerr())
                .field("wrperr", &self.wrperr())
                .field("pgaerr", &self.pgaerr())
                .field("sizerr", &self.sizerr())
                .field("pgserr", &self.pgserr())
                .field("optwerr", &self.optwerr())
                .field("bsy", &self.bsy())
                .field("wdw", &self.wdw())
                .field("oem1lock", &self.oem1lock())
                .field("oem2lock", &self.oem2lock())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ eop: {=bool:?}, operr: {=bool:?}, progerr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, pgserr: {=bool:?}, optwerr: {=bool:?}, bsy: {=bool:?}, wdw: {=bool:?}, oem1lock: {=bool:?}, oem2lock: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?} }}" , self . eop () , self . operr () , self . progerr () , self . wrperr () , self . pgaerr () , self . sizerr () , self . pgserr () , self . optwerr () , self . bsy () , self . wdw () , self . oem1lock () , self . oem2lock () , self . pd1 () , self . pd2 ())
        }
    }
    #[doc = "FLASH secure status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssr(pub u32);
    impl Ssr {
        #[doc = "Secure end of operation."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure end of operation."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure operation error."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure operation error."]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure programming error."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming error."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure write protection error."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Secure write protection error."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Secure programming alignment error."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming alignment error."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Secure size error."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure size error."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure programming sequence error."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming sequence error."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure busy."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure busy."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure wait data to write."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure wait data to write."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ssr {
        #[inline(always)]
        fn default() -> Ssr {
            Ssr(0)
        }
    }
    impl core::fmt::Debug for Ssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssr")
                .field("eop", &self.eop())
                .field("operr", &self.operr())
                .field("progerr", &self.progerr())
                .field("wrperr", &self.wrperr())
                .field("pgaerr", &self.pgaerr())
                .field("sizerr", &self.sizerr())
                .field("pgserr", &self.pgserr())
                .field("bsy", &self.bsy())
                .field("wdw", &self.wdw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ssr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ssr {{ eop: {=bool:?}, operr: {=bool:?}, progerr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, pgserr: {=bool:?}, bsy: {=bool:?}, wdw: {=bool:?} }}" , self . eop () , self . operr () , self . progerr () , self . wrperr () , self . pgaerr () , self . sizerr () , self . pgserr () , self . bsy () , self . wdw ())
        }
    }
    #[doc = "FLASH WRP1 area A address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "Bank 1 WPR first area A start page."]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WPR first area A start page."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WPR first area A end page."]
        #[inline(always)]
        pub const fn end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WPR first area A end page."]
        #[inline(always)]
        pub fn set_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 1 WPR first area A unlock."]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 WPR first area A unlock."]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp1ar {
        #[inline(always)]
        fn default() -> Wrp1ar {
            Wrp1ar(0)
        }
    }
    impl core::fmt::Debug for Wrp1ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1ar")
                .field("strt", &self.strt())
                .field("end", &self.end())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1ar {{ strt: {=u8:?}, end: {=u8:?}, unlock: {=bool:?} }}",
                self.strt(),
                self.end(),
                self.unlock()
            )
        }
    }
    #[doc = "FLASH WRP1 area B address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "Bank 1 WRP second area B start page."]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B start page."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WRP second area B end page."]
        #[inline(always)]
        pub const fn end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B end page."]
        #[inline(always)]
        pub fn set_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 1 WPR second area B unlock."]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 WPR second area B unlock."]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp1br {
        #[inline(always)]
        fn default() -> Wrp1br {
            Wrp1br(0)
        }
    }
    impl core::fmt::Debug for Wrp1br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1br")
                .field("strt", &self.strt())
                .field("end", &self.end())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1br {{ strt: {=u8:?}, end: {=u8:?}, unlock: {=bool:?} }}",
                self.strt(),
                self.end(),
                self.unlock()
            )
        }
    }
    #[doc = "FLASH WPR2 area A address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2ar(pub u32);
    impl Wrp2ar {
        #[doc = "Bank 2 WPR first area A start page."]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR first area A start page."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 2 WPR first area A end page."]
        #[inline(always)]
        pub const fn end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR first area A end page."]
        #[inline(always)]
        pub fn set_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 2 WPR first area A unlock."]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 WPR first area A unlock."]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp2ar {
        #[inline(always)]
        fn default() -> Wrp2ar {
            Wrp2ar(0)
        }
    }
    impl core::fmt::Debug for Wrp2ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2ar")
                .field("strt", &self.strt())
                .field("end", &self.end())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp2ar {{ strt: {=u8:?}, end: {=u8:?}, unlock: {=bool:?} }}",
                self.strt(),
                self.end(),
                self.unlock()
            )
        }
    }
    #[doc = "FLASH WPR2 area B address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2br(pub u32);
    impl Wrp2br {
        #[doc = "Bank 2 WPR second area B start page."]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR second area B start page."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 2 WPR second area B end page."]
        #[inline(always)]
        pub const fn end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR second area B end page."]
        #[inline(always)]
        pub fn set_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 2 WPR second area B unlock."]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 WPR second area B unlock."]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp2br {
        #[inline(always)]
        fn default() -> Wrp2br {
            Wrp2br(0)
        }
    }
    impl core::fmt::Debug for Wrp2br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2br")
                .field("strt", &self.strt())
                .field("end", &self.end())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp2br {{ strt: {=u8:?}, end: {=u8:?}, unlock: {=bool:?} }}",
                self.strt(),
                self.end(),
                self.unlock()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BorLev {
        #[doc = "BOR level 0 (reset level threshold around 1."]
        LEVEL0 = 0x0,
        #[doc = "BOR level 1 (reset level threshold around 2."]
        LEVEL1 = 0x01,
        #[doc = "BOR level 2 (reset level threshold around 2."]
        LEVEL2 = 0x02,
        #[doc = "BOR level 3 (reset level threshold around 2."]
        LEVEL3 = 0x03,
        #[doc = "BOR level 4 (reset level threshold around 2."]
        LEVEL4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl BorLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BorLev {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BorLev {
        #[inline(always)]
        fn from(val: u8) -> BorLev {
            BorLev::from_bits(val)
        }
    }
    impl From<BorLev> for u8 {
        #[inline(always)]
        fn from(val: BorLev) -> u8 {
            BorLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CodeOp {
        #[doc = "No flash operation interrupted by previous reset."]
        NO_FLASH_INT = 0x0,
        #[doc = "Single write operation interrupted."]
        SINGLE_WR_INT = 0x01,
        #[doc = "Burst write operation interrupted."]
        BURST_WR_INT = 0x02,
        #[doc = "Page erase operation interrupted."]
        PG_ERASE_INT = 0x03,
        #[doc = "Bank erase operation interrupted."]
        BANK_ERASE_INT = 0x04,
        #[doc = "Mass erase operation interrupted."]
        MASS_ERASE_INT = 0x05,
        #[doc = "Option change operation interrupted."]
        OPT_CHANGE_INT = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CodeOp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CodeOp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CodeOp {
        #[inline(always)]
        fn from(val: u8) -> CodeOp {
            CodeOp::from_bits(val)
        }
    }
    impl From<CodeOp> for u8 {
        #[inline(always)]
        fn from(val: CodeOp) -> u8 {
            CodeOp::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdp(u8);
    impl Rdp {
        #[doc = "Level 0."]
        pub const LEVEL0: Self = Self(0x55);
        #[doc = "Level 0 (readout protection not active)."]
        pub const LEVEL0ROPROT_ACTIVE: Self = Self(0xaa);
        #[doc = "Level 2 (chip readout protection active)."]
        pub const LEVEL2: Self = Self(0xcc);
    }
    impl Rdp {
        pub const fn from_bits(val: u8) -> Rdp {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Rdp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x55 => f.write_str("LEVEL0"),
                0xaa => f.write_str("LEVEL0ROPROT_ACTIVE"),
                0xcc => f.write_str("LEVEL2"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x55 => defmt::write!(f, "LEVEL0"),
                0xaa => defmt::write!(f, "LEVEL0ROPROT_ACTIVE"),
                0xcc => defmt::write!(f, "LEVEL2"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Rdp {
        #[inline(always)]
        fn from(val: u8) -> Rdp {
            Rdp::from_bits(val)
        }
    }
    impl From<Rdp> for u8 {
        #[inline(always)]
        fn from(val: Rdp) -> u8 {
            Rdp::to_bits(val)
        }
    }
}
