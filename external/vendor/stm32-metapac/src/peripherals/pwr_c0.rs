#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWR address block description"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PWR control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWR control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PWR control register 4"]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PWR status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PWR status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PWR status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PWR Port pull-up control register"]
    #[inline(always)]
    pub const fn pucr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "PWR Port pull-down control register"]
    #[inline(always)]
    pub const fn pdcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
        #[inline(always)]
        pub const fn fpd_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
        #[inline(always)]
        pub fn set_fpd_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
        #[inline(always)]
        pub const fn fpd_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
        #[inline(always)]
        pub fn set_fpd_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("lpms", &self.lpms())
                .field("fpd_stop", &self.fpd_stop())
                .field("fpd_slp", &self.fpd_slp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ lpms: {:?}, fpd_stop: {=bool:?}, fpd_slp: {=bool:?} }}",
                self.lpms(),
                self.fpd_stop(),
                self.fpd_slp()
            )
        }
    }
    #[doc = "PWR control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Enable Wakeup pin"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("ewup[0]", &self.ewup(0usize))
                .field("ewup[1]", &self.ewup(1usize))
                .field("ewup[2]", &self.ewup(2usize))
                .field("ewup[3]", &self.ewup(3usize))
                .field("ewup[4]", &self.ewup(4usize))
                .field("ewup[5]", &self.ewup(5usize))
                .field("apc", &self.apc())
                .field("eiwul", &self.eiwul())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ ewup[0]: {=bool:?}, ewup[1]: {=bool:?}, ewup[2]: {=bool:?}, ewup[3]: {=bool:?}, ewup[4]: {=bool:?}, ewup[5]: {=bool:?}, apc: {=bool:?}, eiwul: {=bool:?} }}" , self . ewup (0usize) , self . ewup (1usize) , self . ewup (2usize) , self . ewup (3usize) , self . ewup (4usize) , self . ewup (5usize) , self . apc () , self . eiwul ())
        }
    }
    #[doc = "PWR control register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub const fn wp(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub fn set_wp(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr4 {
        #[inline(always)]
        fn default() -> Cr4 {
            Cr4(0)
        }
    }
    impl core::fmt::Debug for Cr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr4")
                .field("wp[0]", &self.wp(0usize))
                .field("wp[1]", &self.wp(1usize))
                .field("wp[2]", &self.wp(2usize))
                .field("wp[3]", &self.wp(3usize))
                .field("wp[4]", &self.wp(4usize))
                .field("wp[5]", &self.wp(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr4 {{ wp[0]: {=bool:?}, wp[1]: {=bool:?}, wp[2]: {=bool:?}, wp[3]: {=bool:?}, wp[4]: {=bool:?}, wp[5]: {=bool:?} }}" , self . wp (0usize) , self . wp (1usize) , self . wp (2usize) , self . wp (3usize) , self . wp (4usize) , self . wp (5usize))
        }
    }
    #[doc = "Power Port pull control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub const fn p(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_p(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field("p[0]", &self.p(0usize))
                .field("p[1]", &self.p(1usize))
                .field("p[2]", &self.p(2usize))
                .field("p[3]", &self.p(3usize))
                .field("p[4]", &self.p(4usize))
                .field("p[5]", &self.p(5usize))
                .field("p[6]", &self.p(6usize))
                .field("p[7]", &self.p(7usize))
                .field("p[8]", &self.p(8usize))
                .field("p[9]", &self.p(9usize))
                .field("p[10]", &self.p(10usize))
                .field("p[11]", &self.p(11usize))
                .field("p[12]", &self.p(12usize))
                .field("p[13]", &self.p(13usize))
                .field("p[14]", &self.p(14usize))
                .field("p[15]", &self.p(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcr {{ p[0]: {=bool:?}, p[1]: {=bool:?}, p[2]: {=bool:?}, p[3]: {=bool:?}, p[4]: {=bool:?}, p[5]: {=bool:?}, p[6]: {=bool:?}, p[7]: {=bool:?}, p[8]: {=bool:?}, p[9]: {=bool:?}, p[10]: {=bool:?}, p[11]: {=bool:?}, p[12]: {=bool:?}, p[13]: {=bool:?}, p[14]: {=bool:?}, p[15]: {=bool:?} }}" , self . p (0usize) , self . p (1usize) , self . p (2usize) , self . p (3usize) , self . p (4usize) , self . p (5usize) , self . p (6usize) , self . p (7usize) , self . p (8usize) , self . p (9usize) , self . p (10usize) , self . p (11usize) , self . p (12usize) , self . p (13usize) , self . p (14usize) , self . p (15usize))
        }
    }
    #[doc = "PWR status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear Wakeup flag"]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear Wakeup flag"]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn csbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_csbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("cwuf[0]", &self.cwuf(0usize))
                .field("cwuf[1]", &self.cwuf(1usize))
                .field("cwuf[2]", &self.cwuf(2usize))
                .field("cwuf[3]", &self.cwuf(3usize))
                .field("cwuf[4]", &self.cwuf(4usize))
                .field("cwuf[5]", &self.cwuf(5usize))
                .field("csbf", &self.csbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ cwuf[0]: {=bool:?}, cwuf[1]: {=bool:?}, cwuf[2]: {=bool:?}, cwuf[3]: {=bool:?}, cwuf[4]: {=bool:?}, cwuf[5]: {=bool:?}, csbf: {=bool:?} }}" , self . cwuf (0usize) , self . cwuf (1usize) , self . cwuf (2usize) , self . cwuf (3usize) , self . cwuf (4usize) , self . cwuf (5usize) , self . csbf ())
        }
    }
    #[doc = "PWR status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wakeup flag"]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag"]
        #[inline(always)]
        pub fn set_wuf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Standby/Shutdown flag This bit is set by hardware when the device enters Standby or Shutdown mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Standby/Shutdown flag This bit is set by hardware when the device enters Standby or Shutdown mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wakeup flag internal This bit is set when a wakeup condition is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
        #[inline(always)]
        pub const fn wufi(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag internal This bit is set when a wakeup condition is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
        #[inline(always)]
        pub fn set_wufi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
    impl core::fmt::Debug for Sr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1")
                .field("wuf[0]", &self.wuf(0usize))
                .field("wuf[1]", &self.wuf(1usize))
                .field("wuf[2]", &self.wuf(2usize))
                .field("wuf[3]", &self.wuf(3usize))
                .field("wuf[4]", &self.wuf(4usize))
                .field("wuf[5]", &self.wuf(5usize))
                .field("sbf", &self.sbf())
                .field("wufi", &self.wufi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1 {{ wuf[0]: {=bool:?}, wuf[1]: {=bool:?}, wuf[2]: {=bool:?}, wuf[3]: {=bool:?}, wuf[4]: {=bool:?}, wuf[5]: {=bool:?}, sbf: {=bool:?}, wufi: {=bool:?} }}" , self . wuf (0usize) , self . wuf (1usize) , self . wuf (2usize) , self . wuf (3usize) , self . wuf (4usize) , self . wuf (5usize) , self . sbf () , self . wufi ())
        }
    }
    #[doc = "PWR status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory."]
        #[inline(always)]
        pub const fn flash_rdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory."]
        #[inline(always)]
        pub fn set_flash_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
    impl core::fmt::Debug for Sr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2").field("flash_rdy", &self.flash_rdy()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sr2 {{ flash_rdy: {=bool:?} }}", self.flash_rdy())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "Selects Stop mode when entering DeepSleep."]
        STOP = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Selects Standby mode when entering DeepSleep."]
        STANDBY = 0x03,
        #[doc = "Selects Shutdown mode when entering DeepSleep."]
        SHUTDOWN = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpms {
        #[inline(always)]
        fn from(val: u8) -> Lpms {
            Lpms::from_bits(val)
        }
    }
    impl From<Lpms> for u8 {
        #[inline(always)]
        fn from(val: Lpms) -> u8 {
            Lpms::to_bits(val)
        }
    }
}
