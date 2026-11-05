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
    #[doc = "peripheral mode configuration register"]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SYSCFG compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SYSCFG compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "SYSCFG package register"]
    #[inline(always)]
    pub const fn pkgr(self) -> crate::common::Reg<regs::Pkgr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "SYSCFG user register 0"]
    #[inline(always)]
    pub const fn ur0(self) -> crate::common::Reg<regs::Ur0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "SYSCFG user register 2"]
    #[inline(always)]
    pub const fn ur2(self) -> crate::common::Reg<regs::Ur2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "SYSCFG user register 3"]
    #[inline(always)]
    pub const fn ur3(self) -> crate::common::Reg<regs::Ur3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "SYSCFG user register 4"]
    #[inline(always)]
    pub const fn ur4(self) -> crate::common::Reg<regs::Ur4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "SYSCFG user register 5"]
    #[inline(always)]
    pub const fn ur5(self) -> crate::common::Reg<regs::Ur5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "SYSCFG user register 6"]
    #[inline(always)]
    pub const fn ur6(self) -> crate::common::Reg<regs::Ur6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[doc = "SYSCFG user register 7"]
    #[inline(always)]
    pub const fn ur7(self) -> crate::common::Reg<regs::Ur7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[doc = "SYSCFG user register 8"]
    #[inline(always)]
    pub const fn ur8(self) -> crate::common::Reg<regs::Ur8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "SYSCFG user register 9"]
    #[inline(always)]
    pub const fn ur9(self) -> crate::common::Reg<regs::Ur9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "SYSCFG user register 10"]
    #[inline(always)]
    pub const fn ur10(self) -> crate::common::Reg<regs::Ur10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "SYSCFG user register 11"]
    #[inline(always)]
    pub const fn ur11(self) -> crate::common::Reg<regs::Ur11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "SYSCFG user register 12"]
    #[inline(always)]
    pub const fn ur12(self) -> crate::common::Reg<regs::Ur12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[doc = "SYSCFG user register 13"]
    #[inline(always)]
    pub const fn ur13(self) -> crate::common::Reg<regs::Ur13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[doc = "SYSCFG user register 14"]
    #[inline(always)]
    pub const fn ur14(self) -> crate::common::Reg<regs::Ur14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[doc = "SYSCFG user register 15"]
    #[inline(always)]
    pub const fn ur15(self) -> crate::common::Reg<regs::Ur15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[doc = "SYSCFG user register 16"]
    #[inline(always)]
    pub const fn ur16(self) -> crate::common::Reg<regs::Ur16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "SYSCFG user register 17"]
    #[inline(always)]
    pub const fn ur17(self) -> crate::common::Reg<regs::Ur17, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "SYSCFG user register 18"]
    #[inline(always)]
    pub const fn ur18(self) -> crate::common::Reg<regs::Ur18, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
    }
}
pub mod regs {
    #[doc = "SYSCFG compensation cell code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "NMOS compensation code"]
        #[inline(always)]
        pub const fn ncc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code"]
        #[inline(always)]
        pub fn set_ncc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation code"]
        #[inline(always)]
        pub const fn pcc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code"]
        #[inline(always)]
        pub fn set_pcc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cccr {
        #[inline(always)]
        fn default() -> Cccr {
            Cccr(0)
        }
    }
    impl core::fmt::Debug for Cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccr")
                .field("ncc", &self.ncc())
                .field("pcc", &self.pcc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cccr {{ ncc: {=u8:?}, pcc: {=u8:?} }}", self.ncc(), self.pcc())
        }
    }
    #[doc = "compensation cell control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Code selection"]
        #[inline(always)]
        pub const fn cs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Code selection"]
        #[inline(always)]
        pub fn set_cs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Compensation cell ready flag"]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell ready flag"]
        #[inline(always)]
        pub fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "High-speed at low-voltage"]
        #[inline(always)]
        pub const fn hslv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed at low-voltage"]
        #[inline(always)]
        pub fn set_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cccsr {
        #[inline(always)]
        fn default() -> Cccsr {
            Cccsr(0)
        }
    }
    impl core::fmt::Debug for Cccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccsr")
                .field("en", &self.en())
                .field("cs", &self.cs())
                .field("rdy", &self.rdy())
                .field("hslv", &self.hslv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cccsr {{ en: {=bool:?}, cs: {=bool:?}, rdy: {=bool:?}, hslv: {=bool:?} }}",
                self.en(),
                self.cs(),
                self.rdy(),
                self.hslv()
            )
        }
    }
    #[doc = "SYSCFG compensation cell value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "NMOS compensation value"]
        #[inline(always)]
        pub const fn ncv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation value"]
        #[inline(always)]
        pub fn set_ncv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation value"]
        #[inline(always)]
        pub const fn pcv(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation value"]
        #[inline(always)]
        pub fn set_pcv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Ccvr {
        #[inline(always)]
        fn default() -> Ccvr {
            Ccvr(0)
        }
    }
    impl core::fmt::Debug for Ccvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccvr")
                .field("ncv", &self.ncv())
                .field("pcv", &self.pcv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccvr {{ ncv: {=u8:?}, pcv: {=u8:?} }}", self.ncv(), self.pcv())
        }
    }
    #[doc = "external interrupt configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI x configuration (x = 4 to 7)"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI x configuration (x = 4 to 7)"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "SYSCFG package register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pkgr(pub u32);
    impl Pkgr {
        #[doc = "Package"]
        #[inline(always)]
        pub const fn pkg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Package"]
        #[inline(always)]
        pub fn set_pkg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Pkgr {
        #[inline(always)]
        fn default() -> Pkgr {
            Pkgr(0)
        }
    }
    impl core::fmt::Debug for Pkgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pkgr").field("pkg", &self.pkg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pkgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pkgr {{ pkg: {=u8:?} }}", self.pkg())
        }
    }
    #[doc = "peripheral mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "I2C1 Fm+"]
        #[inline(always)]
        pub const fn i2c1fmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fm+"]
        #[inline(always)]
        pub fn set_i2c1fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C2 Fm+"]
        #[inline(always)]
        pub const fn i2c2fmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Fm+"]
        #[inline(always)]
        pub fn set_i2c2fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I2C3 Fm+"]
        #[inline(always)]
        pub const fn i2c3fmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Fm+"]
        #[inline(always)]
        pub fn set_i2c3fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I2C4 Fm+"]
        #[inline(always)]
        pub const fn i2c4fmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Fm+"]
        #[inline(always)]
        pub fn set_i2c4fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PB(6) Fm+"]
        #[inline(always)]
        pub const fn pb6fmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PB(6) Fm+"]
        #[inline(always)]
        pub fn set_pb6fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PB(7) Fast Mode Plus"]
        #[inline(always)]
        pub const fn pb7fmp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PB(7) Fast Mode Plus"]
        #[inline(always)]
        pub fn set_pb7fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PB(8) Fast Mode Plus"]
        #[inline(always)]
        pub const fn pb8fmp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PB(8) Fast Mode Plus"]
        #[inline(always)]
        pub fn set_pb8fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PB(9) Fm+"]
        #[inline(always)]
        pub const fn pb9fmp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PB(9) Fm+"]
        #[inline(always)]
        pub fn set_pb9fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Booster Enable"]
        #[inline(always)]
        pub const fn booste(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Booster Enable"]
        #[inline(always)]
        pub fn set_booste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Analog switch supply voltage selection"]
        #[inline(always)]
        pub const fn boostvddsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Analog switch supply voltage selection"]
        #[inline(always)]
        pub fn set_boostvddsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Ethernet PHY interface selection."]
        #[inline(always)]
        pub const fn eth_sel_phy(&self) -> super::vals::EthSelPhy {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::EthSelPhy::from_bits(val as u8)
        }
        #[doc = "Ethernet PHY interface selection."]
        #[inline(always)]
        pub fn set_eth_sel_phy(&mut self, val: super::vals::EthSelPhy) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "PA0 Switch Open"]
        #[inline(always)]
        pub const fn pa0so(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PA0 Switch Open"]
        #[inline(always)]
        pub fn set_pa0so(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PA1 Switch Open"]
        #[inline(always)]
        pub const fn pa1so(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PA1 Switch Open"]
        #[inline(always)]
        pub fn set_pa1so(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PC2 Switch Open"]
        #[inline(always)]
        pub const fn pc2so(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PC2 Switch Open"]
        #[inline(always)]
        pub fn set_pc2so(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PC3 Switch Open"]
        #[inline(always)]
        pub const fn pc3so(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PC3 Switch Open"]
        #[inline(always)]
        pub fn set_pc3so(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Pmcr {
        #[inline(always)]
        fn default() -> Pmcr {
            Pmcr(0)
        }
    }
    impl core::fmt::Debug for Pmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmcr")
                .field("i2c1fmp", &self.i2c1fmp())
                .field("i2c2fmp", &self.i2c2fmp())
                .field("i2c3fmp", &self.i2c3fmp())
                .field("i2c4fmp", &self.i2c4fmp())
                .field("pb6fmp", &self.pb6fmp())
                .field("pb7fmp", &self.pb7fmp())
                .field("pb8fmp", &self.pb8fmp())
                .field("pb9fmp", &self.pb9fmp())
                .field("booste", &self.booste())
                .field("boostvddsel", &self.boostvddsel())
                .field("eth_sel_phy", &self.eth_sel_phy())
                .field("pa0so", &self.pa0so())
                .field("pa1so", &self.pa1so())
                .field("pc2so", &self.pc2so())
                .field("pc3so", &self.pc3so())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pmcr {{ i2c1fmp: {=bool:?}, i2c2fmp: {=bool:?}, i2c3fmp: {=bool:?}, i2c4fmp: {=bool:?}, pb6fmp: {=bool:?}, pb7fmp: {=bool:?}, pb8fmp: {=bool:?}, pb9fmp: {=bool:?}, booste: {=bool:?}, boostvddsel: {=bool:?}, eth_sel_phy: {:?}, pa0so: {=bool:?}, pa1so: {=bool:?}, pc2so: {=bool:?}, pc3so: {=bool:?} }}" , self . i2c1fmp () , self . i2c2fmp () , self . i2c3fmp () , self . i2c4fmp () , self . pb6fmp () , self . pb7fmp () , self . pb8fmp () , self . pb9fmp () , self . booste () , self . boostvddsel () , self . eth_sel_phy () , self . pa0so () , self . pa1so () , self . pc2so () , self . pc3so ())
        }
    }
    #[doc = "SYSCFG user register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur0(pub u32);
    impl Ur0 {
        #[doc = "Bank Swap"]
        #[inline(always)]
        pub const fn bks(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bank Swap"]
        #[inline(always)]
        pub fn set_bks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Readout protection"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Readout protection"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ur0 {
        #[inline(always)]
        fn default() -> Ur0 {
            Ur0(0)
        }
    }
    impl core::fmt::Debug for Ur0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur0")
                .field("bks", &self.bks())
                .field("rdp", &self.rdp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur0 {{ bks: {=bool:?}, rdp: {=u8:?} }}", self.bks(), self.rdp())
        }
    }
    #[doc = "SYSCFG user register 10"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur10(pub u32);
    impl Ur10 {
        #[doc = "Protected area end address for bank 2"]
        #[inline(always)]
        pub const fn pa_end_2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Protected area end address for bank 2"]
        #[inline(always)]
        pub fn set_pa_end_2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Secured area start address for bank 2"]
        #[inline(always)]
        pub const fn sa_beg_2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Secured area start address for bank 2"]
        #[inline(always)]
        pub fn set_sa_beg_2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Ur10 {
        #[inline(always)]
        fn default() -> Ur10 {
            Ur10(0)
        }
    }
    impl core::fmt::Debug for Ur10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur10")
                .field("pa_end_2", &self.pa_end_2())
                .field("sa_beg_2", &self.sa_beg_2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur10 {{ pa_end_2: {=u16:?}, sa_beg_2: {=u16:?} }}",
                self.pa_end_2(),
                self.sa_beg_2()
            )
        }
    }
    #[doc = "SYSCFG user register 11"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur11(pub u32);
    impl Ur11 {
        #[doc = "Secured area end address for bank 2"]
        #[inline(always)]
        pub const fn sa_end_2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Secured area end address for bank 2"]
        #[inline(always)]
        pub fn set_sa_end_2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Independent Watchdog 1 mode"]
        #[inline(always)]
        pub const fn iwdg1m(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent Watchdog 1 mode"]
        #[inline(always)]
        pub fn set_iwdg1m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur11 {
        #[inline(always)]
        fn default() -> Ur11 {
            Ur11(0)
        }
    }
    impl core::fmt::Debug for Ur11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur11")
                .field("sa_end_2", &self.sa_end_2())
                .field("iwdg1m", &self.iwdg1m())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur11 {{ sa_end_2: {=u16:?}, iwdg1m: {=bool:?} }}",
                self.sa_end_2(),
                self.iwdg1m()
            )
        }
    }
    #[doc = "SYSCFG user register 12"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur12(pub u32);
    impl Ur12 {
        #[doc = "Secure mode"]
        #[inline(always)]
        pub const fn secure(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure mode"]
        #[inline(always)]
        pub fn set_secure(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur12 {
        #[inline(always)]
        fn default() -> Ur12 {
            Ur12(0)
        }
    }
    impl core::fmt::Debug for Ur12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur12").field("secure", &self.secure()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur12 {{ secure: {=bool:?} }}", self.secure())
        }
    }
    #[doc = "SYSCFG user register 13"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur13(pub u32);
    impl Ur13 {
        #[doc = "Secured DTCM RAM Size"]
        #[inline(always)]
        pub const fn sdrs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Secured DTCM RAM Size"]
        #[inline(always)]
        pub fn set_sdrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "D1 Standby reset"]
        #[inline(always)]
        pub const fn d1sbrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "D1 Standby reset"]
        #[inline(always)]
        pub fn set_d1sbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur13 {
        #[inline(always)]
        fn default() -> Ur13 {
            Ur13(0)
        }
    }
    impl core::fmt::Debug for Ur13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur13")
                .field("sdrs", &self.sdrs())
                .field("d1sbrst", &self.d1sbrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur13 {{ sdrs: {=u8:?}, d1sbrst: {=bool:?} }}",
                self.sdrs(),
                self.d1sbrst()
            )
        }
    }
    #[doc = "SYSCFG user register 14"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur14(pub u32);
    impl Ur14 {
        #[doc = "D1 Stop Reset"]
        #[inline(always)]
        pub const fn d1stprst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "D1 Stop Reset"]
        #[inline(always)]
        pub fn set_d1stprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ur14 {
        #[inline(always)]
        fn default() -> Ur14 {
            Ur14(0)
        }
    }
    impl core::fmt::Debug for Ur14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur14").field("d1stprst", &self.d1stprst()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur14 {{ d1stprst: {=bool:?} }}", self.d1stprst())
        }
    }
    #[doc = "SYSCFG user register 15"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur15(pub u32);
    impl Ur15 {
        #[doc = "Freeze independent watchdog in Standby mode"]
        #[inline(always)]
        pub const fn fziwdgstb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Freeze independent watchdog in Standby mode"]
        #[inline(always)]
        pub fn set_fziwdgstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur15 {
        #[inline(always)]
        fn default() -> Ur15 {
            Ur15(0)
        }
    }
    impl core::fmt::Debug for Ur15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur15").field("fziwdgstb", &self.fziwdgstb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur15 {{ fziwdgstb: {=bool:?} }}", self.fziwdgstb())
        }
    }
    #[doc = "SYSCFG user register 16"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur16(pub u32);
    impl Ur16 {
        #[doc = "Freeze independent watchdog in Stop mode"]
        #[inline(always)]
        pub const fn fziwdgstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Freeze independent watchdog in Stop mode"]
        #[inline(always)]
        pub fn set_fziwdgstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Private key programmed"]
        #[inline(always)]
        pub const fn pkp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Private key programmed"]
        #[inline(always)]
        pub fn set_pkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur16 {
        #[inline(always)]
        fn default() -> Ur16 {
            Ur16(0)
        }
    }
    impl core::fmt::Debug for Ur16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur16")
                .field("fziwdgstp", &self.fziwdgstp())
                .field("pkp", &self.pkp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur16 {{ fziwdgstp: {=bool:?}, pkp: {=bool:?} }}",
                self.fziwdgstp(),
                self.pkp()
            )
        }
    }
    #[doc = "SYSCFG user register 17"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur17(pub u32);
    impl Ur17 {
        #[doc = "I/O high speed / low voltage"]
        #[inline(always)]
        pub const fn io_hslv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O high speed / low voltage"]
        #[inline(always)]
        pub fn set_io_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ITCM-RAM / AXI-SRAM size"]
        #[inline(always)]
        pub const fn tcm_axi_shared_cfg(&self) -> super::vals::ItcmAxiRamSize {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::ItcmAxiRamSize::from_bits(val as u8)
        }
        #[doc = "ITCM-RAM / AXI-SRAM size"]
        #[inline(always)]
        pub fn set_tcm_axi_shared_cfg(&mut self, val: super::vals::ItcmAxiRamSize) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Ur17 {
        #[inline(always)]
        fn default() -> Ur17 {
            Ur17(0)
        }
    }
    impl core::fmt::Debug for Ur17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur17")
                .field("io_hslv", &self.io_hslv())
                .field("tcm_axi_shared_cfg", &self.tcm_axi_shared_cfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur17 {{ io_hslv: {=bool:?}, tcm_axi_shared_cfg: {:?} }}",
                self.io_hslv(),
                self.tcm_axi_shared_cfg()
            )
        }
    }
    #[doc = "SYSCFG user register 18"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur18(pub u32);
    impl Ur18 {
        #[doc = "CPU maximum frequency boost enable"]
        #[inline(always)]
        pub const fn cpu_freq_boost(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CPU maximum frequency boost enable"]
        #[inline(always)]
        pub fn set_cpu_freq_boost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ur18 {
        #[inline(always)]
        fn default() -> Ur18 {
            Ur18(0)
        }
    }
    impl core::fmt::Debug for Ur18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur18")
                .field("cpu_freq_boost", &self.cpu_freq_boost())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur18 {{ cpu_freq_boost: {=bool:?} }}", self.cpu_freq_boost())
        }
    }
    #[doc = "SYSCFG user register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur2(pub u32);
    impl Ur2 {
        #[doc = "BOR_LVL Brownout Reset Threshold Level"]
        #[inline(always)]
        pub const fn borh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "BOR_LVL Brownout Reset Threshold Level"]
        #[inline(always)]
        pub fn set_borh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Boot Address 0"]
        #[inline(always)]
        pub const fn boot_add0(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot Address 0"]
        #[inline(always)]
        pub fn set_boot_add0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ur2 {
        #[inline(always)]
        fn default() -> Ur2 {
            Ur2(0)
        }
    }
    impl core::fmt::Debug for Ur2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur2")
                .field("borh", &self.borh())
                .field("boot_add0", &self.boot_add0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur2 {{ borh: {=u8:?}, boot_add0: {=u16:?} }}",
                self.borh(),
                self.boot_add0()
            )
        }
    }
    #[doc = "SYSCFG user register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur3(pub u32);
    impl Ur3 {
        #[doc = "Boot Address 1"]
        #[inline(always)]
        pub const fn boot_add1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot Address 1"]
        #[inline(always)]
        pub fn set_boot_add1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ur3 {
        #[inline(always)]
        fn default() -> Ur3 {
            Ur3(0)
        }
    }
    impl core::fmt::Debug for Ur3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur3").field("boot_add1", &self.boot_add1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur3 {{ boot_add1: {=u16:?} }}", self.boot_add1())
        }
    }
    #[doc = "SYSCFG user register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur4(pub u32);
    impl Ur4 {
        #[doc = "Mass Erase Protected Area Disabled for bank 1"]
        #[inline(always)]
        pub const fn mepad_1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Mass Erase Protected Area Disabled for bank 1"]
        #[inline(always)]
        pub fn set_mepad_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur4 {
        #[inline(always)]
        fn default() -> Ur4 {
            Ur4(0)
        }
    }
    impl core::fmt::Debug for Ur4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur4").field("mepad_1", &self.mepad_1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ur4 {{ mepad_1: {=bool:?} }}", self.mepad_1())
        }
    }
    #[doc = "SYSCFG user register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur5(pub u32);
    impl Ur5 {
        #[doc = "Mass erase secured area disabled for bank 1"]
        #[inline(always)]
        pub const fn mesad_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase secured area disabled for bank 1"]
        #[inline(always)]
        pub fn set_mesad_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write protection for flash bank 1"]
        #[inline(always)]
        pub const fn wrpn_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Write protection for flash bank 1"]
        #[inline(always)]
        pub fn set_wrpn_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ur5 {
        #[inline(always)]
        fn default() -> Ur5 {
            Ur5(0)
        }
    }
    impl core::fmt::Debug for Ur5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur5")
                .field("mesad_1", &self.mesad_1())
                .field("wrpn_1", &self.wrpn_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur5 {{ mesad_1: {=bool:?}, wrpn_1: {=u8:?} }}",
                self.mesad_1(),
                self.wrpn_1()
            )
        }
    }
    #[doc = "SYSCFG user register 6"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur6(pub u32);
    impl Ur6 {
        #[doc = "Protected area start address for bank 1"]
        #[inline(always)]
        pub const fn pa_beg_1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Protected area start address for bank 1"]
        #[inline(always)]
        pub fn set_pa_beg_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Protected area end address for bank 1"]
        #[inline(always)]
        pub const fn pa_end_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Protected area end address for bank 1"]
        #[inline(always)]
        pub fn set_pa_end_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Ur6 {
        #[inline(always)]
        fn default() -> Ur6 {
            Ur6(0)
        }
    }
    impl core::fmt::Debug for Ur6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur6")
                .field("pa_beg_1", &self.pa_beg_1())
                .field("pa_end_1", &self.pa_end_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur6 {{ pa_beg_1: {=u16:?}, pa_end_1: {=u16:?} }}",
                self.pa_beg_1(),
                self.pa_end_1()
            )
        }
    }
    #[doc = "SYSCFG user register 7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur7(pub u32);
    impl Ur7 {
        #[doc = "Secured area start address for bank 1"]
        #[inline(always)]
        pub const fn sa_beg_1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Secured area start address for bank 1"]
        #[inline(always)]
        pub fn set_sa_beg_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Secured area end address for bank 1"]
        #[inline(always)]
        pub const fn sa_end_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Secured area end address for bank 1"]
        #[inline(always)]
        pub fn set_sa_end_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Ur7 {
        #[inline(always)]
        fn default() -> Ur7 {
            Ur7(0)
        }
    }
    impl core::fmt::Debug for Ur7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur7")
                .field("sa_beg_1", &self.sa_beg_1())
                .field("sa_end_1", &self.sa_end_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur7 {{ sa_beg_1: {=u16:?}, sa_end_1: {=u16:?} }}",
                self.sa_beg_1(),
                self.sa_end_1()
            )
        }
    }
    #[doc = "SYSCFG user register 8"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur8(pub u32);
    impl Ur8 {
        #[doc = "Mass erase protected area disabled for bank 2"]
        #[inline(always)]
        pub const fn mepad_2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase protected area disabled for bank 2"]
        #[inline(always)]
        pub fn set_mepad_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mass erase secured area disabled for bank 2"]
        #[inline(always)]
        pub const fn mesad_2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase secured area disabled for bank 2"]
        #[inline(always)]
        pub fn set_mesad_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ur8 {
        #[inline(always)]
        fn default() -> Ur8 {
            Ur8(0)
        }
    }
    impl core::fmt::Debug for Ur8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur8")
                .field("mepad_2", &self.mepad_2())
                .field("mesad_2", &self.mesad_2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur8 {{ mepad_2: {=bool:?}, mesad_2: {=bool:?} }}",
                self.mepad_2(),
                self.mesad_2()
            )
        }
    }
    #[doc = "SYSCFG user register 9"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ur9(pub u32);
    impl Ur9 {
        #[doc = "Write protection for flash bank 2"]
        #[inline(always)]
        pub const fn wrpn_2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write protection for flash bank 2"]
        #[inline(always)]
        pub fn set_wrpn_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Protected area start address for bank 2"]
        #[inline(always)]
        pub const fn pa_beg_2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Protected area start address for bank 2"]
        #[inline(always)]
        pub fn set_pa_beg_2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Ur9 {
        #[inline(always)]
        fn default() -> Ur9 {
            Ur9(0)
        }
    }
    impl core::fmt::Debug for Ur9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ur9")
                .field("wrpn_2", &self.wrpn_2())
                .field("pa_beg_2", &self.pa_beg_2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ur9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ur9 {{ wrpn_2: {=u8:?}, pa_beg_2: {=u16:?} }}",
                self.wrpn_2(),
                self.pa_beg_2()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EthSelPhy {
        #[doc = "GMII or MII"]
        MII_GMII = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "RMII"]
        RMII = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl EthSelPhy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EthSelPhy {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EthSelPhy {
        #[inline(always)]
        fn from(val: u8) -> EthSelPhy {
            EthSelPhy::from_bits(val)
        }
    }
    impl From<EthSelPhy> for u8 {
        #[inline(always)]
        fn from(val: EthSelPhy) -> u8 {
            EthSelPhy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ItcmAxiRamSize {
        #[doc = "64 Kbyte ITCM-RAM / 320 Kbyte AXI-SRAM"]
        ITCM64AXI320 = 0x0,
        #[doc = "128 Kbyte ITCM-RAM / 256 Kbyte AXI-SRAM"]
        ITCM128AXI256 = 0x01,
        #[doc = "192 Kbyte ITCM-RAM / 192 Kbyte AXI-SRAM"]
        ITCM192AXI192 = 0x02,
        #[doc = "256 Kbyte ITCM-RAM / 128 Kbyte AXI-SRAM"]
        ITCM256AXI128 = 0x03,
    }
    impl ItcmAxiRamSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ItcmAxiRamSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ItcmAxiRamSize {
        #[inline(always)]
        fn from(val: u8) -> ItcmAxiRamSize {
            ItcmAxiRamSize::from_bits(val)
        }
    }
    impl From<ItcmAxiRamSize> for u8 {
        #[inline(always)]
        fn from(val: ItcmAxiRamSize) -> u8 {
            ItcmAxiRamSize::to_bits(val)
        }
    }
}
