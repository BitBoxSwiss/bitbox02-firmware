#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Inter-integrated circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmpi2c {
    ptr: *mut u8,
}
unsafe impl Send for Fmpi2c {}
unsafe impl Sync for Fmpi2c {}
impl Fmpi2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Own address register 1"]
    #[inline(always)]
    pub const fn oar1(self) -> crate::common::Reg<regs::Oar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Own address register 2"]
    #[inline(always)]
    pub const fn oar2(self) -> crate::common::Reg<regs::Oar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Timing register"]
    #[inline(always)]
    pub const fn timingr(self) -> crate::common::Reg<regs::Timingr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Timeout register"]
    #[inline(always)]
    pub const fn timeoutr(self) -> crate::common::Reg<regs::Timeoutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Interrupt and Status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PEC register"]
    #[inline(always)]
    pub const fn pecr(self) -> crate::common::Reg<regs::Pecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receive data register"]
    #[inline(always)]
    pub const fn rxdr(self) -> crate::common::Reg<regs::Rxdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn txdr(self) -> crate::common::Reg<regs::Txdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Peripheral enable"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral enable"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX Interrupt enable"]
        #[inline(always)]
        pub const fn txie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX Interrupt enable"]
        #[inline(always)]
        pub fn set_txie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX Interrupt enable"]
        #[inline(always)]
        pub const fn rxie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX Interrupt enable"]
        #[inline(always)]
        pub fn set_rxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address match interrupt enable (slave only)"]
        #[inline(always)]
        pub const fn addrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address match interrupt enable (slave only)"]
        #[inline(always)]
        pub fn set_addrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Not acknowledge received interrupt enable"]
        #[inline(always)]
        pub const fn nackie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Not acknowledge received interrupt enable"]
        #[inline(always)]
        pub fn set_nackie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "STOP detection Interrupt enable"]
        #[inline(always)]
        pub const fn stopie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "STOP detection Interrupt enable"]
        #[inline(always)]
        pub fn set_stopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transfer Complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Error interrupts enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupts enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Digital noise filter"]
        #[inline(always)]
        pub const fn dnf(&self) -> super::vals::Dnf {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Dnf::from_bits(val as u8)
        }
        #[doc = "Digital noise filter"]
        #[inline(always)]
        pub fn set_dnf(&mut self, val: super::vals::Dnf) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Analog noise filter OFF"]
        #[inline(always)]
        pub const fn anfoff(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Analog noise filter OFF"]
        #[inline(always)]
        pub fn set_anfoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA transmission requests enable"]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transmission requests enable"]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DMA reception requests enable"]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DMA reception requests enable"]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Slave byte control"]
        #[inline(always)]
        pub const fn sbc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Slave byte control"]
        #[inline(always)]
        pub fn set_sbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clock stretching disable"]
        #[inline(always)]
        pub const fn nostretch(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clock stretching disable"]
        #[inline(always)]
        pub fn set_nostretch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "General call enable"]
        #[inline(always)]
        pub const fn gcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "General call enable"]
        #[inline(always)]
        pub fn set_gcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SMBus Host address enable"]
        #[inline(always)]
        pub const fn smbhen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus Host address enable"]
        #[inline(always)]
        pub fn set_smbhen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SMBus Device Default address enable"]
        #[inline(always)]
        pub const fn smbden(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus Device Default address enable"]
        #[inline(always)]
        pub fn set_smbden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SMBUS alert enable"]
        #[inline(always)]
        pub const fn alerten(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SMBUS alert enable"]
        #[inline(always)]
        pub fn set_alerten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PEC enable"]
        #[inline(always)]
        pub const fn pecen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PEC enable"]
        #[inline(always)]
        pub fn set_pecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("pe", &self.pe())
                .field("txie", &self.txie())
                .field("rxie", &self.rxie())
                .field("addrie", &self.addrie())
                .field("nackie", &self.nackie())
                .field("stopie", &self.stopie())
                .field("tcie", &self.tcie())
                .field("errie", &self.errie())
                .field("dnf", &self.dnf())
                .field("anfoff", &self.anfoff())
                .field("txdmaen", &self.txdmaen())
                .field("rxdmaen", &self.rxdmaen())
                .field("sbc", &self.sbc())
                .field("nostretch", &self.nostretch())
                .field("gcen", &self.gcen())
                .field("smbhen", &self.smbhen())
                .field("smbden", &self.smbden())
                .field("alerten", &self.alerten())
                .field("pecen", &self.pecen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ pe: {=bool:?}, txie: {=bool:?}, rxie: {=bool:?}, addrie: {=bool:?}, nackie: {=bool:?}, stopie: {=bool:?}, tcie: {=bool:?}, errie: {=bool:?}, dnf: {:?}, anfoff: {=bool:?}, txdmaen: {=bool:?}, rxdmaen: {=bool:?}, sbc: {=bool:?}, nostretch: {=bool:?}, gcen: {=bool:?}, smbhen: {=bool:?}, smbden: {=bool:?}, alerten: {=bool:?}, pecen: {=bool:?} }}" , self . pe () , self . txie () , self . rxie () , self . addrie () , self . nackie () , self . stopie () , self . tcie () , self . errie () , self . dnf () , self . anfoff () , self . txdmaen () , self . rxdmaen () , self . sbc () , self . nostretch () , self . gcen () , self . smbhen () , self . smbden () , self . alerten () , self . pecen ())
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Slave address bit (master mode)"]
        #[inline(always)]
        pub const fn sadd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Slave address bit (master mode)"]
        #[inline(always)]
        pub fn set_sadd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Transfer direction (master mode)"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Transfer direction (master mode)"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "10-bit addressing mode (master mode)"]
        #[inline(always)]
        pub const fn add10(&self) -> super::vals::Addmode {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Addmode::from_bits(val as u8)
        }
        #[doc = "10-bit addressing mode (master mode)"]
        #[inline(always)]
        pub fn set_add10(&mut self, val: super::vals::Addmode) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "10-bit address header only read direction (master receiver mode)"]
        #[inline(always)]
        pub const fn head10r(&self) -> super::vals::Headr {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Headr::from_bits(val as u8)
        }
        #[doc = "10-bit address header only read direction (master receiver mode)"]
        #[inline(always)]
        pub fn set_head10r(&mut self, val: super::vals::Headr) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Start generation"]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Start generation"]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Stop generation (master mode)"]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Stop generation (master mode)"]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "NACK generation (slave mode)"]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "NACK generation (slave mode)"]
        #[inline(always)]
        pub fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Number of bytes"]
        #[inline(always)]
        pub const fn nbytes(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Number of bytes"]
        #[inline(always)]
        pub fn set_nbytes(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "NBYTES reload mode"]
        #[inline(always)]
        pub const fn reload(&self) -> super::vals::Reload {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Reload::from_bits(val as u8)
        }
        #[doc = "NBYTES reload mode"]
        #[inline(always)]
        pub fn set_reload(&mut self, val: super::vals::Reload) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Automatic end mode (master mode)"]
        #[inline(always)]
        pub const fn autoend(&self) -> super::vals::Autoend {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Autoend::from_bits(val as u8)
        }
        #[doc = "Automatic end mode (master mode)"]
        #[inline(always)]
        pub fn set_autoend(&mut self, val: super::vals::Autoend) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Packet error checking byte"]
        #[inline(always)]
        pub const fn pecbyte(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Packet error checking byte"]
        #[inline(always)]
        pub fn set_pecbyte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("sadd", &self.sadd())
                .field("dir", &self.dir())
                .field("add10", &self.add10())
                .field("head10r", &self.head10r())
                .field("start", &self.start())
                .field("stop", &self.stop())
                .field("nack", &self.nack())
                .field("nbytes", &self.nbytes())
                .field("reload", &self.reload())
                .field("autoend", &self.autoend())
                .field("pecbyte", &self.pecbyte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ sadd: {=u16:?}, dir: {:?}, add10: {:?}, head10r: {:?}, start: {=bool:?}, stop: {=bool:?}, nack: {=bool:?}, nbytes: {=u8:?}, reload: {:?}, autoend: {:?}, pecbyte: {=bool:?} }}" , self . sadd () , self . dir () , self . add10 () , self . head10r () , self . start () , self . stop () , self . nack () , self . nbytes () , self . reload () , self . autoend () , self . pecbyte ())
        }
    }
    #[doc = "Interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Address Matched flag clear"]
        #[inline(always)]
        pub const fn addrcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address Matched flag clear"]
        #[inline(always)]
        pub fn set_addrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Not Acknowledge flag clear"]
        #[inline(always)]
        pub const fn nackcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Not Acknowledge flag clear"]
        #[inline(always)]
        pub fn set_nackcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Stop detection flag clear"]
        #[inline(always)]
        pub const fn stopcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stop detection flag clear"]
        #[inline(always)]
        pub fn set_stopcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Bus error flag clear"]
        #[inline(always)]
        pub const fn berrcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bus error flag clear"]
        #[inline(always)]
        pub fn set_berrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Arbitration lost flag clear"]
        #[inline(always)]
        pub const fn arlocf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost flag clear"]
        #[inline(always)]
        pub fn set_arlocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Overrun/Underrun flag clear"]
        #[inline(always)]
        pub const fn ovrcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun/Underrun flag clear"]
        #[inline(always)]
        pub fn set_ovrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PEC Error flag clear"]
        #[inline(always)]
        pub const fn peccf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PEC Error flag clear"]
        #[inline(always)]
        pub fn set_peccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timeout detection flag clear"]
        #[inline(always)]
        pub const fn timoutcf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout detection flag clear"]
        #[inline(always)]
        pub fn set_timoutcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Alert flag clear"]
        #[inline(always)]
        pub const fn alertcf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Alert flag clear"]
        #[inline(always)]
        pub fn set_alertcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("addrcf", &self.addrcf())
                .field("nackcf", &self.nackcf())
                .field("stopcf", &self.stopcf())
                .field("berrcf", &self.berrcf())
                .field("arlocf", &self.arlocf())
                .field("ovrcf", &self.ovrcf())
                .field("peccf", &self.peccf())
                .field("timoutcf", &self.timoutcf())
                .field("alertcf", &self.alertcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ addrcf: {=bool:?}, nackcf: {=bool:?}, stopcf: {=bool:?}, berrcf: {=bool:?}, arlocf: {=bool:?}, ovrcf: {=bool:?}, peccf: {=bool:?}, timoutcf: {=bool:?}, alertcf: {=bool:?} }}" , self . addrcf () , self . nackcf () , self . stopcf () , self . berrcf () , self . arlocf () , self . ovrcf () , self . peccf () , self . timoutcf () , self . alertcf ())
        }
    }
    #[doc = "Interrupt and Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Transmit data register empty (transmitters)"]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data register empty (transmitters)"]
        #[inline(always)]
        pub fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit interrupt status (transmitters)"]
        #[inline(always)]
        pub const fn txis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit interrupt status (transmitters)"]
        #[inline(always)]
        pub fn set_txis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive data register not empty (receivers)"]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data register not empty (receivers)"]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address matched (slave mode)"]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address matched (slave mode)"]
        #[inline(always)]
        pub fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Not acknowledge received flag"]
        #[inline(always)]
        pub const fn nackf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Not acknowledge received flag"]
        #[inline(always)]
        pub fn set_nackf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Stop detection flag"]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stop detection flag"]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transfer Complete (master mode)"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Complete (master mode)"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transfer Complete Reload"]
        #[inline(always)]
        pub const fn tcr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Complete Reload"]
        #[inline(always)]
        pub fn set_tcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Bus error"]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bus error"]
        #[inline(always)]
        pub fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Arbitration lost"]
        #[inline(always)]
        pub const fn arlo(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost"]
        #[inline(always)]
        pub fn set_arlo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Overrun/Underrun (slave mode)"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun/Underrun (slave mode)"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PEC Error in reception"]
        #[inline(always)]
        pub const fn pecerr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PEC Error in reception"]
        #[inline(always)]
        pub fn set_pecerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timeout or t_low detection flag"]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout or t_low detection flag"]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SMBus alert"]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus alert"]
        #[inline(always)]
        pub fn set_alert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Bus busy"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Bus busy"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transfer direction (Slave mode)"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Transfer direction (Slave mode)"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Address match code (Slave mode)"]
        #[inline(always)]
        pub const fn addcode(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "Address match code (Slave mode)"]
        #[inline(always)]
        pub fn set_addcode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
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
                .field("txe", &self.txe())
                .field("txis", &self.txis())
                .field("rxne", &self.rxne())
                .field("addr", &self.addr())
                .field("nackf", &self.nackf())
                .field("stopf", &self.stopf())
                .field("tc", &self.tc())
                .field("tcr", &self.tcr())
                .field("berr", &self.berr())
                .field("arlo", &self.arlo())
                .field("ovr", &self.ovr())
                .field("pecerr", &self.pecerr())
                .field("timeout", &self.timeout())
                .field("alert", &self.alert())
                .field("busy", &self.busy())
                .field("dir", &self.dir())
                .field("addcode", &self.addcode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ txe: {=bool:?}, txis: {=bool:?}, rxne: {=bool:?}, addr: {=bool:?}, nackf: {=bool:?}, stopf: {=bool:?}, tc: {=bool:?}, tcr: {=bool:?}, berr: {=bool:?}, arlo: {=bool:?}, ovr: {=bool:?}, pecerr: {=bool:?}, timeout: {=bool:?}, alert: {=bool:?}, busy: {=bool:?}, dir: {:?}, addcode: {=u8:?} }}" , self . txe () , self . txis () , self . rxne () , self . addr () , self . nackf () , self . stopf () , self . tc () , self . tcr () , self . berr () , self . arlo () , self . ovr () , self . pecerr () , self . timeout () , self . alert () , self . busy () , self . dir () , self . addcode ())
        }
    }
    #[doc = "Own address register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oar1(pub u32);
    impl Oar1 {
        #[doc = "Interface address"]
        #[inline(always)]
        pub const fn oa1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Interface address"]
        #[inline(always)]
        pub fn set_oa1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Own Address 1 10-bit mode"]
        #[inline(always)]
        pub const fn oa1mode(&self) -> super::vals::Addmode {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Addmode::from_bits(val as u8)
        }
        #[doc = "Own Address 1 10-bit mode"]
        #[inline(always)]
        pub fn set_oa1mode(&mut self, val: super::vals::Addmode) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Own Address 1 enable"]
        #[inline(always)]
        pub const fn oa1en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Own Address 1 enable"]
        #[inline(always)]
        pub fn set_oa1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Oar1 {
        #[inline(always)]
        fn default() -> Oar1 {
            Oar1(0)
        }
    }
    impl core::fmt::Debug for Oar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oar1")
                .field("oa1", &self.oa1())
                .field("oa1mode", &self.oa1mode())
                .field("oa1en", &self.oa1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Oar1 {{ oa1: {=u16:?}, oa1mode: {:?}, oa1en: {=bool:?} }}",
                self.oa1(),
                self.oa1mode(),
                self.oa1en()
            )
        }
    }
    #[doc = "Own address register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oar2(pub u32);
    impl Oar2 {
        #[doc = "Interface address"]
        #[inline(always)]
        pub const fn oa2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Interface address"]
        #[inline(always)]
        pub fn set_oa2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "Own Address 2 masks"]
        #[inline(always)]
        pub const fn oa2msk(&self) -> super::vals::Oamsk {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Oamsk::from_bits(val as u8)
        }
        #[doc = "Own Address 2 masks"]
        #[inline(always)]
        pub fn set_oa2msk(&mut self, val: super::vals::Oamsk) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Own Address 2 enable"]
        #[inline(always)]
        pub const fn oa2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Own Address 2 enable"]
        #[inline(always)]
        pub fn set_oa2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Oar2 {
        #[inline(always)]
        fn default() -> Oar2 {
            Oar2(0)
        }
    }
    impl core::fmt::Debug for Oar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oar2")
                .field("oa2", &self.oa2())
                .field("oa2msk", &self.oa2msk())
                .field("oa2en", &self.oa2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Oar2 {{ oa2: {=u8:?}, oa2msk: {:?}, oa2en: {=bool:?} }}",
                self.oa2(),
                self.oa2msk(),
                self.oa2en()
            )
        }
    }
    #[doc = "PEC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pecr(pub u32);
    impl Pecr {
        #[doc = "Packet error checking register"]
        #[inline(always)]
        pub const fn pec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Packet error checking register"]
        #[inline(always)]
        pub fn set_pec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Pecr {
        #[inline(always)]
        fn default() -> Pecr {
            Pecr(0)
        }
    }
    impl core::fmt::Debug for Pecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pecr").field("pec", &self.pec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pecr {{ pec: {=u8:?} }}", self.pec())
        }
    }
    #[doc = "Receive data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdr(pub u32);
    impl Rxdr {
        #[doc = "8-bit receive data"]
        #[inline(always)]
        pub const fn rxdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit receive data"]
        #[inline(always)]
        pub fn set_rxdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rxdr {
        #[inline(always)]
        fn default() -> Rxdr {
            Rxdr(0)
        }
    }
    impl core::fmt::Debug for Rxdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdr").field("rxdata", &self.rxdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxdr {{ rxdata: {=u8:?} }}", self.rxdata())
        }
    }
    #[doc = "Timeout register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timeoutr(pub u32);
    impl Timeoutr {
        #[doc = "Bus timeout A"]
        #[inline(always)]
        pub const fn timeouta(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bus timeout A"]
        #[inline(always)]
        pub fn set_timeouta(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Idle clock timeout detection"]
        #[inline(always)]
        pub const fn tidle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Idle clock timeout detection"]
        #[inline(always)]
        pub fn set_tidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Clock timeout enable"]
        #[inline(always)]
        pub const fn timouten(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Clock timeout enable"]
        #[inline(always)]
        pub fn set_timouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Bus timeout B"]
        #[inline(always)]
        pub const fn timeoutb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bus timeout B"]
        #[inline(always)]
        pub fn set_timeoutb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Extended clock timeout enable"]
        #[inline(always)]
        pub const fn texten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Extended clock timeout enable"]
        #[inline(always)]
        pub fn set_texten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timeoutr {
        #[inline(always)]
        fn default() -> Timeoutr {
            Timeoutr(0)
        }
    }
    impl core::fmt::Debug for Timeoutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timeoutr")
                .field("timeouta", &self.timeouta())
                .field("tidle", &self.tidle())
                .field("timouten", &self.timouten())
                .field("timeoutb", &self.timeoutb())
                .field("texten", &self.texten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timeoutr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Timeoutr {{ timeouta: {=u16:?}, tidle: {=bool:?}, timouten: {=bool:?}, timeoutb: {=u16:?}, texten: {=bool:?} }}" , self . timeouta () , self . tidle () , self . timouten () , self . timeoutb () , self . texten ())
        }
    }
    #[doc = "Timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr(pub u32);
    impl Timingr {
        #[doc = "SCL low period (master mode)"]
        #[inline(always)]
        pub const fn scll(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low period (master mode)"]
        #[inline(always)]
        pub fn set_scll(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "SCL high period (master mode)"]
        #[inline(always)]
        pub const fn sclh(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high period (master mode)"]
        #[inline(always)]
        pub fn set_sclh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data hold time"]
        #[inline(always)]
        pub const fn sdadel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Data hold time"]
        #[inline(always)]
        pub fn set_sdadel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Data setup time"]
        #[inline(always)]
        pub const fn scldel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Data setup time"]
        #[inline(always)]
        pub fn set_scldel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Timing prescaler"]
        #[inline(always)]
        pub const fn presc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Timing prescaler"]
        #[inline(always)]
        pub fn set_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Timingr {
        #[inline(always)]
        fn default() -> Timingr {
            Timingr(0)
        }
    }
    impl core::fmt::Debug for Timingr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr")
                .field("scll", &self.scll())
                .field("sclh", &self.sclh())
                .field("sdadel", &self.sdadel())
                .field("scldel", &self.scldel())
                .field("presc", &self.presc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timingr {{ scll: {=u8:?}, sclh: {=u8:?}, sdadel: {=u8:?}, scldel: {=u8:?}, presc: {=u8:?} }}",
                self.scll(),
                self.sclh(),
                self.sdadel(),
                self.scldel(),
                self.presc()
            )
        }
    }
    #[doc = "Transmit data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdr(pub u32);
    impl Txdr {
        #[doc = "8-bit transmit data"]
        #[inline(always)]
        pub const fn txdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit transmit data"]
        #[inline(always)]
        pub fn set_txdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Txdr {
        #[inline(always)]
        fn default() -> Txdr {
            Txdr(0)
        }
    }
    impl core::fmt::Debug for Txdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdr").field("txdata", &self.txdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txdr {{ txdata: {=u8:?} }}", self.txdata())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Addmode {
        #[doc = "7-bit addressing mode"]
        BIT7 = 0x0,
        #[doc = "10-bit addressing mode"]
        BIT10 = 0x01,
    }
    impl Addmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Addmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Addmode {
        #[inline(always)]
        fn from(val: u8) -> Addmode {
            Addmode::from_bits(val)
        }
    }
    impl From<Addmode> for u8 {
        #[inline(always)]
        fn from(val: Addmode) -> u8 {
            Addmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Autoend {
        #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
        SOFTWARE = 0x0,
        #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
        AUTOMATIC = 0x01,
    }
    impl Autoend {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Autoend {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Autoend {
        #[inline(always)]
        fn from(val: u8) -> Autoend {
            Autoend::from_bits(val)
        }
    }
    impl From<Autoend> for u8 {
        #[inline(always)]
        fn from(val: Autoend) -> u8 {
            Autoend::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "Write transfer, slave enters receiver mode"]
        WRITE = 0x0,
        #[doc = "Read transfer, slave enters transmitter mode"]
        READ = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dnf {
        #[doc = "Digital filter disabled"]
        NO_FILTER = 0x0,
        #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
        FILTER1 = 0x01,
        #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
        FILTER2 = 0x02,
        #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
        FILTER3 = 0x03,
        #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
        FILTER4 = 0x04,
        #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
        FILTER5 = 0x05,
        #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
        FILTER6 = 0x06,
        #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
        FILTER7 = 0x07,
        #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
        FILTER8 = 0x08,
        #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
        FILTER9 = 0x09,
        #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
        FILTER10 = 0x0a,
        #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
        FILTER11 = 0x0b,
        #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
        FILTER12 = 0x0c,
        #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
        FILTER13 = 0x0d,
        #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
        FILTER14 = 0x0e,
        #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
        FILTER15 = 0x0f,
    }
    impl Dnf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dnf {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dnf {
        #[inline(always)]
        fn from(val: u8) -> Dnf {
            Dnf::from_bits(val)
        }
    }
    impl From<Dnf> for u8 {
        #[inline(always)]
        fn from(val: Dnf) -> u8 {
            Dnf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Headr {
        #[doc = "The master sends the complete 10 bit slave address read sequence"]
        COMPLETE = 0x0,
        #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
        PARTIAL = 0x01,
    }
    impl Headr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Headr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Headr {
        #[inline(always)]
        fn from(val: u8) -> Headr {
            Headr::from_bits(val)
        }
    }
    impl From<Headr> for u8 {
        #[inline(always)]
        fn from(val: Headr) -> u8 {
            Headr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Oamsk {
        #[doc = "No mask"]
        NO_MASK = 0x0,
        #[doc = "OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
        MASK1 = 0x01,
        #[doc = "OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
        MASK2 = 0x02,
        #[doc = "OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
        MASK3 = 0x03,
        #[doc = "OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
        MASK4 = 0x04,
        #[doc = "OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
        MASK5 = 0x05,
        #[doc = "OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
        MASK6 = 0x06,
        #[doc = "OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
        MASK7 = 0x07,
    }
    impl Oamsk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Oamsk {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Oamsk {
        #[inline(always)]
        fn from(val: u8) -> Oamsk {
            Oamsk::from_bits(val)
        }
    }
    impl From<Oamsk> for u8 {
        #[inline(always)]
        fn from(val: Oamsk) -> u8 {
            Oamsk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Reload {
        #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
        COMPLETED = 0x0,
        #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
        NOT_COMPLETED = 0x01,
    }
    impl Reload {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Reload {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Reload {
        #[inline(always)]
        fn from(val: u8) -> Reload {
            Reload::from_bits(val)
        }
    }
    impl From<Reload> for u8 {
        #[inline(always)]
        fn from(val: Reload) -> u8 {
            Reload::to_bits(val)
        }
    }
}
