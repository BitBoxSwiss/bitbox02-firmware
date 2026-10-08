#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Secure digital input/output interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmmc {
    ptr: *mut u8,
}
unsafe impl Send for Sdmmc {}
unsafe impl Sync for Sdmmc {}
impl Sdmmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "power control register"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SDI clock control register"]
    #[inline(always)]
    pub const fn clkcr(self) -> crate::common::Reg<regs::Clkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "argument register"]
    #[inline(always)]
    pub const fn argr(self) -> crate::common::Reg<regs::Argr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "command register"]
    #[inline(always)]
    pub const fn cmdr(self) -> crate::common::Reg<regs::Cmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "command response register"]
    #[inline(always)]
    pub const fn respcmdr(self) -> crate::common::Reg<regs::Respcmdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "response 1..4 register"]
    #[inline(always)]
    pub const fn respr(self, n: usize) -> crate::common::Reg<regs::RespxR, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "data timer register"]
    #[inline(always)]
    pub const fn dtimer(self) -> crate::common::Reg<regs::Dtimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "data length register"]
    #[inline(always)]
    pub const fn dlenr(self) -> crate::common::Reg<regs::Dlenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "data control register"]
    #[inline(always)]
    pub const fn dctrl(self) -> crate::common::Reg<regs::Dctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "data counter register"]
    #[inline(always)]
    pub const fn dcntr(self) -> crate::common::Reg<regs::Dcntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn star(self) -> crate::common::Reg<regs::Star, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "mask register"]
    #[inline(always)]
    pub const fn maskr(self) -> crate::common::Reg<regs::Maskr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "FIFO counter register"]
    #[inline(always)]
    pub const fn fifocnt(self) -> crate::common::Reg<regs::Fifocnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "data FIFO register"]
    #[inline(always)]
    pub const fn fifor(self) -> crate::common::Reg<regs::Fifor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
}
pub mod regs {
    #[doc = "argument register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Argr(pub u32);
    impl Argr {
        #[doc = "Command argument"]
        #[inline(always)]
        pub const fn cmdarg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Command argument"]
        #[inline(always)]
        pub fn set_cmdarg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Argr {
        #[inline(always)]
        fn default() -> Argr {
            Argr(0)
        }
    }
    impl core::fmt::Debug for Argr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Argr").field("cmdarg", &self.cmdarg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Argr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Argr {{ cmdarg: {=u32:?} }}", self.cmdarg())
        }
    }
    #[doc = "SDI clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkcr(pub u32);
    impl Clkcr {
        #[doc = "Clock divide factor"]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clock divide factor"]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Clock enable bit"]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable bit"]
        #[inline(always)]
        pub fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Power saving configuration bit"]
        #[inline(always)]
        pub const fn pwrsav(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Power saving configuration bit"]
        #[inline(always)]
        pub fn set_pwrsav(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock divider bypass enable bit"]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock divider bypass enable bit"]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wide bus mode enable bit"]
        #[inline(always)]
        pub const fn widbus(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Wide bus mode enable bit"]
        #[inline(always)]
        pub fn set_widbus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "SDIO_CK dephasing selection bit"]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO_CK dephasing selection bit"]
        #[inline(always)]
        pub fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HW Flow Control enable"]
        #[inline(always)]
        pub const fn hwfc_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "HW Flow Control enable"]
        #[inline(always)]
        pub fn set_hwfc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Clkcr {
        #[inline(always)]
        fn default() -> Clkcr {
            Clkcr(0)
        }
    }
    impl core::fmt::Debug for Clkcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkcr")
                .field("clkdiv", &self.clkdiv())
                .field("clken", &self.clken())
                .field("pwrsav", &self.pwrsav())
                .field("bypass", &self.bypass())
                .field("widbus", &self.widbus())
                .field("negedge", &self.negedge())
                .field("hwfc_en", &self.hwfc_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clkcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Clkcr {{ clkdiv: {=u8:?}, clken: {=bool:?}, pwrsav: {=bool:?}, bypass: {=bool:?}, widbus: {=u8:?}, negedge: {=bool:?}, hwfc_en: {=bool:?} }}" , self . clkdiv () , self . clken () , self . pwrsav () , self . bypass () , self . widbus () , self . negedge () , self . hwfc_en ())
        }
    }
    #[doc = "command register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdr(pub u32);
    impl Cmdr {
        #[doc = "Command index"]
        #[inline(always)]
        pub const fn cmdindex(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Command index"]
        #[inline(always)]
        pub fn set_cmdindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Wait for response bits"]
        #[inline(always)]
        pub const fn waitresp(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Wait for response bits"]
        #[inline(always)]
        pub fn set_waitresp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "CPSM waits for interrupt request"]
        #[inline(always)]
        pub const fn waitint(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM waits for interrupt request"]
        #[inline(always)]
        pub fn set_waitint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal)"]
        #[inline(always)]
        pub const fn waitpend(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal)"]
        #[inline(always)]
        pub fn set_waitpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Command path state machine (CPSM) Enable bit"]
        #[inline(always)]
        pub const fn cpsmen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Command path state machine (CPSM) Enable bit"]
        #[inline(always)]
        pub fn set_cpsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SD I/O suspend command"]
        #[inline(always)]
        pub const fn sdiosuspend(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SD I/O suspend command"]
        #[inline(always)]
        pub fn set_sdiosuspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Cmdr {
        #[inline(always)]
        fn default() -> Cmdr {
            Cmdr(0)
        }
    }
    impl core::fmt::Debug for Cmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdr")
                .field("cmdindex", &self.cmdindex())
                .field("waitresp", &self.waitresp())
                .field("waitint", &self.waitint())
                .field("waitpend", &self.waitpend())
                .field("cpsmen", &self.cpsmen())
                .field("sdiosuspend", &self.sdiosuspend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmdr {{ cmdindex: {=u8:?}, waitresp: {=u8:?}, waitint: {=bool:?}, waitpend: {=bool:?}, cpsmen: {=bool:?}, sdiosuspend: {=bool:?} }}" , self . cmdindex () , self . waitresp () , self . waitint () , self . waitpend () , self . cpsmen () , self . sdiosuspend ())
        }
    }
    #[doc = "data counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcntr(pub u32);
    impl Dcntr {
        #[doc = "Data count value"]
        #[inline(always)]
        pub const fn datacount(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data count value"]
        #[inline(always)]
        pub fn set_datacount(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dcntr {
        #[inline(always)]
        fn default() -> Dcntr {
            Dcntr(0)
        }
    }
    impl core::fmt::Debug for Dcntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcntr").field("datacount", &self.datacount()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcntr {{ datacount: {=u32:?} }}", self.datacount())
        }
    }
    #[doc = "data control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctrl(pub u32);
    impl Dctrl {
        #[doc = "DTEN"]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DTEN"]
        #[inline(always)]
        pub fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data transfer direction selection"]
        #[inline(always)]
        pub const fn dtdir(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer direction selection"]
        #[inline(always)]
        pub fn set_dtdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer"]
        #[inline(always)]
        pub const fn dtmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer"]
        #[inline(always)]
        pub fn set_dtmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA enable bit"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable bit"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data block size"]
        #[inline(always)]
        pub const fn dblocksize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data block size"]
        #[inline(always)]
        pub fn set_dblocksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Read wait start"]
        #[inline(always)]
        pub const fn rwstart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait start"]
        #[inline(always)]
        pub fn set_rwstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Read wait stop"]
        #[inline(always)]
        pub const fn rwstop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait stop"]
        #[inline(always)]
        pub fn set_rwstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Read wait mode"]
        #[inline(always)]
        pub const fn rwmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait mode"]
        #[inline(always)]
        pub fn set_rwmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SD I/O enable functions"]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SD I/O enable functions"]
        #[inline(always)]
        pub fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dctrl {
        #[inline(always)]
        fn default() -> Dctrl {
            Dctrl(0)
        }
    }
    impl core::fmt::Debug for Dctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dctrl")
                .field("dten", &self.dten())
                .field("dtdir", &self.dtdir())
                .field("dtmode", &self.dtmode())
                .field("dmaen", &self.dmaen())
                .field("dblocksize", &self.dblocksize())
                .field("rwstart", &self.rwstart())
                .field("rwstop", &self.rwstop())
                .field("rwmod", &self.rwmod())
                .field("sdioen", &self.sdioen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dctrl {{ dten: {=bool:?}, dtdir: {=bool:?}, dtmode: {=bool:?}, dmaen: {=bool:?}, dblocksize: {=u8:?}, rwstart: {=bool:?}, rwstop: {=bool:?}, rwmod: {=bool:?}, sdioen: {=bool:?} }}" , self . dten () , self . dtdir () , self . dtmode () , self . dmaen () , self . dblocksize () , self . rwstart () , self . rwstop () , self . rwmod () , self . sdioen ())
        }
    }
    #[doc = "data length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlenr(pub u32);
    impl Dlenr {
        #[doc = "Data length value"]
        #[inline(always)]
        pub const fn datalength(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data length value"]
        #[inline(always)]
        pub fn set_datalength(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dlenr {
        #[inline(always)]
        fn default() -> Dlenr {
            Dlenr(0)
        }
    }
    impl core::fmt::Debug for Dlenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlenr").field("datalength", &self.datalength()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dlenr {{ datalength: {=u32:?} }}", self.datalength())
        }
    }
    #[doc = "data timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtimer(pub u32);
    impl Dtimer {
        #[doc = "Data timeout period"]
        #[inline(always)]
        pub const fn datatime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data timeout period"]
        #[inline(always)]
        pub fn set_datatime(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dtimer {
        #[inline(always)]
        fn default() -> Dtimer {
            Dtimer(0)
        }
    }
    impl core::fmt::Debug for Dtimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtimer").field("datatime", &self.datatime()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtimer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dtimer {{ datatime: {=u32:?} }}", self.datatime())
        }
    }
    #[doc = "FIFO counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifocnt(pub u32);
    impl Fifocnt {
        #[doc = "Remaining number of words to be written to or read from the FIFO"]
        #[inline(always)]
        pub const fn fifocount(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Remaining number of words to be written to or read from the FIFO"]
        #[inline(always)]
        pub fn set_fifocount(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Fifocnt {
        #[inline(always)]
        fn default() -> Fifocnt {
            Fifocnt(0)
        }
    }
    impl core::fmt::Debug for Fifocnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifocnt").field("fifocount", &self.fifocount()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifocnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifocnt {{ fifocount: {=u32:?} }}", self.fifocount())
        }
    }
    #[doc = "data FIFO register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifor(pub u32);
    impl Fifor {
        #[doc = "Receive and transmit FIFO data"]
        #[inline(always)]
        pub const fn fifodata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive and transmit FIFO data"]
        #[inline(always)]
        pub fn set_fifodata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifor {
        #[inline(always)]
        fn default() -> Fifor {
            Fifor(0)
        }
    }
    impl core::fmt::Debug for Fifor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifor").field("fifodata", &self.fifodata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifor {{ fifodata: {=u32:?} }}", self.fifodata())
        }
    }
    #[doc = "interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "CCRCFAIL flag clear bit"]
        #[inline(always)]
        pub const fn ccrcfailc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CCRCFAIL flag clear bit"]
        #[inline(always)]
        pub fn set_ccrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCRCFAIL flag clear bit"]
        #[inline(always)]
        pub const fn dcrcfailc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCRCFAIL flag clear bit"]
        #[inline(always)]
        pub fn set_dcrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CTIMEOUT flag clear bit"]
        #[inline(always)]
        pub const fn ctimeoutc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CTIMEOUT flag clear bit"]
        #[inline(always)]
        pub fn set_ctimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DTIMEOUT flag clear bit"]
        #[inline(always)]
        pub const fn dtimeoutc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTIMEOUT flag clear bit"]
        #[inline(always)]
        pub fn set_dtimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TXUNDERR flag clear bit"]
        #[inline(always)]
        pub const fn txunderrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TXUNDERR flag clear bit"]
        #[inline(always)]
        pub fn set_txunderrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXOVERR flag clear bit"]
        #[inline(always)]
        pub const fn rxoverrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXOVERR flag clear bit"]
        #[inline(always)]
        pub fn set_rxoverrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMDREND flag clear bit"]
        #[inline(always)]
        pub const fn cmdrendc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMDREND flag clear bit"]
        #[inline(always)]
        pub fn set_cmdrendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CMDSENT flag clear bit"]
        #[inline(always)]
        pub const fn cmdsentc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMDSENT flag clear bit"]
        #[inline(always)]
        pub fn set_cmdsentc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DATAEND flag clear bit"]
        #[inline(always)]
        pub const fn dataendc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DATAEND flag clear bit"]
        #[inline(always)]
        pub fn set_dataendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "STBITERR flag clear bit"]
        #[inline(always)]
        pub const fn stbiterrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "STBITERR flag clear bit"]
        #[inline(always)]
        pub fn set_stbiterrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DBCKEND flag clear bit"]
        #[inline(always)]
        pub const fn dbckendc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DBCKEND flag clear bit"]
        #[inline(always)]
        pub fn set_dbckendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SDIOIT flag clear bit"]
        #[inline(always)]
        pub const fn sdioitc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIOIT flag clear bit"]
        #[inline(always)]
        pub fn set_sdioitc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("ccrcfailc", &self.ccrcfailc())
                .field("dcrcfailc", &self.dcrcfailc())
                .field("ctimeoutc", &self.ctimeoutc())
                .field("dtimeoutc", &self.dtimeoutc())
                .field("txunderrc", &self.txunderrc())
                .field("rxoverrc", &self.rxoverrc())
                .field("cmdrendc", &self.cmdrendc())
                .field("cmdsentc", &self.cmdsentc())
                .field("dataendc", &self.dataendc())
                .field("stbiterrc", &self.stbiterrc())
                .field("dbckendc", &self.dbckendc())
                .field("sdioitc", &self.sdioitc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ ccrcfailc: {=bool:?}, dcrcfailc: {=bool:?}, ctimeoutc: {=bool:?}, dtimeoutc: {=bool:?}, txunderrc: {=bool:?}, rxoverrc: {=bool:?}, cmdrendc: {=bool:?}, cmdsentc: {=bool:?}, dataendc: {=bool:?}, stbiterrc: {=bool:?}, dbckendc: {=bool:?}, sdioitc: {=bool:?} }}" , self . ccrcfailc () , self . dcrcfailc () , self . ctimeoutc () , self . dtimeoutc () , self . txunderrc () , self . rxoverrc () , self . cmdrendc () , self . cmdsentc () , self . dataendc () , self . stbiterrc () , self . dbckendc () , self . sdioitc ())
        }
    }
    #[doc = "mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maskr(pub u32);
    impl Maskr {
        #[doc = "Command CRC fail interrupt enable"]
        #[inline(always)]
        pub const fn ccrcfailie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command CRC fail interrupt enable"]
        #[inline(always)]
        pub fn set_ccrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data CRC fail interrupt enable"]
        #[inline(always)]
        pub const fn dcrcfailie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data CRC fail interrupt enable"]
        #[inline(always)]
        pub fn set_dcrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command timeout interrupt enable"]
        #[inline(always)]
        pub const fn ctimeoutie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command timeout interrupt enable"]
        #[inline(always)]
        pub fn set_ctimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout interrupt enable"]
        #[inline(always)]
        pub const fn dtimeoutie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout interrupt enable"]
        #[inline(always)]
        pub fn set_dtimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tx FIFO underrun error interrupt enable"]
        #[inline(always)]
        pub const fn txunderrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO underrun error interrupt enable"]
        #[inline(always)]
        pub fn set_txunderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO overrun error interrupt enable"]
        #[inline(always)]
        pub const fn rxoverrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO overrun error interrupt enable"]
        #[inline(always)]
        pub fn set_rxoverrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received interrupt enable"]
        #[inline(always)]
        pub const fn cmdrendie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received interrupt enable"]
        #[inline(always)]
        pub fn set_cmdrendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent interrupt enable"]
        #[inline(always)]
        pub const fn cmdsentie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent interrupt enable"]
        #[inline(always)]
        pub fn set_cmdsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data end interrupt enable"]
        #[inline(always)]
        pub const fn dataendie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data end interrupt enable"]
        #[inline(always)]
        pub fn set_dataendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "STBITERR interrupt enable"]
        #[inline(always)]
        pub const fn stbiterre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "STBITERR interrupt enable"]
        #[inline(always)]
        pub fn set_stbiterre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block end interrupt enable"]
        #[inline(always)]
        pub const fn dbckendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block end interrupt enable"]
        #[inline(always)]
        pub fn set_dbckendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Command acting interrupt enable"]
        #[inline(always)]
        pub const fn cmdactie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Command acting interrupt enable"]
        #[inline(always)]
        pub fn set_cmdactie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data transmit acting interrupt enable"]
        #[inline(always)]
        pub const fn txactie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data transmit acting interrupt enable"]
        #[inline(always)]
        pub fn set_txactie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data receive acting interrupt enable"]
        #[inline(always)]
        pub const fn rxactie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Data receive acting interrupt enable"]
        #[inline(always)]
        pub fn set_rxactie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx FIFO half empty interrupt enable"]
        #[inline(always)]
        pub const fn txfifoheie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO half empty interrupt enable"]
        #[inline(always)]
        pub fn set_txfifoheie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Rx FIFO half full interrupt enable"]
        #[inline(always)]
        pub const fn rxfifohfie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO half full interrupt enable"]
        #[inline(always)]
        pub fn set_rxfifohfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Tx FIFO full interrupt enable"]
        #[inline(always)]
        pub const fn txfifofie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO full interrupt enable"]
        #[inline(always)]
        pub fn set_txfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Rx FIFO full interrupt enable"]
        #[inline(always)]
        pub const fn rxfifofie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO full interrupt enable"]
        #[inline(always)]
        pub fn set_rxfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Tx FIFO empty interrupt enable"]
        #[inline(always)]
        pub const fn txfifoeie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty interrupt enable"]
        #[inline(always)]
        pub fn set_txfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Rx FIFO empty interrupt enable"]
        #[inline(always)]
        pub const fn rxfifoeie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO empty interrupt enable"]
        #[inline(always)]
        pub fn set_rxfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data available in Tx FIFO interrupt enable"]
        #[inline(always)]
        pub const fn txdavlie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in Tx FIFO interrupt enable"]
        #[inline(always)]
        pub fn set_txdavlie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data available in Rx FIFO interrupt enable"]
        #[inline(always)]
        pub const fn rxdavlie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in Rx FIFO interrupt enable"]
        #[inline(always)]
        pub fn set_rxdavlie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO mode interrupt received interrupt enable"]
        #[inline(always)]
        pub const fn sdioitie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO mode interrupt received interrupt enable"]
        #[inline(always)]
        pub fn set_sdioitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Maskr {
        #[inline(always)]
        fn default() -> Maskr {
            Maskr(0)
        }
    }
    impl core::fmt::Debug for Maskr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maskr")
                .field("ccrcfailie", &self.ccrcfailie())
                .field("dcrcfailie", &self.dcrcfailie())
                .field("ctimeoutie", &self.ctimeoutie())
                .field("dtimeoutie", &self.dtimeoutie())
                .field("txunderrie", &self.txunderrie())
                .field("rxoverrie", &self.rxoverrie())
                .field("cmdrendie", &self.cmdrendie())
                .field("cmdsentie", &self.cmdsentie())
                .field("dataendie", &self.dataendie())
                .field("stbiterre", &self.stbiterre())
                .field("dbckendie", &self.dbckendie())
                .field("cmdactie", &self.cmdactie())
                .field("txactie", &self.txactie())
                .field("rxactie", &self.rxactie())
                .field("txfifoheie", &self.txfifoheie())
                .field("rxfifohfie", &self.rxfifohfie())
                .field("txfifofie", &self.txfifofie())
                .field("rxfifofie", &self.rxfifofie())
                .field("txfifoeie", &self.txfifoeie())
                .field("rxfifoeie", &self.rxfifoeie())
                .field("txdavlie", &self.txdavlie())
                .field("rxdavlie", &self.rxdavlie())
                .field("sdioitie", &self.sdioitie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maskr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Maskr {{ ccrcfailie: {=bool:?}, dcrcfailie: {=bool:?}, ctimeoutie: {=bool:?}, dtimeoutie: {=bool:?}, txunderrie: {=bool:?}, rxoverrie: {=bool:?}, cmdrendie: {=bool:?}, cmdsentie: {=bool:?}, dataendie: {=bool:?}, stbiterre: {=bool:?}, dbckendie: {=bool:?}, cmdactie: {=bool:?}, txactie: {=bool:?}, rxactie: {=bool:?}, txfifoheie: {=bool:?}, rxfifohfie: {=bool:?}, txfifofie: {=bool:?}, rxfifofie: {=bool:?}, txfifoeie: {=bool:?}, rxfifoeie: {=bool:?}, txdavlie: {=bool:?}, rxdavlie: {=bool:?}, sdioitie: {=bool:?} }}" , self . ccrcfailie () , self . dcrcfailie () , self . ctimeoutie () , self . dtimeoutie () , self . txunderrie () , self . rxoverrie () , self . cmdrendie () , self . cmdsentie () , self . dataendie () , self . stbiterre () , self . dbckendie () , self . cmdactie () , self . txactie () , self . rxactie () , self . txfifoheie () , self . rxfifohfie () , self . txfifofie () , self . rxfifofie () , self . txfifoeie () , self . rxfifoeie () , self . txdavlie () , self . rxdavlie () , self . sdioitie ())
        }
    }
    #[doc = "power control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power(pub u32);
    impl Power {
        #[doc = "PWRCTRL"]
        #[inline(always)]
        pub const fn pwrctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "PWRCTRL"]
        #[inline(always)]
        pub fn set_pwrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Power {
        #[inline(always)]
        fn default() -> Power {
            Power(0)
        }
    }
    impl core::fmt::Debug for Power {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Power").field("pwrctrl", &self.pwrctrl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Power {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Power {{ pwrctrl: {=u8:?} }}", self.pwrctrl())
        }
    }
    #[doc = "command response register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Respcmdr(pub u32);
    impl Respcmdr {
        #[doc = "Response command index"]
        #[inline(always)]
        pub const fn respcmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Response command index"]
        #[inline(always)]
        pub fn set_respcmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Respcmdr {
        #[inline(always)]
        fn default() -> Respcmdr {
            Respcmdr(0)
        }
    }
    impl core::fmt::Debug for Respcmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Respcmdr").field("respcmd", &self.respcmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Respcmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Respcmdr {{ respcmd: {=u8:?} }}", self.respcmd())
        }
    }
    #[doc = "response 1..4 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RespxR(pub u32);
    impl RespxR {
        #[doc = "see Table 132"]
        #[inline(always)]
        pub const fn cardstatus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "see Table 132"]
        #[inline(always)]
        pub fn set_cardstatus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RespxR {
        #[inline(always)]
        fn default() -> RespxR {
            RespxR(0)
        }
    }
    impl core::fmt::Debug for RespxR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RespxR")
                .field("cardstatus", &self.cardstatus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RespxR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RespxR {{ cardstatus: {=u32:?} }}", self.cardstatus())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star(pub u32);
    impl Star {
        #[doc = "Command response received (CRC check failed)"]
        #[inline(always)]
        pub const fn ccrcfail(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check failed)"]
        #[inline(always)]
        pub fn set_ccrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data block sent/received (CRC check failed)"]
        #[inline(always)]
        pub const fn dcrcfail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received (CRC check failed)"]
        #[inline(always)]
        pub fn set_dcrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command response timeout"]
        #[inline(always)]
        pub const fn ctimeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command response timeout"]
        #[inline(always)]
        pub fn set_ctimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout"]
        #[inline(always)]
        pub const fn dtimeout(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout"]
        #[inline(always)]
        pub fn set_dtimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit FIFO underrun error"]
        #[inline(always)]
        pub const fn txunderr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO underrun error"]
        #[inline(always)]
        pub fn set_txunderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Received FIFO overrun error"]
        #[inline(always)]
        pub const fn rxoverr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received FIFO overrun error"]
        #[inline(always)]
        pub fn set_rxoverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received (CRC check passed)"]
        #[inline(always)]
        pub const fn cmdrend(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check passed)"]
        #[inline(always)]
        pub fn set_cmdrend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent (no response required)"]
        #[inline(always)]
        pub const fn cmdsent(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent (no response required)"]
        #[inline(always)]
        pub fn set_cmdsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data end (data counter, SDIDCOUNT, is zero)"]
        #[inline(always)]
        pub const fn dataend(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data end (data counter, SDIDCOUNT, is zero)"]
        #[inline(always)]
        pub fn set_dataend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Start bit not detected on all data signals in wide bus mode"]
        #[inline(always)]
        pub const fn stbiterr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start bit not detected on all data signals in wide bus mode"]
        #[inline(always)]
        pub fn set_stbiterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block sent/received (CRC check passed)"]
        #[inline(always)]
        pub const fn dbckend(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received (CRC check passed)"]
        #[inline(always)]
        pub fn set_dbckend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Command transfer in progress"]
        #[inline(always)]
        pub const fn cmdact(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Command transfer in progress"]
        #[inline(always)]
        pub fn set_cmdact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data transmit in progress"]
        #[inline(always)]
        pub const fn txact(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data transmit in progress"]
        #[inline(always)]
        pub fn set_txact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data receive in progress"]
        #[inline(always)]
        pub const fn rxact(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Data receive in progress"]
        #[inline(always)]
        pub fn set_rxact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
        #[inline(always)]
        pub const fn txfifohe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
        #[inline(always)]
        pub fn set_txfifohe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive FIFO half full: there are at least 8 words in the FIFO"]
        #[inline(always)]
        pub const fn rxfifohf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO half full: there are at least 8 words in the FIFO"]
        #[inline(always)]
        pub fn set_rxfifohf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmit FIFO full"]
        #[inline(always)]
        pub const fn txfifof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO full"]
        #[inline(always)]
        pub fn set_txfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive FIFO full"]
        #[inline(always)]
        pub const fn rxfifof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO full"]
        #[inline(always)]
        pub fn set_rxfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Transmit FIFO empty"]
        #[inline(always)]
        pub const fn txfifoe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO empty"]
        #[inline(always)]
        pub fn set_txfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receive FIFO empty"]
        #[inline(always)]
        pub const fn rxfifoe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO empty"]
        #[inline(always)]
        pub fn set_rxfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data available in transmit FIFO"]
        #[inline(always)]
        pub const fn txdavl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in transmit FIFO"]
        #[inline(always)]
        pub fn set_txdavl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data available in receive FIFO"]
        #[inline(always)]
        pub const fn rxdavl(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in receive FIFO"]
        #[inline(always)]
        pub fn set_rxdavl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO interrupt received"]
        #[inline(always)]
        pub const fn sdioit(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO interrupt received"]
        #[inline(always)]
        pub fn set_sdioit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Star {
        #[inline(always)]
        fn default() -> Star {
            Star(0)
        }
    }
    impl core::fmt::Debug for Star {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Star")
                .field("ccrcfail", &self.ccrcfail())
                .field("dcrcfail", &self.dcrcfail())
                .field("ctimeout", &self.ctimeout())
                .field("dtimeout", &self.dtimeout())
                .field("txunderr", &self.txunderr())
                .field("rxoverr", &self.rxoverr())
                .field("cmdrend", &self.cmdrend())
                .field("cmdsent", &self.cmdsent())
                .field("dataend", &self.dataend())
                .field("stbiterr", &self.stbiterr())
                .field("dbckend", &self.dbckend())
                .field("cmdact", &self.cmdact())
                .field("txact", &self.txact())
                .field("rxact", &self.rxact())
                .field("txfifohe", &self.txfifohe())
                .field("rxfifohf", &self.rxfifohf())
                .field("txfifof", &self.txfifof())
                .field("rxfifof", &self.rxfifof())
                .field("txfifoe", &self.txfifoe())
                .field("rxfifoe", &self.rxfifoe())
                .field("txdavl", &self.txdavl())
                .field("rxdavl", &self.rxdavl())
                .field("sdioit", &self.sdioit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Star {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Star {{ ccrcfail: {=bool:?}, dcrcfail: {=bool:?}, ctimeout: {=bool:?}, dtimeout: {=bool:?}, txunderr: {=bool:?}, rxoverr: {=bool:?}, cmdrend: {=bool:?}, cmdsent: {=bool:?}, dataend: {=bool:?}, stbiterr: {=bool:?}, dbckend: {=bool:?}, cmdact: {=bool:?}, txact: {=bool:?}, rxact: {=bool:?}, txfifohe: {=bool:?}, rxfifohf: {=bool:?}, txfifof: {=bool:?}, rxfifof: {=bool:?}, txfifoe: {=bool:?}, rxfifoe: {=bool:?}, txdavl: {=bool:?}, rxdavl: {=bool:?}, sdioit: {=bool:?} }}" , self . ccrcfail () , self . dcrcfail () , self . ctimeout () , self . dtimeout () , self . txunderr () , self . rxoverr () , self . cmdrend () , self . cmdsent () , self . dataend () , self . stbiterr () , self . dbckend () , self . cmdact () , self . txact () , self . rxact () , self . txfifohe () , self . rxfifohf () , self . txfifof () , self . rxfifof () , self . txfifoe () , self . rxfifoe () , self . txdavl () , self . rxdavl () , self . sdioit ())
        }
    }
}
