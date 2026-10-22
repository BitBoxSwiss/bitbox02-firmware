#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control"]
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
    #[doc = "Power control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Power control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Power control register 4"]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Power status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Power status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Power status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn pucr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn pdcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Power control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "Low-power run"]
        #[inline(always)]
        pub const fn lpr(&self) -> super::vals::Lpr {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Lpr::from_bits(val as u8)
        }
        #[doc = "Low-power run"]
        #[inline(always)]
        pub fn set_lpr(&mut self, val: super::vals::Lpr) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
                .field("dbp", &self.dbp())
                .field("vos", &self.vos())
                .field("lpr", &self.lpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ lpms: {:?}, dbp: {=bool:?}, vos: {:?}, lpr: {:?} }}",
                self.lpms(),
                self.dbp(),
                self.vos(),
                self.lpr()
            )
        }
    }
    #[doc = "Power control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power voltage detector level selection"]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "Power voltage detector level selection"]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
        #[inline(always)]
        pub const fn pvme1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
        #[inline(always)]
        pub fn set_pvme1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
        #[inline(always)]
        pub const fn pvme2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
        #[inline(always)]
        pub fn set_pvme2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub const fn pvme3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub fn set_pvme3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
        #[inline(always)]
        pub const fn pvme4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
        #[inline(always)]
        pub fn set_pvme4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "VDDIO2 Independent I/Os supply valid"]
        #[inline(always)]
        pub const fn iosv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 Independent I/Os supply valid"]
        #[inline(always)]
        pub fn set_iosv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "VDDUSB USB supply valid"]
        #[inline(always)]
        pub const fn usv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "VDDUSB USB supply valid"]
        #[inline(always)]
        pub fn set_usv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("pvme1", &self.pvme1())
                .field("pvme2", &self.pvme2())
                .field("pvme3", &self.pvme3())
                .field("pvme4", &self.pvme4())
                .field("iosv", &self.iosv())
                .field("usv", &self.usv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ pvde: {=bool:?}, pls: {:?}, pvme1: {=bool:?}, pvme2: {=bool:?}, pvme3: {=bool:?}, pvme4: {=bool:?}, iosv: {=bool:?}, usv: {=bool:?} }}" , self . pvde () , self . pls () , self . pvme1 () , self . pvme2 () , self . pvme3 () , self . pvme4 () , self . iosv () , self . usv ())
        }
    }
    #[doc = "Power control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Enable Wakeup pin WKUP"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub const fn rrs(&self) -> super::vals::Rrs {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Rrs::from_bits(val as u8)
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub fn set_rrs(&mut self, val: super::vals::Rrs) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Apply pull-up and pull-down configuration"]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable internal wakeup line"]
        #[inline(always)]
        pub const fn ewf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line"]
        #[inline(always)]
        pub fn set_ewf(&mut self, val: bool) {
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
                .field("rrs", &self.rrs())
                .field("apc", &self.apc())
                .field("ewf", &self.ewf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ ewup[0]: {=bool:?}, ewup[1]: {=bool:?}, ewup[2]: {=bool:?}, ewup[3]: {=bool:?}, ewup[4]: {=bool:?}, rrs: {:?}, apc: {=bool:?}, ewf: {=bool:?} }}" , self . ewup (0usize) , self . ewup (1usize) , self . ewup (2usize) , self . ewup (3usize) , self . ewup (4usize) , self . rrs () , self . apc () , self . ewf ())
        }
    }
    #[doc = "Power control register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub const fn wp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub fn set_wp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup pin WKUP2 polarity"]
        #[inline(always)]
        pub const fn wp2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP2 polarity"]
        #[inline(always)]
        pub fn set_wp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup pin WKUP3 polarity"]
        #[inline(always)]
        pub const fn wp3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP3 polarity"]
        #[inline(always)]
        pub fn set_wp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup pin WKUP4 polarity"]
        #[inline(always)]
        pub const fn wp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP4 polarity"]
        #[inline(always)]
        pub fn set_wp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup pin WKUP5 polarity"]
        #[inline(always)]
        pub const fn wp5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP5 polarity"]
        #[inline(always)]
        pub fn set_wp5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("wp1", &self.wp1())
                .field("wp2", &self.wp2())
                .field("wp3", &self.wp3())
                .field("wp4", &self.wp4())
                .field("wp5", &self.wp5())
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr4 {{ wp1: {=bool:?}, wp2: {=bool:?}, wp3: {=bool:?}, wp4: {=bool:?}, wp5: {=bool:?}, vbe: {=bool:?}, vbrs: {=bool:?} }}" , self . wp1 () , self . wp2 () , self . wp3 () , self . wp4 () , self . wp5 () , self . vbe () , self . vbrs ())
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
    #[doc = "Power status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear wakeup flag"]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag"]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear standby flag"]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear standby flag"]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
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
                .field("sbf", &self.sbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ cwuf[0]: {=bool:?}, cwuf[1]: {=bool:?}, cwuf[2]: {=bool:?}, cwuf[3]: {=bool:?}, cwuf[4]: {=bool:?}, sbf: {=bool:?} }}" , self . cwuf (0usize) , self . cwuf (1usize) , self . cwuf (2usize) , self . cwuf (3usize) , self . cwuf (4usize) , self . sbf ())
        }
    }
    #[doc = "Power status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup flag 2"]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 2"]
        #[inline(always)]
        pub fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup flag 3"]
        #[inline(always)]
        pub const fn cwuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 3"]
        #[inline(always)]
        pub fn set_cwuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup flag 4"]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 4"]
        #[inline(always)]
        pub fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup flag 5"]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 5"]
        #[inline(always)]
        pub fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Standby flag"]
        #[inline(always)]
        pub const fn csbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Standby flag"]
        #[inline(always)]
        pub fn set_csbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wakeup flag internal"]
        #[inline(always)]
        pub const fn wufi(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag internal"]
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
                .field("cwuf1", &self.cwuf1())
                .field("cwuf2", &self.cwuf2())
                .field("cwuf3", &self.cwuf3())
                .field("cwuf4", &self.cwuf4())
                .field("cwuf5", &self.cwuf5())
                .field("csbf", &self.csbf())
                .field("wufi", &self.wufi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1 {{ cwuf1: {=bool:?}, cwuf2: {=bool:?}, cwuf3: {=bool:?}, cwuf4: {=bool:?}, cwuf5: {=bool:?}, csbf: {=bool:?}, wufi: {=bool:?} }}" , self . cwuf1 () , self . cwuf2 () , self . cwuf3 () , self . cwuf4 () , self . cwuf5 () , self . csbf () , self . wufi ())
        }
    }
    #[doc = "Power status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "Low-power regulator started"]
        #[inline(always)]
        pub const fn reglps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator started"]
        #[inline(always)]
        pub fn set_reglps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power regulator flag"]
        #[inline(always)]
        pub const fn reglpf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator flag"]
        #[inline(always)]
        pub fn set_reglpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub const fn vosf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub fn set_vosf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
        #[inline(always)]
        pub const fn pvmo1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
        #[inline(always)]
        pub fn set_pvmo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
        #[inline(always)]
        pub const fn pvmo2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
        #[inline(always)]
        pub fn set_pvmo2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub const fn pvmo3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub fn set_pvmo3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
        #[inline(always)]
        pub const fn pvmo4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
        #[inline(always)]
        pub fn set_pvmo4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            f.debug_struct("Sr2")
                .field("reglps", &self.reglps())
                .field("reglpf", &self.reglpf())
                .field("vosf", &self.vosf())
                .field("pvdo", &self.pvdo())
                .field("pvmo1", &self.pvmo1())
                .field("pvmo2", &self.pvmo2())
                .field("pvmo3", &self.pvmo3())
                .field("pvmo4", &self.pvmo4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr2 {{ reglps: {=bool:?}, reglpf: {=bool:?}, vosf: {=bool:?}, pvdo: {=bool:?}, pvmo1: {=bool:?}, pvmo2: {=bool:?}, pvmo3: {=bool:?}, pvmo4: {=bool:?} }}" , self . reglps () , self . reglpf () , self . vosf () , self . pvdo () , self . pvmo1 () , self . pvmo2 () , self . pvmo3 () , self . pvmo4 ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "Stop 0 mode"]
        STOP0 = 0x0,
        #[doc = "Stop 1 mode"]
        STOP1 = 0x01,
        #[doc = "Stop 2 mode"]
        STOP2 = 0x02,
        #[doc = "Standby mode"]
        STANDBY = 0x03,
        #[doc = "Shutdown mode"]
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpr {
        #[doc = "Voltage regulator in Main mode"]
        MAIN_MODE = 0x0,
        #[doc = "Voltage regulator in low-power mode"]
        LOW_POWER_MODE = 0x01,
    }
    impl Lpr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpr {
        #[inline(always)]
        fn from(val: u8) -> Lpr {
            Lpr::from_bits(val)
        }
    }
    impl From<Lpr> for u8 {
        #[inline(always)]
        fn from(val: Lpr) -> u8 {
            Lpr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "2.0V"]
        V2_0 = 0x0,
        #[doc = "2.2V"]
        V2_2 = 0x01,
        #[doc = "2.4V"]
        V2_4 = 0x02,
        #[doc = "2.5V"]
        V2_5 = 0x03,
        #[doc = "2.6V"]
        V2_6 = 0x04,
        #[doc = "2.8V"]
        V2_8 = 0x05,
        #[doc = "2.9V"]
        V2_9 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
        EXTERNAL = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rrs {
        #[doc = "SRAM2 powered off in Standby mode (SRAM2 content lost)"]
        POWER_OFF = 0x0,
        #[doc = "SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)"]
        ON_LPR = 0x01,
    }
    impl Rrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rrs {
        #[inline(always)]
        fn from(val: u8) -> Rrs {
            Rrs::from_bits(val)
        }
    }
    impl From<Rrs> for u8 {
        #[inline(always)]
        fn from(val: Rrs) -> u8 {
            Rrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "Range 1"]
        RANGE1 = 0x01,
        #[doc = "Range 2"]
        RANGE2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
}
