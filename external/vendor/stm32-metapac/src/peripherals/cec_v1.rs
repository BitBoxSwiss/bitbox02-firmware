#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "HDMI-CEC controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cec {
    ptr: *mut u8,
}
unsafe impl Send for Cec {}
unsafe impl Sync for Cec {}
impl Cec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CEC own address register."]
    #[inline(always)]
    pub const fn oar(self) -> crate::common::Reg<regs::Oar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Rx Data Register."]
    #[inline(always)]
    pub const fn pres(self) -> crate::common::Reg<regs::Pres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CEC error status register."]
    #[inline(always)]
    pub const fn esr(self) -> crate::common::Reg<regs::Esr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CEC control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CEC Tx data register."]
    #[inline(always)]
    pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CEC Rx data register."]
    #[inline(always)]
    pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Peripheral enable."]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral enable."]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt enable."]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable."]
        #[inline(always)]
        pub fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bit timing error mode."]
        #[inline(always)]
        pub const fn btem(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bit timing error mode."]
        #[inline(always)]
        pub fn set_btem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bit period error mode."]
        #[inline(always)]
        pub const fn bpem(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bit period error mode."]
        #[inline(always)]
        pub fn set_bpem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("pe", &self.pe())
                .field("ie", &self.ie())
                .field("btem", &self.btem())
                .field("bpem", &self.bpem())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ pe: {=bool:?}, ie: {=bool:?}, btem: {=bool:?}, bpem: {=bool:?} }}",
                self.pe(),
                self.ie(),
                self.btem(),
                self.bpem()
            )
        }
    }
    #[doc = "CEC control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Tx start of message."]
        #[inline(always)]
        pub const fn tsom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tx start of message."]
        #[inline(always)]
        pub fn set_tsom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tx end of message."]
        #[inline(always)]
        pub const fn teom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tx end of message."]
        #[inline(always)]
        pub fn set_teom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Tx error."]
        #[inline(always)]
        pub const fn terr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Tx error."]
        #[inline(always)]
        pub fn set_terr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tx byte transfer request or block transfer finished."]
        #[inline(always)]
        pub const fn tbtrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Tx byte transfer request or block transfer finished."]
        #[inline(always)]
        pub fn set_tbtrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rx start of message."]
        #[inline(always)]
        pub const fn rsom(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx start of message."]
        #[inline(always)]
        pub fn set_rsom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx end of message."]
        #[inline(always)]
        pub const fn reom(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx end of message."]
        #[inline(always)]
        pub fn set_reom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rx error."]
        #[inline(always)]
        pub const fn rerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rx error."]
        #[inline(always)]
        pub fn set_rerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Rx byte/block transfer finished."]
        #[inline(always)]
        pub const fn rbtf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Rx byte/block transfer finished."]
        #[inline(always)]
        pub fn set_rbtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("tsom", &self.tsom())
                .field("teom", &self.teom())
                .field("terr", &self.terr())
                .field("tbtrf", &self.tbtrf())
                .field("rsom", &self.rsom())
                .field("reom", &self.reom())
                .field("rerr", &self.rerr())
                .field("rbtf", &self.rbtf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ tsom: {=bool:?}, teom: {=bool:?}, terr: {=bool:?}, tbtrf: {=bool:?}, rsom: {=bool:?}, reom: {=bool:?}, rerr: {=bool:?}, rbtf: {=bool:?} }}" , self . tsom () , self . teom () , self . terr () , self . tbtrf () , self . rsom () , self . reom () , self . rerr () , self . rbtf ())
        }
    }
    #[doc = "CEC error status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esr(pub u32);
    impl Esr {
        #[doc = "Bit timing error."]
        #[inline(always)]
        pub const fn bte(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bit timing error."]
        #[inline(always)]
        pub fn set_bte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bit period error."]
        #[inline(always)]
        pub const fn bpe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bit period error."]
        #[inline(always)]
        pub fn set_bpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rx block transfer finished error."]
        #[inline(always)]
        pub const fn rbtfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rx block transfer finished error."]
        #[inline(always)]
        pub fn set_rbtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start bit error."]
        #[inline(always)]
        pub const fn sbe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start bit error."]
        #[inline(always)]
        pub fn set_sbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Block acknowledge error."]
        #[inline(always)]
        pub const fn acke(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Block acknowledge error."]
        #[inline(always)]
        pub fn set_acke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Line error."]
        #[inline(always)]
        pub const fn line(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Line error."]
        #[inline(always)]
        pub fn set_line(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Tx block transfer finished error."]
        #[inline(always)]
        pub const fn tbtfe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Tx block transfer finished error."]
        #[inline(always)]
        pub fn set_tbtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Esr {
        #[inline(always)]
        fn default() -> Esr {
            Esr(0)
        }
    }
    impl core::fmt::Debug for Esr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esr")
                .field("bte", &self.bte())
                .field("bpe", &self.bpe())
                .field("rbtfe", &self.rbtfe())
                .field("sbe", &self.sbe())
                .field("acke", &self.acke())
                .field("line", &self.line())
                .field("tbtfe", &self.tbtfe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Esr {{ bte: {=bool:?}, bpe: {=bool:?}, rbtfe: {=bool:?}, sbe: {=bool:?}, acke: {=bool:?}, line: {=bool:?}, tbtfe: {=bool:?} }}" , self . bte () , self . bpe () , self . rbtfe () , self . sbe () , self . acke () , self . line () , self . tbtfe ())
        }
    }
    #[doc = "CEC own address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oar(pub u32);
    impl Oar {
        #[doc = "Own address."]
        #[inline(always)]
        pub const fn oa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Own address."]
        #[inline(always)]
        pub fn set_oa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Oar {
        #[inline(always)]
        fn default() -> Oar {
            Oar(0)
        }
    }
    impl core::fmt::Debug for Oar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oar").field("oa", &self.oa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oar {{ oa: {=u8:?} }}", self.oa())
        }
    }
    #[doc = "Rx Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pres(pub u32);
    impl Pres {
        #[doc = "CEC Rx Data Register."]
        #[inline(always)]
        pub const fn presc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "CEC Rx Data Register."]
        #[inline(always)]
        pub fn set_presc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Pres {
        #[inline(always)]
        fn default() -> Pres {
            Pres(0)
        }
    }
    impl core::fmt::Debug for Pres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pres").field("presc", &self.presc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pres {{ presc: {=u16:?} }}", self.presc())
        }
    }
    #[doc = "CEC Rx data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd(pub u32);
    impl Rxd {
        #[doc = "Rx data."]
        #[inline(always)]
        pub const fn rxd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Rx data."]
        #[inline(always)]
        pub fn set_rxd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rxd {
        #[inline(always)]
        fn default() -> Rxd {
            Rxd(0)
        }
    }
    impl core::fmt::Debug for Rxd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxd {{ rxd: {=u8:?} }}", self.rxd())
        }
    }
    #[doc = "CEC Tx data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd(pub u32);
    impl Txd {
        #[doc = "Tx Data register."]
        #[inline(always)]
        pub const fn txd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Tx Data register."]
        #[inline(always)]
        pub fn set_txd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Txd {
        #[inline(always)]
        fn default() -> Txd {
            Txd(0)
        }
    }
    impl core::fmt::Debug for Txd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txd").field("txd", &self.txd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txd {{ txd: {=u8:?} }}", self.txd())
        }
    }
}
