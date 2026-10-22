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
    #[doc = "SYSCFG secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SYSCFG CPU non-secure lock register"]
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::Cnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SYSCFG CPU secure lock register"]
    #[inline(always)]
    pub const fn cslockr(self) -> crate::common::Reg<regs::Cslockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SCSR"]
    #[inline(always)]
    pub const fn scsr(self) -> crate::common::Reg<regs::Scsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SKR"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SWPR"]
    #[inline(always)]
    pub const fn swpr(self) -> crate::common::Reg<regs::Swpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SWPR2"]
    #[inline(always)]
    pub const fn swpr2(self) -> crate::common::Reg<regs::Swpr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RSSCMDR"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
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
        #[doc = "GPIO analog switch control voltage selection"]
        #[inline(always)]
        pub const fn anaswvdd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO analog switch control voltage selection"]
        #[inline(always)]
        pub fn set_anaswvdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
        #[doc = "I2C4_FMP"]
        #[inline(always)]
        pub const fn i2c4_fmp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4_FMP"]
        #[inline(always)]
        pub fn set_i2c4_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("boosten", &self.boosten())
                .field("anaswvdd", &self.anaswvdd())
                .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
                .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
                .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
                .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("i2c3_fmp", &self.i2c3_fmp())
                .field("i2c4_fmp", &self.i2c4_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ boosten: {=bool:?}, anaswvdd: {=bool:?}, i2c_pb6_fmp: {=bool:?}, i2c_pb7_fmp: {=bool:?}, i2c_pb8_fmp: {=bool:?}, i2c_pb9_fmp: {=bool:?}, i2c1_fmp: {=bool:?}, i2c2_fmp: {=bool:?}, i2c3_fmp: {=bool:?}, i2c4_fmp: {=bool:?} }}" , self . boosten () , self . anaswvdd () , self . i2c_pb6_fmp () , self . i2c_pb7_fmp () , self . i2c_pb8_fmp () , self . i2c_pb9_fmp () , self . i2c1_fmp () , self . i2c2_fmp () , self . i2c3_fmp () , self . i2c4_fmp ())
        }
    }
    #[doc = "CFGR2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "LOCKUP (hardfault) output enable bit"]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP (hardfault) output enable bit"]
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
    #[doc = "SYSCFG CPU non-secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnslckr(pub u32);
    impl Cnslckr {
        #[doc = "VTOR_NS register lock"]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_NS register lock"]
        #[inline(always)]
        pub fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure MPU registers lock"]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure MPU registers lock"]
        #[inline(always)]
        pub fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cnslckr {
        #[inline(always)]
        fn default() -> Cnslckr {
            Cnslckr(0)
        }
    }
    impl core::fmt::Debug for Cnslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnslckr")
                .field("locknsvtor", &self.locknsvtor())
                .field("locknsmpu", &self.locknsmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnslckr {{ locknsvtor: {=bool:?}, locknsmpu: {=bool:?} }}",
                self.locknsvtor(),
                self.locknsmpu()
            )
        }
    }
    #[doc = "SYSCFG CPU secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cslockr(pub u32);
    impl Cslockr {
        #[doc = "LOCKSVTAIRCR"]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKSVTAIRCR"]
        #[inline(always)]
        pub fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LOCKSMPU"]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKSMPU"]
        #[inline(always)]
        pub fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LOCKSAU"]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKSAU"]
        #[inline(always)]
        pub fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cslockr {
        #[inline(always)]
        fn default() -> Cslockr {
            Cslockr(0)
        }
    }
    impl core::fmt::Debug for Cslockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cslockr")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locksmpu", &self.locksmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cslockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cslockr {{ locksvtaircr: {=bool:?}, locksmpu: {=bool:?}, locksau: {=bool:?} }}",
                self.locksvtaircr(),
                self.locksmpu(),
                self.locksau()
            )
        }
    }
    #[doc = "FPU interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
        #[doc = "Floating point unit interrupts enable bits"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Floating point unit interrupts enable bits"]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Fpuimr {
        #[inline(always)]
        fn default() -> Fpuimr {
            Fpuimr(0)
        }
    }
    impl core::fmt::Debug for Fpuimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpuimr").field("fpu_ie", &self.fpu_ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpuimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fpuimr {{ fpu_ie: {=u8:?} }}", self.fpu_ie())
        }
    }
    #[doc = "RSSCMDR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS commands"]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RSS commands"]
        #[inline(always)]
        pub fn set_rsscmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rsscmdr {
        #[inline(always)]
        fn default() -> Rsscmdr {
            Rsscmdr(0)
        }
    }
    impl core::fmt::Debug for Rsscmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsscmdr").field("rsscmd", &self.rsscmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rsscmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rsscmdr {{ rsscmd: {=u8:?} }}", self.rsscmd())
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
    #[doc = "SYSCFG secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "SYSCFG clock control security"]
        #[inline(always)]
        pub const fn syscfgsec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock control security"]
        #[inline(always)]
        pub fn set_syscfgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ClassB security"]
        #[inline(always)]
        pub const fn classbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ClassB security"]
        #[inline(always)]
        pub fn set_classbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 security"]
        #[inline(always)]
        pub const fn sram2sec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 security"]
        #[inline(always)]
        pub fn set_sram2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FPUSEC"]
        #[inline(always)]
        pub const fn fpusec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FPUSEC"]
        #[inline(always)]
        pub fn set_fpusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("syscfgsec", &self.syscfgsec())
                .field("classbsec", &self.classbsec())
                .field("sram2sec", &self.sram2sec())
                .field("fpusec", &self.fpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ syscfgsec: {=bool:?}, classbsec: {=bool:?}, sram2sec: {=bool:?}, fpusec: {=bool:?} }}",
                self.syscfgsec(),
                self.classbsec(),
                self.sram2sec(),
                self.fpusec()
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
        #[doc = "P0WP"]
        #[inline(always)]
        pub const fn p0wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "P0WP"]
        #[inline(always)]
        pub fn set_p0wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "P1WP"]
        #[inline(always)]
        pub const fn p1wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "P1WP"]
        #[inline(always)]
        pub fn set_p1wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "P2WP"]
        #[inline(always)]
        pub const fn p2wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "P2WP"]
        #[inline(always)]
        pub fn set_p2wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "P3WP"]
        #[inline(always)]
        pub const fn p3wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "P3WP"]
        #[inline(always)]
        pub fn set_p3wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "P4WP"]
        #[inline(always)]
        pub const fn p4wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "P4WP"]
        #[inline(always)]
        pub fn set_p4wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "P5WP"]
        #[inline(always)]
        pub const fn p5wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "P5WP"]
        #[inline(always)]
        pub fn set_p5wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "P6WP"]
        #[inline(always)]
        pub const fn p6wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "P6WP"]
        #[inline(always)]
        pub fn set_p6wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "P7WP"]
        #[inline(always)]
        pub const fn p7wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "P7WP"]
        #[inline(always)]
        pub fn set_p7wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "P8WP"]
        #[inline(always)]
        pub const fn p8wp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "P8WP"]
        #[inline(always)]
        pub fn set_p8wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "P9WP"]
        #[inline(always)]
        pub const fn p9wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "P9WP"]
        #[inline(always)]
        pub fn set_p9wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "P10WP"]
        #[inline(always)]
        pub const fn p10wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "P10WP"]
        #[inline(always)]
        pub fn set_p10wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "P11WP"]
        #[inline(always)]
        pub const fn p11wp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "P11WP"]
        #[inline(always)]
        pub fn set_p11wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "P12WP"]
        #[inline(always)]
        pub const fn p12wp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "P12WP"]
        #[inline(always)]
        pub fn set_p12wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "P13WP"]
        #[inline(always)]
        pub const fn p13wp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "P13WP"]
        #[inline(always)]
        pub fn set_p13wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "P14WP"]
        #[inline(always)]
        pub const fn p14wp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "P14WP"]
        #[inline(always)]
        pub fn set_p14wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "P15WP"]
        #[inline(always)]
        pub const fn p15wp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "P15WP"]
        #[inline(always)]
        pub fn set_p15wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "P16WP"]
        #[inline(always)]
        pub const fn p16wp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "P16WP"]
        #[inline(always)]
        pub fn set_p16wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "P17WP"]
        #[inline(always)]
        pub const fn p17wp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "P17WP"]
        #[inline(always)]
        pub fn set_p17wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "P18WP"]
        #[inline(always)]
        pub const fn p18wp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "P18WP"]
        #[inline(always)]
        pub fn set_p18wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "P19WP"]
        #[inline(always)]
        pub const fn p19wp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "P19WP"]
        #[inline(always)]
        pub fn set_p19wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "P20WP"]
        #[inline(always)]
        pub const fn p20wp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "P20WP"]
        #[inline(always)]
        pub fn set_p20wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "P21WP"]
        #[inline(always)]
        pub const fn p21wp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "P21WP"]
        #[inline(always)]
        pub fn set_p21wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "P22WP"]
        #[inline(always)]
        pub const fn p22wp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "P22WP"]
        #[inline(always)]
        pub fn set_p22wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "P23WP"]
        #[inline(always)]
        pub const fn p23wp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "P23WP"]
        #[inline(always)]
        pub fn set_p23wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "P24WP"]
        #[inline(always)]
        pub const fn p24wp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "P24WP"]
        #[inline(always)]
        pub fn set_p24wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "P25WP"]
        #[inline(always)]
        pub const fn p25wp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "P25WP"]
        #[inline(always)]
        pub fn set_p25wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "P26WP"]
        #[inline(always)]
        pub const fn p26wp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "P26WP"]
        #[inline(always)]
        pub fn set_p26wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "P27WP"]
        #[inline(always)]
        pub const fn p27wp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "P27WP"]
        #[inline(always)]
        pub fn set_p27wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "P28WP"]
        #[inline(always)]
        pub const fn p28wp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "P28WP"]
        #[inline(always)]
        pub fn set_p28wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "P29WP"]
        #[inline(always)]
        pub const fn p29wp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "P29WP"]
        #[inline(always)]
        pub fn set_p29wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "P30WP"]
        #[inline(always)]
        pub const fn p30wp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "P30WP"]
        #[inline(always)]
        pub fn set_p30wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM2 page 31 write protection"]
        #[inline(always)]
        pub const fn p31wp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 page 31 write protection"]
        #[inline(always)]
        pub fn set_p31wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("p0wp", &self.p0wp())
                .field("p1wp", &self.p1wp())
                .field("p2wp", &self.p2wp())
                .field("p3wp", &self.p3wp())
                .field("p4wp", &self.p4wp())
                .field("p5wp", &self.p5wp())
                .field("p6wp", &self.p6wp())
                .field("p7wp", &self.p7wp())
                .field("p8wp", &self.p8wp())
                .field("p9wp", &self.p9wp())
                .field("p10wp", &self.p10wp())
                .field("p11wp", &self.p11wp())
                .field("p12wp", &self.p12wp())
                .field("p13wp", &self.p13wp())
                .field("p14wp", &self.p14wp())
                .field("p15wp", &self.p15wp())
                .field("p16wp", &self.p16wp())
                .field("p17wp", &self.p17wp())
                .field("p18wp", &self.p18wp())
                .field("p19wp", &self.p19wp())
                .field("p20wp", &self.p20wp())
                .field("p21wp", &self.p21wp())
                .field("p22wp", &self.p22wp())
                .field("p23wp", &self.p23wp())
                .field("p24wp", &self.p24wp())
                .field("p25wp", &self.p25wp())
                .field("p26wp", &self.p26wp())
                .field("p27wp", &self.p27wp())
                .field("p28wp", &self.p28wp())
                .field("p29wp", &self.p29wp())
                .field("p30wp", &self.p30wp())
                .field("p31wp", &self.p31wp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Swpr {{ p0wp: {=bool:?}, p1wp: {=bool:?}, p2wp: {=bool:?}, p3wp: {=bool:?}, p4wp: {=bool:?}, p5wp: {=bool:?}, p6wp: {=bool:?}, p7wp: {=bool:?}, p8wp: {=bool:?}, p9wp: {=bool:?}, p10wp: {=bool:?}, p11wp: {=bool:?}, p12wp: {=bool:?}, p13wp: {=bool:?}, p14wp: {=bool:?}, p15wp: {=bool:?}, p16wp: {=bool:?}, p17wp: {=bool:?}, p18wp: {=bool:?}, p19wp: {=bool:?}, p20wp: {=bool:?}, p21wp: {=bool:?}, p22wp: {=bool:?}, p23wp: {=bool:?}, p24wp: {=bool:?}, p25wp: {=bool:?}, p26wp: {=bool:?}, p27wp: {=bool:?}, p28wp: {=bool:?}, p29wp: {=bool:?}, p30wp: {=bool:?}, p31wp: {=bool:?} }}" , self . p0wp () , self . p1wp () , self . p2wp () , self . p3wp () , self . p4wp () , self . p5wp () , self . p6wp () , self . p7wp () , self . p8wp () , self . p9wp () , self . p10wp () , self . p11wp () , self . p12wp () , self . p13wp () , self . p14wp () , self . p15wp () , self . p16wp () , self . p17wp () , self . p18wp () , self . p19wp () , self . p20wp () , self . p21wp () , self . p22wp () , self . p23wp () , self . p24wp () , self . p25wp () , self . p26wp () , self . p27wp () , self . p28wp () , self . p29wp () , self . p30wp () , self . p31wp ())
        }
    }
    #[doc = "SWPR2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr2(pub u32);
    impl Swpr2 {
        #[doc = "P32WP"]
        #[inline(always)]
        pub const fn p32wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "P32WP"]
        #[inline(always)]
        pub fn set_p32wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "P33WP"]
        #[inline(always)]
        pub const fn p33wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "P33WP"]
        #[inline(always)]
        pub fn set_p33wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "P34WP"]
        #[inline(always)]
        pub const fn p34wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "P34WP"]
        #[inline(always)]
        pub fn set_p34wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "P35WP"]
        #[inline(always)]
        pub const fn p35wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "P35WP"]
        #[inline(always)]
        pub fn set_p35wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "P36WP"]
        #[inline(always)]
        pub const fn p36wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "P36WP"]
        #[inline(always)]
        pub fn set_p36wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "P37WP"]
        #[inline(always)]
        pub const fn p37wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "P37WP"]
        #[inline(always)]
        pub fn set_p37wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "P38WP"]
        #[inline(always)]
        pub const fn p38wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "P38WP"]
        #[inline(always)]
        pub fn set_p38wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "P39WP"]
        #[inline(always)]
        pub const fn p39wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "P39WP"]
        #[inline(always)]
        pub fn set_p39wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "P40WP"]
        #[inline(always)]
        pub const fn p40wp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "P40WP"]
        #[inline(always)]
        pub fn set_p40wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "P41WP"]
        #[inline(always)]
        pub const fn p41wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "P41WP"]
        #[inline(always)]
        pub fn set_p41wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "P42WP"]
        #[inline(always)]
        pub const fn p42wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "P42WP"]
        #[inline(always)]
        pub fn set_p42wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "P43WP"]
        #[inline(always)]
        pub const fn p43wp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "P43WP"]
        #[inline(always)]
        pub fn set_p43wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "P44WP"]
        #[inline(always)]
        pub const fn p44wp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "P44WP"]
        #[inline(always)]
        pub fn set_p44wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "P45WP"]
        #[inline(always)]
        pub const fn p45wp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "P45WP"]
        #[inline(always)]
        pub fn set_p45wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "P46WP"]
        #[inline(always)]
        pub const fn p46wp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "P46WP"]
        #[inline(always)]
        pub fn set_p46wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "P47WP"]
        #[inline(always)]
        pub const fn p47wp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "P47WP"]
        #[inline(always)]
        pub fn set_p47wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "P48WP"]
        #[inline(always)]
        pub const fn p48wp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "P48WP"]
        #[inline(always)]
        pub fn set_p48wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "P49WP"]
        #[inline(always)]
        pub const fn p49wp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "P49WP"]
        #[inline(always)]
        pub fn set_p49wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "P50WP"]
        #[inline(always)]
        pub const fn p50wp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "P50WP"]
        #[inline(always)]
        pub fn set_p50wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "P51WP"]
        #[inline(always)]
        pub const fn p51wp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "P51WP"]
        #[inline(always)]
        pub fn set_p51wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "P52WP"]
        #[inline(always)]
        pub const fn p52wp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "P52WP"]
        #[inline(always)]
        pub fn set_p52wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "P53WP"]
        #[inline(always)]
        pub const fn p53wp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "P53WP"]
        #[inline(always)]
        pub fn set_p53wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "P54WP"]
        #[inline(always)]
        pub const fn p54wp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "P54WP"]
        #[inline(always)]
        pub fn set_p54wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "P55WP"]
        #[inline(always)]
        pub const fn p55wp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "P55WP"]
        #[inline(always)]
        pub fn set_p55wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "P56WP"]
        #[inline(always)]
        pub const fn p56wp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "P56WP"]
        #[inline(always)]
        pub fn set_p56wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "P57WP"]
        #[inline(always)]
        pub const fn p57wp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "P57WP"]
        #[inline(always)]
        pub fn set_p57wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "P58WP"]
        #[inline(always)]
        pub const fn p58wp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "P58WP"]
        #[inline(always)]
        pub fn set_p58wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "P59WP"]
        #[inline(always)]
        pub const fn p59wp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "P59WP"]
        #[inline(always)]
        pub fn set_p59wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "P60WP"]
        #[inline(always)]
        pub const fn p60wp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "P60WP"]
        #[inline(always)]
        pub fn set_p60wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "P61WP"]
        #[inline(always)]
        pub const fn p61wp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "P61WP"]
        #[inline(always)]
        pub fn set_p61wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "P62WP"]
        #[inline(always)]
        pub const fn p62wp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "P62WP"]
        #[inline(always)]
        pub fn set_p62wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "P63WP"]
        #[inline(always)]
        pub const fn p63wp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "P63WP"]
        #[inline(always)]
        pub fn set_p63wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Swpr2 {
        #[inline(always)]
        fn default() -> Swpr2 {
            Swpr2(0)
        }
    }
    impl core::fmt::Debug for Swpr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swpr2")
                .field("p32wp", &self.p32wp())
                .field("p33wp", &self.p33wp())
                .field("p34wp", &self.p34wp())
                .field("p35wp", &self.p35wp())
                .field("p36wp", &self.p36wp())
                .field("p37wp", &self.p37wp())
                .field("p38wp", &self.p38wp())
                .field("p39wp", &self.p39wp())
                .field("p40wp", &self.p40wp())
                .field("p41wp", &self.p41wp())
                .field("p42wp", &self.p42wp())
                .field("p43wp", &self.p43wp())
                .field("p44wp", &self.p44wp())
                .field("p45wp", &self.p45wp())
                .field("p46wp", &self.p46wp())
                .field("p47wp", &self.p47wp())
                .field("p48wp", &self.p48wp())
                .field("p49wp", &self.p49wp())
                .field("p50wp", &self.p50wp())
                .field("p51wp", &self.p51wp())
                .field("p52wp", &self.p52wp())
                .field("p53wp", &self.p53wp())
                .field("p54wp", &self.p54wp())
                .field("p55wp", &self.p55wp())
                .field("p56wp", &self.p56wp())
                .field("p57wp", &self.p57wp())
                .field("p58wp", &self.p58wp())
                .field("p59wp", &self.p59wp())
                .field("p60wp", &self.p60wp())
                .field("p61wp", &self.p61wp())
                .field("p62wp", &self.p62wp())
                .field("p63wp", &self.p63wp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Swpr2 {{ p32wp: {=bool:?}, p33wp: {=bool:?}, p34wp: {=bool:?}, p35wp: {=bool:?}, p36wp: {=bool:?}, p37wp: {=bool:?}, p38wp: {=bool:?}, p39wp: {=bool:?}, p40wp: {=bool:?}, p41wp: {=bool:?}, p42wp: {=bool:?}, p43wp: {=bool:?}, p44wp: {=bool:?}, p45wp: {=bool:?}, p46wp: {=bool:?}, p47wp: {=bool:?}, p48wp: {=bool:?}, p49wp: {=bool:?}, p50wp: {=bool:?}, p51wp: {=bool:?}, p52wp: {=bool:?}, p53wp: {=bool:?}, p54wp: {=bool:?}, p55wp: {=bool:?}, p56wp: {=bool:?}, p57wp: {=bool:?}, p58wp: {=bool:?}, p59wp: {=bool:?}, p60wp: {=bool:?}, p61wp: {=bool:?}, p62wp: {=bool:?}, p63wp: {=bool:?} }}" , self . p32wp () , self . p33wp () , self . p34wp () , self . p35wp () , self . p36wp () , self . p37wp () , self . p38wp () , self . p39wp () , self . p40wp () , self . p41wp () , self . p42wp () , self . p43wp () , self . p44wp () , self . p45wp () , self . p46wp () , self . p47wp () , self . p48wp () , self . p49wp () , self . p50wp () , self . p51wp () , self . p52wp () , self . p53wp () , self . p54wp () , self . p55wp () , self . p56wp () , self . p57wp () , self . p58wp () , self . p59wp () , self . p60wp () , self . p61wp () , self . p62wp () , self . p63wp ())
        }
    }
}
