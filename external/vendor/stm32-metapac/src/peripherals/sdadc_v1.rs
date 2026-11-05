#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Sigma-delta analog-to-digital converter."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadc {
    ptr: *mut u8,
}
unsafe impl Send for Sdadc {}
unsafe impl Sync for Sdadc {}
impl Sdadc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt and status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "interrupt and status clear register."]
    #[inline(always)]
    pub const fn clrisr(self) -> crate::common::Reg<regs::Clrisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "injected channel group selection register."]
    #[inline(always)]
    pub const fn jchgr(self) -> crate::common::Reg<regs::Jchgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "configuration 0 register."]
    #[inline(always)]
    pub const fn confr(self, n: usize) -> crate::common::Reg<regs::Confr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "channel configuration register 1."]
    #[inline(always)]
    pub const fn confchr1(self) -> crate::common::Reg<regs::Confchr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "channel configuration register 2."]
    #[inline(always)]
    pub const fn confchr2(self) -> crate::common::Reg<regs::Confchr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "data register for injected group."]
    #[inline(always)]
    pub const fn jdatar(self) -> crate::common::Reg<regs::Jdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "data register for the regular channel."]
    #[inline(always)]
    pub const fn rdatar(self) -> crate::common::Reg<regs::Rdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "SDADC1 and SDADC2 injected data register."]
    #[inline(always)]
    pub const fn jdata12r(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "SDADC1 and SDADC2 regular data register."]
    #[inline(always)]
    pub const fn rdata12r(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "SDADC1 and SDADC3 injected data register."]
    #[inline(always)]
    pub const fn jdata13r(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "SDADC1 and SDADC3 regular data register."]
    #[inline(always)]
    pub const fn rdata13r(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "interrupt and status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clrisr(pub u32);
    impl Clrisr {
        #[doc = "Clear the end of calibration flag."]
        #[inline(always)]
        pub const fn clreocalf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the end of calibration flag."]
        #[inline(always)]
        pub fn set_clreocalf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub const fn clrjovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub fn set_clrjovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub const fn clrrovrf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub fn set_clrrovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Clrisr {
        #[inline(always)]
        fn default() -> Clrisr {
            Clrisr(0)
        }
    }
    impl core::fmt::Debug for Clrisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clrisr")
                .field("clreocalf", &self.clreocalf())
                .field("clrjovrf", &self.clrjovrf())
                .field("clrrovrf", &self.clrrovrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clrisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clrisr {{ clreocalf: {=bool:?}, clrjovrf: {=bool:?}, clrrovrf: {=bool:?} }}",
                self.clreocalf(),
                self.clrjovrf(),
                self.clrrovrf()
            )
        }
    }
    #[doc = "channel configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confchr1(pub u32);
    impl Confchr1 {
        #[doc = "CONFCH0."]
        #[inline(always)]
        pub const fn confch(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "CONFCH0."]
        #[inline(always)]
        pub fn set_confch(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
    }
    impl Default for Confchr1 {
        #[inline(always)]
        fn default() -> Confchr1 {
            Confchr1(0)
        }
    }
    impl core::fmt::Debug for Confchr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confchr1")
                .field("confch[0]", &self.confch(0usize))
                .field("confch[1]", &self.confch(1usize))
                .field("confch[2]", &self.confch(2usize))
                .field("confch[3]", &self.confch(3usize))
                .field("confch[4]", &self.confch(4usize))
                .field("confch[5]", &self.confch(5usize))
                .field("confch[6]", &self.confch(6usize))
                .field("confch[7]", &self.confch(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Confchr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Confchr1 {{ confch[0]: {=u8:?}, confch[1]: {=u8:?}, confch[2]: {=u8:?}, confch[3]: {=u8:?}, confch[4]: {=u8:?}, confch[5]: {=u8:?}, confch[6]: {=u8:?}, confch[7]: {=u8:?} }}" , self . confch (0usize) , self . confch (1usize) , self . confch (2usize) , self . confch (3usize) , self . confch (4usize) , self . confch (5usize) , self . confch (6usize) , self . confch (7usize))
        }
    }
    #[doc = "channel configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confchr2(pub u32);
    impl Confchr2 {
        #[doc = "Channel 8 configuration."]
        #[inline(always)]
        pub const fn confch(&self, n: usize) -> u8 {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Channel 8 configuration."]
        #[inline(always)]
        pub fn set_confch(&mut self, n: usize, val: u8) {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
    }
    impl Default for Confchr2 {
        #[inline(always)]
        fn default() -> Confchr2 {
            Confchr2(0)
        }
    }
    impl core::fmt::Debug for Confchr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confchr2")
                .field("confch[0]", &self.confch(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Confchr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Confchr2 {{ confch[0]: {=u8:?} }}", self.confch(0usize))
        }
    }
    #[doc = "configuration 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confr(pub u32);
    impl Confr {
        #[doc = "Twelve-bit calibration offset for configuration 0."]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Twelve-bit calibration offset for configuration 0."]
        #[inline(always)]
        pub fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Gain setting for configuration 0."]
        #[inline(always)]
        pub const fn gain(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Gain setting for configuration 0."]
        #[inline(always)]
        pub fn set_gain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Single-ended mode for configuration 0."]
        #[inline(always)]
        pub const fn se(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "Single-ended mode for configuration 0."]
        #[inline(always)]
        pub fn set_se(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "Common mode for configuration 0."]
        #[inline(always)]
        pub const fn common(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Common mode for configuration 0."]
        #[inline(always)]
        pub fn set_common(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Confr {
        #[inline(always)]
        fn default() -> Confr {
            Confr(0)
        }
    }
    impl core::fmt::Debug for Confr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confr")
                .field("offset", &self.offset())
                .field("gain", &self.gain())
                .field("se", &self.se())
                .field("common", &self.common())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Confr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Confr {{ offset: {=u16:?}, gain: {=u8:?}, se: {=u8:?}, common: {=u8:?} }}",
                self.offset(),
                self.gain(),
                self.se(),
                self.common()
            )
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "End of calibration interrupt enable."]
        #[inline(always)]
        pub const fn eocalie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of calibration interrupt enable."]
        #[inline(always)]
        pub fn set_eocalie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Injected end of conversion interrupt enable."]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Injected end of conversion interrupt enable."]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected data overrun interrupt enable."]
        #[inline(always)]
        pub const fn jovrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected data overrun interrupt enable."]
        #[inline(always)]
        pub fn set_jovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular end of conversion interrupt enable."]
        #[inline(always)]
        pub const fn reocie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular end of conversion interrupt enable."]
        #[inline(always)]
        pub fn set_reocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular data overrun interrupt enable."]
        #[inline(always)]
        pub const fn rovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular data overrun interrupt enable."]
        #[inline(always)]
        pub fn set_rovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Reference voltage selection."]
        #[inline(always)]
        pub const fn refv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Reference voltage selection."]
        #[inline(always)]
        pub fn set_refv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Slow clock mode enable."]
        #[inline(always)]
        pub const fn slowck(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Slow clock mode enable."]
        #[inline(always)]
        pub fn set_slowck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enter Standby mode when idle."]
        #[inline(always)]
        pub const fn sbi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Standby mode when idle."]
        #[inline(always)]
        pub fn set_sbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enter power down mode when idle."]
        #[inline(always)]
        pub const fn pdi(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enter power down mode when idle."]
        #[inline(always)]
        pub fn set_pdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Launch a injected conversion synchronously with SDADC1."]
        #[inline(always)]
        pub const fn jsync(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Launch a injected conversion synchronously with SDADC1."]
        #[inline(always)]
        pub fn set_jsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Launch regular conversion synchronously with SDADC1."]
        #[inline(always)]
        pub const fn rsync(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Launch regular conversion synchronously with SDADC1."]
        #[inline(always)]
        pub fn set_rsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DMA channel enabled to read data for the injected channel group."]
        #[inline(always)]
        pub const fn jdmaen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the injected channel group."]
        #[inline(always)]
        pub fn set_jdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA channel enabled to read data for the regular channel."]
        #[inline(always)]
        pub const fn rdmaen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the regular channel."]
        #[inline(always)]
        pub fn set_rdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Initialization mode request."]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization mode request."]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("eocalie", &self.eocalie())
                .field("jeocie", &self.jeocie())
                .field("jovrie", &self.jovrie())
                .field("reocie", &self.reocie())
                .field("rovrie", &self.rovrie())
                .field("refv", &self.refv())
                .field("slowck", &self.slowck())
                .field("sbi", &self.sbi())
                .field("pdi", &self.pdi())
                .field("jsync", &self.jsync())
                .field("rsync", &self.rsync())
                .field("jdmaen", &self.jdmaen())
                .field("rdmaen", &self.rdmaen())
                .field("init", &self.init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ eocalie: {=bool:?}, jeocie: {=bool:?}, jovrie: {=bool:?}, reocie: {=bool:?}, rovrie: {=bool:?}, refv: {=u8:?}, slowck: {=bool:?}, sbi: {=bool:?}, pdi: {=bool:?}, jsync: {=bool:?}, rsync: {=bool:?}, jdmaen: {=bool:?}, rdmaen: {=bool:?}, init: {=bool:?} }}" , self . eocalie () , self . jeocie () , self . jovrie () , self . reocie () , self . rovrie () , self . refv () , self . slowck () , self . sbi () , self . pdi () , self . jsync () , self . rsync () , self . jdmaen () , self . rdmaen () , self . init ())
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "SDADC enable."]
        #[inline(always)]
        pub const fn adon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SDADC enable."]
        #[inline(always)]
        pub fn set_adon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Number of calibration sequences to be performed (number of valid configurations)."]
        #[inline(always)]
        pub const fn calibcnt(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Number of calibration sequences to be performed (number of valid configurations)."]
        #[inline(always)]
        pub fn set_calibcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Start calibration."]
        #[inline(always)]
        pub const fn startcalib(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Start calibration."]
        #[inline(always)]
        pub fn set_startcalib(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Continuous mode selection for injected conversions."]
        #[inline(always)]
        pub const fn jcont(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode selection for injected conversions."]
        #[inline(always)]
        pub fn set_jcont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Delay start of injected conversions."]
        #[inline(always)]
        pub const fn jds(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Delay start of injected conversions."]
        #[inline(always)]
        pub fn set_jds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Trigger signal selection for launching injected conversions."]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Trigger signal selection for launching injected conversions."]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions."]
        #[inline(always)]
        pub const fn jexten(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions."]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Start a conversion of the injected group of channels."]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Start a conversion of the injected group of channels."]
        #[inline(always)]
        pub fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Regular channel selection."]
        #[inline(always)]
        pub const fn rch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel selection."]
        #[inline(always)]
        pub fn set_rch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Continuous mode selection for regular conversions."]
        #[inline(always)]
        pub const fn rcont(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode selection for regular conversions."]
        #[inline(always)]
        pub fn set_rcont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Software start of a conversion on the regular channel."]
        #[inline(always)]
        pub const fn rswstart(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Software start of a conversion on the regular channel."]
        #[inline(always)]
        pub fn set_rswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fast conversion mode selection."]
        #[inline(always)]
        pub const fn fast(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Fast conversion mode selection."]
        #[inline(always)]
        pub fn set_fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("adon", &self.adon())
                .field("calibcnt", &self.calibcnt())
                .field("startcalib", &self.startcalib())
                .field("jcont", &self.jcont())
                .field("jds", &self.jds())
                .field("jextsel", &self.jextsel())
                .field("jexten", &self.jexten())
                .field("jswstart", &self.jswstart())
                .field("rch", &self.rch())
                .field("rcont", &self.rcont())
                .field("rswstart", &self.rswstart())
                .field("fast", &self.fast())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ adon: {=bool:?}, calibcnt: {=u8:?}, startcalib: {=bool:?}, jcont: {=bool:?}, jds: {=bool:?}, jextsel: {=u8:?}, jexten: {=u8:?}, jswstart: {=bool:?}, rch: {=u8:?}, rcont: {=bool:?}, rswstart: {=bool:?}, fast: {=bool:?} }}" , self . adon () , self . calibcnt () , self . startcalib () , self . jcont () , self . jds () , self . jextsel () , self . jexten () , self . jswstart () , self . rch () , self . rcont () , self . rswstart () , self . fast ())
        }
    }
    #[doc = "interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "End of calibration flag."]
        #[inline(always)]
        pub const fn eocalf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of calibration flag."]
        #[inline(always)]
        pub fn set_eocalf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of injected conversion flag."]
        #[inline(always)]
        pub const fn jeocf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag."]
        #[inline(always)]
        pub fn set_jeocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected conversion overrun flag."]
        #[inline(always)]
        pub const fn jovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion overrun flag."]
        #[inline(always)]
        pub fn set_jovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular conversion flag."]
        #[inline(always)]
        pub const fn reocf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion flag."]
        #[inline(always)]
        pub fn set_reocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular conversion overrun flag."]
        #[inline(always)]
        pub const fn rovrf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion overrun flag."]
        #[inline(always)]
        pub fn set_rovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Calibration in progress status."]
        #[inline(always)]
        pub const fn calibip(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration in progress status."]
        #[inline(always)]
        pub fn set_calibip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Injected conversion in progress status."]
        #[inline(always)]
        pub const fn jcip(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion in progress status."]
        #[inline(always)]
        pub fn set_jcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Regular conversion in progress status."]
        #[inline(always)]
        pub const fn rcip(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion in progress status."]
        #[inline(always)]
        pub fn set_rcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Stabilization in progress status."]
        #[inline(always)]
        pub const fn stabip(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Stabilization in progress status."]
        #[inline(always)]
        pub fn set_stabip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Initialization mode is ready."]
        #[inline(always)]
        pub const fn initrdy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization mode is ready."]
        #[inline(always)]
        pub fn set_initrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("eocalf", &self.eocalf())
                .field("jeocf", &self.jeocf())
                .field("jovrf", &self.jovrf())
                .field("reocf", &self.reocf())
                .field("rovrf", &self.rovrf())
                .field("calibip", &self.calibip())
                .field("jcip", &self.jcip())
                .field("rcip", &self.rcip())
                .field("stabip", &self.stabip())
                .field("initrdy", &self.initrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ eocalf: {=bool:?}, jeocf: {=bool:?}, jovrf: {=bool:?}, reocf: {=bool:?}, rovrf: {=bool:?}, calibip: {=bool:?}, jcip: {=bool:?}, rcip: {=bool:?}, stabip: {=bool:?}, initrdy: {=bool:?} }}" , self . eocalf () , self . jeocf () , self . jovrf () , self . reocf () , self . rovrf () , self . calibip () , self . jcip () , self . rcip () , self . stabip () , self . initrdy ())
        }
    }
    #[doc = "injected channel group selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jchgr(pub u32);
    impl Jchgr {
        #[doc = "Injected channel group selection."]
        #[inline(always)]
        pub const fn jchg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Injected channel group selection."]
        #[inline(always)]
        pub fn set_jchg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Jchgr {
        #[inline(always)]
        fn default() -> Jchgr {
            Jchgr(0)
        }
    }
    impl core::fmt::Debug for Jchgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jchgr").field("jchg", &self.jchg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jchgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jchgr {{ jchg: {=u16:?} }}", self.jchg())
        }
    }
    #[doc = "data register for injected group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdatar(pub u32);
    impl Jdatar {
        #[doc = "Injected group conversion data."]
        #[inline(always)]
        pub const fn jdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected group conversion data."]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Injected channel most recently converted."]
        #[inline(always)]
        pub const fn jdatach(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Injected channel most recently converted."]
        #[inline(always)]
        pub fn set_jdatach(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Jdatar {
        #[inline(always)]
        fn default() -> Jdatar {
            Jdatar(0)
        }
    }
    impl core::fmt::Debug for Jdatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdatar")
                .field("jdata", &self.jdata())
                .field("jdatach", &self.jdatach())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdatar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Jdatar {{ jdata: {=u16:?}, jdatach: {=u8:?} }}",
                self.jdata(),
                self.jdatach()
            )
        }
    }
    #[doc = "data register for the regular channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdatar(pub u32);
    impl Rdatar {
        #[doc = "Regular channel conversion data."]
        #[inline(always)]
        pub const fn rdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular channel conversion data."]
        #[inline(always)]
        pub fn set_rdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rdatar {
        #[inline(always)]
        fn default() -> Rdatar {
            Rdatar(0)
        }
    }
    impl core::fmt::Debug for Rdatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdatar").field("rdata", &self.rdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdatar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rdatar {{ rdata: {=u16:?} }}", self.rdata())
        }
    }
}
