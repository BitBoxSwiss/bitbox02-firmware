#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Clock recovery system"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs {
    ptr: *mut u8,
}
unsafe impl Send for Crs {}
unsafe impl Sync for Crs {}
impl Crs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Counter reload value"]
        #[inline(always)]
        pub const fn reload(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter reload value"]
        #[inline(always)]
        pub fn set_reload(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Frequency error limit"]
        #[inline(always)]
        pub const fn felim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Frequency error limit"]
        #[inline(always)]
        pub fn set_felim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "SYNC divider"]
        #[inline(always)]
        pub const fn syncdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "SYNC divider"]
        #[inline(always)]
        pub fn set_syncdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "SYNC signal source selection"]
        #[inline(always)]
        pub const fn syncsrc(&self) -> super::vals::Syncsrc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Syncsrc::from_bits(val as u8)
        }
        #[doc = "SYNC signal source selection"]
        #[inline(always)]
        pub fn set_syncsrc(&mut self, val: super::vals::Syncsrc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "SYNC polarity selection"]
        #[inline(always)]
        pub const fn syncpol(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC polarity selection"]
        #[inline(always)]
        pub fn set_syncpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("reload", &self.reload())
                .field("felim", &self.felim())
                .field("syncdiv", &self.syncdiv())
                .field("syncsrc", &self.syncsrc())
                .field("syncpol", &self.syncpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ reload: {=u16:?}, felim: {=u8:?}, syncdiv: {=u8:?}, syncsrc: {:?}, syncpol: {=bool:?} }}",
                self.reload(),
                self.felim(),
                self.syncdiv(),
                self.syncsrc(),
                self.syncpol()
            )
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "SYNC event OK interrupt enable"]
        #[inline(always)]
        pub const fn syncokie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC event OK interrupt enable"]
        #[inline(always)]
        pub fn set_syncokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SYNC warning interrupt enable"]
        #[inline(always)]
        pub const fn syncwarnie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC warning interrupt enable"]
        #[inline(always)]
        pub fn set_syncwarnie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronization or trimming error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization or trimming error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Expected SYNC interrupt enable"]
        #[inline(always)]
        pub const fn esyncie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Expected SYNC interrupt enable"]
        #[inline(always)]
        pub fn set_esyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Frequency error counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Frequency error counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic trimming enable"]
        #[inline(always)]
        pub const fn autotrimen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic trimming enable"]
        #[inline(always)]
        pub fn set_autotrimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Generate software SYNC event"]
        #[inline(always)]
        pub const fn swsync(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Generate software SYNC event"]
        #[inline(always)]
        pub fn set_swsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HSI48 oscillator smooth trimming"]
        #[inline(always)]
        pub const fn trim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSI48 oscillator smooth trimming"]
        #[inline(always)]
        pub fn set_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
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
                .field("syncokie", &self.syncokie())
                .field("syncwarnie", &self.syncwarnie())
                .field("errie", &self.errie())
                .field("esyncie", &self.esyncie())
                .field("cen", &self.cen())
                .field("autotrimen", &self.autotrimen())
                .field("swsync", &self.swsync())
                .field("trim", &self.trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ syncokie: {=bool:?}, syncwarnie: {=bool:?}, errie: {=bool:?}, esyncie: {=bool:?}, cen: {=bool:?}, autotrimen: {=bool:?}, swsync: {=bool:?}, trim: {=u8:?} }}" , self . syncokie () , self . syncwarnie () , self . errie () , self . esyncie () , self . cen () , self . autotrimen () , self . swsync () , self . trim ())
        }
    }
    #[doc = "interrupt flag clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "SYNC event OK clear flag"]
        #[inline(always)]
        pub const fn syncokc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC event OK clear flag"]
        #[inline(always)]
        pub fn set_syncokc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SYNC warning clear flag"]
        #[inline(always)]
        pub const fn syncwarnc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC warning clear flag"]
        #[inline(always)]
        pub fn set_syncwarnc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error clear flag"]
        #[inline(always)]
        pub const fn errc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error clear flag"]
        #[inline(always)]
        pub fn set_errc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Expected SYNC clear flag"]
        #[inline(always)]
        pub const fn esyncc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Expected SYNC clear flag"]
        #[inline(always)]
        pub fn set_esyncc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("syncokc", &self.syncokc())
                .field("syncwarnc", &self.syncwarnc())
                .field("errc", &self.errc())
                .field("esyncc", &self.esyncc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ syncokc: {=bool:?}, syncwarnc: {=bool:?}, errc: {=bool:?}, esyncc: {=bool:?} }}",
                self.syncokc(),
                self.syncwarnc(),
                self.errc(),
                self.esyncc()
            )
        }
    }
    #[doc = "interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "SYNC event OK flag"]
        #[inline(always)]
        pub const fn syncokf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC event OK flag"]
        #[inline(always)]
        pub fn set_syncokf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SYNC warning flag"]
        #[inline(always)]
        pub const fn syncwarnf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC warning flag"]
        #[inline(always)]
        pub fn set_syncwarnf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error flag"]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error flag"]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Expected SYNC flag"]
        #[inline(always)]
        pub const fn esyncf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Expected SYNC flag"]
        #[inline(always)]
        pub fn set_esyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SYNC error"]
        #[inline(always)]
        pub const fn syncerr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC error"]
        #[inline(always)]
        pub fn set_syncerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SYNC missed"]
        #[inline(always)]
        pub const fn syncmiss(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC missed"]
        #[inline(always)]
        pub fn set_syncmiss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Trimming overflow or underflow"]
        #[inline(always)]
        pub const fn trimovf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Trimming overflow or underflow"]
        #[inline(always)]
        pub fn set_trimovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Frequency error direction"]
        #[inline(always)]
        pub const fn fedir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Frequency error direction"]
        #[inline(always)]
        pub fn set_fedir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Frequency error capture"]
        #[inline(always)]
        pub const fn fecap(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Frequency error capture"]
        #[inline(always)]
        pub fn set_fecap(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("syncokf", &self.syncokf())
                .field("syncwarnf", &self.syncwarnf())
                .field("errf", &self.errf())
                .field("esyncf", &self.esyncf())
                .field("syncerr", &self.syncerr())
                .field("syncmiss", &self.syncmiss())
                .field("trimovf", &self.trimovf())
                .field("fedir", &self.fedir())
                .field("fecap", &self.fecap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ syncokf: {=bool:?}, syncwarnf: {=bool:?}, errf: {=bool:?}, esyncf: {=bool:?}, syncerr: {=bool:?}, syncmiss: {=bool:?}, trimovf: {=bool:?}, fedir: {=bool:?}, fecap: {=u16:?} }}" , self . syncokf () , self . syncwarnf () , self . errf () , self . esyncf () , self . syncerr () , self . syncmiss () , self . trimovf () , self . fedir () , self . fecap ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syncsrc {
        #[doc = "GPIO selected as SYNC signal source"]
        GPIO = 0x0,
        #[doc = "LSE selected as SYNC signal source"]
        LSE = 0x01,
        #[doc = "USB SOF selected as SYNC signal source"]
        USB = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Syncsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncsrc {
        #[inline(always)]
        fn from(val: u8) -> Syncsrc {
            Syncsrc::from_bits(val)
        }
    }
    impl From<Syncsrc> for u8 {
        #[inline(always)]
        fn from(val: Syncsrc) -> u8 {
            Syncsrc::to_bits(val)
        }
    }
}
