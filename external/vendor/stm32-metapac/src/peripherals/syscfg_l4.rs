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
    #[doc = "memory remap register"]
    #[inline(always)]
    pub const fn memrmp(self) -> crate::common::Reg<regs::Memrmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "SCSR"]
    #[inline(always)]
    pub const fn scsr(self) -> crate::common::Reg<regs::Scsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SWPR"]
    #[inline(always)]
    pub const fn swpr(self) -> crate::common::Reg<regs::Swpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SKR"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Firewall disable"]
        #[inline(always)]
        pub const fn fwdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Firewall disable"]
        #[inline(always)]
        pub fn set_fwdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C1 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C2 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C3 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Floating Point Unit interrupts enable bits"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "Floating Point Unit interrupts enable bits"]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
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
                .field("fwdis", &self.fwdis())
                .field("boosten", &self.boosten())
                .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
                .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
                .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
                .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("i2c3_fmp", &self.i2c3_fmp())
                .field("fpu_ie", &self.fpu_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ fwdis: {=bool:?}, boosten: {=bool:?}, i2c_pb6_fmp: {=bool:?}, i2c_pb7_fmp: {=bool:?}, i2c_pb8_fmp: {=bool:?}, i2c_pb9_fmp: {=bool:?}, i2c1_fmp: {=bool:?}, i2c2_fmp: {=bool:?}, i2c3_fmp: {=bool:?}, fpu_ie: {=u8:?} }}" , self . fwdis () , self . boosten () , self . i2c_pb6_fmp () , self . i2c_pb7_fmp () , self . i2c_pb8_fmp () , self . i2c_pb9_fmp () , self . i2c1_fmp () , self . i2c2_fmp () , self . i2c3_fmp () , self . fpu_ie ())
        }
    }
    #[doc = "CFGR2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 parity lock bit"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity lock bit"]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM2 parity error flag"]
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity error flag"]
        #[inline(always)]
        pub fn set_spf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("cll", &self.cll())
                .field("spl", &self.spl())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .field("spf", &self.spf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ cll: {=bool:?}, spl: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?}, spf: {=bool:?} }}",
                self.cll(),
                self.spl(),
                self.pvdl(),
                self.eccl(),
                self.spf()
            )
        }
    }
    #[doc = "external interrupt configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI12 configuration bits"]
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
    #[doc = "memory remap register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrmp(pub u32);
    impl Memrmp {
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "QUADSPI memory mapping swap"]
        #[inline(always)]
        pub const fn qfs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI memory mapping swap"]
        #[inline(always)]
        pub fn set_qfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash Bank mode selection"]
        #[inline(always)]
        pub const fn fb_mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Bank mode selection"]
        #[inline(always)]
        pub fn set_fb_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Memrmp {
        #[inline(always)]
        fn default() -> Memrmp {
            Memrmp(0)
        }
    }
    impl core::fmt::Debug for Memrmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memrmp")
                .field("mem_mode", &self.mem_mode())
                .field("qfs", &self.qfs())
                .field("fb_mode", &self.fb_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memrmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Memrmp {{ mem_mode: {=u8:?}, qfs: {=bool:?}, fb_mode: {=bool:?} }}",
                self.mem_mode(),
                self.qfs(),
                self.fb_mode()
            )
        }
    }
    #[doc = "SCSR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scsr(pub u32);
    impl Scsr {
        #[doc = "SRAM2 Erase"]
        #[inline(always)]
        pub const fn sram2er(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Erase"]
        #[inline(always)]
        pub fn set_sram2er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 busy by erase operation"]
        #[inline(always)]
        pub const fn sram2bsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 busy by erase operation"]
        #[inline(always)]
        pub fn set_sram2bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Scsr {
        #[inline(always)]
        fn default() -> Scsr {
            Scsr(0)
        }
    }
    impl core::fmt::Debug for Scsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scsr")
                .field("sram2er", &self.sram2er())
                .field("sram2bsy", &self.sram2bsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scsr {{ sram2er: {=bool:?}, sram2bsy: {=bool:?} }}",
                self.sram2er(),
                self.sram2bsy()
            )
        }
    }
    #[doc = "SKR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Skr(pub u32);
    impl Skr {
        #[doc = "SRAM2 write protection key for software erase"]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SRAM2 write protection key for software erase"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Skr {
        #[inline(always)]
        fn default() -> Skr {
            Skr(0)
        }
    }
    impl core::fmt::Debug for Skr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Skr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Skr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Skr {{ key: {=u8:?} }}", self.key())
        }
    }
    #[doc = "SWPR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr(pub u32);
    impl Swpr {
        #[doc = "SRAWM2 write protection."]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAWM2 write protection."]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swpr {
        #[inline(always)]
        fn default() -> Swpr {
            Swpr(0)
        }
    }
    impl core::fmt::Debug for Swpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swpr")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Swpr {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize) , self . pwp (16usize) , self . pwp (17usize) , self . pwp (18usize) , self . pwp (19usize) , self . pwp (20usize) , self . pwp (21usize) , self . pwp (22usize) , self . pwp (23usize) , self . pwp (24usize) , self . pwp (25usize) , self . pwp (26usize) , self . pwp (27usize) , self . pwp (28usize) , self . pwp (29usize) , self . pwp (30usize) , self . pwp (31usize))
        }
    }
}
