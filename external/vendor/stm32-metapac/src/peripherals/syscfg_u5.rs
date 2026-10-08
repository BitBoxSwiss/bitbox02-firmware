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
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "memory erase status register"]
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::Mesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RSS command register"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "USB Type C and Power Delivery register"]
    #[inline(always)]
    pub const fn ucpdr(self) -> crate::common::Reg<regs::Ucpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "OTG_HS PHY register"]
    #[inline(always)]
    pub const fn otghsphycr(self) -> crate::common::Reg<regs::Otghsphycr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "OTG_HS PHY tune register 2"]
    #[inline(always)]
    pub const fn otghsphytuner2(self) -> crate::common::Reg<regs::Otghsphytuner2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "compensation cell code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "NCC1"]
        #[inline(always)]
        pub const fn ncc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NCC1"]
        #[inline(always)]
        pub fn set_ncc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PCC1"]
        #[inline(always)]
        pub const fn pcc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PCC1"]
        #[inline(always)]
        pub fn set_pcc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NCC2"]
        #[inline(always)]
        pub const fn ncc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NCC2"]
        #[inline(always)]
        pub fn set_ncc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PCC2"]
        #[inline(always)]
        pub const fn pcc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PCC2"]
        #[inline(always)]
        pub fn set_pcc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
                .field("ncc1", &self.ncc1())
                .field("pcc1", &self.pcc1())
                .field("ncc2", &self.ncc2())
                .field("pcc2", &self.pcc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cccr {{ ncc1: {=u8:?}, pcc1: {=u8:?}, ncc2: {=u8:?}, pcc2: {=u8:?} }}",
                self.ncc1(),
                self.pcc1(),
                self.ncc2(),
                self.pcc2()
            )
        }
    }
    #[doc = "compensation cell control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "EN1"]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EN1"]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CS1"]
        #[inline(always)]
        pub const fn cs1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CS1"]
        #[inline(always)]
        pub fn set_cs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EN2"]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EN2"]
        #[inline(always)]
        pub fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CS2"]
        #[inline(always)]
        pub const fn cs2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CS2"]
        #[inline(always)]
        pub fn set_cs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RDY1"]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RDY1"]
        #[inline(always)]
        pub fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RDY2"]
        #[inline(always)]
        pub const fn rdy2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RDY2"]
        #[inline(always)]
        pub fn set_rdy2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("en1", &self.en1())
                .field("cs1", &self.cs1())
                .field("en2", &self.en2())
                .field("cs2", &self.cs2())
                .field("rdy1", &self.rdy1())
                .field("rdy2", &self.rdy2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cccsr {{ en1: {=bool:?}, cs1: {=bool:?}, en2: {=bool:?}, cs2: {=bool:?}, rdy1: {=bool:?}, rdy2: {=bool:?} }}" , self . en1 () , self . cs1 () , self . en2 () , self . cs2 () , self . rdy1 () , self . rdy2 ())
        }
    }
    #[doc = "compensation cell value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "NCV1"]
        #[inline(always)]
        pub const fn ncv1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NCV1"]
        #[inline(always)]
        pub fn set_ncv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PCV1"]
        #[inline(always)]
        pub const fn pcv1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PCV1"]
        #[inline(always)]
        pub fn set_pcv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NCV2"]
        #[inline(always)]
        pub const fn ncv2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NCV2"]
        #[inline(always)]
        pub fn set_ncv2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PCV2"]
        #[inline(always)]
        pub const fn pcv2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PCV2"]
        #[inline(always)]
        pub fn set_pcv2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
                .field("ncv1", &self.ncv1())
                .field("pcv1", &self.pcv1())
                .field("ncv2", &self.ncv2())
                .field("pcv2", &self.pcv2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccvr {{ ncv1: {=u8:?}, pcv1: {=u8:?}, ncv2: {=u8:?}, pcv2: {=u8:?} }}",
                self.ncv1(),
                self.pcv1(),
                self.ncv2(),
                self.pcv2()
            )
        }
    }
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
        #[doc = "PB6_FMP"]
        #[inline(always)]
        pub const fn pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_FMP"]
        #[inline(always)]
        pub fn set_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB7_FMP"]
        #[inline(always)]
        pub const fn pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_FMP"]
        #[inline(always)]
        pub fn set_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB8_FMP"]
        #[inline(always)]
        pub const fn pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_FMP"]
        #[inline(always)]
        pub fn set_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB9_FMP"]
        #[inline(always)]
        pub const fn pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB9_FMP"]
        #[inline(always)]
        pub fn set_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("pb6_fmp", &self.pb6_fmp())
                .field("pb7_fmp", &self.pb7_fmp())
                .field("pb8_fmp", &self.pb8_fmp())
                .field("pb9_fmp", &self.pb9_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ boosten: {=bool:?}, anaswvdd: {=bool:?}, pb6_fmp: {=bool:?}, pb7_fmp: {=bool:?}, pb8_fmp: {=bool:?}, pb9_fmp: {=bool:?} }}" , self . boosten () , self . anaswvdd () , self . pb6_fmp () , self . pb7_fmp () , self . pb8_fmp () , self . pb9_fmp ())
        }
    }
    #[doc = "configuration register 2"]
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
        #[doc = "SRAM ECC lock bit"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM ECC lock bit"]
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ cll: {=bool:?}, spl: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?} }}",
                self.cll(),
                self.spl(),
                self.pvdl(),
                self.eccl()
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
    #[doc = "memory erase status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mesr(pub u32);
    impl Mesr {
        #[doc = "MCLR"]
        #[inline(always)]
        pub const fn mclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MCLR"]
        #[inline(always)]
        pub fn set_mclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IPMEE"]
        #[inline(always)]
        pub const fn ipmee(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IPMEE"]
        #[inline(always)]
        pub fn set_ipmee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mesr {
        #[inline(always)]
        fn default() -> Mesr {
            Mesr(0)
        }
    }
    impl core::fmt::Debug for Mesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mesr")
                .field("mclr", &self.mclr())
                .field("ipmee", &self.ipmee())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mesr {{ mclr: {=bool:?}, ipmee: {=bool:?} }}",
                self.mclr(),
                self.ipmee()
            )
        }
    }
    #[doc = "OTG_HS PHY register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otghsphycr(pub u32);
    impl Otghsphycr {
        #[doc = "PHY Enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Common block power-down control"]
        #[inline(always)]
        pub const fn pdctrl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Common block power-down control"]
        #[inline(always)]
        pub fn set_pdctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reference clock frequency selection"]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Usbrefcksel {
            let val = (self.0 >> 2usize) & 0x0f;
            super::vals::Usbrefcksel::from_bits(val as u8)
        }
        #[doc = "Reference clock frequency selection"]
        #[inline(always)]
        pub fn set_clksel(&mut self, val: super::vals::Usbrefcksel) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
        }
    }
    impl Default for Otghsphycr {
        #[inline(always)]
        fn default() -> Otghsphycr {
            Otghsphycr(0)
        }
    }
    impl core::fmt::Debug for Otghsphycr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otghsphycr")
                .field("en", &self.en())
                .field("pdctrl", &self.pdctrl())
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otghsphycr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otghsphycr {{ en: {=bool:?}, pdctrl: {=bool:?}, clksel: {:?} }}",
                self.en(),
                self.pdctrl(),
                self.clksel()
            )
        }
    }
    #[doc = "OTG_HS tune register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otghsphytuner2(pub u32);
    impl Otghsphytuner2 {
        #[doc = "Disconnect threshold adjustment"]
        #[inline(always)]
        pub const fn compdistune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Disconnect threshold adjustment"]
        #[inline(always)]
        pub fn set_compdistune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Squelch threshold adjustment"]
        #[inline(always)]
        pub const fn sqrxtune(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Squelch threshold adjustment"]
        #[inline(always)]
        pub fn set_sqrxtune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "HS transmitter preemphasis current control"]
        #[inline(always)]
        pub const fn txpreempamptune(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "HS transmitter preemphasis current control"]
        #[inline(always)]
        pub fn set_txpreempamptune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
    }
    impl Default for Otghsphytuner2 {
        #[inline(always)]
        fn default() -> Otghsphytuner2 {
            Otghsphytuner2(0)
        }
    }
    impl core::fmt::Debug for Otghsphytuner2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otghsphytuner2")
                .field("compdistune", &self.compdistune())
                .field("sqrxtune", &self.sqrxtune())
                .field("txpreempamptune", &self.txpreempamptune())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otghsphytuner2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otghsphytuner2 {{ compdistune: {=u8:?}, sqrxtune: {=u8:?}, txpreempamptune: {=u8:?} }}",
                self.compdistune(),
                self.sqrxtune(),
                self.txpreempamptune()
            )
        }
    }
    #[doc = "RSS command register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS commands"]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RSS commands"]
        #[inline(always)]
        pub fn set_rsscmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            defmt::write!(f, "Rsscmdr {{ rsscmd: {=u16:?} }}", self.rsscmd())
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
        #[doc = "CLASSBSEC"]
        #[inline(always)]
        pub const fn classbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CLASSBSEC"]
        #[inline(always)]
        pub fn set_classbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("fpusec", &self.fpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ syscfgsec: {=bool:?}, classbsec: {=bool:?}, fpusec: {=bool:?} }}",
                self.syscfgsec(),
                self.classbsec(),
                self.fpusec()
            )
        }
    }
    #[doc = "USB Type C and Power Delivery register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucpdr(pub u32);
    impl Ucpdr {
        #[doc = "CC1ENRXFILTER"]
        #[inline(always)]
        pub const fn cc1enrxfilter(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC1ENRXFILTER"]
        #[inline(always)]
        pub fn set_cc1enrxfilter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC2ENRXFILTER"]
        #[inline(always)]
        pub const fn cc2enrxfilter(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC2ENRXFILTER"]
        #[inline(always)]
        pub fn set_cc2enrxfilter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ucpdr {
        #[inline(always)]
        fn default() -> Ucpdr {
            Ucpdr(0)
        }
    }
    impl core::fmt::Debug for Ucpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ucpdr")
                .field("cc1enrxfilter", &self.cc1enrxfilter())
                .field("cc2enrxfilter", &self.cc2enrxfilter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ucpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ucpdr {{ cc1enrxfilter: {=bool:?}, cc2enrxfilter: {=bool:?} }}",
                self.cc1enrxfilter(),
                self.cc2enrxfilter()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbrefcksel {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 16 MHz."]
        MHZ16 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 19.2 MHz."]
        MHZ19_2 = 0x08,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 20MHz."]
        MHZ20 = 0x09,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 24 MHz (default after reset)."]
        MHZ24 = 0x0a,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 32 MHz."]
        MHZ32 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 26 MHz."]
        MHZ26 = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Usbrefcksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbrefcksel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbrefcksel {
        #[inline(always)]
        fn from(val: u8) -> Usbrefcksel {
            Usbrefcksel::from_bits(val)
        }
    }
    impl From<Usbrefcksel> for u8 {
        #[inline(always)]
        fn from(val: Usbrefcksel) -> u8 {
            Usbrefcksel::to_bits(val)
        }
    }
}
