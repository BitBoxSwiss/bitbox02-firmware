#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "VREFBUF control and status register"]
    #[inline(always)]
    pub const fn vrefbuf_csr(self) -> crate::common::Reg<regs::VrefbufCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "VREFBUF calibration control register"]
    #[inline(always)]
    pub const fn vrefbuf_ccr(self) -> crate::common::Reg<regs::VrefbufCcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "interrupt line 0 status register"]
    #[inline(always)]
    pub const fn itline0(self) -> crate::common::Reg<regs::Itline0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "interrupt line 1 status register"]
    #[inline(always)]
    pub const fn itline1(self) -> crate::common::Reg<regs::Itline1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "interrupt line 2 status register"]
    #[inline(always)]
    pub const fn itline2(self) -> crate::common::Reg<regs::Itline2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "interrupt line 3 status register"]
    #[inline(always)]
    pub const fn itline3(self) -> crate::common::Reg<regs::Itline3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "interrupt line 4 status register"]
    #[inline(always)]
    pub const fn itline4(self) -> crate::common::Reg<regs::Itline4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "interrupt line 5 status register"]
    #[inline(always)]
    pub const fn itline5(self) -> crate::common::Reg<regs::Itline5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "interrupt line 6 status register"]
    #[inline(always)]
    pub const fn itline6(self) -> crate::common::Reg<regs::Itline6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "interrupt line 7 status register"]
    #[inline(always)]
    pub const fn itline7(self) -> crate::common::Reg<regs::Itline7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "interrupt line 8 status register"]
    #[inline(always)]
    pub const fn itline8(self) -> crate::common::Reg<regs::Itline8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "interrupt line 9 status register"]
    #[inline(always)]
    pub const fn itline9(self) -> crate::common::Reg<regs::Itline9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "interrupt line 10 status register"]
    #[inline(always)]
    pub const fn itline10(self) -> crate::common::Reg<regs::Itline10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "interrupt line 11 status register"]
    #[inline(always)]
    pub const fn itline11(self) -> crate::common::Reg<regs::Itline11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "interrupt line 12 status register"]
    #[inline(always)]
    pub const fn itline12(self) -> crate::common::Reg<regs::Itline12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "interrupt line 13 status register"]
    #[inline(always)]
    pub const fn itline13(self) -> crate::common::Reg<regs::Itline13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "interrupt line 14 status register"]
    #[inline(always)]
    pub const fn itline14(self) -> crate::common::Reg<regs::Itline14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "interrupt line 15 status register"]
    #[inline(always)]
    pub const fn itline15(self) -> crate::common::Reg<regs::Itline15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "interrupt line 16 status register"]
    #[inline(always)]
    pub const fn itline16(self) -> crate::common::Reg<regs::Itline16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "interrupt line 17 status register"]
    #[inline(always)]
    pub const fn itline17(self) -> crate::common::Reg<regs::Itline17, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "interrupt line 18 status register"]
    #[inline(always)]
    pub const fn itline18(self) -> crate::common::Reg<regs::Itline18, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "interrupt line 19 status register"]
    #[inline(always)]
    pub const fn itline19(self) -> crate::common::Reg<regs::Itline19, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "interrupt line 20 status register"]
    #[inline(always)]
    pub const fn itline20(self) -> crate::common::Reg<regs::Itline20, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "interrupt line 21 status register"]
    #[inline(always)]
    pub const fn itline21(self) -> crate::common::Reg<regs::Itline21, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "interrupt line 22 status register"]
    #[inline(always)]
    pub const fn itline22(self) -> crate::common::Reg<regs::Itline22, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "interrupt line 23 status register"]
    #[inline(always)]
    pub const fn itline23(self) -> crate::common::Reg<regs::Itline23, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "interrupt line 24 status register"]
    #[inline(always)]
    pub const fn itline24(self) -> crate::common::Reg<regs::Itline24, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "interrupt line 25 status register"]
    #[inline(always)]
    pub const fn itline25(self) -> crate::common::Reg<regs::Itline25, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "interrupt line 26 status register"]
    #[inline(always)]
    pub const fn itline26(self) -> crate::common::Reg<regs::Itline26, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "interrupt line 27 status register"]
    #[inline(always)]
    pub const fn itline27(self) -> crate::common::Reg<regs::Itline27, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "interrupt line 28 status register"]
    #[inline(always)]
    pub const fn itline28(self) -> crate::common::Reg<regs::Itline28, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "interrupt line 29 status register"]
    #[inline(always)]
    pub const fn itline29(self) -> crate::common::Reg<regs::Itline29, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "interrupt line 30 status register"]
    #[inline(always)]
    pub const fn itline30(self) -> crate::common::Reg<regs::Itline30, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "interrupt line 31 status register"]
    #[inline(always)]
    pub const fn itline31(self) -> crate::common::Reg<regs::Itline31, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Memory mapping selection bits. This bitfield controlled by software selects the memory internally mapped at the address 0x0000_0000. Its reset value is determined by the boot mode configuration. Refer to Reference Manual section 2.5 for more details."]
        #[inline(always)]
        pub const fn mem_mode(&self) -> super::vals::MemMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::MemMode::from_bits(val as u8)
        }
        #[doc = "Memory mapping selection bits. This bitfield controlled by software selects the memory internally mapped at the address 0x0000_0000. Its reset value is determined by the boot mode configuration. Refer to Reference Manual section 2.5 for more details."]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: super::vals::MemMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
        #[inline(always)]
        pub const fn pa11_rmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
        #[inline(always)]
        pub fn set_pa11_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
        #[inline(always)]
        pub const fn pa12_rmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
        #[inline(always)]
        pub fn set_pa12_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IR output polarity selection"]
        #[inline(always)]
        pub const fn ir_pol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IR output polarity selection"]
        #[inline(always)]
        pub fn set_ir_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IR Modulation Envelope signal selection."]
        #[inline(always)]
        pub const fn ir_mod(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "IR Modulation Envelope signal selection."]
        #[inline(always)]
        pub fn set_ir_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "I/O analog switch voltage booster enable"]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable"]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Strobe signal bit for UCPD1"]
        #[inline(always)]
        pub const fn ucpd1_strobe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe signal bit for UCPD1"]
        #[inline(always)]
        pub fn set_ucpd1_strobe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Strobe signal bit for UCPD2"]
        #[inline(always)]
        pub const fn ucpd2_strobe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe signal bit for UCPD2"]
        #[inline(always)]
        pub fn set_ucpd2_strobe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub const fn i2c_pbx_fmp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub fn set_i2c_pbx_fmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "FM+ driving capability activation for I2C1"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "FM+ driving capability activation for I2C1"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "FM+ driving capability activation for I2C2"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "FM+ driving capability activation for I2C2"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub const fn i2c_pax_fmp(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub fn set_i2c_pax_fmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("mem_mode", &self.mem_mode())
                .field("pa11_rmp", &self.pa11_rmp())
                .field("pa12_rmp", &self.pa12_rmp())
                .field("ir_pol", &self.ir_pol())
                .field("ir_mod", &self.ir_mod())
                .field("boosten", &self.boosten())
                .field("ucpd1_strobe", &self.ucpd1_strobe())
                .field("ucpd2_strobe", &self.ucpd2_strobe())
                .field("i2c_pbx_fmp", &self.i2c_pbx_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("i2c_pax_fmp", &self.i2c_pax_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ mem_mode: {:?}, pa11_rmp: {=bool:?}, pa12_rmp: {=bool:?}, ir_pol: {=bool:?}, ir_mod: {=u8:?}, boosten: {=bool:?}, ucpd1_strobe: {=bool:?}, ucpd2_strobe: {=bool:?}, i2c_pbx_fmp: {=u8:?}, i2c1_fmp: {=bool:?}, i2c2_fmp: {=bool:?}, i2c_pax_fmp: {=u8:?} }}" , self . mem_mode () , self . pa11_rmp () , self . pa12_rmp () , self . ir_pol () , self . ir_mod () , self . boosten () , self . ucpd1_strobe () , self . ucpd2_strobe () , self . i2c_pbx_fmp () , self . i2c1_fmp () , self . i2c2_fmp () , self . i2c_pax_fmp ())
        }
    }
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex-M0+ LOCKUP bit enable bit"]
        #[inline(always)]
        pub const fn lockup_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M0+ LOCKUP bit enable bit"]
        #[inline(always)]
        pub fn set_lockup_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM parity lock bit"]
        #[inline(always)]
        pub const fn sram_parity_lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM parity lock bit"]
        #[inline(always)]
        pub fn set_sram_parity_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub const fn pvd_lock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub fn set_pvd_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC error lock bit"]
        #[inline(always)]
        pub const fn ecc_lock(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC error lock bit"]
        #[inline(always)]
        pub fn set_ecc_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM parity error flag"]
        #[inline(always)]
        pub const fn sram_pef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM parity error flag"]
        #[inline(always)]
        pub fn set_sram_pef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PA1_CDEN"]
        #[inline(always)]
        pub const fn pa1_cden(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PA1_CDEN"]
        #[inline(always)]
        pub fn set_pa1_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PA3_CDEN"]
        #[inline(always)]
        pub const fn pa3_cden(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_CDEN"]
        #[inline(always)]
        pub fn set_pa3_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PA5_CDEN"]
        #[inline(always)]
        pub const fn pa5_cden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PA5_CDEN"]
        #[inline(always)]
        pub fn set_pa5_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PA6_CDEN"]
        #[inline(always)]
        pub const fn pa6_cden(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PA6_CDEN"]
        #[inline(always)]
        pub fn set_pa6_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PA13_CDEN"]
        #[inline(always)]
        pub const fn pa13_cden(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PA13_CDEN"]
        #[inline(always)]
        pub fn set_pa13_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PB0_CDEN"]
        #[inline(always)]
        pub const fn pb0_cden(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PB0_CDEN"]
        #[inline(always)]
        pub fn set_pb0_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PB1_CDEN"]
        #[inline(always)]
        pub const fn pb1_cden(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB1_CDEN"]
        #[inline(always)]
        pub fn set_pb1_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PB2_CDEN"]
        #[inline(always)]
        pub const fn pb2_cden(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PB2_CDEN"]
        #[inline(always)]
        pub fn set_pb2_cden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("lockup_lock", &self.lockup_lock())
                .field("sram_parity_lock", &self.sram_parity_lock())
                .field("pvd_lock", &self.pvd_lock())
                .field("ecc_lock", &self.ecc_lock())
                .field("sram_pef", &self.sram_pef())
                .field("pa1_cden", &self.pa1_cden())
                .field("pa3_cden", &self.pa3_cden())
                .field("pa5_cden", &self.pa5_cden())
                .field("pa6_cden", &self.pa6_cden())
                .field("pa13_cden", &self.pa13_cden())
                .field("pb0_cden", &self.pb0_cden())
                .field("pb1_cden", &self.pb1_cden())
                .field("pb2_cden", &self.pb2_cden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr2 {{ lockup_lock: {=bool:?}, sram_parity_lock: {=bool:?}, pvd_lock: {=bool:?}, ecc_lock: {=bool:?}, sram_pef: {=bool:?}, pa1_cden: {=bool:?}, pa3_cden: {=bool:?}, pa5_cden: {=bool:?}, pa6_cden: {=bool:?}, pa13_cden: {=bool:?}, pb0_cden: {=bool:?}, pb1_cden: {=bool:?}, pb2_cden: {=bool:?} }}" , self . lockup_lock () , self . sram_parity_lock () , self . pvd_lock () , self . ecc_lock () , self . sram_pef () , self . pa1_cden () , self . pa3_cden () , self . pa5_cden () , self . pa6_cden () , self . pa13_cden () , self . pb0_cden () , self . pb1_cden () , self . pb2_cden ())
        }
    }
    #[doc = "interrupt line 0 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline0(pub u32);
    impl Itline0 {
        #[doc = "Window watchdog interrupt pending flag"]
        #[inline(always)]
        pub const fn wwdg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog interrupt pending flag"]
        #[inline(always)]
        pub fn set_wwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline0 {
        #[inline(always)]
        fn default() -> Itline0 {
            Itline0(0)
        }
    }
    impl core::fmt::Debug for Itline0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline0").field("wwdg", &self.wwdg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline0 {{ wwdg: {=bool:?} }}", self.wwdg())
        }
    }
    #[doc = "interrupt line 1 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline1(pub u32);
    impl Itline1 {
        #[doc = "PVD supply monitoring interrupt request pending (EXTI line 16)."]
        #[inline(always)]
        pub const fn pvdout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PVD supply monitoring interrupt request pending (EXTI line 16)."]
        #[inline(always)]
        pub fn set_pvdout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline1 {
        #[inline(always)]
        fn default() -> Itline1 {
            Itline1(0)
        }
    }
    impl core::fmt::Debug for Itline1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline1").field("pvdout", &self.pvdout()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline1 {{ pvdout: {=bool:?} }}", self.pvdout())
        }
    }
    #[doc = "interrupt line 10 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline10(pub u32);
    impl Itline10 {
        #[doc = "DMA1_CH1"]
        #[inline(always)]
        pub const fn dma1_ch2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH1"]
        #[inline(always)]
        pub fn set_dma1_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1_CH3"]
        #[inline(always)]
        pub const fn dma1_ch3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH3"]
        #[inline(always)]
        pub fn set_dma1_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline10 {
        #[inline(always)]
        fn default() -> Itline10 {
            Itline10(0)
        }
    }
    impl core::fmt::Debug for Itline10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline10")
                .field("dma1_ch2", &self.dma1_ch2())
                .field("dma1_ch3", &self.dma1_ch3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline10 {{ dma1_ch2: {=bool:?}, dma1_ch3: {=bool:?} }}",
                self.dma1_ch2(),
                self.dma1_ch3()
            )
        }
    }
    #[doc = "interrupt line 11 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline11(pub u32);
    impl Itline11 {
        #[doc = "DMAMUX"]
        #[inline(always)]
        pub const fn dmamux(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX"]
        #[inline(always)]
        pub fn set_dmamux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1_CH4"]
        #[inline(always)]
        pub const fn dma1_ch4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH4"]
        #[inline(always)]
        pub fn set_dma1_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA1_CH5"]
        #[inline(always)]
        pub const fn dma1_ch5(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH5"]
        #[inline(always)]
        pub fn set_dma1_ch5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA1_CH6"]
        #[inline(always)]
        pub const fn dma1_ch6(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH6"]
        #[inline(always)]
        pub fn set_dma1_ch6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA1_CH7"]
        #[inline(always)]
        pub const fn dma1_ch7(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH7"]
        #[inline(always)]
        pub fn set_dma1_ch7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Itline11 {
        #[inline(always)]
        fn default() -> Itline11 {
            Itline11(0)
        }
    }
    impl core::fmt::Debug for Itline11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline11")
                .field("dmamux", &self.dmamux())
                .field("dma1_ch4", &self.dma1_ch4())
                .field("dma1_ch5", &self.dma1_ch5())
                .field("dma1_ch6", &self.dma1_ch6())
                .field("dma1_ch7", &self.dma1_ch7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline11 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Itline11 {{ dmamux: {=bool:?}, dma1_ch4: {=bool:?}, dma1_ch5: {=bool:?}, dma1_ch6: {=bool:?}, dma1_ch7: {=bool:?} }}" , self . dmamux () , self . dma1_ch4 () , self . dma1_ch5 () , self . dma1_ch6 () , self . dma1_ch7 ())
        }
    }
    #[doc = "interrupt line 12 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline12(pub u32);
    impl Itline12 {
        #[doc = "ADC"]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC"]
        #[inline(always)]
        pub fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "COMP1"]
        #[inline(always)]
        pub const fn comp1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1"]
        #[inline(always)]
        pub fn set_comp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "COMP2"]
        #[inline(always)]
        pub const fn comp2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2"]
        #[inline(always)]
        pub fn set_comp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Itline12 {
        #[inline(always)]
        fn default() -> Itline12 {
            Itline12(0)
        }
    }
    impl core::fmt::Debug for Itline12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline12")
                .field("adc", &self.adc())
                .field("comp1", &self.comp1())
                .field("comp2", &self.comp2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline12 {{ adc: {=bool:?}, comp1: {=bool:?}, comp2: {=bool:?} }}",
                self.adc(),
                self.comp1(),
                self.comp2()
            )
        }
    }
    #[doc = "interrupt line 13 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline13(pub u32);
    impl Itline13 {
        #[doc = "TIM1_CCU"]
        #[inline(always)]
        pub const fn tim1_ccu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1_CCU"]
        #[inline(always)]
        pub fn set_tim1_ccu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM1_TRG"]
        #[inline(always)]
        pub const fn tim1_trg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1_TRG"]
        #[inline(always)]
        pub fn set_tim1_trg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM1_UPD"]
        #[inline(always)]
        pub const fn tim1_upd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1_UPD"]
        #[inline(always)]
        pub fn set_tim1_upd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM1_BRK"]
        #[inline(always)]
        pub const fn tim1_brk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1_BRK"]
        #[inline(always)]
        pub fn set_tim1_brk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Itline13 {
        #[inline(always)]
        fn default() -> Itline13 {
            Itline13(0)
        }
    }
    impl core::fmt::Debug for Itline13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline13")
                .field("tim1_ccu", &self.tim1_ccu())
                .field("tim1_trg", &self.tim1_trg())
                .field("tim1_upd", &self.tim1_upd())
                .field("tim1_brk", &self.tim1_brk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline13 {{ tim1_ccu: {=bool:?}, tim1_trg: {=bool:?}, tim1_upd: {=bool:?}, tim1_brk: {=bool:?} }}",
                self.tim1_ccu(),
                self.tim1_trg(),
                self.tim1_upd(),
                self.tim1_brk()
            )
        }
    }
    #[doc = "interrupt line 14 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline14(pub u32);
    impl Itline14 {
        #[doc = "TIM1_CC"]
        #[inline(always)]
        pub const fn tim1_cc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1_CC"]
        #[inline(always)]
        pub fn set_tim1_cc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline14 {
        #[inline(always)]
        fn default() -> Itline14 {
            Itline14(0)
        }
    }
    impl core::fmt::Debug for Itline14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline14").field("tim1_cc", &self.tim1_cc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline14 {{ tim1_cc: {=bool:?} }}", self.tim1_cc())
        }
    }
    #[doc = "interrupt line 15 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline15(pub u32);
    impl Itline15 {
        #[doc = "TIM2"]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2"]
        #[inline(always)]
        pub fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline15 {
        #[inline(always)]
        fn default() -> Itline15 {
            Itline15(0)
        }
    }
    impl core::fmt::Debug for Itline15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline15").field("tim2", &self.tim2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline15 {{ tim2: {=bool:?} }}", self.tim2())
        }
    }
    #[doc = "interrupt line 16 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline16(pub u32);
    impl Itline16 {
        #[doc = "TIM3"]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3"]
        #[inline(always)]
        pub fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline16 {
        #[inline(always)]
        fn default() -> Itline16 {
            Itline16(0)
        }
    }
    impl core::fmt::Debug for Itline16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline16").field("tim3", &self.tim3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline16 {{ tim3: {=bool:?} }}", self.tim3())
        }
    }
    #[doc = "interrupt line 17 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline17(pub u32);
    impl Itline17 {
        #[doc = "TIM6"]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6"]
        #[inline(always)]
        pub fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC"]
        #[inline(always)]
        pub const fn dac(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC"]
        #[inline(always)]
        pub fn set_dac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM1"]
        #[inline(always)]
        pub const fn lptim1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1"]
        #[inline(always)]
        pub fn set_lptim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Itline17 {
        #[inline(always)]
        fn default() -> Itline17 {
            Itline17(0)
        }
    }
    impl core::fmt::Debug for Itline17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline17")
                .field("tim6", &self.tim6())
                .field("dac", &self.dac())
                .field("lptim1", &self.lptim1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline17 {{ tim6: {=bool:?}, dac: {=bool:?}, lptim1: {=bool:?} }}",
                self.tim6(),
                self.dac(),
                self.lptim1()
            )
        }
    }
    #[doc = "interrupt line 18 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline18(pub u32);
    impl Itline18 {
        #[doc = "TIM7"]
        #[inline(always)]
        pub const fn tim7(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7"]
        #[inline(always)]
        pub fn set_tim7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPTIM2"]
        #[inline(always)]
        pub const fn lptim2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2"]
        #[inline(always)]
        pub fn set_lptim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline18 {
        #[inline(always)]
        fn default() -> Itline18 {
            Itline18(0)
        }
    }
    impl core::fmt::Debug for Itline18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline18")
                .field("tim7", &self.tim7())
                .field("lptim2", &self.lptim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline18 {{ tim7: {=bool:?}, lptim2: {=bool:?} }}",
                self.tim7(),
                self.lptim2()
            )
        }
    }
    #[doc = "interrupt line 19 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline19(pub u32);
    impl Itline19 {
        #[doc = "TIM14"]
        #[inline(always)]
        pub const fn tim14(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14"]
        #[inline(always)]
        pub fn set_tim14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline19 {
        #[inline(always)]
        fn default() -> Itline19 {
            Itline19(0)
        }
    }
    impl core::fmt::Debug for Itline19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline19").field("tim14", &self.tim14()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline19 {{ tim14: {=bool:?} }}", self.tim14())
        }
    }
    #[doc = "interrupt line 2 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline2(pub u32);
    impl Itline2 {
        #[doc = "TAMP"]
        #[inline(always)]
        pub const fn tamp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP"]
        #[inline(always)]
        pub fn set_tamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTC"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTC"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline2 {
        #[inline(always)]
        fn default() -> Itline2 {
            Itline2(0)
        }
    }
    impl core::fmt::Debug for Itline2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline2")
                .field("tamp", &self.tamp())
                .field("rtc", &self.rtc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline2 {{ tamp: {=bool:?}, rtc: {=bool:?} }}",
                self.tamp(),
                self.rtc()
            )
        }
    }
    #[doc = "interrupt line 20 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline20(pub u32);
    impl Itline20 {
        #[doc = "TIM15"]
        #[inline(always)]
        pub const fn tim15(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15"]
        #[inline(always)]
        pub fn set_tim15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline20 {
        #[inline(always)]
        fn default() -> Itline20 {
            Itline20(0)
        }
    }
    impl core::fmt::Debug for Itline20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline20").field("tim15", &self.tim15()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline20 {{ tim15: {=bool:?} }}", self.tim15())
        }
    }
    #[doc = "interrupt line 21 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline21(pub u32);
    impl Itline21 {
        #[doc = "TIM16"]
        #[inline(always)]
        pub const fn tim16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16"]
        #[inline(always)]
        pub fn set_tim16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline21 {
        #[inline(always)]
        fn default() -> Itline21 {
            Itline21(0)
        }
    }
    impl core::fmt::Debug for Itline21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline21").field("tim16", &self.tim16()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline21 {{ tim16: {=bool:?} }}", self.tim16())
        }
    }
    #[doc = "interrupt line 22 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline22(pub u32);
    impl Itline22 {
        #[doc = "TIM17"]
        #[inline(always)]
        pub const fn tim17(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17"]
        #[inline(always)]
        pub fn set_tim17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline22 {
        #[inline(always)]
        fn default() -> Itline22 {
            Itline22(0)
        }
    }
    impl core::fmt::Debug for Itline22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline22").field("tim17", &self.tim17()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline22 {{ tim17: {=bool:?} }}", self.tim17())
        }
    }
    #[doc = "interrupt line 23 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline23(pub u32);
    impl Itline23 {
        #[doc = "I2C1"]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1"]
        #[inline(always)]
        pub fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline23 {
        #[inline(always)]
        fn default() -> Itline23 {
            Itline23(0)
        }
    }
    impl core::fmt::Debug for Itline23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline23").field("i2c1", &self.i2c1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline23 {{ i2c1: {=bool:?} }}", self.i2c1())
        }
    }
    #[doc = "interrupt line 24 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline24(pub u32);
    impl Itline24 {
        #[doc = "I2C2"]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2"]
        #[inline(always)]
        pub fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline24 {
        #[inline(always)]
        fn default() -> Itline24 {
            Itline24(0)
        }
    }
    impl core::fmt::Debug for Itline24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline24").field("i2c2", &self.i2c2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline24 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline24 {{ i2c2: {=bool:?} }}", self.i2c2())
        }
    }
    #[doc = "interrupt line 25 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline25(pub u32);
    impl Itline25 {
        #[doc = "SPI1"]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1"]
        #[inline(always)]
        pub fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline25 {
        #[inline(always)]
        fn default() -> Itline25 {
            Itline25(0)
        }
    }
    impl core::fmt::Debug for Itline25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline25").field("spi1", &self.spi1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline25 {{ spi1: {=bool:?} }}", self.spi1())
        }
    }
    #[doc = "interrupt line 26 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline26(pub u32);
    impl Itline26 {
        #[doc = "SPI2"]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2"]
        #[inline(always)]
        pub fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline26 {
        #[inline(always)]
        fn default() -> Itline26 {
            Itline26(0)
        }
    }
    impl core::fmt::Debug for Itline26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline26").field("spi2", &self.spi2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline26 {{ spi2: {=bool:?} }}", self.spi2())
        }
    }
    #[doc = "interrupt line 27 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline27(pub u32);
    impl Itline27 {
        #[doc = "USART1"]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART1"]
        #[inline(always)]
        pub fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline27 {
        #[inline(always)]
        fn default() -> Itline27 {
            Itline27(0)
        }
    }
    impl core::fmt::Debug for Itline27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline27").field("usart1", &self.usart1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline27 {{ usart1: {=bool:?} }}", self.usart1())
        }
    }
    #[doc = "interrupt line 28 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline28(pub u32);
    impl Itline28 {
        #[doc = "USART2"]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART2"]
        #[inline(always)]
        pub fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline28 {
        #[inline(always)]
        fn default() -> Itline28 {
            Itline28(0)
        }
    }
    impl core::fmt::Debug for Itline28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline28").field("usart2", &self.usart2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline28 {{ usart2: {=bool:?} }}", self.usart2())
        }
    }
    #[doc = "interrupt line 29 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline29(pub u32);
    impl Itline29 {
        #[inline(always)]
        pub const fn usart3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_usart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn usart4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_usart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn usart5(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_usart5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn usart6(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_usart6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Itline29 {
        #[inline(always)]
        fn default() -> Itline29 {
            Itline29(0)
        }
    }
    impl core::fmt::Debug for Itline29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline29")
                .field("usart3", &self.usart3())
                .field("usart4", &self.usart4())
                .field("usart5", &self.usart5())
                .field("usart6", &self.usart6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline29 {{ usart3: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, usart6: {=bool:?} }}",
                self.usart3(),
                self.usart4(),
                self.usart5(),
                self.usart6()
            )
        }
    }
    #[doc = "interrupt line 3 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline3(pub u32);
    impl Itline3 {
        #[doc = "FLASH_ITF"]
        #[inline(always)]
        pub const fn flash_itf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_ITF"]
        #[inline(always)]
        pub fn set_flash_itf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLASH_ECC"]
        #[inline(always)]
        pub const fn flash_ecc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_ECC"]
        #[inline(always)]
        pub fn set_flash_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline3 {
        #[inline(always)]
        fn default() -> Itline3 {
            Itline3(0)
        }
    }
    impl core::fmt::Debug for Itline3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline3")
                .field("flash_itf", &self.flash_itf())
                .field("flash_ecc", &self.flash_ecc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline3 {{ flash_itf: {=bool:?}, flash_ecc: {=bool:?} }}",
                self.flash_itf(),
                self.flash_ecc()
            )
        }
    }
    #[doc = "interrupt line 30 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline30(pub u32);
    impl Itline30 {
        #[doc = "CEC"]
        #[inline(always)]
        pub const fn cec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CEC"]
        #[inline(always)]
        pub fn set_cec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline30 {
        #[inline(always)]
        fn default() -> Itline30 {
            Itline30(0)
        }
    }
    impl core::fmt::Debug for Itline30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline30").field("cec", &self.cec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline30 {{ cec: {=bool:?} }}", self.cec())
        }
    }
    #[doc = "interrupt line 31 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline31(pub u32);
    impl Itline31 {
        #[doc = "RNG"]
        #[inline(always)]
        pub const fn rng(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RNG"]
        #[inline(always)]
        pub fn set_rng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AES"]
        #[inline(always)]
        pub const fn aes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AES"]
        #[inline(always)]
        pub fn set_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline31 {
        #[inline(always)]
        fn default() -> Itline31 {
            Itline31(0)
        }
    }
    impl core::fmt::Debug for Itline31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline31")
                .field("rng", &self.rng())
                .field("aes", &self.aes())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline31 {{ rng: {=bool:?}, aes: {=bool:?} }}",
                self.rng(),
                self.aes()
            )
        }
    }
    #[doc = "interrupt line 4 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline4(pub u32);
    impl Itline4 {
        #[doc = "RCC"]
        #[inline(always)]
        pub const fn rcc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RCC"]
        #[inline(always)]
        pub fn set_rcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline4 {
        #[inline(always)]
        fn default() -> Itline4 {
            Itline4(0)
        }
    }
    impl core::fmt::Debug for Itline4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline4").field("rcc", &self.rcc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline4 {{ rcc: {=bool:?} }}", self.rcc())
        }
    }
    #[doc = "interrupt line 5 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline5(pub u32);
    impl Itline5 {
        #[doc = "EXTI0"]
        #[inline(always)]
        pub const fn exti0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI0"]
        #[inline(always)]
        pub fn set_exti0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXTI1"]
        #[inline(always)]
        pub const fn exti1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI1"]
        #[inline(always)]
        pub fn set_exti1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline5 {
        #[inline(always)]
        fn default() -> Itline5 {
            Itline5(0)
        }
    }
    impl core::fmt::Debug for Itline5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline5")
                .field("exti0", &self.exti0())
                .field("exti1", &self.exti1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline5 {{ exti0: {=bool:?}, exti1: {=bool:?} }}",
                self.exti0(),
                self.exti1()
            )
        }
    }
    #[doc = "interrupt line 6 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline6(pub u32);
    impl Itline6 {
        #[doc = "EXTI2"]
        #[inline(always)]
        pub const fn exti2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI2"]
        #[inline(always)]
        pub fn set_exti2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXTI3"]
        #[inline(always)]
        pub const fn exti3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI3"]
        #[inline(always)]
        pub fn set_exti3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline6 {
        #[inline(always)]
        fn default() -> Itline6 {
            Itline6(0)
        }
    }
    impl core::fmt::Debug for Itline6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline6")
                .field("exti2", &self.exti2())
                .field("exti3", &self.exti3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline6 {{ exti2: {=bool:?}, exti3: {=bool:?} }}",
                self.exti2(),
                self.exti3()
            )
        }
    }
    #[doc = "interrupt line 7 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline7(pub u32);
    impl Itline7 {
        #[doc = "EXTI4"]
        #[inline(always)]
        pub const fn exti4(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI4"]
        #[inline(always)]
        pub fn set_exti4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXTI5"]
        #[inline(always)]
        pub const fn exti5(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI5"]
        #[inline(always)]
        pub fn set_exti5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EXTI6"]
        #[inline(always)]
        pub const fn exti6(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI6"]
        #[inline(always)]
        pub fn set_exti6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EXTI7"]
        #[inline(always)]
        pub const fn exti7(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI7"]
        #[inline(always)]
        pub fn set_exti7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EXTI8"]
        #[inline(always)]
        pub const fn exti8(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI8"]
        #[inline(always)]
        pub fn set_exti8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EXTI9"]
        #[inline(always)]
        pub const fn exti9(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI9"]
        #[inline(always)]
        pub fn set_exti9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EXTI10"]
        #[inline(always)]
        pub const fn exti10(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI10"]
        #[inline(always)]
        pub fn set_exti10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EXTI11"]
        #[inline(always)]
        pub const fn exti11(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI11"]
        #[inline(always)]
        pub fn set_exti11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "EXTI12"]
        #[inline(always)]
        pub const fn exti12(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI12"]
        #[inline(always)]
        pub fn set_exti12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "EXTI13"]
        #[inline(always)]
        pub const fn exti13(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI13"]
        #[inline(always)]
        pub fn set_exti13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "EXTI14"]
        #[inline(always)]
        pub const fn exti14(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI14"]
        #[inline(always)]
        pub fn set_exti14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EXTI15"]
        #[inline(always)]
        pub const fn exti15(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI15"]
        #[inline(always)]
        pub fn set_exti15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Itline7 {
        #[inline(always)]
        fn default() -> Itline7 {
            Itline7(0)
        }
    }
    impl core::fmt::Debug for Itline7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline7")
                .field("exti4", &self.exti4())
                .field("exti5", &self.exti5())
                .field("exti6", &self.exti6())
                .field("exti7", &self.exti7())
                .field("exti8", &self.exti8())
                .field("exti9", &self.exti9())
                .field("exti10", &self.exti10())
                .field("exti11", &self.exti11())
                .field("exti12", &self.exti12())
                .field("exti13", &self.exti13())
                .field("exti14", &self.exti14())
                .field("exti15", &self.exti15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline7 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Itline7 {{ exti4: {=bool:?}, exti5: {=bool:?}, exti6: {=bool:?}, exti7: {=bool:?}, exti8: {=bool:?}, exti9: {=bool:?}, exti10: {=bool:?}, exti11: {=bool:?}, exti12: {=bool:?}, exti13: {=bool:?}, exti14: {=bool:?}, exti15: {=bool:?} }}" , self . exti4 () , self . exti5 () , self . exti6 () , self . exti7 () , self . exti8 () , self . exti9 () , self . exti10 () , self . exti11 () , self . exti12 () , self . exti13 () , self . exti14 () , self . exti15 ())
        }
    }
    #[doc = "interrupt line 8 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline8(pub u32);
    impl Itline8 {
        #[doc = "UCPD1"]
        #[inline(always)]
        pub const fn ucpd1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1"]
        #[inline(always)]
        pub fn set_ucpd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UCPD2"]
        #[inline(always)]
        pub const fn ucpd2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD2"]
        #[inline(always)]
        pub fn set_ucpd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USB"]
        #[inline(always)]
        pub const fn usb(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB"]
        #[inline(always)]
        pub fn set_usb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Itline8 {
        #[inline(always)]
        fn default() -> Itline8 {
            Itline8(0)
        }
    }
    impl core::fmt::Debug for Itline8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline8")
                .field("ucpd1", &self.ucpd1())
                .field("ucpd2", &self.ucpd2())
                .field("usb", &self.usb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline8 {{ ucpd1: {=bool:?}, ucpd2: {=bool:?}, usb: {=bool:?} }}",
                self.ucpd1(),
                self.ucpd2(),
                self.usb()
            )
        }
    }
    #[doc = "interrupt line 9 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline9(pub u32);
    impl Itline9 {
        #[doc = "DMA1_CH1"]
        #[inline(always)]
        pub const fn dma1_ch1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1_CH1"]
        #[inline(always)]
        pub fn set_dma1_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline9 {
        #[inline(always)]
        fn default() -> Itline9 {
            Itline9(0)
        }
    }
    impl core::fmt::Debug for Itline9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline9").field("dma1_ch1", &self.dma1_ch1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline9 {{ dma1_ch1: {=bool:?} }}", self.dma1_ch1())
        }
    }
    #[doc = "VREFBUF calibration control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VrefbufCcr(pub u32);
    impl VrefbufCcr {
        #[doc = "Trimming code These bits are automatically initialized after reset with the trimming value stored in the Flash memory during the production test. Writing into these bits allows to tune the internal reference buffer voltage."]
        #[inline(always)]
        pub const fn trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Trimming code These bits are automatically initialized after reset with the trimming value stored in the Flash memory during the production test. Writing into these bits allows to tune the internal reference buffer voltage."]
        #[inline(always)]
        pub fn set_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for VrefbufCcr {
        #[inline(always)]
        fn default() -> VrefbufCcr {
            VrefbufCcr(0)
        }
    }
    impl core::fmt::Debug for VrefbufCcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VrefbufCcr").field("trim", &self.trim()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VrefbufCcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VrefbufCcr {{ trim: {=u8:?} }}", self.trim())
        }
    }
    #[doc = "VREFBUF control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VrefbufCsr(pub u32);
    impl VrefbufCsr {
        #[doc = "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
        #[inline(always)]
        pub const fn envr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
        #[inline(always)]
        pub fn set_envr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
        #[inline(always)]
        pub const fn hiz(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
        #[inline(always)]
        pub fn set_hiz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage reference scale. 0: Voltage reference set to VREF_OUT1 (around 2.048 V). 1: Voltage reference set to VREF_OUT2 (around 2.5 V)."]
        #[inline(always)]
        pub const fn vrs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage reference scale. 0: Voltage reference set to VREF_OUT1 (around 2.048 V). 1: Voltage reference set to VREF_OUT2 (around 2.5 V)."]
        #[inline(always)]
        pub fn set_vrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Voltage reference buffer ready"]
        #[inline(always)]
        pub const fn vrr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage reference buffer ready"]
        #[inline(always)]
        pub fn set_vrr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for VrefbufCsr {
        #[inline(always)]
        fn default() -> VrefbufCsr {
            VrefbufCsr(0)
        }
    }
    impl core::fmt::Debug for VrefbufCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VrefbufCsr")
                .field("envr", &self.envr())
                .field("hiz", &self.hiz())
                .field("vrs", &self.vrs())
                .field("vrr", &self.vrr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VrefbufCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VrefbufCsr {{ envr: {=bool:?}, hiz: {=bool:?}, vrs: {=bool:?}, vrr: {=bool:?} }}",
                self.envr(),
                self.hiz(),
                self.vrs(),
                self.vrr()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemMode {
        #[doc = "Main Flash memory mapped at address 0"]
        MAIN_FLASH = 0x0,
        #[doc = "System Flash memory mapped at address 0"]
        SYSTEM_FLASH = 0x01,
        #[doc = "Main Flash memory mapped at address 0 (alternate encoding)"]
        MAIN_FLASH_ALT = 0x02,
        #[doc = "Embedded SRAM mapped at address 0"]
        SRAM = 0x03,
    }
    impl MemMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemMode {
        #[inline(always)]
        fn from(val: u8) -> MemMode {
            MemMode::from_bits(val)
        }
    }
    impl From<MemMode> for u8 {
        #[inline(always)]
        fn from(val: MemMode) -> u8 {
            MemMode::to_bits(val)
        }
    }
}
