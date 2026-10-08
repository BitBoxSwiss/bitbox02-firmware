#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Backup registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp {
    ptr: *mut u8,
}
unsafe impl Send for Bkp {}
unsafe impl Sync for Bkp {}
impl Bkp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self, n: usize) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        assert!(n < 42usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.add(
                0x0usize
                    + ([
                        4usize, 8usize, 12usize, 16usize, 20usize, 24usize, 28usize, 32usize, 36usize, 40usize,
                        64usize, 68usize, 72usize, 76usize, 80usize, 84usize, 88usize, 92usize, 96usize, 100usize,
                        104usize, 108usize, 112usize, 116usize, 120usize, 124usize, 128usize, 132usize, 136usize,
                        140usize, 144usize, 148usize, 152usize, 156usize, 160usize, 164usize, 168usize, 172usize,
                        176usize, 180usize, 184usize, 188usize,
                    ][n] as usize),
            ) as _)
        }
    }
    #[doc = "RTC clock calibration register"]
    #[inline(always)]
    pub const fn rtccr(self) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Tamper pin enable"]
        #[inline(always)]
        pub const fn tpe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper pin enable"]
        #[inline(always)]
        pub fn set_tpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tamper pin active level"]
        #[inline(always)]
        pub const fn tpal(&self) -> super::vals::Tpal {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Tpal::from_bits(val as u8)
        }
        #[doc = "Tamper pin active level"]
        #[inline(always)]
        pub fn set_tpal(&mut self, val: super::vals::Tpal) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
                .field("tpe", &self.tpe())
                .field("tpal", &self.tpal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ tpe: {=bool:?}, tpal: {:?} }}", self.tpe(), self.tpal())
        }
    }
    #[doc = "Control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Clear Tamper event"]
        #[inline(always)]
        pub const fn cte(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Tamper event"]
        #[inline(always)]
        pub fn set_cte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear Tamper Interrupt"]
        #[inline(always)]
        pub const fn cti(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Tamper Interrupt"]
        #[inline(always)]
        pub fn set_cti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Tamper Pin interrupt enable"]
        #[inline(always)]
        pub const fn tpie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper Pin interrupt enable"]
        #[inline(always)]
        pub fn set_tpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tamper Event Flag"]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper Event Flag"]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tamper Interrupt Flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper Interrupt Flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("cte", &self.cte())
                .field("cti", &self.cti())
                .field("tpie", &self.tpie())
                .field("tef", &self.tef())
                .field("tif", &self.tif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ cte: {=bool:?}, cti: {=bool:?}, tpie: {=bool:?}, tef: {=bool:?}, tif: {=bool:?} }}",
                self.cte(),
                self.cti(),
                self.tpie(),
                self.tef(),
                self.tif()
            )
        }
    }
    #[doc = "Data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Backup data"]
        #[inline(always)]
        pub const fn d(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Backup data"]
        #[inline(always)]
        pub fn set_d(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("d", &self.d()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ d: {=u16:?} }}", self.d())
        }
    }
    #[doc = "RTC clock calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtccr(pub u32);
    impl Rtccr {
        #[doc = "Calibration value"]
        #[inline(always)]
        pub const fn cal(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Calibration value"]
        #[inline(always)]
        pub fn set_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Calibration Clock Output"]
        #[inline(always)]
        pub const fn cco(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Clock Output"]
        #[inline(always)]
        pub fn set_cco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Alarm or second output enable"]
        #[inline(always)]
        pub const fn asoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm or second output enable"]
        #[inline(always)]
        pub fn set_asoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Alarm or second output selection"]
        #[inline(always)]
        pub const fn asos(&self) -> super::vals::Asos {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Asos::from_bits(val as u8)
        }
        #[doc = "Alarm or second output selection"]
        #[inline(always)]
        pub fn set_asos(&mut self, val: super::vals::Asos) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Rtccr {
        #[inline(always)]
        fn default() -> Rtccr {
            Rtccr(0)
        }
    }
    impl core::fmt::Debug for Rtccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtccr")
                .field("cal", &self.cal())
                .field("cco", &self.cco())
                .field("asoe", &self.asoe())
                .field("asos", &self.asos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rtccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rtccr {{ cal: {=u8:?}, cco: {=bool:?}, asoe: {=bool:?}, asos: {:?} }}",
                self.cal(),
                self.cco(),
                self.asoe(),
                self.asos()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Asos {
        #[doc = "RTC Alarm pulse output selected"]
        ALARM = 0x0,
        #[doc = "RTC Second pulse output selected"]
        SECOND = 0x01,
    }
    impl Asos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Asos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Asos {
        #[inline(always)]
        fn from(val: u8) -> Asos {
            Asos::from_bits(val)
        }
    }
    impl From<Asos> for u8 {
        #[inline(always)]
        fn from(val: Asos) -> u8 {
            Asos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tpal {
        #[doc = "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
        HIGH = 0x0,
        #[doc = "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
        LOW = 0x01,
    }
    impl Tpal {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tpal {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tpal {
        #[inline(always)]
        fn from(val: u8) -> Tpal {
            Tpal::from_bits(val)
        }
    }
    impl From<Tpal> for u8 {
        #[inline(always)]
        fn from(val: Tpal) -> u8 {
            Tpal::to_bits(val)
        }
    }
}
