#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flash"]
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
    #[doc = "FLASH access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FLASH non-secure key register"]
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH secure key register"]
    #[inline(always)]
    pub const fn seckeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH bank 1 power-down key register"]
    #[inline(always)]
    pub const fn pdkey1r(self) -> crate::common::Reg<regs::Pdkey1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH bank 2 power-down key register"]
    #[inline(always)]
    pub const fn pdkey2r(self) -> crate::common::Reg<regs::Pdkey2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FLASH non-secure status register"]
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH secure status register"]
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH non-secure control register"]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH secure control register"]
    #[inline(always)]
    pub const fn seccr(self) -> crate::common::Reg<regs::Seccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FLASH ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH operation status register"]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "FLASH option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FLASH non-secure boot address 0 register"]
    #[inline(always)]
    pub const fn nsbootadd0r(self) -> crate::common::Reg<regs::Nsbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FLASH non-secure boot address 1 register"]
    #[inline(always)]
    pub const fn nsbootadd1r(self) -> crate::common::Reg<regs::Nsbootadd1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FLASH secure boot address 0 register"]
    #[inline(always)]
    pub const fn secbootadd0r(self) -> crate::common::Reg<regs::Secbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "FLASH secure watermark1 register 1"]
    #[inline(always)]
    pub const fn secwm1r1(self) -> crate::common::Reg<regs::Secwm1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FLASH secure watermark1 register 2"]
    #[inline(always)]
    pub const fn secwm1r2(self) -> crate::common::Reg<regs::Secwm1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FLASH WRP1 area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "FLASH WRP1 area B address register"]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "FLASH secure watermark2 register 1"]
    #[inline(always)]
    pub const fn secwm2r1(self) -> crate::common::Reg<regs::Secwm2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "FLASH secure watermark2 register 2"]
    #[inline(always)]
    pub const fn secwm2r2(self) -> crate::common::Reg<regs::Secwm2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "FLASH WPR2 area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(self) -> crate::common::Reg<regs::Wrp2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "FLASH WPR2 area B address register"]
    #[inline(always)]
    pub const fn wrp2br(self) -> crate::common::Reg<regs::Wrp2br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "FLASH OEM1 key register 1"]
    #[inline(always)]
    pub const fn oem1keyr1(self) -> crate::common::Reg<regs::Oem1keyr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "FLASH OEM1 key register 2"]
    #[inline(always)]
    pub const fn oem1keyr2(self) -> crate::common::Reg<regs::Oem1keyr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 1"]
    #[inline(always)]
    pub const fn oem2keyr1(self) -> crate::common::Reg<regs::Oem2keyr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 2"]
    #[inline(always)]
    pub const fn oem2keyr2(self) -> crate::common::Reg<regs::Oem2keyr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 1"]
    #[inline(always)]
    pub const fn sec1bbr1(self) -> crate::common::Reg<regs::Sec1bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 2"]
    #[inline(always)]
    pub const fn sec1bbr2(self) -> crate::common::Reg<regs::Sec1bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 3"]
    #[inline(always)]
    pub const fn sec1bbr3(self) -> crate::common::Reg<regs::Sec1bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 4"]
    #[inline(always)]
    pub const fn sec1bbr4(self) -> crate::common::Reg<regs::Sec1bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 1"]
    #[inline(always)]
    pub const fn sec2bbr1(self) -> crate::common::Reg<regs::Sec2bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 2"]
    #[inline(always)]
    pub const fn sec2bbr2(self) -> crate::common::Reg<regs::Sec2bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 3"]
    #[inline(always)]
    pub const fn sec2bbr3(self) -> crate::common::Reg<regs::Sec2bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 4"]
    #[inline(always)]
    pub const fn sec2bbr4(self) -> crate::common::Reg<regs::Sec2bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FLASH secure HDP control register"]
    #[inline(always)]
    pub const fn sechdpcr(self) -> crate::common::Reg<regs::Sechdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 1"]
    #[inline(always)]
    pub const fn priv1bbr1(self) -> crate::common::Reg<regs::Priv1bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 2"]
    #[inline(always)]
    pub const fn priv1bbr2(self) -> crate::common::Reg<regs::Priv1bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 3"]
    #[inline(always)]
    pub const fn priv1bbr3(self) -> crate::common::Reg<regs::Priv1bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 4"]
    #[inline(always)]
    pub const fn priv1bbr4(self) -> crate::common::Reg<regs::Priv1bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 1"]
    #[inline(always)]
    pub const fn priv2bbr1(self) -> crate::common::Reg<regs::Priv2bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 2"]
    #[inline(always)]
    pub const fn priv2bbr2(self) -> crate::common::Reg<regs::Priv2bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 3"]
    #[inline(always)]
    pub const fn priv2bbr3(self) -> crate::common::Reg<regs::Priv2bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 4"]
    #[inline(always)]
    pub const fn priv2bbr4(self) -> crate::common::Reg<regs::Priv2bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power read mode This bit puts the Flash memory in low-power read mode."]
        #[inline(always)]
        pub const fn lpm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power read mode This bit puts the Flash memory in low-power read mode."]
        #[inline(always)]
        pub fn set_lpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
        #[inline(always)]
        pub const fn pdreq1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
        #[inline(always)]
        pub fn set_pdreq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
        #[inline(always)]
        pub const fn pdreq2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
        #[inline(always)]
        pub fn set_pdreq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
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
    #[doc = "FLASH ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "ECC fail bank"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
        #[inline(always)]
        pub fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccr {
        #[inline(always)]
        fn default() -> Eccr {
            Eccr(0)
        }
    }
    impl core::fmt::Debug for Eccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr")
                .field("addr_ecc", &self.addr_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("eccie", &self.eccie())
                .field("eccc", &self.eccc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Eccr {{ addr_ecc: {=u32:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, eccie: {=bool:?}, eccc: {=bool:?}, eccd: {=bool:?} }}" , self . addr_ecc () , self . bk_ecc () , self . sysf_ecc () , self . eccie () , self . eccc () , self . eccd ())
        }
    }
    #[doc = "FLASH non-secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd0r(pub u32);
    impl Nsbootadd0r {
        #[doc = "Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub const fn nsbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub fn set_nsbootadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Nsbootadd0r {
        #[inline(always)]
        fn default() -> Nsbootadd0r {
            Nsbootadd0r(0)
        }
    }
    impl core::fmt::Debug for Nsbootadd0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsbootadd0r")
                .field("nsbootadd0", &self.nsbootadd0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsbootadd0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsbootadd0r {{ nsbootadd0: {=u32:?} }}", self.nsbootadd0())
        }
    }
    #[doc = "FLASH non-secure boot address 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd1r(pub u32);
    impl Nsbootadd1r {
        #[doc = "Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub const fn nsbootadd1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub fn set_nsbootadd1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Nsbootadd1r {
        #[inline(always)]
        fn default() -> Nsbootadd1r {
            Nsbootadd1r(0)
        }
    }
    impl core::fmt::Debug for Nsbootadd1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsbootadd1r")
                .field("nsbootadd1", &self.nsbootadd1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsbootadd1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsbootadd1r {{ nsbootadd1: {=u32:?} }}", self.nsbootadd1())
        }
    }
    #[doc = "FLASH non-secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "Non-secure programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Non-secure page number selection These bits select the page to erase. ..."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0xff;
            val as u8
        }
        #[doc = "Non-secure page number selection These bits select the page to erase. ..."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
        }
        #[doc = "Non-secure bank selection for page erase"]
        #[inline(always)]
        pub const fn bker(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure bank selection for page erase"]
        #[inline(always)]
        pub fn set_bker(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub const fn mer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub fn set_mer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    impl core::fmt::Debug for Nscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nscr")
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
    impl defmt::Format for Nscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nscr {{ pg: {=bool:?}, per: {=bool:?}, mer1: {=bool:?}, pnb: {=u8:?}, bker: {=bool:?}, bwr: {=bool:?}, mer2: {=bool:?}, strt: {=bool:?}, optstrt: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, obl_launch: {=bool:?}, optlock: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . per () , self . mer1 () , self . pnb () , self . bker () , self . bwr () , self . mer2 () , self . strt () , self . optstrt () , self . eopie () , self . errie () , self . obl_launch () , self . optlock () , self . lock ())
        }
    }
    #[doc = "FLASH non-secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nssr(pub u32);
    impl Nssr {
        #[doc = "Non-secure end of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure end of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure operation error"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure operation error"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn optwerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_optwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active."]
        #[inline(always)]
        pub const fn oem1lock(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active."]
        #[inline(always)]
        pub fn set_oem1lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active."]
        #[inline(always)]
        pub const fn oem2lock(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active."]
        #[inline(always)]
        pub fn set_oem2lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    impl core::fmt::Debug for Nssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nssr")
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
    impl defmt::Format for Nssr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nssr {{ eop: {=bool:?}, operr: {=bool:?}, progerr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, pgserr: {=bool:?}, optwerr: {=bool:?}, bsy: {=bool:?}, wdw: {=bool:?}, oem1lock: {=bool:?}, oem2lock: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?} }}" , self . eop () , self . operr () , self . progerr () , self . wrperr () , self . pgaerr () , self . sizerr () , self . pgserr () , self . optwerr () , self . bsy () , self . wdw () , self . oem1lock () , self . oem2lock () , self . pd1 () , self . pd2 ())
        }
    }
    #[doc = "FLASH OEM1 key register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr1(pub u32);
    impl Oem1keyr1 {
        #[doc = "OEM1 least significant bytes key"]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1 least significant bytes key"]
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
    #[doc = "FLASH OEM1 key register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr2(pub u32);
    impl Oem1keyr2 {
        #[doc = "OEM1 most significant bytes key"]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1 most significant bytes key"]
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
    #[doc = "FLASH OEM2 key register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr1(pub u32);
    impl Oem2keyr1 {
        #[doc = "OEM2 least significant bytes key"]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2 least significant bytes key"]
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
    #[doc = "FLASH OEM2 key register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr2(pub u32);
    impl Oem2keyr2 {
        #[doc = "OEM2 most significant bytes key"]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2 most significant bytes key"]
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
    #[doc = "FLASH operation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0."]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0."]
        #[inline(always)]
        pub fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred"]
        #[inline(always)]
        pub const fn bk_op(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred"]
        #[inline(always)]
        pub fn set_bk_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory."]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory."]
        #[inline(always)]
        pub fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:"]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:"]
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
    #[doc = "FLASH option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::BorLev {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::BorLev::from_bits(val as u8)
        }
        #[doc = "BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::BorLev) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Reset generation in Stop mode"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Stop mode"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset generation in Standby mode"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Standby mode"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset generation in Shutdown mode"]
        #[inline(always)]
        pub const fn n_rst_shdw(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Shutdown mode"]
        #[inline(always)]
        pub fn set_n_rst_shdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM1, SRAM3 and SRAM4 erase upon system reset"]
        #[inline(always)]
        pub const fn sram1345_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1, SRAM3 and SRAM4 erase upon system reset"]
        #[inline(always)]
        pub fn set_sram1345_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Swap banks"]
        #[inline(always)]
        pub const fn swap_bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Swap banks"]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
        #[inline(always)]
        pub const fn dualbank(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
        #[inline(always)]
        pub fn set_dualbank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Backup RAM ECC detection and correction enable"]
        #[inline(always)]
        pub const fn bkpsram_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM ECC detection and correction enable"]
        #[inline(always)]
        pub fn set_bkpsram_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SRAM3 ECC detection and correction enable"]
        #[inline(always)]
        pub const fn sram3_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 ECC detection and correction enable"]
        #[inline(always)]
        pub fn set_sram3_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SRAM2 ECC detection and correction enable"]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 ECC detection and correction enable"]
        #[inline(always)]
        pub fn set_sram2_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Software BOOT0"]
        #[inline(always)]
        pub const fn n_swboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Software BOOT0"]
        #[inline(always)]
        pub fn set_n_swboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "nBOOT0 option bit"]
        #[inline(always)]
        pub const fn n_boot0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "nBOOT0 option bit"]
        #[inline(always)]
        pub fn set_n_boot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PA15 pull-up enable"]
        #[inline(always)]
        pub const fn pa15_pupen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PA15 pull-up enable"]
        #[inline(always)]
        pub fn set_pa15_pupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
        #[inline(always)]
        pub const fn io_vdd_hslv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
        #[inline(always)]
        pub fn set_io_vdd_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
        #[inline(always)]
        pub const fn io_vddio2_hslv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
        #[inline(always)]
        pub fn set_io_vddio2_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Global TrustZone security enable"]
        #[inline(always)]
        pub const fn tzen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Global TrustZone security enable"]
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
                .field("n_rst_stop", &self.n_rst_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("n_rst_shdw", &self.n_rst_shdw())
                .field("sram1345_rst", &self.sram1345_rst())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("swap_bank", &self.swap_bank())
                .field("dualbank", &self.dualbank())
                .field("bkpsram_ecc", &self.bkpsram_ecc())
                .field("sram3_ecc", &self.sram3_ecc())
                .field("sram2_ecc", &self.sram2_ecc())
                .field("sram2_rst", &self.sram2_rst())
                .field("n_swboot0", &self.n_swboot0())
                .field("n_boot0", &self.n_boot0())
                .field("pa15_pupen", &self.pa15_pupen())
                .field("io_vdd_hslv", &self.io_vdd_hslv())
                .field("io_vddio2_hslv", &self.io_vddio2_hslv())
                .field("tzen", &self.tzen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optr {{ rdp: {:?}, bor_lev: {:?}, n_rst_stop: {=bool:?}, n_rst_stdby: {=bool:?}, n_rst_shdw: {=bool:?}, sram1345_rst: {=bool:?}, iwdg_sw: {=bool:?}, iwdg_stop: {=bool:?}, iwdg_stdby: {=bool:?}, wwdg_sw: {=bool:?}, swap_bank: {=bool:?}, dualbank: {=bool:?}, bkpsram_ecc: {=bool:?}, sram3_ecc: {=bool:?}, sram2_ecc: {=bool:?}, sram2_rst: {=bool:?}, n_swboot0: {=bool:?}, n_boot0: {=bool:?}, pa15_pupen: {=bool:?}, io_vdd_hslv: {=bool:?}, io_vddio2_hslv: {=bool:?}, tzen: {=bool:?} }}" , self . rdp () , self . bor_lev () , self . n_rst_stop () , self . n_rst_stdby () , self . n_rst_shdw () , self . sram1345_rst () , self . iwdg_sw () , self . iwdg_stop () , self . iwdg_stdby () , self . wwdg_sw () , self . swap_bank () , self . dualbank () , self . bkpsram_ecc () , self . sram3_ecc () , self . sram2_ecc () , self . sram2_rst () , self . n_swboot0 () , self . n_boot0 () , self . pa15_pupen () , self . io_vdd_hslv () , self . io_vddio2_hslv () , self . tzen ())
        }
    }
    #[doc = "FLASH bank 1 power-down key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdkey1r(pub u32);
    impl Pdkey1r {
        #[doc = "Bank 1 power-down key"]
        #[inline(always)]
        pub const fn pdkey1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank 1 power-down key"]
        #[inline(always)]
        pub fn set_pdkey1(&mut self, val: u32) {
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
            f.debug_struct("Pdkey1r").field("pdkey1", &self.pdkey1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdkey1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdkey1r {{ pdkey1: {=u32:?} }}", self.pdkey1())
        }
    }
    #[doc = "FLASH bank 2 power-down key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdkey2r(pub u32);
    impl Pdkey2r {
        #[doc = "Bank 2 power-down key"]
        #[inline(always)]
        pub const fn pdkey2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank 2 power-down key"]
        #[inline(always)]
        pub fn set_pdkey2(&mut self, val: u32) {
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
            f.debug_struct("Pdkey2r").field("pdkey2", &self.pdkey2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdkey2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdkey2r {{ pdkey2: {=u32:?} }}", self.pdkey2())
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr1(pub u32);
    impl Priv1bbr1 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr1 {
        #[inline(always)]
        fn default() -> Priv1bbr1 {
            Priv1bbr1(0)
        }
    }
    impl core::fmt::Debug for Priv1bbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv1bbr1")
                .field("priv1bb0", &self.priv1bb0())
                .field("priv1bb1", &self.priv1bb1())
                .field("priv1bb2", &self.priv1bb2())
                .field("priv1bb3", &self.priv1bb3())
                .field("priv1bb4", &self.priv1bb4())
                .field("priv1bb5", &self.priv1bb5())
                .field("priv1bb6", &self.priv1bb6())
                .field("priv1bb7", &self.priv1bb7())
                .field("priv1bb8", &self.priv1bb8())
                .field("priv1bb9", &self.priv1bb9())
                .field("priv1bb10", &self.priv1bb10())
                .field("priv1bb11", &self.priv1bb11())
                .field("priv1bb12", &self.priv1bb12())
                .field("priv1bb13", &self.priv1bb13())
                .field("priv1bb14", &self.priv1bb14())
                .field("priv1bb15", &self.priv1bb15())
                .field("priv1bb16", &self.priv1bb16())
                .field("priv1bb17", &self.priv1bb17())
                .field("priv1bb18", &self.priv1bb18())
                .field("priv1bb19", &self.priv1bb19())
                .field("priv1bb20", &self.priv1bb20())
                .field("priv1bb21", &self.priv1bb21())
                .field("priv1bb22", &self.priv1bb22())
                .field("priv1bb23", &self.priv1bb23())
                .field("priv1bb24", &self.priv1bb24())
                .field("priv1bb25", &self.priv1bb25())
                .field("priv1bb26", &self.priv1bb26())
                .field("priv1bb27", &self.priv1bb27())
                .field("priv1bb28", &self.priv1bb28())
                .field("priv1bb29", &self.priv1bb29())
                .field("priv1bb30", &self.priv1bb30())
                .field("priv1bb31", &self.priv1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv1bbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv1bbr1 {{ priv1bb0: {=bool:?}, priv1bb1: {=bool:?}, priv1bb2: {=bool:?}, priv1bb3: {=bool:?}, priv1bb4: {=bool:?}, priv1bb5: {=bool:?}, priv1bb6: {=bool:?}, priv1bb7: {=bool:?}, priv1bb8: {=bool:?}, priv1bb9: {=bool:?}, priv1bb10: {=bool:?}, priv1bb11: {=bool:?}, priv1bb12: {=bool:?}, priv1bb13: {=bool:?}, priv1bb14: {=bool:?}, priv1bb15: {=bool:?}, priv1bb16: {=bool:?}, priv1bb17: {=bool:?}, priv1bb18: {=bool:?}, priv1bb19: {=bool:?}, priv1bb20: {=bool:?}, priv1bb21: {=bool:?}, priv1bb22: {=bool:?}, priv1bb23: {=bool:?}, priv1bb24: {=bool:?}, priv1bb25: {=bool:?}, priv1bb26: {=bool:?}, priv1bb27: {=bool:?}, priv1bb28: {=bool:?}, priv1bb29: {=bool:?}, priv1bb30: {=bool:?}, priv1bb31: {=bool:?} }}" , self . priv1bb0 () , self . priv1bb1 () , self . priv1bb2 () , self . priv1bb3 () , self . priv1bb4 () , self . priv1bb5 () , self . priv1bb6 () , self . priv1bb7 () , self . priv1bb8 () , self . priv1bb9 () , self . priv1bb10 () , self . priv1bb11 () , self . priv1bb12 () , self . priv1bb13 () , self . priv1bb14 () , self . priv1bb15 () , self . priv1bb16 () , self . priv1bb17 () , self . priv1bb18 () , self . priv1bb19 () , self . priv1bb20 () , self . priv1bb21 () , self . priv1bb22 () , self . priv1bb23 () , self . priv1bb24 () , self . priv1bb25 () , self . priv1bb26 () , self . priv1bb27 () , self . priv1bb28 () , self . priv1bb29 () , self . priv1bb30 () , self . priv1bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr2(pub u32);
    impl Priv1bbr2 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr2 {
        #[inline(always)]
        fn default() -> Priv1bbr2 {
            Priv1bbr2(0)
        }
    }
    impl core::fmt::Debug for Priv1bbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv1bbr2")
                .field("priv1bb0", &self.priv1bb0())
                .field("priv1bb1", &self.priv1bb1())
                .field("priv1bb2", &self.priv1bb2())
                .field("priv1bb3", &self.priv1bb3())
                .field("priv1bb4", &self.priv1bb4())
                .field("priv1bb5", &self.priv1bb5())
                .field("priv1bb6", &self.priv1bb6())
                .field("priv1bb7", &self.priv1bb7())
                .field("priv1bb8", &self.priv1bb8())
                .field("priv1bb9", &self.priv1bb9())
                .field("priv1bb10", &self.priv1bb10())
                .field("priv1bb11", &self.priv1bb11())
                .field("priv1bb12", &self.priv1bb12())
                .field("priv1bb13", &self.priv1bb13())
                .field("priv1bb14", &self.priv1bb14())
                .field("priv1bb15", &self.priv1bb15())
                .field("priv1bb16", &self.priv1bb16())
                .field("priv1bb17", &self.priv1bb17())
                .field("priv1bb18", &self.priv1bb18())
                .field("priv1bb19", &self.priv1bb19())
                .field("priv1bb20", &self.priv1bb20())
                .field("priv1bb21", &self.priv1bb21())
                .field("priv1bb22", &self.priv1bb22())
                .field("priv1bb23", &self.priv1bb23())
                .field("priv1bb24", &self.priv1bb24())
                .field("priv1bb25", &self.priv1bb25())
                .field("priv1bb26", &self.priv1bb26())
                .field("priv1bb27", &self.priv1bb27())
                .field("priv1bb28", &self.priv1bb28())
                .field("priv1bb29", &self.priv1bb29())
                .field("priv1bb30", &self.priv1bb30())
                .field("priv1bb31", &self.priv1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv1bbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv1bbr2 {{ priv1bb0: {=bool:?}, priv1bb1: {=bool:?}, priv1bb2: {=bool:?}, priv1bb3: {=bool:?}, priv1bb4: {=bool:?}, priv1bb5: {=bool:?}, priv1bb6: {=bool:?}, priv1bb7: {=bool:?}, priv1bb8: {=bool:?}, priv1bb9: {=bool:?}, priv1bb10: {=bool:?}, priv1bb11: {=bool:?}, priv1bb12: {=bool:?}, priv1bb13: {=bool:?}, priv1bb14: {=bool:?}, priv1bb15: {=bool:?}, priv1bb16: {=bool:?}, priv1bb17: {=bool:?}, priv1bb18: {=bool:?}, priv1bb19: {=bool:?}, priv1bb20: {=bool:?}, priv1bb21: {=bool:?}, priv1bb22: {=bool:?}, priv1bb23: {=bool:?}, priv1bb24: {=bool:?}, priv1bb25: {=bool:?}, priv1bb26: {=bool:?}, priv1bb27: {=bool:?}, priv1bb28: {=bool:?}, priv1bb29: {=bool:?}, priv1bb30: {=bool:?}, priv1bb31: {=bool:?} }}" , self . priv1bb0 () , self . priv1bb1 () , self . priv1bb2 () , self . priv1bb3 () , self . priv1bb4 () , self . priv1bb5 () , self . priv1bb6 () , self . priv1bb7 () , self . priv1bb8 () , self . priv1bb9 () , self . priv1bb10 () , self . priv1bb11 () , self . priv1bb12 () , self . priv1bb13 () , self . priv1bb14 () , self . priv1bb15 () , self . priv1bb16 () , self . priv1bb17 () , self . priv1bb18 () , self . priv1bb19 () , self . priv1bb20 () , self . priv1bb21 () , self . priv1bb22 () , self . priv1bb23 () , self . priv1bb24 () , self . priv1bb25 () , self . priv1bb26 () , self . priv1bb27 () , self . priv1bb28 () , self . priv1bb29 () , self . priv1bb30 () , self . priv1bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr3(pub u32);
    impl Priv1bbr3 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr3 {
        #[inline(always)]
        fn default() -> Priv1bbr3 {
            Priv1bbr3(0)
        }
    }
    impl core::fmt::Debug for Priv1bbr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv1bbr3")
                .field("priv1bb0", &self.priv1bb0())
                .field("priv1bb1", &self.priv1bb1())
                .field("priv1bb2", &self.priv1bb2())
                .field("priv1bb3", &self.priv1bb3())
                .field("priv1bb4", &self.priv1bb4())
                .field("priv1bb5", &self.priv1bb5())
                .field("priv1bb6", &self.priv1bb6())
                .field("priv1bb7", &self.priv1bb7())
                .field("priv1bb8", &self.priv1bb8())
                .field("priv1bb9", &self.priv1bb9())
                .field("priv1bb10", &self.priv1bb10())
                .field("priv1bb11", &self.priv1bb11())
                .field("priv1bb12", &self.priv1bb12())
                .field("priv1bb13", &self.priv1bb13())
                .field("priv1bb14", &self.priv1bb14())
                .field("priv1bb15", &self.priv1bb15())
                .field("priv1bb16", &self.priv1bb16())
                .field("priv1bb17", &self.priv1bb17())
                .field("priv1bb18", &self.priv1bb18())
                .field("priv1bb19", &self.priv1bb19())
                .field("priv1bb20", &self.priv1bb20())
                .field("priv1bb21", &self.priv1bb21())
                .field("priv1bb22", &self.priv1bb22())
                .field("priv1bb23", &self.priv1bb23())
                .field("priv1bb24", &self.priv1bb24())
                .field("priv1bb25", &self.priv1bb25())
                .field("priv1bb26", &self.priv1bb26())
                .field("priv1bb27", &self.priv1bb27())
                .field("priv1bb28", &self.priv1bb28())
                .field("priv1bb29", &self.priv1bb29())
                .field("priv1bb30", &self.priv1bb30())
                .field("priv1bb31", &self.priv1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv1bbr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv1bbr3 {{ priv1bb0: {=bool:?}, priv1bb1: {=bool:?}, priv1bb2: {=bool:?}, priv1bb3: {=bool:?}, priv1bb4: {=bool:?}, priv1bb5: {=bool:?}, priv1bb6: {=bool:?}, priv1bb7: {=bool:?}, priv1bb8: {=bool:?}, priv1bb9: {=bool:?}, priv1bb10: {=bool:?}, priv1bb11: {=bool:?}, priv1bb12: {=bool:?}, priv1bb13: {=bool:?}, priv1bb14: {=bool:?}, priv1bb15: {=bool:?}, priv1bb16: {=bool:?}, priv1bb17: {=bool:?}, priv1bb18: {=bool:?}, priv1bb19: {=bool:?}, priv1bb20: {=bool:?}, priv1bb21: {=bool:?}, priv1bb22: {=bool:?}, priv1bb23: {=bool:?}, priv1bb24: {=bool:?}, priv1bb25: {=bool:?}, priv1bb26: {=bool:?}, priv1bb27: {=bool:?}, priv1bb28: {=bool:?}, priv1bb29: {=bool:?}, priv1bb30: {=bool:?}, priv1bb31: {=bool:?} }}" , self . priv1bb0 () , self . priv1bb1 () , self . priv1bb2 () , self . priv1bb3 () , self . priv1bb4 () , self . priv1bb5 () , self . priv1bb6 () , self . priv1bb7 () , self . priv1bb8 () , self . priv1bb9 () , self . priv1bb10 () , self . priv1bb11 () , self . priv1bb12 () , self . priv1bb13 () , self . priv1bb14 () , self . priv1bb15 () , self . priv1bb16 () , self . priv1bb17 () , self . priv1bb18 () , self . priv1bb19 () , self . priv1bb20 () , self . priv1bb21 () , self . priv1bb22 () , self . priv1bb23 () , self . priv1bb24 () , self . priv1bb25 () , self . priv1bb26 () , self . priv1bb27 () , self . priv1bb28 () , self . priv1bb29 () , self . priv1bb30 () , self . priv1bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr4(pub u32);
    impl Priv1bbr4 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr4 {
        #[inline(always)]
        fn default() -> Priv1bbr4 {
            Priv1bbr4(0)
        }
    }
    impl core::fmt::Debug for Priv1bbr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv1bbr4")
                .field("priv1bb0", &self.priv1bb0())
                .field("priv1bb1", &self.priv1bb1())
                .field("priv1bb2", &self.priv1bb2())
                .field("priv1bb3", &self.priv1bb3())
                .field("priv1bb4", &self.priv1bb4())
                .field("priv1bb5", &self.priv1bb5())
                .field("priv1bb6", &self.priv1bb6())
                .field("priv1bb7", &self.priv1bb7())
                .field("priv1bb8", &self.priv1bb8())
                .field("priv1bb9", &self.priv1bb9())
                .field("priv1bb10", &self.priv1bb10())
                .field("priv1bb11", &self.priv1bb11())
                .field("priv1bb12", &self.priv1bb12())
                .field("priv1bb13", &self.priv1bb13())
                .field("priv1bb14", &self.priv1bb14())
                .field("priv1bb15", &self.priv1bb15())
                .field("priv1bb16", &self.priv1bb16())
                .field("priv1bb17", &self.priv1bb17())
                .field("priv1bb18", &self.priv1bb18())
                .field("priv1bb19", &self.priv1bb19())
                .field("priv1bb20", &self.priv1bb20())
                .field("priv1bb21", &self.priv1bb21())
                .field("priv1bb22", &self.priv1bb22())
                .field("priv1bb23", &self.priv1bb23())
                .field("priv1bb24", &self.priv1bb24())
                .field("priv1bb25", &self.priv1bb25())
                .field("priv1bb26", &self.priv1bb26())
                .field("priv1bb27", &self.priv1bb27())
                .field("priv1bb28", &self.priv1bb28())
                .field("priv1bb29", &self.priv1bb29())
                .field("priv1bb30", &self.priv1bb30())
                .field("priv1bb31", &self.priv1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv1bbr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv1bbr4 {{ priv1bb0: {=bool:?}, priv1bb1: {=bool:?}, priv1bb2: {=bool:?}, priv1bb3: {=bool:?}, priv1bb4: {=bool:?}, priv1bb5: {=bool:?}, priv1bb6: {=bool:?}, priv1bb7: {=bool:?}, priv1bb8: {=bool:?}, priv1bb9: {=bool:?}, priv1bb10: {=bool:?}, priv1bb11: {=bool:?}, priv1bb12: {=bool:?}, priv1bb13: {=bool:?}, priv1bb14: {=bool:?}, priv1bb15: {=bool:?}, priv1bb16: {=bool:?}, priv1bb17: {=bool:?}, priv1bb18: {=bool:?}, priv1bb19: {=bool:?}, priv1bb20: {=bool:?}, priv1bb21: {=bool:?}, priv1bb22: {=bool:?}, priv1bb23: {=bool:?}, priv1bb24: {=bool:?}, priv1bb25: {=bool:?}, priv1bb26: {=bool:?}, priv1bb27: {=bool:?}, priv1bb28: {=bool:?}, priv1bb29: {=bool:?}, priv1bb30: {=bool:?}, priv1bb31: {=bool:?} }}" , self . priv1bb0 () , self . priv1bb1 () , self . priv1bb2 () , self . priv1bb3 () , self . priv1bb4 () , self . priv1bb5 () , self . priv1bb6 () , self . priv1bb7 () , self . priv1bb8 () , self . priv1bb9 () , self . priv1bb10 () , self . priv1bb11 () , self . priv1bb12 () , self . priv1bb13 () , self . priv1bb14 () , self . priv1bb15 () , self . priv1bb16 () , self . priv1bb17 () , self . priv1bb18 () , self . priv1bb19 () , self . priv1bb20 () , self . priv1bb21 () , self . priv1bb22 () , self . priv1bb23 () , self . priv1bb24 () , self . priv1bb25 () , self . priv1bb26 () , self . priv1bb27 () , self . priv1bb28 () , self . priv1bb29 () , self . priv1bb30 () , self . priv1bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr1(pub u32);
    impl Priv2bbr1 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr1 {
        #[inline(always)]
        fn default() -> Priv2bbr1 {
            Priv2bbr1(0)
        }
    }
    impl core::fmt::Debug for Priv2bbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv2bbr1")
                .field("priv2bb0", &self.priv2bb0())
                .field("priv2bb1", &self.priv2bb1())
                .field("priv2bb2", &self.priv2bb2())
                .field("priv2bb3", &self.priv2bb3())
                .field("priv2bb4", &self.priv2bb4())
                .field("priv2bb5", &self.priv2bb5())
                .field("priv2bb6", &self.priv2bb6())
                .field("priv2bb7", &self.priv2bb7())
                .field("priv2bb8", &self.priv2bb8())
                .field("priv2bb9", &self.priv2bb9())
                .field("priv2bb10", &self.priv2bb10())
                .field("priv2bb11", &self.priv2bb11())
                .field("priv2bb12", &self.priv2bb12())
                .field("priv2bb13", &self.priv2bb13())
                .field("priv2bb14", &self.priv2bb14())
                .field("priv2bb15", &self.priv2bb15())
                .field("priv2bb16", &self.priv2bb16())
                .field("priv2bb17", &self.priv2bb17())
                .field("priv2bb18", &self.priv2bb18())
                .field("priv2bb19", &self.priv2bb19())
                .field("priv2bb20", &self.priv2bb20())
                .field("priv2bb21", &self.priv2bb21())
                .field("priv2bb22", &self.priv2bb22())
                .field("priv2bb23", &self.priv2bb23())
                .field("priv2bb24", &self.priv2bb24())
                .field("priv2bb25", &self.priv2bb25())
                .field("priv2bb26", &self.priv2bb26())
                .field("priv2bb27", &self.priv2bb27())
                .field("priv2bb28", &self.priv2bb28())
                .field("priv2bb29", &self.priv2bb29())
                .field("priv2bb30", &self.priv2bb30())
                .field("priv2bb31", &self.priv2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv2bbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv2bbr1 {{ priv2bb0: {=bool:?}, priv2bb1: {=bool:?}, priv2bb2: {=bool:?}, priv2bb3: {=bool:?}, priv2bb4: {=bool:?}, priv2bb5: {=bool:?}, priv2bb6: {=bool:?}, priv2bb7: {=bool:?}, priv2bb8: {=bool:?}, priv2bb9: {=bool:?}, priv2bb10: {=bool:?}, priv2bb11: {=bool:?}, priv2bb12: {=bool:?}, priv2bb13: {=bool:?}, priv2bb14: {=bool:?}, priv2bb15: {=bool:?}, priv2bb16: {=bool:?}, priv2bb17: {=bool:?}, priv2bb18: {=bool:?}, priv2bb19: {=bool:?}, priv2bb20: {=bool:?}, priv2bb21: {=bool:?}, priv2bb22: {=bool:?}, priv2bb23: {=bool:?}, priv2bb24: {=bool:?}, priv2bb25: {=bool:?}, priv2bb26: {=bool:?}, priv2bb27: {=bool:?}, priv2bb28: {=bool:?}, priv2bb29: {=bool:?}, priv2bb30: {=bool:?}, priv2bb31: {=bool:?} }}" , self . priv2bb0 () , self . priv2bb1 () , self . priv2bb2 () , self . priv2bb3 () , self . priv2bb4 () , self . priv2bb5 () , self . priv2bb6 () , self . priv2bb7 () , self . priv2bb8 () , self . priv2bb9 () , self . priv2bb10 () , self . priv2bb11 () , self . priv2bb12 () , self . priv2bb13 () , self . priv2bb14 () , self . priv2bb15 () , self . priv2bb16 () , self . priv2bb17 () , self . priv2bb18 () , self . priv2bb19 () , self . priv2bb20 () , self . priv2bb21 () , self . priv2bb22 () , self . priv2bb23 () , self . priv2bb24 () , self . priv2bb25 () , self . priv2bb26 () , self . priv2bb27 () , self . priv2bb28 () , self . priv2bb29 () , self . priv2bb30 () , self . priv2bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr2(pub u32);
    impl Priv2bbr2 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr2 {
        #[inline(always)]
        fn default() -> Priv2bbr2 {
            Priv2bbr2(0)
        }
    }
    impl core::fmt::Debug for Priv2bbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv2bbr2")
                .field("priv2bb0", &self.priv2bb0())
                .field("priv2bb1", &self.priv2bb1())
                .field("priv2bb2", &self.priv2bb2())
                .field("priv2bb3", &self.priv2bb3())
                .field("priv2bb4", &self.priv2bb4())
                .field("priv2bb5", &self.priv2bb5())
                .field("priv2bb6", &self.priv2bb6())
                .field("priv2bb7", &self.priv2bb7())
                .field("priv2bb8", &self.priv2bb8())
                .field("priv2bb9", &self.priv2bb9())
                .field("priv2bb10", &self.priv2bb10())
                .field("priv2bb11", &self.priv2bb11())
                .field("priv2bb12", &self.priv2bb12())
                .field("priv2bb13", &self.priv2bb13())
                .field("priv2bb14", &self.priv2bb14())
                .field("priv2bb15", &self.priv2bb15())
                .field("priv2bb16", &self.priv2bb16())
                .field("priv2bb17", &self.priv2bb17())
                .field("priv2bb18", &self.priv2bb18())
                .field("priv2bb19", &self.priv2bb19())
                .field("priv2bb20", &self.priv2bb20())
                .field("priv2bb21", &self.priv2bb21())
                .field("priv2bb22", &self.priv2bb22())
                .field("priv2bb23", &self.priv2bb23())
                .field("priv2bb24", &self.priv2bb24())
                .field("priv2bb25", &self.priv2bb25())
                .field("priv2bb26", &self.priv2bb26())
                .field("priv2bb27", &self.priv2bb27())
                .field("priv2bb28", &self.priv2bb28())
                .field("priv2bb29", &self.priv2bb29())
                .field("priv2bb30", &self.priv2bb30())
                .field("priv2bb31", &self.priv2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv2bbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv2bbr2 {{ priv2bb0: {=bool:?}, priv2bb1: {=bool:?}, priv2bb2: {=bool:?}, priv2bb3: {=bool:?}, priv2bb4: {=bool:?}, priv2bb5: {=bool:?}, priv2bb6: {=bool:?}, priv2bb7: {=bool:?}, priv2bb8: {=bool:?}, priv2bb9: {=bool:?}, priv2bb10: {=bool:?}, priv2bb11: {=bool:?}, priv2bb12: {=bool:?}, priv2bb13: {=bool:?}, priv2bb14: {=bool:?}, priv2bb15: {=bool:?}, priv2bb16: {=bool:?}, priv2bb17: {=bool:?}, priv2bb18: {=bool:?}, priv2bb19: {=bool:?}, priv2bb20: {=bool:?}, priv2bb21: {=bool:?}, priv2bb22: {=bool:?}, priv2bb23: {=bool:?}, priv2bb24: {=bool:?}, priv2bb25: {=bool:?}, priv2bb26: {=bool:?}, priv2bb27: {=bool:?}, priv2bb28: {=bool:?}, priv2bb29: {=bool:?}, priv2bb30: {=bool:?}, priv2bb31: {=bool:?} }}" , self . priv2bb0 () , self . priv2bb1 () , self . priv2bb2 () , self . priv2bb3 () , self . priv2bb4 () , self . priv2bb5 () , self . priv2bb6 () , self . priv2bb7 () , self . priv2bb8 () , self . priv2bb9 () , self . priv2bb10 () , self . priv2bb11 () , self . priv2bb12 () , self . priv2bb13 () , self . priv2bb14 () , self . priv2bb15 () , self . priv2bb16 () , self . priv2bb17 () , self . priv2bb18 () , self . priv2bb19 () , self . priv2bb20 () , self . priv2bb21 () , self . priv2bb22 () , self . priv2bb23 () , self . priv2bb24 () , self . priv2bb25 () , self . priv2bb26 () , self . priv2bb27 () , self . priv2bb28 () , self . priv2bb29 () , self . priv2bb30 () , self . priv2bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr3(pub u32);
    impl Priv2bbr3 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr3 {
        #[inline(always)]
        fn default() -> Priv2bbr3 {
            Priv2bbr3(0)
        }
    }
    impl core::fmt::Debug for Priv2bbr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv2bbr3")
                .field("priv2bb0", &self.priv2bb0())
                .field("priv2bb1", &self.priv2bb1())
                .field("priv2bb2", &self.priv2bb2())
                .field("priv2bb3", &self.priv2bb3())
                .field("priv2bb4", &self.priv2bb4())
                .field("priv2bb5", &self.priv2bb5())
                .field("priv2bb6", &self.priv2bb6())
                .field("priv2bb7", &self.priv2bb7())
                .field("priv2bb8", &self.priv2bb8())
                .field("priv2bb9", &self.priv2bb9())
                .field("priv2bb10", &self.priv2bb10())
                .field("priv2bb11", &self.priv2bb11())
                .field("priv2bb12", &self.priv2bb12())
                .field("priv2bb13", &self.priv2bb13())
                .field("priv2bb14", &self.priv2bb14())
                .field("priv2bb15", &self.priv2bb15())
                .field("priv2bb16", &self.priv2bb16())
                .field("priv2bb17", &self.priv2bb17())
                .field("priv2bb18", &self.priv2bb18())
                .field("priv2bb19", &self.priv2bb19())
                .field("priv2bb20", &self.priv2bb20())
                .field("priv2bb21", &self.priv2bb21())
                .field("priv2bb22", &self.priv2bb22())
                .field("priv2bb23", &self.priv2bb23())
                .field("priv2bb24", &self.priv2bb24())
                .field("priv2bb25", &self.priv2bb25())
                .field("priv2bb26", &self.priv2bb26())
                .field("priv2bb27", &self.priv2bb27())
                .field("priv2bb28", &self.priv2bb28())
                .field("priv2bb29", &self.priv2bb29())
                .field("priv2bb30", &self.priv2bb30())
                .field("priv2bb31", &self.priv2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv2bbr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv2bbr3 {{ priv2bb0: {=bool:?}, priv2bb1: {=bool:?}, priv2bb2: {=bool:?}, priv2bb3: {=bool:?}, priv2bb4: {=bool:?}, priv2bb5: {=bool:?}, priv2bb6: {=bool:?}, priv2bb7: {=bool:?}, priv2bb8: {=bool:?}, priv2bb9: {=bool:?}, priv2bb10: {=bool:?}, priv2bb11: {=bool:?}, priv2bb12: {=bool:?}, priv2bb13: {=bool:?}, priv2bb14: {=bool:?}, priv2bb15: {=bool:?}, priv2bb16: {=bool:?}, priv2bb17: {=bool:?}, priv2bb18: {=bool:?}, priv2bb19: {=bool:?}, priv2bb20: {=bool:?}, priv2bb21: {=bool:?}, priv2bb22: {=bool:?}, priv2bb23: {=bool:?}, priv2bb24: {=bool:?}, priv2bb25: {=bool:?}, priv2bb26: {=bool:?}, priv2bb27: {=bool:?}, priv2bb28: {=bool:?}, priv2bb29: {=bool:?}, priv2bb30: {=bool:?}, priv2bb31: {=bool:?} }}" , self . priv2bb0 () , self . priv2bb1 () , self . priv2bb2 () , self . priv2bb3 () , self . priv2bb4 () , self . priv2bb5 () , self . priv2bb6 () , self . priv2bb7 () , self . priv2bb8 () , self . priv2bb9 () , self . priv2bb10 () , self . priv2bb11 () , self . priv2bb12 () , self . priv2bb13 () , self . priv2bb14 () , self . priv2bb15 () , self . priv2bb16 () , self . priv2bb17 () , self . priv2bb18 () , self . priv2bb19 () , self . priv2bb20 () , self . priv2bb21 () , self . priv2bb22 () , self . priv2bb23 () , self . priv2bb24 () , self . priv2bb25 () , self . priv2bb26 () , self . priv2bb27 () , self . priv2bb28 () , self . priv2bb29 () , self . priv2bb30 () , self . priv2bb31 ())
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr4(pub u32);
    impl Priv2bbr4 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr4 {
        #[inline(always)]
        fn default() -> Priv2bbr4 {
            Priv2bbr4(0)
        }
    }
    impl core::fmt::Debug for Priv2bbr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv2bbr4")
                .field("priv2bb0", &self.priv2bb0())
                .field("priv2bb1", &self.priv2bb1())
                .field("priv2bb2", &self.priv2bb2())
                .field("priv2bb3", &self.priv2bb3())
                .field("priv2bb4", &self.priv2bb4())
                .field("priv2bb5", &self.priv2bb5())
                .field("priv2bb6", &self.priv2bb6())
                .field("priv2bb7", &self.priv2bb7())
                .field("priv2bb8", &self.priv2bb8())
                .field("priv2bb9", &self.priv2bb9())
                .field("priv2bb10", &self.priv2bb10())
                .field("priv2bb11", &self.priv2bb11())
                .field("priv2bb12", &self.priv2bb12())
                .field("priv2bb13", &self.priv2bb13())
                .field("priv2bb14", &self.priv2bb14())
                .field("priv2bb15", &self.priv2bb15())
                .field("priv2bb16", &self.priv2bb16())
                .field("priv2bb17", &self.priv2bb17())
                .field("priv2bb18", &self.priv2bb18())
                .field("priv2bb19", &self.priv2bb19())
                .field("priv2bb20", &self.priv2bb20())
                .field("priv2bb21", &self.priv2bb21())
                .field("priv2bb22", &self.priv2bb22())
                .field("priv2bb23", &self.priv2bb23())
                .field("priv2bb24", &self.priv2bb24())
                .field("priv2bb25", &self.priv2bb25())
                .field("priv2bb26", &self.priv2bb26())
                .field("priv2bb27", &self.priv2bb27())
                .field("priv2bb28", &self.priv2bb28())
                .field("priv2bb29", &self.priv2bb29())
                .field("priv2bb30", &self.priv2bb30())
                .field("priv2bb31", &self.priv2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv2bbr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv2bbr4 {{ priv2bb0: {=bool:?}, priv2bb1: {=bool:?}, priv2bb2: {=bool:?}, priv2bb3: {=bool:?}, priv2bb4: {=bool:?}, priv2bb5: {=bool:?}, priv2bb6: {=bool:?}, priv2bb7: {=bool:?}, priv2bb8: {=bool:?}, priv2bb9: {=bool:?}, priv2bb10: {=bool:?}, priv2bb11: {=bool:?}, priv2bb12: {=bool:?}, priv2bb13: {=bool:?}, priv2bb14: {=bool:?}, priv2bb15: {=bool:?}, priv2bb16: {=bool:?}, priv2bb17: {=bool:?}, priv2bb18: {=bool:?}, priv2bb19: {=bool:?}, priv2bb20: {=bool:?}, priv2bb21: {=bool:?}, priv2bb22: {=bool:?}, priv2bb23: {=bool:?}, priv2bb24: {=bool:?}, priv2bb25: {=bool:?}, priv2bb26: {=bool:?}, priv2bb27: {=bool:?}, priv2bb28: {=bool:?}, priv2bb29: {=bool:?}, priv2bb30: {=bool:?}, priv2bb31: {=bool:?} }}" , self . priv2bb0 () , self . priv2bb1 () , self . priv2bb2 () , self . priv2bb3 () , self . priv2bb4 () , self . priv2bb5 () , self . priv2bb6 () , self . priv2bb7 () , self . priv2bb8 () , self . priv2bb9 () , self . priv2bb10 () , self . priv2bb11 () , self . priv2bb12 () , self . priv2bb13 () , self . priv2bb14 () , self . priv2bb15 () , self . priv2bb16 () , self . priv2bb17 () , self . priv2bb18 () , self . priv2bb19 () , self . priv2bb20 () , self . priv2bb21 () , self . priv2bb22 () , self . priv2bb23 () , self . priv2bb24 () , self . priv2bb25 () , self . priv2bb26 () , self . priv2bb27 () , self . priv2bb28 () , self . priv2bb29 () , self . priv2bb30 () , self . priv2bb31 ())
        }
    }
    #[doc = "FLASH privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: bool) {
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
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, nspriv: {=bool:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "FLASH secure block based bank 1 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr1(pub u32);
    impl Sec1bbr1 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr1 {
        #[inline(always)]
        fn default() -> Sec1bbr1 {
            Sec1bbr1(0)
        }
    }
    impl core::fmt::Debug for Sec1bbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec1bbr1")
                .field("sec1bb0", &self.sec1bb0())
                .field("sec1bb1", &self.sec1bb1())
                .field("sec1bb2", &self.sec1bb2())
                .field("sec1bb3", &self.sec1bb3())
                .field("sec1bb4", &self.sec1bb4())
                .field("sec1bb5", &self.sec1bb5())
                .field("sec1bb6", &self.sec1bb6())
                .field("sec1bb7", &self.sec1bb7())
                .field("sec1bb8", &self.sec1bb8())
                .field("sec1bb9", &self.sec1bb9())
                .field("sec1bb10", &self.sec1bb10())
                .field("sec1bb11", &self.sec1bb11())
                .field("sec1bb12", &self.sec1bb12())
                .field("sec1bb13", &self.sec1bb13())
                .field("sec1bb14", &self.sec1bb14())
                .field("sec1bb15", &self.sec1bb15())
                .field("sec1bb16", &self.sec1bb16())
                .field("sec1bb17", &self.sec1bb17())
                .field("sec1bb18", &self.sec1bb18())
                .field("sec1bb19", &self.sec1bb19())
                .field("sec1bb20", &self.sec1bb20())
                .field("sec1bb21", &self.sec1bb21())
                .field("sec1bb22", &self.sec1bb22())
                .field("sec1bb23", &self.sec1bb23())
                .field("sec1bb24", &self.sec1bb24())
                .field("sec1bb25", &self.sec1bb25())
                .field("sec1bb26", &self.sec1bb26())
                .field("sec1bb27", &self.sec1bb27())
                .field("sec1bb28", &self.sec1bb28())
                .field("sec1bb29", &self.sec1bb29())
                .field("sec1bb30", &self.sec1bb30())
                .field("sec1bb31", &self.sec1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec1bbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec1bbr1 {{ sec1bb0: {=bool:?}, sec1bb1: {=bool:?}, sec1bb2: {=bool:?}, sec1bb3: {=bool:?}, sec1bb4: {=bool:?}, sec1bb5: {=bool:?}, sec1bb6: {=bool:?}, sec1bb7: {=bool:?}, sec1bb8: {=bool:?}, sec1bb9: {=bool:?}, sec1bb10: {=bool:?}, sec1bb11: {=bool:?}, sec1bb12: {=bool:?}, sec1bb13: {=bool:?}, sec1bb14: {=bool:?}, sec1bb15: {=bool:?}, sec1bb16: {=bool:?}, sec1bb17: {=bool:?}, sec1bb18: {=bool:?}, sec1bb19: {=bool:?}, sec1bb20: {=bool:?}, sec1bb21: {=bool:?}, sec1bb22: {=bool:?}, sec1bb23: {=bool:?}, sec1bb24: {=bool:?}, sec1bb25: {=bool:?}, sec1bb26: {=bool:?}, sec1bb27: {=bool:?}, sec1bb28: {=bool:?}, sec1bb29: {=bool:?}, sec1bb30: {=bool:?}, sec1bb31: {=bool:?} }}" , self . sec1bb0 () , self . sec1bb1 () , self . sec1bb2 () , self . sec1bb3 () , self . sec1bb4 () , self . sec1bb5 () , self . sec1bb6 () , self . sec1bb7 () , self . sec1bb8 () , self . sec1bb9 () , self . sec1bb10 () , self . sec1bb11 () , self . sec1bb12 () , self . sec1bb13 () , self . sec1bb14 () , self . sec1bb15 () , self . sec1bb16 () , self . sec1bb17 () , self . sec1bb18 () , self . sec1bb19 () , self . sec1bb20 () , self . sec1bb21 () , self . sec1bb22 () , self . sec1bb23 () , self . sec1bb24 () , self . sec1bb25 () , self . sec1bb26 () , self . sec1bb27 () , self . sec1bb28 () , self . sec1bb29 () , self . sec1bb30 () , self . sec1bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr2(pub u32);
    impl Sec1bbr2 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr2 {
        #[inline(always)]
        fn default() -> Sec1bbr2 {
            Sec1bbr2(0)
        }
    }
    impl core::fmt::Debug for Sec1bbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec1bbr2")
                .field("sec1bb0", &self.sec1bb0())
                .field("sec1bb1", &self.sec1bb1())
                .field("sec1bb2", &self.sec1bb2())
                .field("sec1bb3", &self.sec1bb3())
                .field("sec1bb4", &self.sec1bb4())
                .field("sec1bb5", &self.sec1bb5())
                .field("sec1bb6", &self.sec1bb6())
                .field("sec1bb7", &self.sec1bb7())
                .field("sec1bb8", &self.sec1bb8())
                .field("sec1bb9", &self.sec1bb9())
                .field("sec1bb10", &self.sec1bb10())
                .field("sec1bb11", &self.sec1bb11())
                .field("sec1bb12", &self.sec1bb12())
                .field("sec1bb13", &self.sec1bb13())
                .field("sec1bb14", &self.sec1bb14())
                .field("sec1bb15", &self.sec1bb15())
                .field("sec1bb16", &self.sec1bb16())
                .field("sec1bb17", &self.sec1bb17())
                .field("sec1bb18", &self.sec1bb18())
                .field("sec1bb19", &self.sec1bb19())
                .field("sec1bb20", &self.sec1bb20())
                .field("sec1bb21", &self.sec1bb21())
                .field("sec1bb22", &self.sec1bb22())
                .field("sec1bb23", &self.sec1bb23())
                .field("sec1bb24", &self.sec1bb24())
                .field("sec1bb25", &self.sec1bb25())
                .field("sec1bb26", &self.sec1bb26())
                .field("sec1bb27", &self.sec1bb27())
                .field("sec1bb28", &self.sec1bb28())
                .field("sec1bb29", &self.sec1bb29())
                .field("sec1bb30", &self.sec1bb30())
                .field("sec1bb31", &self.sec1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec1bbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec1bbr2 {{ sec1bb0: {=bool:?}, sec1bb1: {=bool:?}, sec1bb2: {=bool:?}, sec1bb3: {=bool:?}, sec1bb4: {=bool:?}, sec1bb5: {=bool:?}, sec1bb6: {=bool:?}, sec1bb7: {=bool:?}, sec1bb8: {=bool:?}, sec1bb9: {=bool:?}, sec1bb10: {=bool:?}, sec1bb11: {=bool:?}, sec1bb12: {=bool:?}, sec1bb13: {=bool:?}, sec1bb14: {=bool:?}, sec1bb15: {=bool:?}, sec1bb16: {=bool:?}, sec1bb17: {=bool:?}, sec1bb18: {=bool:?}, sec1bb19: {=bool:?}, sec1bb20: {=bool:?}, sec1bb21: {=bool:?}, sec1bb22: {=bool:?}, sec1bb23: {=bool:?}, sec1bb24: {=bool:?}, sec1bb25: {=bool:?}, sec1bb26: {=bool:?}, sec1bb27: {=bool:?}, sec1bb28: {=bool:?}, sec1bb29: {=bool:?}, sec1bb30: {=bool:?}, sec1bb31: {=bool:?} }}" , self . sec1bb0 () , self . sec1bb1 () , self . sec1bb2 () , self . sec1bb3 () , self . sec1bb4 () , self . sec1bb5 () , self . sec1bb6 () , self . sec1bb7 () , self . sec1bb8 () , self . sec1bb9 () , self . sec1bb10 () , self . sec1bb11 () , self . sec1bb12 () , self . sec1bb13 () , self . sec1bb14 () , self . sec1bb15 () , self . sec1bb16 () , self . sec1bb17 () , self . sec1bb18 () , self . sec1bb19 () , self . sec1bb20 () , self . sec1bb21 () , self . sec1bb22 () , self . sec1bb23 () , self . sec1bb24 () , self . sec1bb25 () , self . sec1bb26 () , self . sec1bb27 () , self . sec1bb28 () , self . sec1bb29 () , self . sec1bb30 () , self . sec1bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr3(pub u32);
    impl Sec1bbr3 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr3 {
        #[inline(always)]
        fn default() -> Sec1bbr3 {
            Sec1bbr3(0)
        }
    }
    impl core::fmt::Debug for Sec1bbr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec1bbr3")
                .field("sec1bb0", &self.sec1bb0())
                .field("sec1bb1", &self.sec1bb1())
                .field("sec1bb2", &self.sec1bb2())
                .field("sec1bb3", &self.sec1bb3())
                .field("sec1bb4", &self.sec1bb4())
                .field("sec1bb5", &self.sec1bb5())
                .field("sec1bb6", &self.sec1bb6())
                .field("sec1bb7", &self.sec1bb7())
                .field("sec1bb8", &self.sec1bb8())
                .field("sec1bb9", &self.sec1bb9())
                .field("sec1bb10", &self.sec1bb10())
                .field("sec1bb11", &self.sec1bb11())
                .field("sec1bb12", &self.sec1bb12())
                .field("sec1bb13", &self.sec1bb13())
                .field("sec1bb14", &self.sec1bb14())
                .field("sec1bb15", &self.sec1bb15())
                .field("sec1bb16", &self.sec1bb16())
                .field("sec1bb17", &self.sec1bb17())
                .field("sec1bb18", &self.sec1bb18())
                .field("sec1bb19", &self.sec1bb19())
                .field("sec1bb20", &self.sec1bb20())
                .field("sec1bb21", &self.sec1bb21())
                .field("sec1bb22", &self.sec1bb22())
                .field("sec1bb23", &self.sec1bb23())
                .field("sec1bb24", &self.sec1bb24())
                .field("sec1bb25", &self.sec1bb25())
                .field("sec1bb26", &self.sec1bb26())
                .field("sec1bb27", &self.sec1bb27())
                .field("sec1bb28", &self.sec1bb28())
                .field("sec1bb29", &self.sec1bb29())
                .field("sec1bb30", &self.sec1bb30())
                .field("sec1bb31", &self.sec1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec1bbr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec1bbr3 {{ sec1bb0: {=bool:?}, sec1bb1: {=bool:?}, sec1bb2: {=bool:?}, sec1bb3: {=bool:?}, sec1bb4: {=bool:?}, sec1bb5: {=bool:?}, sec1bb6: {=bool:?}, sec1bb7: {=bool:?}, sec1bb8: {=bool:?}, sec1bb9: {=bool:?}, sec1bb10: {=bool:?}, sec1bb11: {=bool:?}, sec1bb12: {=bool:?}, sec1bb13: {=bool:?}, sec1bb14: {=bool:?}, sec1bb15: {=bool:?}, sec1bb16: {=bool:?}, sec1bb17: {=bool:?}, sec1bb18: {=bool:?}, sec1bb19: {=bool:?}, sec1bb20: {=bool:?}, sec1bb21: {=bool:?}, sec1bb22: {=bool:?}, sec1bb23: {=bool:?}, sec1bb24: {=bool:?}, sec1bb25: {=bool:?}, sec1bb26: {=bool:?}, sec1bb27: {=bool:?}, sec1bb28: {=bool:?}, sec1bb29: {=bool:?}, sec1bb30: {=bool:?}, sec1bb31: {=bool:?} }}" , self . sec1bb0 () , self . sec1bb1 () , self . sec1bb2 () , self . sec1bb3 () , self . sec1bb4 () , self . sec1bb5 () , self . sec1bb6 () , self . sec1bb7 () , self . sec1bb8 () , self . sec1bb9 () , self . sec1bb10 () , self . sec1bb11 () , self . sec1bb12 () , self . sec1bb13 () , self . sec1bb14 () , self . sec1bb15 () , self . sec1bb16 () , self . sec1bb17 () , self . sec1bb18 () , self . sec1bb19 () , self . sec1bb20 () , self . sec1bb21 () , self . sec1bb22 () , self . sec1bb23 () , self . sec1bb24 () , self . sec1bb25 () , self . sec1bb26 () , self . sec1bb27 () , self . sec1bb28 () , self . sec1bb29 () , self . sec1bb30 () , self . sec1bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 1 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr4(pub u32);
    impl Sec1bbr4 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr4 {
        #[inline(always)]
        fn default() -> Sec1bbr4 {
            Sec1bbr4(0)
        }
    }
    impl core::fmt::Debug for Sec1bbr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec1bbr4")
                .field("sec1bb0", &self.sec1bb0())
                .field("sec1bb1", &self.sec1bb1())
                .field("sec1bb2", &self.sec1bb2())
                .field("sec1bb3", &self.sec1bb3())
                .field("sec1bb4", &self.sec1bb4())
                .field("sec1bb5", &self.sec1bb5())
                .field("sec1bb6", &self.sec1bb6())
                .field("sec1bb7", &self.sec1bb7())
                .field("sec1bb8", &self.sec1bb8())
                .field("sec1bb9", &self.sec1bb9())
                .field("sec1bb10", &self.sec1bb10())
                .field("sec1bb11", &self.sec1bb11())
                .field("sec1bb12", &self.sec1bb12())
                .field("sec1bb13", &self.sec1bb13())
                .field("sec1bb14", &self.sec1bb14())
                .field("sec1bb15", &self.sec1bb15())
                .field("sec1bb16", &self.sec1bb16())
                .field("sec1bb17", &self.sec1bb17())
                .field("sec1bb18", &self.sec1bb18())
                .field("sec1bb19", &self.sec1bb19())
                .field("sec1bb20", &self.sec1bb20())
                .field("sec1bb21", &self.sec1bb21())
                .field("sec1bb22", &self.sec1bb22())
                .field("sec1bb23", &self.sec1bb23())
                .field("sec1bb24", &self.sec1bb24())
                .field("sec1bb25", &self.sec1bb25())
                .field("sec1bb26", &self.sec1bb26())
                .field("sec1bb27", &self.sec1bb27())
                .field("sec1bb28", &self.sec1bb28())
                .field("sec1bb29", &self.sec1bb29())
                .field("sec1bb30", &self.sec1bb30())
                .field("sec1bb31", &self.sec1bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec1bbr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec1bbr4 {{ sec1bb0: {=bool:?}, sec1bb1: {=bool:?}, sec1bb2: {=bool:?}, sec1bb3: {=bool:?}, sec1bb4: {=bool:?}, sec1bb5: {=bool:?}, sec1bb6: {=bool:?}, sec1bb7: {=bool:?}, sec1bb8: {=bool:?}, sec1bb9: {=bool:?}, sec1bb10: {=bool:?}, sec1bb11: {=bool:?}, sec1bb12: {=bool:?}, sec1bb13: {=bool:?}, sec1bb14: {=bool:?}, sec1bb15: {=bool:?}, sec1bb16: {=bool:?}, sec1bb17: {=bool:?}, sec1bb18: {=bool:?}, sec1bb19: {=bool:?}, sec1bb20: {=bool:?}, sec1bb21: {=bool:?}, sec1bb22: {=bool:?}, sec1bb23: {=bool:?}, sec1bb24: {=bool:?}, sec1bb25: {=bool:?}, sec1bb26: {=bool:?}, sec1bb27: {=bool:?}, sec1bb28: {=bool:?}, sec1bb29: {=bool:?}, sec1bb30: {=bool:?}, sec1bb31: {=bool:?} }}" , self . sec1bb0 () , self . sec1bb1 () , self . sec1bb2 () , self . sec1bb3 () , self . sec1bb4 () , self . sec1bb5 () , self . sec1bb6 () , self . sec1bb7 () , self . sec1bb8 () , self . sec1bb9 () , self . sec1bb10 () , self . sec1bb11 () , self . sec1bb12 () , self . sec1bb13 () , self . sec1bb14 () , self . sec1bb15 () , self . sec1bb16 () , self . sec1bb17 () , self . sec1bb18 () , self . sec1bb19 () , self . sec1bb20 () , self . sec1bb21 () , self . sec1bb22 () , self . sec1bb23 () , self . sec1bb24 () , self . sec1bb25 () , self . sec1bb26 () , self . sec1bb27 () , self . sec1bb28 () , self . sec1bb29 () , self . sec1bb30 () , self . sec1bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr1(pub u32);
    impl Sec2bbr1 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr1 {
        #[inline(always)]
        fn default() -> Sec2bbr1 {
            Sec2bbr1(0)
        }
    }
    impl core::fmt::Debug for Sec2bbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec2bbr1")
                .field("sec2bb0", &self.sec2bb0())
                .field("sec2bb1", &self.sec2bb1())
                .field("sec2bb2", &self.sec2bb2())
                .field("sec2bb3", &self.sec2bb3())
                .field("sec2bb4", &self.sec2bb4())
                .field("sec2bb5", &self.sec2bb5())
                .field("sec2bb6", &self.sec2bb6())
                .field("sec2bb7", &self.sec2bb7())
                .field("sec2bb8", &self.sec2bb8())
                .field("sec2bb9", &self.sec2bb9())
                .field("sec2bb10", &self.sec2bb10())
                .field("sec2bb11", &self.sec2bb11())
                .field("sec2bb12", &self.sec2bb12())
                .field("sec2bb13", &self.sec2bb13())
                .field("sec2bb14", &self.sec2bb14())
                .field("sec2bb15", &self.sec2bb15())
                .field("sec2bb16", &self.sec2bb16())
                .field("sec2bb17", &self.sec2bb17())
                .field("sec2bb18", &self.sec2bb18())
                .field("sec2bb19", &self.sec2bb19())
                .field("sec2bb20", &self.sec2bb20())
                .field("sec2bb21", &self.sec2bb21())
                .field("sec2bb22", &self.sec2bb22())
                .field("sec2bb23", &self.sec2bb23())
                .field("sec2bb24", &self.sec2bb24())
                .field("sec2bb25", &self.sec2bb25())
                .field("sec2bb26", &self.sec2bb26())
                .field("sec2bb27", &self.sec2bb27())
                .field("sec2bb28", &self.sec2bb28())
                .field("sec2bb29", &self.sec2bb29())
                .field("sec2bb30", &self.sec2bb30())
                .field("sec2bb31", &self.sec2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec2bbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec2bbr1 {{ sec2bb0: {=bool:?}, sec2bb1: {=bool:?}, sec2bb2: {=bool:?}, sec2bb3: {=bool:?}, sec2bb4: {=bool:?}, sec2bb5: {=bool:?}, sec2bb6: {=bool:?}, sec2bb7: {=bool:?}, sec2bb8: {=bool:?}, sec2bb9: {=bool:?}, sec2bb10: {=bool:?}, sec2bb11: {=bool:?}, sec2bb12: {=bool:?}, sec2bb13: {=bool:?}, sec2bb14: {=bool:?}, sec2bb15: {=bool:?}, sec2bb16: {=bool:?}, sec2bb17: {=bool:?}, sec2bb18: {=bool:?}, sec2bb19: {=bool:?}, sec2bb20: {=bool:?}, sec2bb21: {=bool:?}, sec2bb22: {=bool:?}, sec2bb23: {=bool:?}, sec2bb24: {=bool:?}, sec2bb25: {=bool:?}, sec2bb26: {=bool:?}, sec2bb27: {=bool:?}, sec2bb28: {=bool:?}, sec2bb29: {=bool:?}, sec2bb30: {=bool:?}, sec2bb31: {=bool:?} }}" , self . sec2bb0 () , self . sec2bb1 () , self . sec2bb2 () , self . sec2bb3 () , self . sec2bb4 () , self . sec2bb5 () , self . sec2bb6 () , self . sec2bb7 () , self . sec2bb8 () , self . sec2bb9 () , self . sec2bb10 () , self . sec2bb11 () , self . sec2bb12 () , self . sec2bb13 () , self . sec2bb14 () , self . sec2bb15 () , self . sec2bb16 () , self . sec2bb17 () , self . sec2bb18 () , self . sec2bb19 () , self . sec2bb20 () , self . sec2bb21 () , self . sec2bb22 () , self . sec2bb23 () , self . sec2bb24 () , self . sec2bb25 () , self . sec2bb26 () , self . sec2bb27 () , self . sec2bb28 () , self . sec2bb29 () , self . sec2bb30 () , self . sec2bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr2(pub u32);
    impl Sec2bbr2 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr2 {
        #[inline(always)]
        fn default() -> Sec2bbr2 {
            Sec2bbr2(0)
        }
    }
    impl core::fmt::Debug for Sec2bbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec2bbr2")
                .field("sec2bb0", &self.sec2bb0())
                .field("sec2bb1", &self.sec2bb1())
                .field("sec2bb2", &self.sec2bb2())
                .field("sec2bb3", &self.sec2bb3())
                .field("sec2bb4", &self.sec2bb4())
                .field("sec2bb5", &self.sec2bb5())
                .field("sec2bb6", &self.sec2bb6())
                .field("sec2bb7", &self.sec2bb7())
                .field("sec2bb8", &self.sec2bb8())
                .field("sec2bb9", &self.sec2bb9())
                .field("sec2bb10", &self.sec2bb10())
                .field("sec2bb11", &self.sec2bb11())
                .field("sec2bb12", &self.sec2bb12())
                .field("sec2bb13", &self.sec2bb13())
                .field("sec2bb14", &self.sec2bb14())
                .field("sec2bb15", &self.sec2bb15())
                .field("sec2bb16", &self.sec2bb16())
                .field("sec2bb17", &self.sec2bb17())
                .field("sec2bb18", &self.sec2bb18())
                .field("sec2bb19", &self.sec2bb19())
                .field("sec2bb20", &self.sec2bb20())
                .field("sec2bb21", &self.sec2bb21())
                .field("sec2bb22", &self.sec2bb22())
                .field("sec2bb23", &self.sec2bb23())
                .field("sec2bb24", &self.sec2bb24())
                .field("sec2bb25", &self.sec2bb25())
                .field("sec2bb26", &self.sec2bb26())
                .field("sec2bb27", &self.sec2bb27())
                .field("sec2bb28", &self.sec2bb28())
                .field("sec2bb29", &self.sec2bb29())
                .field("sec2bb30", &self.sec2bb30())
                .field("sec2bb31", &self.sec2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec2bbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec2bbr2 {{ sec2bb0: {=bool:?}, sec2bb1: {=bool:?}, sec2bb2: {=bool:?}, sec2bb3: {=bool:?}, sec2bb4: {=bool:?}, sec2bb5: {=bool:?}, sec2bb6: {=bool:?}, sec2bb7: {=bool:?}, sec2bb8: {=bool:?}, sec2bb9: {=bool:?}, sec2bb10: {=bool:?}, sec2bb11: {=bool:?}, sec2bb12: {=bool:?}, sec2bb13: {=bool:?}, sec2bb14: {=bool:?}, sec2bb15: {=bool:?}, sec2bb16: {=bool:?}, sec2bb17: {=bool:?}, sec2bb18: {=bool:?}, sec2bb19: {=bool:?}, sec2bb20: {=bool:?}, sec2bb21: {=bool:?}, sec2bb22: {=bool:?}, sec2bb23: {=bool:?}, sec2bb24: {=bool:?}, sec2bb25: {=bool:?}, sec2bb26: {=bool:?}, sec2bb27: {=bool:?}, sec2bb28: {=bool:?}, sec2bb29: {=bool:?}, sec2bb30: {=bool:?}, sec2bb31: {=bool:?} }}" , self . sec2bb0 () , self . sec2bb1 () , self . sec2bb2 () , self . sec2bb3 () , self . sec2bb4 () , self . sec2bb5 () , self . sec2bb6 () , self . sec2bb7 () , self . sec2bb8 () , self . sec2bb9 () , self . sec2bb10 () , self . sec2bb11 () , self . sec2bb12 () , self . sec2bb13 () , self . sec2bb14 () , self . sec2bb15 () , self . sec2bb16 () , self . sec2bb17 () , self . sec2bb18 () , self . sec2bb19 () , self . sec2bb20 () , self . sec2bb21 () , self . sec2bb22 () , self . sec2bb23 () , self . sec2bb24 () , self . sec2bb25 () , self . sec2bb26 () , self . sec2bb27 () , self . sec2bb28 () , self . sec2bb29 () , self . sec2bb30 () , self . sec2bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr3(pub u32);
    impl Sec2bbr3 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr3 {
        #[inline(always)]
        fn default() -> Sec2bbr3 {
            Sec2bbr3(0)
        }
    }
    impl core::fmt::Debug for Sec2bbr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec2bbr3")
                .field("sec2bb0", &self.sec2bb0())
                .field("sec2bb1", &self.sec2bb1())
                .field("sec2bb2", &self.sec2bb2())
                .field("sec2bb3", &self.sec2bb3())
                .field("sec2bb4", &self.sec2bb4())
                .field("sec2bb5", &self.sec2bb5())
                .field("sec2bb6", &self.sec2bb6())
                .field("sec2bb7", &self.sec2bb7())
                .field("sec2bb8", &self.sec2bb8())
                .field("sec2bb9", &self.sec2bb9())
                .field("sec2bb10", &self.sec2bb10())
                .field("sec2bb11", &self.sec2bb11())
                .field("sec2bb12", &self.sec2bb12())
                .field("sec2bb13", &self.sec2bb13())
                .field("sec2bb14", &self.sec2bb14())
                .field("sec2bb15", &self.sec2bb15())
                .field("sec2bb16", &self.sec2bb16())
                .field("sec2bb17", &self.sec2bb17())
                .field("sec2bb18", &self.sec2bb18())
                .field("sec2bb19", &self.sec2bb19())
                .field("sec2bb20", &self.sec2bb20())
                .field("sec2bb21", &self.sec2bb21())
                .field("sec2bb22", &self.sec2bb22())
                .field("sec2bb23", &self.sec2bb23())
                .field("sec2bb24", &self.sec2bb24())
                .field("sec2bb25", &self.sec2bb25())
                .field("sec2bb26", &self.sec2bb26())
                .field("sec2bb27", &self.sec2bb27())
                .field("sec2bb28", &self.sec2bb28())
                .field("sec2bb29", &self.sec2bb29())
                .field("sec2bb30", &self.sec2bb30())
                .field("sec2bb31", &self.sec2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec2bbr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec2bbr3 {{ sec2bb0: {=bool:?}, sec2bb1: {=bool:?}, sec2bb2: {=bool:?}, sec2bb3: {=bool:?}, sec2bb4: {=bool:?}, sec2bb5: {=bool:?}, sec2bb6: {=bool:?}, sec2bb7: {=bool:?}, sec2bb8: {=bool:?}, sec2bb9: {=bool:?}, sec2bb10: {=bool:?}, sec2bb11: {=bool:?}, sec2bb12: {=bool:?}, sec2bb13: {=bool:?}, sec2bb14: {=bool:?}, sec2bb15: {=bool:?}, sec2bb16: {=bool:?}, sec2bb17: {=bool:?}, sec2bb18: {=bool:?}, sec2bb19: {=bool:?}, sec2bb20: {=bool:?}, sec2bb21: {=bool:?}, sec2bb22: {=bool:?}, sec2bb23: {=bool:?}, sec2bb24: {=bool:?}, sec2bb25: {=bool:?}, sec2bb26: {=bool:?}, sec2bb27: {=bool:?}, sec2bb28: {=bool:?}, sec2bb29: {=bool:?}, sec2bb30: {=bool:?}, sec2bb31: {=bool:?} }}" , self . sec2bb0 () , self . sec2bb1 () , self . sec2bb2 () , self . sec2bb3 () , self . sec2bb4 () , self . sec2bb5 () , self . sec2bb6 () , self . sec2bb7 () , self . sec2bb8 () , self . sec2bb9 () , self . sec2bb10 () , self . sec2bb11 () , self . sec2bb12 () , self . sec2bb13 () , self . sec2bb14 () , self . sec2bb15 () , self . sec2bb16 () , self . sec2bb17 () , self . sec2bb18 () , self . sec2bb19 () , self . sec2bb20 () , self . sec2bb21 () , self . sec2bb22 () , self . sec2bb23 () , self . sec2bb24 () , self . sec2bb25 () , self . sec2bb26 () , self . sec2bb27 () , self . sec2bb28 () , self . sec2bb29 () , self . sec2bb30 () , self . sec2bb31 ())
        }
    }
    #[doc = "FLASH secure block based bank 2 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr4(pub u32);
    impl Sec2bbr4 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr4 {
        #[inline(always)]
        fn default() -> Sec2bbr4 {
            Sec2bbr4(0)
        }
    }
    impl core::fmt::Debug for Sec2bbr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec2bbr4")
                .field("sec2bb0", &self.sec2bb0())
                .field("sec2bb1", &self.sec2bb1())
                .field("sec2bb2", &self.sec2bb2())
                .field("sec2bb3", &self.sec2bb3())
                .field("sec2bb4", &self.sec2bb4())
                .field("sec2bb5", &self.sec2bb5())
                .field("sec2bb6", &self.sec2bb6())
                .field("sec2bb7", &self.sec2bb7())
                .field("sec2bb8", &self.sec2bb8())
                .field("sec2bb9", &self.sec2bb9())
                .field("sec2bb10", &self.sec2bb10())
                .field("sec2bb11", &self.sec2bb11())
                .field("sec2bb12", &self.sec2bb12())
                .field("sec2bb13", &self.sec2bb13())
                .field("sec2bb14", &self.sec2bb14())
                .field("sec2bb15", &self.sec2bb15())
                .field("sec2bb16", &self.sec2bb16())
                .field("sec2bb17", &self.sec2bb17())
                .field("sec2bb18", &self.sec2bb18())
                .field("sec2bb19", &self.sec2bb19())
                .field("sec2bb20", &self.sec2bb20())
                .field("sec2bb21", &self.sec2bb21())
                .field("sec2bb22", &self.sec2bb22())
                .field("sec2bb23", &self.sec2bb23())
                .field("sec2bb24", &self.sec2bb24())
                .field("sec2bb25", &self.sec2bb25())
                .field("sec2bb26", &self.sec2bb26())
                .field("sec2bb27", &self.sec2bb27())
                .field("sec2bb28", &self.sec2bb28())
                .field("sec2bb29", &self.sec2bb29())
                .field("sec2bb30", &self.sec2bb30())
                .field("sec2bb31", &self.sec2bb31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec2bbr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec2bbr4 {{ sec2bb0: {=bool:?}, sec2bb1: {=bool:?}, sec2bb2: {=bool:?}, sec2bb3: {=bool:?}, sec2bb4: {=bool:?}, sec2bb5: {=bool:?}, sec2bb6: {=bool:?}, sec2bb7: {=bool:?}, sec2bb8: {=bool:?}, sec2bb9: {=bool:?}, sec2bb10: {=bool:?}, sec2bb11: {=bool:?}, sec2bb12: {=bool:?}, sec2bb13: {=bool:?}, sec2bb14: {=bool:?}, sec2bb15: {=bool:?}, sec2bb16: {=bool:?}, sec2bb17: {=bool:?}, sec2bb18: {=bool:?}, sec2bb19: {=bool:?}, sec2bb20: {=bool:?}, sec2bb21: {=bool:?}, sec2bb22: {=bool:?}, sec2bb23: {=bool:?}, sec2bb24: {=bool:?}, sec2bb25: {=bool:?}, sec2bb26: {=bool:?}, sec2bb27: {=bool:?}, sec2bb28: {=bool:?}, sec2bb29: {=bool:?}, sec2bb30: {=bool:?}, sec2bb31: {=bool:?} }}" , self . sec2bb0 () , self . sec2bb1 () , self . sec2bb2 () , self . sec2bb3 () , self . sec2bb4 () , self . sec2bb5 () , self . sec2bb6 () , self . sec2bb7 () , self . sec2bb8 () , self . sec2bb9 () , self . sec2bb10 () , self . sec2bb11 () , self . sec2bb12 () , self . sec2bb13 () , self . sec2bb14 () , self . sec2bb15 () , self . sec2bb16 () , self . sec2bb17 () , self . sec2bb18 () , self . sec2bb19 () , self . sec2bb20 () , self . sec2bb21 () , self . sec2bb22 () , self . sec2bb23 () , self . sec2bb24 () , self . sec2bb25 () , self . sec2bb26 () , self . sec2bb27 () , self . sec2bb28 () , self . sec2bb29 () , self . sec2bb30 () , self . sec2bb31 ())
        }
    }
    #[doc = "FLASH secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbootadd0r(pub u32);
    impl Secbootadd0r {
        #[doc = "Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\\[24:0\\]
option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0."]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\\[24:0\\]
option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0."]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \\[31:7\\]
The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\\[24:0\\]
= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\\[24:0\\]
= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\\[24:0\\]
= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)"]
        #[inline(always)]
        pub const fn secbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \\[31:7\\]
The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\\[24:0\\]
= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\\[24:0\\]
= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\\[24:0\\]
= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)"]
        #[inline(always)]
        pub fn set_secbootadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Secbootadd0r {
        #[inline(always)]
        fn default() -> Secbootadd0r {
            Secbootadd0r(0)
        }
    }
    impl core::fmt::Debug for Secbootadd0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbootadd0r")
                .field("boot_lock", &self.boot_lock())
                .field("secbootadd0", &self.secbootadd0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbootadd0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secbootadd0r {{ boot_lock: {=bool:?}, secbootadd0: {=u32:?} }}",
                self.boot_lock(),
                self.secbootadd0()
            )
        }
    }
    #[doc = "FLASH secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr(pub u32);
    impl Seccr {
        #[doc = "Secure programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure page number selection These bits select the page to erase: ..."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0xff;
            val as u8
        }
        #[doc = "Secure page number selection These bits select the page to erase: ..."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
        }
        #[doc = "Secure bank selection for page erase"]
        #[inline(always)]
        pub const fn bker(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank selection for page erase"]
        #[inline(always)]
        pub fn set_bker(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub const fn mer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub fn set_mer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Secure error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Secure error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Secure PCROP read error interrupt enable"]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Secure PCROP read error interrupt enable"]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Flash memory security state invert This bit inverts the Flash memory security state."]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory security state invert This bit inverts the Flash memory security state."]
        #[inline(always)]
        pub fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccr {
        #[inline(always)]
        fn default() -> Seccr {
            Seccr(0)
        }
    }
    impl core::fmt::Debug for Seccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccr")
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
                .field("rderrie", &self.rderrie())
                .field("inv", &self.inv())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccr {{ pg: {=bool:?}, per: {=bool:?}, mer1: {=bool:?}, pnb: {=u8:?}, bker: {=bool:?}, bwr: {=bool:?}, mer2: {=bool:?}, strt: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, rderrie: {=bool:?}, inv: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . per () , self . mer1 () , self . pnb () , self . bker () , self . bwr () , self . mer2 () , self . strt () , self . eopie () , self . errie () , self . rderrie () , self . inv () , self . lock ())
        }
    }
    #[doc = "FLASH secure HDP control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sechdpcr(pub u32);
    impl Sechdpcr {
        #[doc = "HDP1 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub const fn hdp1_accdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HDP1 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub fn set_hdp1_accdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HDP2 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub const fn hdp2_accdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HDP2 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub fn set_hdp2_accdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sechdpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sechdpcr {{ hdp1_accdis: {=bool:?}, hdp2_accdis: {=bool:?} }}",
                self.hdp1_accdis(),
                self.hdp2_accdis()
            )
        }
    }
    #[doc = "FLASH secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure busy This bit indicates that a Flash memory secure or non-secure operation is in progress. This is set on the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure busy This bit indicates that a Flash memory secure or non-secure operation is in progress. This is set on the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
    impl core::fmt::Debug for Secsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secsr")
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
    impl defmt::Format for Secsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secsr {{ eop: {=bool:?}, operr: {=bool:?}, progerr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, pgserr: {=bool:?}, bsy: {=bool:?}, wdw: {=bool:?} }}" , self . eop () , self . operr () , self . progerr () , self . wrperr () , self . pgaerr () , self . sizerr () , self . pgserr () , self . bsy () , self . wdw ())
        }
    }
    #[doc = "FLASH secure watermark1 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r1(pub u32);
    impl Secwm1r1 {
        #[doc = "Start page of first secure area This field contains the first page of the secure area in bank 1."]
        #[inline(always)]
        pub const fn secwm1_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of first secure area This field contains the first page of the secure area in bank 1."]
        #[inline(always)]
        pub fn set_secwm1_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of first secure area This field contains the last page of the secure area in bank 1."]
        #[inline(always)]
        pub const fn secwm1_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of first secure area This field contains the last page of the secure area in bank 1."]
        #[inline(always)]
        pub fn set_secwm1_pend(&mut self, val: u8) {
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
                .field("secwm1_pstrt", &self.secwm1_pstrt())
                .field("secwm1_pend", &self.secwm1_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm1r1 {{ secwm1_pstrt: {=u8:?}, secwm1_pend: {=u8:?} }}",
                self.secwm1_pstrt(),
                self.secwm1_pend()
            )
        }
    }
    #[doc = "FLASH secure watermark1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r2(pub u32);
    impl Secwm1r2 {
        #[doc = "End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
        #[inline(always)]
        pub const fn hdp1_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
        #[inline(always)]
        pub fn set_hdp1_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Hide protection first area enable"]
        #[inline(always)]
        pub const fn hdp1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Hide protection first area enable"]
        #[inline(always)]
        pub fn set_hdp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("hdp1_pend", &self.hdp1_pend())
                .field("hdp1en", &self.hdp1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm1r2 {{ hdp1_pend: {=u8:?}, hdp1en: {=bool:?} }}",
                self.hdp1_pend(),
                self.hdp1en()
            )
        }
    }
    #[doc = "FLASH secure watermark2 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r1(pub u32);
    impl Secwm2r1 {
        #[doc = "Start page of second secure area This field contains the first page of the secure area in bank 2."]
        #[inline(always)]
        pub const fn secwm2_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of second secure area This field contains the first page of the secure area in bank 2."]
        #[inline(always)]
        pub fn set_secwm2_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of second secure area This field contains the last page of the secure area in bank 2."]
        #[inline(always)]
        pub const fn secwm2_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of second secure area This field contains the last page of the secure area in bank 2."]
        #[inline(always)]
        pub fn set_secwm2_pend(&mut self, val: u8) {
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
                .field("secwm2_pstrt", &self.secwm2_pstrt())
                .field("secwm2_pend", &self.secwm2_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm2r1 {{ secwm2_pstrt: {=u8:?}, secwm2_pend: {=u8:?} }}",
                self.secwm2_pstrt(),
                self.secwm2_pend()
            )
        }
    }
    #[doc = "FLASH secure watermark2 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r2(pub u32);
    impl Secwm2r2 {
        #[doc = "End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
        #[inline(always)]
        pub const fn hdp2_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
        #[inline(always)]
        pub fn set_hdp2_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Hide protection second area enable"]
        #[inline(always)]
        pub const fn hdp2en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Hide protection second area enable"]
        #[inline(always)]
        pub fn set_hdp2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("hdp2_pend", &self.hdp2_pend())
                .field("hdp2en", &self.hdp2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm2r2 {{ hdp2_pend: {=u8:?}, hdp2en: {=bool:?} }}",
                self.hdp2_pend(),
                self.hdp2en()
            )
        }
    }
    #[doc = "FLASH WRP1 area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
        #[inline(always)]
        pub const fn wrp1a_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
        #[inline(always)]
        pub fn set_wrp1a_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
        #[inline(always)]
        pub const fn wrp1a_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
        #[inline(always)]
        pub fn set_wrp1a_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 1 WPR first area A unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 WPR first area A unlock"]
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
                .field("wrp1a_pstrt", &self.wrp1a_pstrt())
                .field("wrp1a_pend", &self.wrp1a_pend())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1ar {{ wrp1a_pstrt: {=u8:?}, wrp1a_pend: {=u8:?}, unlock: {=bool:?} }}",
                self.wrp1a_pstrt(),
                self.wrp1a_pend(),
                self.unlock()
            )
        }
    }
    #[doc = "FLASH WRP1 area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1."]
        #[inline(always)]
        pub const fn wrp1b_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1."]
        #[inline(always)]
        pub fn set_wrp1b_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1."]
        #[inline(always)]
        pub const fn wrp1b_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1."]
        #[inline(always)]
        pub fn set_wrp1b_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 1 WPR second area B unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 WPR second area B unlock"]
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
                .field("wrp1b_pstrt", &self.wrp1b_pstrt())
                .field("wrp1b_pend", &self.wrp1b_pend())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1br {{ wrp1b_pstrt: {=u8:?}, wrp1b_pend: {=u8:?}, unlock: {=bool:?} }}",
                self.wrp1b_pstrt(),
                self.wrp1b_pend(),
                self.unlock()
            )
        }
    }
    #[doc = "FLASH WPR2 area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2ar(pub u32);
    impl Wrp2ar {
        #[doc = "Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
        #[inline(always)]
        pub const fn wrp2a_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
        #[inline(always)]
        pub fn set_wrp2a_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
        #[inline(always)]
        pub const fn wrp2a_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
        #[inline(always)]
        pub fn set_wrp2a_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 2 WPR first area A unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 WPR first area A unlock"]
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
                .field("wrp2a_pstrt", &self.wrp2a_pstrt())
                .field("wrp2a_pend", &self.wrp2a_pend())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp2ar {{ wrp2a_pstrt: {=u8:?}, wrp2a_pend: {=u8:?}, unlock: {=bool:?} }}",
                self.wrp2a_pstrt(),
                self.wrp2a_pend(),
                self.unlock()
            )
        }
    }
    #[doc = "FLASH WPR2 area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2br(pub u32);
    impl Wrp2br {
        #[doc = "Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
        #[inline(always)]
        pub const fn wrp2b_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
        #[inline(always)]
        pub fn set_wrp2b_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
        #[inline(always)]
        pub const fn wrp2b_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
        #[inline(always)]
        pub fn set_wrp2b_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 2 WPR second area B unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 WPR second area B unlock"]
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
                .field("wrp2b_pstrt", &self.wrp2b_pstrt())
                .field("wrp2b_pend", &self.wrp2b_pend())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp2br {{ wrp2b_pstrt: {=u8:?}, wrp2b_pend: {=u8:?}, unlock: {=bool:?} }}",
                self.wrp2b_pstrt(),
                self.wrp2b_pend(),
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
        #[doc = "BOR level 0 (reset level threshold around 1.7V)"]
        LEVEL0 = 0x0,
        #[doc = "BOR level 1 (reset level threshold around 2.0V)"]
        LEVEL1 = 0x01,
        #[doc = "BOR level 2 (reset level threshold around 2.2V)"]
        LEVEL2 = 0x02,
        #[doc = "BOR level 3 (reset level threshold around 2.5V)"]
        LEVEL3 = 0x03,
        #[doc = "BOR level 4 (reset level threshold around 2.8V)"]
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
        #[doc = "No Flash operation interrupted by previous reset"]
        NO_FLASH_INT = 0x0,
        #[doc = "Single write operation interrupted"]
        SINGLE_WR_INT = 0x01,
        #[doc = "Burst write operation interrupted"]
        BURST_WR_INT = 0x02,
        #[doc = "Page erase operation interrupted"]
        PG_ERASE_INT = 0x03,
        #[doc = "Bank erase operation interrupted"]
        BANK_ERASE_INT = 0x04,
        #[doc = "Mass erase operation interrupted"]
        MASS_ERASE_INT = 0x05,
        #[doc = "Option change operation interrupted"]
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
        #[doc = "Level 0.5 (readout protection not active, only non-secure debug access is possible). Only available when TrustZone is active (TZEN=1)"]
        pub const B_0X55: Self = Self(0x55);
        #[doc = "Level 0 (readout protection not active)"]
        pub const B_0X_AA: Self = Self(0xaa);
        #[doc = "Level 2 (chip readout protection active)"]
        pub const B_0X_CC: Self = Self(0xcc);
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
                0x55 => f.write_str("B_0X55"),
                0xaa => f.write_str("B_0X_AA"),
                0xcc => f.write_str("B_0X_CC"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x55 => defmt::write!(f, "B_0X55"),
                0xaa => defmt::write!(f, "B_0X_AA"),
                0xcc => defmt::write!(f, "B_0X_CC"),
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
