#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Serial peripheral interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Interrupt/Status Flags Clear Register"]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Transmit Data Register - half-word sized"]
    #[inline(always)]
    pub const fn txdr16(self) -> crate::common::Reg<u16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn txdr32(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Transmit Data Register - byte sized"]
    #[inline(always)]
    pub const fn txdr8(self) -> crate::common::Reg<u8, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receive Data Register - half-word sized"]
    #[inline(always)]
    pub const fn rxdr16(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn rxdr32(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Receive Data Register - byte sized"]
    #[inline(always)]
    pub const fn rxdr8(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Polynomial Register"]
    #[inline(always)]
    pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Transmitter CRC Register"]
    #[inline(always)]
    pub const fn txcrc(self) -> crate::common::Reg<regs::Txcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Receiver CRC Register"]
    #[inline(always)]
    pub const fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Underrun Data Register"]
    #[inline(always)]
    pub const fn udrdr(self) -> crate::common::Reg<regs::Udrdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "I2S Configuration Register"]
    #[inline(always)]
    pub const fn i2scfgr(self) -> crate::common::Reg<regs::I2scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "Number of bits in at single SPI data frame"]
        #[inline(always)]
        pub const fn dsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of bits in at single SPI data frame"]
        #[inline(always)]
        pub fn set_dsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "threshold level"]
        #[inline(always)]
        pub const fn fthlv(&self) -> super::vals::Fthlv {
            let val = (self.0 >> 5usize) & 0x0f;
            super::vals::Fthlv::from_bits(val as u8)
        }
        #[doc = "threshold level"]
        #[inline(always)]
        pub fn set_fthlv(&mut self, val: super::vals::Fthlv) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
        }
        #[doc = "Behavior of slave transmitter at underrun condition"]
        #[inline(always)]
        pub const fn udrcfg(&self) -> super::vals::Udrcfg {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Udrcfg::from_bits(val as u8)
        }
        #[doc = "Behavior of slave transmitter at underrun condition"]
        #[inline(always)]
        pub fn set_udrcfg(&mut self, val: super::vals::Udrcfg) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "Rx DMA stream enable"]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Rx DMA stream enable"]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx DMA stream enable"]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx DMA stream enable"]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Length of CRC frame to be transacted and compared"]
        #[inline(always)]
        pub const fn crcsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Length of CRC frame to be transacted and compared"]
        #[inline(always)]
        pub fn set_crcsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Hardware CRC computation enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware CRC computation enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Master baud rate"]
        #[inline(always)]
        pub const fn mbr(&self) -> super::vals::Mbr {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mbr::from_bits(val as u8)
        }
        #[doc = "Master baud rate"]
        #[inline(always)]
        pub fn set_mbr(&mut self, val: super::vals::Mbr) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
        #[doc = "bypass of the prescaler at master baud rate clock generator"]
        #[inline(always)]
        pub const fn bpass(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "bypass of the prescaler at master baud rate clock generator"]
        #[inline(always)]
        pub fn set_bpass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    impl core::fmt::Debug for Cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg1")
                .field("dsize", &self.dsize())
                .field("fthlv", &self.fthlv())
                .field("udrcfg", &self.udrcfg())
                .field("rxdmaen", &self.rxdmaen())
                .field("txdmaen", &self.txdmaen())
                .field("crcsize", &self.crcsize())
                .field("crcen", &self.crcen())
                .field("mbr", &self.mbr())
                .field("bpass", &self.bpass())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg1 {{ dsize: {=u8:?}, fthlv: {:?}, udrcfg: {:?}, rxdmaen: {=bool:?}, txdmaen: {=bool:?}, crcsize: {=u8:?}, crcen: {=bool:?}, mbr: {:?}, bpass: {=bool:?} }}" , self . dsize () , self . fthlv () , self . udrcfg () , self . rxdmaen () , self . txdmaen () , self . crcsize () , self . crcen () , self . mbr () , self . bpass ())
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "Master SS Idleness"]
        #[inline(always)]
        pub const fn mssi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Master SS Idleness"]
        #[inline(always)]
        pub fn set_mssi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Master Inter-Data Idleness"]
        #[inline(always)]
        pub const fn midi(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Master Inter-Data Idleness"]
        #[inline(always)]
        pub fn set_midi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "RDY signal input/output management Note: When DSIZE at the CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
        #[inline(always)]
        pub const fn rdiom(&self) -> super::vals::Rdiom {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Rdiom::from_bits(val as u8)
        }
        #[doc = "RDY signal input/output management Note: When DSIZE at the CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
        #[inline(always)]
        pub fn set_rdiom(&mut self, val: super::vals::Rdiom) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "RDY signal input/output polarity"]
        #[inline(always)]
        pub const fn rdiop(&self) -> super::vals::Rdiop {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Rdiop::from_bits(val as u8)
        }
        #[doc = "RDY signal input/output polarity"]
        #[inline(always)]
        pub fn set_rdiop(&mut self, val: super::vals::Rdiop) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Swap functionality of MISO and MOSI pins"]
        #[inline(always)]
        pub const fn ioswp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Swap functionality of MISO and MOSI pins"]
        #[inline(always)]
        pub fn set_ioswp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPI Communication Mode"]
        #[inline(always)]
        pub const fn comm(&self) -> super::vals::Comm {
            let val = (self.0 >> 17usize) & 0x03;
            super::vals::Comm::from_bits(val as u8)
        }
        #[doc = "SPI Communication Mode"]
        #[inline(always)]
        pub fn set_comm(&mut self, val: super::vals::Comm) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
        }
        #[doc = "Serial Protocol"]
        #[inline(always)]
        pub const fn sp(&self) -> super::vals::Sp {
            let val = (self.0 >> 19usize) & 0x07;
            super::vals::Sp::from_bits(val as u8)
        }
        #[doc = "Serial Protocol"]
        #[inline(always)]
        pub fn set_sp(&mut self, val: super::vals::Sp) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
        }
        #[doc = "SPI Master"]
        #[inline(always)]
        pub const fn master(&self) -> super::vals::Master {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Master::from_bits(val as u8)
        }
        #[doc = "SPI Master"]
        #[inline(always)]
        pub fn set_master(&mut self, val: super::vals::Master) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Data frame format"]
        #[inline(always)]
        pub const fn lsbfirst(&self) -> super::vals::Lsbfirst {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Lsbfirst::from_bits(val as u8)
        }
        #[doc = "Data frame format"]
        #[inline(always)]
        pub fn set_lsbfirst(&mut self, val: super::vals::Lsbfirst) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub const fn cpha(&self) -> super::vals::Cpha {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Cpha::from_bits(val as u8)
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub fn set_cpha(&mut self, val: super::vals::Cpha) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub const fn cpol(&self) -> super::vals::Cpol {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Cpol::from_bits(val as u8)
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub fn set_cpol(&mut self, val: super::vals::Cpol) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Software management of SS signal input"]
        #[inline(always)]
        pub const fn ssm(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Software management of SS signal input"]
        #[inline(always)]
        pub fn set_ssm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SS input/output polarity"]
        #[inline(always)]
        pub const fn ssiop(&self) -> super::vals::Ssiop {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Ssiop::from_bits(val as u8)
        }
        #[doc = "SS input/output polarity"]
        #[inline(always)]
        pub fn set_ssiop(&mut self, val: super::vals::Ssiop) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "SS output enable"]
        #[inline(always)]
        pub const fn ssoe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SS output enable"]
        #[inline(always)]
        pub fn set_ssoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SS output management in master mode"]
        #[inline(always)]
        pub const fn ssom(&self) -> super::vals::Ssom {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ssom::from_bits(val as u8)
        }
        #[doc = "SS output management in master mode"]
        #[inline(always)]
        pub fn set_ssom(&mut self, val: super::vals::Ssom) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Alternate function always control GPIOs"]
        #[inline(always)]
        pub const fn afcntr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function always control GPIOs"]
        #[inline(always)]
        pub fn set_afcntr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    impl core::fmt::Debug for Cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg2")
                .field("mssi", &self.mssi())
                .field("midi", &self.midi())
                .field("rdiom", &self.rdiom())
                .field("rdiop", &self.rdiop())
                .field("ioswp", &self.ioswp())
                .field("comm", &self.comm())
                .field("sp", &self.sp())
                .field("master", &self.master())
                .field("lsbfirst", &self.lsbfirst())
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("ssm", &self.ssm())
                .field("ssiop", &self.ssiop())
                .field("ssoe", &self.ssoe())
                .field("ssom", &self.ssom())
                .field("afcntr", &self.afcntr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg2 {{ mssi: {=u8:?}, midi: {=u8:?}, rdiom: {:?}, rdiop: {:?}, ioswp: {=bool:?}, comm: {:?}, sp: {:?}, master: {:?}, lsbfirst: {:?}, cpha: {:?}, cpol: {:?}, ssm: {=bool:?}, ssiop: {:?}, ssoe: {=bool:?}, ssom: {:?}, afcntr: {=bool:?} }}" , self . mssi () , self . midi () , self . rdiom () , self . rdiop () , self . ioswp () , self . comm () , self . sp () , self . master () , self . lsbfirst () , self . cpha () , self . cpol () , self . ssm () , self . ssiop () , self . ssoe () , self . ssom () , self . afcntr ())
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Serial Peripheral Enable"]
        #[inline(always)]
        pub const fn spe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Serial Peripheral Enable"]
        #[inline(always)]
        pub fn set_spe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master automatic SUSP in Receive mode"]
        #[inline(always)]
        pub const fn masrx(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master automatic SUSP in Receive mode"]
        #[inline(always)]
        pub fn set_masrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Master transfer start"]
        #[inline(always)]
        pub const fn cstart(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master transfer start"]
        #[inline(always)]
        pub fn set_cstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Master SUSPend request"]
        #[inline(always)]
        pub const fn csusp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Master SUSPend request"]
        #[inline(always)]
        pub fn set_csusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Rx/Tx direction at Half-duplex mode"]
        #[inline(always)]
        pub const fn hddir(&self) -> super::vals::Hddir {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Hddir::from_bits(val as u8)
        }
        #[doc = "Rx/Tx direction at Half-duplex mode"]
        #[inline(always)]
        pub fn set_hddir(&mut self, val: super::vals::Hddir) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Internal SS signal input level"]
        #[inline(always)]
        pub const fn ssi(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Internal SS signal input level"]
        #[inline(always)]
        pub fn set_ssi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Full size (33-bit or 17-bit) CRC polynomial is used"]
        #[inline(always)]
        pub const fn crc33_17(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Full size (33-bit or 17-bit) CRC polynomial is used"]
        #[inline(always)]
        pub fn set_crc33_17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CRC calculation initialization pattern control for receiver"]
        #[inline(always)]
        pub const fn rcrcini(&self) -> super::vals::Rcrcini {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Rcrcini::from_bits(val as u8)
        }
        #[doc = "CRC calculation initialization pattern control for receiver"]
        #[inline(always)]
        pub fn set_rcrcini(&mut self, val: super::vals::Rcrcini) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "CRC calculation initialization pattern control for transmitter"]
        #[inline(always)]
        pub const fn tcrcini(&self) -> super::vals::Tcrcini {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Tcrcini::from_bits(val as u8)
        }
        #[doc = "CRC calculation initialization pattern control for transmitter"]
        #[inline(always)]
        pub fn set_tcrcini(&mut self, val: super::vals::Tcrcini) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Locking the AF configuration of associated IOs"]
        #[inline(always)]
        pub const fn iolock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Locking the AF configuration of associated IOs"]
        #[inline(always)]
        pub fn set_iolock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("spe", &self.spe())
                .field("masrx", &self.masrx())
                .field("cstart", &self.cstart())
                .field("csusp", &self.csusp())
                .field("hddir", &self.hddir())
                .field("ssi", &self.ssi())
                .field("crc33_17", &self.crc33_17())
                .field("rcrcini", &self.rcrcini())
                .field("tcrcini", &self.tcrcini())
                .field("iolock", &self.iolock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ spe: {=bool:?}, masrx: {=bool:?}, cstart: {=bool:?}, csusp: {=bool:?}, hddir: {:?}, ssi: {=bool:?}, crc33_17: {=bool:?}, rcrcini: {:?}, tcrcini: {:?}, iolock: {=bool:?} }}" , self . spe () , self . masrx () , self . cstart () , self . csusp () , self . hddir () , self . ssi () , self . crc33_17 () , self . rcrcini () , self . tcrcini () , self . iolock ())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Number of data at current transfer"]
        #[inline(always)]
        pub const fn tsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data at current transfer"]
        #[inline(always)]
        pub fn set_tsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            f.debug_struct("Cr2").field("tsize", &self.tsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr2 {{ tsize: {=u16:?} }}", self.tsize())
        }
    }
    #[doc = "Polynomial Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcpoly(pub u32);
    impl Crcpoly {
        #[doc = "CRC polynomial register"]
        #[inline(always)]
        pub const fn crcpoly(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC polynomial register"]
        #[inline(always)]
        pub fn set_crcpoly(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crcpoly {
        #[inline(always)]
        fn default() -> Crcpoly {
            Crcpoly(0)
        }
    }
    impl core::fmt::Debug for Crcpoly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcpoly").field("crcpoly", &self.crcpoly()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcpoly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crcpoly {{ crcpoly: {=u32:?} }}", self.crcpoly())
        }
    }
    #[doc = "I2S Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2scfgr(pub u32);
    impl I2scfgr {
        #[doc = "I2S mode selection"]
        #[inline(always)]
        pub const fn i2smod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2S mode selection"]
        #[inline(always)]
        pub fn set_i2smod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2S configuration mode"]
        #[inline(always)]
        pub const fn i2scfg(&self) -> super::vals::I2scfg {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::I2scfg::from_bits(val as u8)
        }
        #[doc = "I2S configuration mode"]
        #[inline(always)]
        pub fn set_i2scfg(&mut self, val: super::vals::I2scfg) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "I2S standard selection"]
        #[inline(always)]
        pub const fn i2sstd(&self) -> super::vals::I2sstd {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::I2sstd::from_bits(val as u8)
        }
        #[doc = "I2S standard selection"]
        #[inline(always)]
        pub fn set_i2sstd(&mut self, val: super::vals::I2sstd) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "PCM frame synchronization"]
        #[inline(always)]
        pub const fn pcmsync(&self) -> super::vals::Pcmsync {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Pcmsync::from_bits(val as u8)
        }
        #[doc = "PCM frame synchronization"]
        #[inline(always)]
        pub fn set_pcmsync(&mut self, val: super::vals::Pcmsync) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Data length to be transferred"]
        #[inline(always)]
        pub const fn datlen(&self) -> super::vals::Datlen {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Datlen::from_bits(val as u8)
        }
        #[doc = "Data length to be transferred"]
        #[inline(always)]
        pub fn set_datlen(&mut self, val: super::vals::Datlen) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Channel length (number of bits per audio channel)"]
        #[inline(always)]
        pub const fn chlen(&self) -> super::vals::Chlen {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Chlen::from_bits(val as u8)
        }
        #[doc = "Channel length (number of bits per audio channel)"]
        #[inline(always)]
        pub fn set_chlen(&mut self, val: super::vals::Chlen) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Serial audio clock polarity"]
        #[inline(always)]
        pub const fn ckpol(&self) -> super::vals::Ckpol {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ckpol::from_bits(val as u8)
        }
        #[doc = "Serial audio clock polarity"]
        #[inline(always)]
        pub fn set_ckpol(&mut self, val: super::vals::Ckpol) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Fixed channel length in slave"]
        #[inline(always)]
        pub const fn fixch(&self) -> super::vals::Fixch {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Fixch::from_bits(val as u8)
        }
        #[doc = "Fixed channel length in slave"]
        #[inline(always)]
        pub fn set_fixch(&mut self, val: super::vals::Fixch) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Word select inversion"]
        #[inline(always)]
        pub const fn wsinv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Word select inversion"]
        #[inline(always)]
        pub fn set_wsinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Data format"]
        #[inline(always)]
        pub const fn datfmt(&self) -> super::vals::Datfmt {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Datfmt::from_bits(val as u8)
        }
        #[doc = "Data format"]
        #[inline(always)]
        pub fn set_datfmt(&mut self, val: super::vals::Datfmt) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "I2S linear prescaler"]
        #[inline(always)]
        pub const fn i2sdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "I2S linear prescaler"]
        #[inline(always)]
        pub fn set_i2sdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Odd factor for the prescaler"]
        #[inline(always)]
        pub const fn odd(&self) -> super::vals::Odd {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Odd::from_bits(val as u8)
        }
        #[doc = "Odd factor for the prescaler"]
        #[inline(always)]
        pub fn set_odd(&mut self, val: super::vals::Odd) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Master clock output enable"]
        #[inline(always)]
        pub const fn mckoe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock output enable"]
        #[inline(always)]
        pub fn set_mckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for I2scfgr {
        #[inline(always)]
        fn default() -> I2scfgr {
            I2scfgr(0)
        }
    }
    impl core::fmt::Debug for I2scfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2scfgr")
                .field("i2smod", &self.i2smod())
                .field("i2scfg", &self.i2scfg())
                .field("i2sstd", &self.i2sstd())
                .field("pcmsync", &self.pcmsync())
                .field("datlen", &self.datlen())
                .field("chlen", &self.chlen())
                .field("ckpol", &self.ckpol())
                .field("fixch", &self.fixch())
                .field("wsinv", &self.wsinv())
                .field("datfmt", &self.datfmt())
                .field("i2sdiv", &self.i2sdiv())
                .field("odd", &self.odd())
                .field("mckoe", &self.mckoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2scfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "I2scfgr {{ i2smod: {=bool:?}, i2scfg: {:?}, i2sstd: {:?}, pcmsync: {:?}, datlen: {:?}, chlen: {:?}, ckpol: {:?}, fixch: {:?}, wsinv: {=bool:?}, datfmt: {:?}, i2sdiv: {=u8:?}, odd: {:?}, mckoe: {=bool:?} }}" , self . i2smod () , self . i2scfg () , self . i2sstd () , self . pcmsync () , self . datlen () , self . chlen () , self . ckpol () , self . fixch () , self . wsinv () , self . datfmt () , self . i2sdiv () , self . odd () , self . mckoe ())
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "RXP Interrupt Enable"]
        #[inline(always)]
        pub const fn rxpie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RXP Interrupt Enable"]
        #[inline(always)]
        pub fn set_rxpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TXP interrupt enable"]
        #[inline(always)]
        pub const fn txpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TXP interrupt enable"]
        #[inline(always)]
        pub fn set_txpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DXP interrupt enabled"]
        #[inline(always)]
        pub const fn dxpie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DXP interrupt enabled"]
        #[inline(always)]
        pub fn set_dxpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOT, SUSP and TXC interrupt enable"]
        #[inline(always)]
        pub const fn eotie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOT, SUSP and TXC interrupt enable"]
        #[inline(always)]
        pub fn set_eotie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TXTFIE interrupt enable"]
        #[inline(always)]
        pub const fn txtfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TXTFIE interrupt enable"]
        #[inline(always)]
        pub fn set_txtfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UDR interrupt enable"]
        #[inline(always)]
        pub const fn udrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UDR interrupt enable"]
        #[inline(always)]
        pub fn set_udrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OVR interrupt enable"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "OVR interrupt enable"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CRC Interrupt enable"]
        #[inline(always)]
        pub const fn crceie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Interrupt enable"]
        #[inline(always)]
        pub fn set_crceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIFRE interrupt enable"]
        #[inline(always)]
        pub const fn tifreie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIFRE interrupt enable"]
        #[inline(always)]
        pub fn set_tifreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Mode Fault interrupt enable"]
        #[inline(always)]
        pub const fn modfie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Mode Fault interrupt enable"]
        #[inline(always)]
        pub fn set_modfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("rxpie", &self.rxpie())
                .field("txpie", &self.txpie())
                .field("dxpie", &self.dxpie())
                .field("eotie", &self.eotie())
                .field("txtfie", &self.txtfie())
                .field("udrie", &self.udrie())
                .field("ovrie", &self.ovrie())
                .field("crceie", &self.crceie())
                .field("tifreie", &self.tifreie())
                .field("modfie", &self.modfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ rxpie: {=bool:?}, txpie: {=bool:?}, dxpie: {=bool:?}, eotie: {=bool:?}, txtfie: {=bool:?}, udrie: {=bool:?}, ovrie: {=bool:?}, crceie: {=bool:?}, tifreie: {=bool:?}, modfie: {=bool:?} }}" , self . rxpie () , self . txpie () , self . dxpie () , self . eotie () , self . txtfie () , self . udrie () , self . ovrie () , self . crceie () , self . tifreie () , self . modfie ())
        }
    }
    #[doc = "Interrupt/Status Flags Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifcr(pub u32);
    impl Ifcr {
        #[doc = "End Of Transfer flag clear"]
        #[inline(always)]
        pub const fn eotc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End Of Transfer flag clear"]
        #[inline(always)]
        pub fn set_eotc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmission Transfer Filled flag clear"]
        #[inline(always)]
        pub const fn txtfc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Transfer Filled flag clear"]
        #[inline(always)]
        pub fn set_txtfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Underrun flag clear"]
        #[inline(always)]
        pub const fn udrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Underrun flag clear"]
        #[inline(always)]
        pub fn set_udrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Overrun flag clear"]
        #[inline(always)]
        pub const fn ovrc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag clear"]
        #[inline(always)]
        pub fn set_ovrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CRC Error flag clear"]
        #[inline(always)]
        pub const fn crcec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Error flag clear"]
        #[inline(always)]
        pub fn set_crcec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TI frame format error flag clear"]
        #[inline(always)]
        pub const fn tifrec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TI frame format error flag clear"]
        #[inline(always)]
        pub fn set_tifrec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Mode Fault flag clear"]
        #[inline(always)]
        pub const fn modfc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Mode Fault flag clear"]
        #[inline(always)]
        pub fn set_modfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SUSPend flag clear"]
        #[inline(always)]
        pub const fn suspc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SUSPend flag clear"]
        #[inline(always)]
        pub fn set_suspc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Ifcr {
        #[inline(always)]
        fn default() -> Ifcr {
            Ifcr(0)
        }
    }
    impl core::fmt::Debug for Ifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifcr")
                .field("eotc", &self.eotc())
                .field("txtfc", &self.txtfc())
                .field("udrc", &self.udrc())
                .field("ovrc", &self.ovrc())
                .field("crcec", &self.crcec())
                .field("tifrec", &self.tifrec())
                .field("modfc", &self.modfc())
                .field("suspc", &self.suspc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ifcr {{ eotc: {=bool:?}, txtfc: {=bool:?}, udrc: {=bool:?}, ovrc: {=bool:?}, crcec: {=bool:?}, tifrec: {=bool:?}, modfc: {=bool:?}, suspc: {=bool:?} }}" , self . eotc () , self . txtfc () , self . udrc () , self . ovrc () , self . crcec () , self . tifrec () , self . modfc () , self . suspc ())
        }
    }
    #[doc = "Receiver CRC Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcrc(pub u32);
    impl Rxcrc {
        #[doc = "CRC register for receiver"]
        #[inline(always)]
        pub const fn rxcrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC register for receiver"]
        #[inline(always)]
        pub fn set_rxcrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxcrc {
        #[inline(always)]
        fn default() -> Rxcrc {
            Rxcrc(0)
        }
    }
    impl core::fmt::Debug for Rxcrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxcrc").field("rxcrc", &self.rxcrc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxcrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxcrc {{ rxcrc: {=u32:?} }}", self.rxcrc())
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Rx-Packet available"]
        #[inline(always)]
        pub const fn rxp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Packet available"]
        #[inline(always)]
        pub fn set_rxp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tx-Packet space available"]
        #[inline(always)]
        pub const fn txp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Packet space available"]
        #[inline(always)]
        pub fn set_txp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Duplex Packet"]
        #[inline(always)]
        pub const fn dxp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Duplex Packet"]
        #[inline(always)]
        pub fn set_dxp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End Of Transfer"]
        #[inline(always)]
        pub const fn eot(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End Of Transfer"]
        #[inline(always)]
        pub fn set_eot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmission Transfer Filled"]
        #[inline(always)]
        pub const fn txtf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Transfer Filled"]
        #[inline(always)]
        pub fn set_txtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Underrun at slave transmission mode"]
        #[inline(always)]
        pub const fn udr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Underrun at slave transmission mode"]
        #[inline(always)]
        pub fn set_udr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Overrun"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CRC Error"]
        #[inline(always)]
        pub const fn crce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Error"]
        #[inline(always)]
        pub fn set_crce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TI frame format error"]
        #[inline(always)]
        pub const fn tifre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TI frame format error"]
        #[inline(always)]
        pub fn set_tifre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Mode Fault"]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Mode Fault"]
        #[inline(always)]
        pub fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SUSPend"]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SUSPend"]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TxFIFO transmission complete"]
        #[inline(always)]
        pub const fn txc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TxFIFO transmission complete"]
        #[inline(always)]
        pub fn set_txc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RxFIFO Packing LeVeL"]
        #[inline(always)]
        pub const fn rxplvl(&self) -> super::vals::Rxplvl {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Rxplvl::from_bits(val as u8)
        }
        #[doc = "RxFIFO Packing LeVeL"]
        #[inline(always)]
        pub fn set_rxplvl(&mut self, val: super::vals::Rxplvl) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "RxFIFO Word Not Empty"]
        #[inline(always)]
        pub const fn rxwne(&self) -> super::vals::Rxwne {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Rxwne::from_bits(val as u8)
        }
        #[doc = "RxFIFO Word Not Empty"]
        #[inline(always)]
        pub fn set_rxwne(&mut self, val: super::vals::Rxwne) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Number of data frames remaining in current TSIZE session"]
        #[inline(always)]
        pub const fn ctsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data frames remaining in current TSIZE session"]
        #[inline(always)]
        pub fn set_ctsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("rxp", &self.rxp())
                .field("txp", &self.txp())
                .field("dxp", &self.dxp())
                .field("eot", &self.eot())
                .field("txtf", &self.txtf())
                .field("udr", &self.udr())
                .field("ovr", &self.ovr())
                .field("crce", &self.crce())
                .field("tifre", &self.tifre())
                .field("modf", &self.modf())
                .field("susp", &self.susp())
                .field("txc", &self.txc())
                .field("rxplvl", &self.rxplvl())
                .field("rxwne", &self.rxwne())
                .field("ctsize", &self.ctsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ rxp: {=bool:?}, txp: {=bool:?}, dxp: {=bool:?}, eot: {=bool:?}, txtf: {=bool:?}, udr: {=bool:?}, ovr: {=bool:?}, crce: {=bool:?}, tifre: {=bool:?}, modf: {=bool:?}, susp: {=bool:?}, txc: {=bool:?}, rxplvl: {:?}, rxwne: {:?}, ctsize: {=u16:?} }}" , self . rxp () , self . txp () , self . dxp () , self . eot () , self . txtf () , self . udr () , self . ovr () , self . crce () , self . tifre () , self . modf () , self . susp () , self . txc () , self . rxplvl () , self . rxwne () , self . ctsize ())
        }
    }
    #[doc = "Transmitter CRC Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txcrc(pub u32);
    impl Txcrc {
        #[doc = "CRC register for transmitter"]
        #[inline(always)]
        pub const fn txcrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC register for transmitter"]
        #[inline(always)]
        pub fn set_txcrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txcrc {
        #[inline(always)]
        fn default() -> Txcrc {
            Txcrc(0)
        }
    }
    impl core::fmt::Debug for Txcrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txcrc").field("txcrc", &self.txcrc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txcrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txcrc {{ txcrc: {=u32:?} }}", self.txcrc())
        }
    }
    #[doc = "Underrun Data Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Udrdr(pub u32);
    impl Udrdr {
        #[doc = "Data at slave underrun condition"]
        #[inline(always)]
        pub const fn udrdr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data at slave underrun condition"]
        #[inline(always)]
        pub fn set_udrdr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Udrdr {
        #[inline(always)]
        fn default() -> Udrdr {
            Udrdr(0)
        }
    }
    impl core::fmt::Debug for Udrdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Udrdr").field("udrdr", &self.udrdr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Udrdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Udrdr {{ udrdr: {=u32:?} }}", self.udrdr())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chlen {
        #[doc = "16 bits per channel"]
        BITS16 = 0x0,
        #[doc = "32 bits per channel"]
        BITS32 = 0x01,
    }
    impl Chlen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chlen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chlen {
        #[inline(always)]
        fn from(val: u8) -> Chlen {
            Chlen::from_bits(val)
        }
    }
    impl From<Chlen> for u8 {
        #[inline(always)]
        fn from(val: Chlen) -> u8 {
            Chlen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckpol {
        #[doc = "CK idle Level is Low. Signals are sampled on rising and changed on falling clock edges"]
        IDLE_LOW = 0x0,
        #[doc = "CK idle level is High. Signals are sampled on falling and changed on rising clock edges"]
        IDLE_HIGH = 0x01,
    }
    impl Ckpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckpol {
        #[inline(always)]
        fn from(val: u8) -> Ckpol {
            Ckpol::from_bits(val)
        }
    }
    impl From<Ckpol> for u8 {
        #[inline(always)]
        fn from(val: Ckpol) -> u8 {
            Ckpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Comm {
        #[doc = "Full duplex"]
        FULL_DUPLEX = 0x0,
        #[doc = "Simplex transmitter only"]
        TRANSMITTER = 0x01,
        #[doc = "Simplex receiver only"]
        RECEIVER = 0x02,
        #[doc = "Half duplex"]
        HALF_DUPLEX = 0x03,
    }
    impl Comm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Comm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Comm {
        #[inline(always)]
        fn from(val: u8) -> Comm {
            Comm::from_bits(val)
        }
    }
    impl From<Comm> for u8 {
        #[inline(always)]
        fn from(val: Comm) -> u8 {
            Comm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpha {
        #[doc = "The first clock transition is the first data capture edge"]
        FIRST_EDGE = 0x0,
        #[doc = "The second clock transition is the first data capture edge"]
        SECOND_EDGE = 0x01,
    }
    impl Cpha {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpha {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpha {
        #[inline(always)]
        fn from(val: u8) -> Cpha {
            Cpha::from_bits(val)
        }
    }
    impl From<Cpha> for u8 {
        #[inline(always)]
        fn from(val: Cpha) -> u8 {
            Cpha::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpol {
        #[doc = "SCK to 0 when idle"]
        IDLE_LOW = 0x0,
        #[doc = "SCK to 1 when idle"]
        IDLE_HIGH = 0x01,
    }
    impl Cpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpol {
        #[inline(always)]
        fn from(val: u8) -> Cpol {
            Cpol::from_bits(val)
        }
    }
    impl From<Cpol> for u8 {
        #[inline(always)]
        fn from(val: Cpol) -> u8 {
            Cpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datfmt {
        #[doc = "The data inside RXDR and TXDR are right aligned"]
        RIGHT_ALIGNED = 0x0,
        #[doc = "The data inside RXDR and TXDR are left aligned"]
        LEFT_ALIGNED = 0x01,
    }
    impl Datfmt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datfmt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datfmt {
        #[inline(always)]
        fn from(val: u8) -> Datfmt {
            Datfmt::from_bits(val)
        }
    }
    impl From<Datfmt> for u8 {
        #[inline(always)]
        fn from(val: Datfmt) -> u8 {
            Datfmt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datlen {
        #[doc = "16-bit data length"]
        BITS16 = 0x0,
        #[doc = "24-bit data length"]
        BITS24 = 0x01,
        #[doc = "32-bit data length"]
        BITS32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Datlen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datlen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datlen {
        #[inline(always)]
        fn from(val: u8) -> Datlen {
            Datlen::from_bits(val)
        }
    }
    impl From<Datlen> for u8 {
        #[inline(always)]
        fn from(val: Datlen) -> u8 {
            Datlen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fixch {
        #[doc = "The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
        NOT_FIXED = 0x0,
        #[doc = "The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
        FIXED = 0x01,
    }
    impl Fixch {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fixch {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fixch {
        #[inline(always)]
        fn from(val: u8) -> Fixch {
            Fixch::from_bits(val)
        }
    }
    impl From<Fixch> for u8 {
        #[inline(always)]
        fn from(val: Fixch) -> u8 {
            Fixch::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fthlv {
        #[doc = "1 frame"]
        ONE_FRAME = 0x0,
        #[doc = "2 frames"]
        TWO_FRAMES = 0x01,
        #[doc = "3 frames"]
        THREE_FRAMES = 0x02,
        #[doc = "4 frames"]
        FOUR_FRAMES = 0x03,
        #[doc = "5 frames"]
        FIVE_FRAMES = 0x04,
        #[doc = "6 frames"]
        SIX_FRAMES = 0x05,
        #[doc = "7 frames"]
        SEVEN_FRAMES = 0x06,
        #[doc = "8 frames"]
        EIGHT_FRAMES = 0x07,
        #[doc = "9 frames"]
        NINE_FRAMES = 0x08,
        #[doc = "10 frames"]
        TEN_FRAMES = 0x09,
        #[doc = "11 frames"]
        ELEVEN_FRAMES = 0x0a,
        #[doc = "12 frames"]
        TWELVE_FRAMES = 0x0b,
        #[doc = "13 frames"]
        THIRTEEN_FRAMES = 0x0c,
        #[doc = "14 frames"]
        FOURTEEN_FRAMES = 0x0d,
        #[doc = "15 frames"]
        FIFTEEN_FRAMES = 0x0e,
        #[doc = "16 frames"]
        SIXTEEN_FRAMES = 0x0f,
    }
    impl Fthlv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fthlv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fthlv {
        #[inline(always)]
        fn from(val: u8) -> Fthlv {
            Fthlv::from_bits(val)
        }
    }
    impl From<Fthlv> for u8 {
        #[inline(always)]
        fn from(val: Fthlv) -> u8 {
            Fthlv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hddir {
        #[doc = "Receiver in half duplex mode"]
        RECEIVER = 0x0,
        #[doc = "Transmitter in half duplex mode"]
        TRANSMITTER = 0x01,
    }
    impl Hddir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hddir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hddir {
        #[inline(always)]
        fn from(val: u8) -> Hddir {
            Hddir::from_bits(val)
        }
    }
    impl From<Hddir> for u8 {
        #[inline(always)]
        fn from(val: Hddir) -> u8 {
            Hddir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2scfg {
        #[doc = "Slave, transmit"]
        SLAVE_TX = 0x0,
        #[doc = "Slave, receive"]
        SLAVE_RX = 0x01,
        #[doc = "Master, transmit"]
        MASTER_TX = 0x02,
        #[doc = "Master, receive"]
        MASTER_RX = 0x03,
        #[doc = "Slave, full duplex"]
        SLAVE_FULL_DUPLEX = 0x04,
        #[doc = "Master, full duplex"]
        MASTER_FULL_DUPLEX = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl I2scfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2scfg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2scfg {
        #[inline(always)]
        fn from(val: u8) -> I2scfg {
            I2scfg::from_bits(val)
        }
    }
    impl From<I2scfg> for u8 {
        #[inline(always)]
        fn from(val: I2scfg) -> u8 {
            I2scfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2sstd {
        #[doc = "I2S Philips standard"]
        PHILIPS = 0x0,
        #[doc = "MSB/left justified standard"]
        MSB = 0x01,
        #[doc = "LSB/right justified standard"]
        LSB = 0x02,
        #[doc = "PCM standard"]
        PCM = 0x03,
    }
    impl I2sstd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2sstd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2sstd {
        #[inline(always)]
        fn from(val: u8) -> I2sstd {
            I2sstd::from_bits(val)
        }
    }
    impl From<I2sstd> for u8 {
        #[inline(always)]
        fn from(val: I2sstd) -> u8 {
            I2sstd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsbfirst {
        #[doc = "Data is transmitted/received with the MSB first"]
        MSBFIRST = 0x0,
        #[doc = "Data is transmitted/received with the LSB first"]
        LSBFIRST = 0x01,
    }
    impl Lsbfirst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsbfirst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsbfirst {
        #[inline(always)]
        fn from(val: u8) -> Lsbfirst {
            Lsbfirst::from_bits(val)
        }
    }
    impl From<Lsbfirst> for u8 {
        #[inline(always)]
        fn from(val: Lsbfirst) -> u8 {
            Lsbfirst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Master {
        #[doc = "Slave configuration"]
        SLAVE = 0x0,
        #[doc = "Master configuration"]
        MASTER = 0x01,
    }
    impl Master {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Master {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Master {
        #[inline(always)]
        fn from(val: u8) -> Master {
            Master::from_bits(val)
        }
    }
    impl From<Master> for u8 {
        #[inline(always)]
        fn from(val: Master) -> u8 {
            Master::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mbr {
        #[doc = "f_spi_ker_ck / 2"]
        DIV2 = 0x0,
        #[doc = "f_spi_ker_ck / 4"]
        DIV4 = 0x01,
        #[doc = "f_spi_ker_ck / 8"]
        DIV8 = 0x02,
        #[doc = "f_spi_ker_ck / 16"]
        DIV16 = 0x03,
        #[doc = "f_spi_ker_ck / 32"]
        DIV32 = 0x04,
        #[doc = "f_spi_ker_ck / 64"]
        DIV64 = 0x05,
        #[doc = "f_spi_ker_ck / 128"]
        DIV128 = 0x06,
        #[doc = "f_spi_ker_ck / 256"]
        DIV256 = 0x07,
    }
    impl Mbr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mbr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mbr {
        #[inline(always)]
        fn from(val: u8) -> Mbr {
            Mbr::from_bits(val)
        }
    }
    impl From<Mbr> for u8 {
        #[inline(always)]
        fn from(val: Mbr) -> u8 {
            Mbr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Odd {
        #[doc = "Real divider value is I2SDIV*2"]
        EVEN = 0x0,
        #[doc = "Real divider value is I2SDIV*2 + 1"]
        ODD = 0x01,
    }
    impl Odd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Odd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Odd {
        #[inline(always)]
        fn from(val: u8) -> Odd {
            Odd::from_bits(val)
        }
    }
    impl From<Odd> for u8 {
        #[inline(always)]
        fn from(val: Odd) -> u8 {
            Odd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pcmsync {
        #[doc = "Short PCM frame synchronization"]
        SHORT = 0x0,
        #[doc = "Long PCM frame synchronization"]
        LONG = 0x01,
    }
    impl Pcmsync {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcmsync {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcmsync {
        #[inline(always)]
        fn from(val: u8) -> Pcmsync {
            Pcmsync::from_bits(val)
        }
    }
    impl From<Pcmsync> for u8 {
        #[inline(always)]
        fn from(val: Pcmsync) -> u8 {
            Pcmsync::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rcrcini {
        #[doc = "All zeros RX CRC initialization pattern"]
        ALL_ZEROS = 0x0,
        #[doc = "All ones RX CRC initialization pattern"]
        ALL_ONES = 0x01,
    }
    impl Rcrcini {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rcrcini {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rcrcini {
        #[inline(always)]
        fn from(val: u8) -> Rcrcini {
            Rcrcini::from_bits(val)
        }
    }
    impl From<Rcrcini> for u8 {
        #[inline(always)]
        fn from(val: Rcrcini) -> u8 {
            Rcrcini::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rdiom {
        #[doc = "RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)"]
        PERMANENTLY_ACTIVE = 0x0,
        #[doc = "RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)"]
        FROM_INPUT = 0x01,
    }
    impl Rdiom {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rdiom {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rdiom {
        #[inline(always)]
        fn from(val: u8) -> Rdiom {
            Rdiom::from_bits(val)
        }
    }
    impl From<Rdiom> for u8 {
        #[inline(always)]
        fn from(val: Rdiom) -> u8 {
            Rdiom::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rdiop {
        #[doc = "high level of the signal means the slave is ready for communication"]
        READY_HIGH = 0x0,
        #[doc = "low level of the signal means the slave is ready for communication"]
        READY_LOW = 0x01,
    }
    impl Rdiop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rdiop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rdiop {
        #[inline(always)]
        fn from(val: u8) -> Rdiop {
            Rdiop::from_bits(val)
        }
    }
    impl From<Rdiop> for u8 {
        #[inline(always)]
        fn from(val: Rdiop) -> u8 {
            Rdiop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxplvl {
        #[doc = "Zero frames beyond packing ratio available"]
        ZERO_FRAMES = 0x0,
        #[doc = "One frame beyond packing ratio available"]
        ONE_FRAME = 0x01,
        #[doc = "Two frame beyond packing ratio available"]
        TWO_FRAMES = 0x02,
        #[doc = "Three frame beyond packing ratio available"]
        THREE_FRAMES = 0x03,
    }
    impl Rxplvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxplvl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxplvl {
        #[inline(always)]
        fn from(val: u8) -> Rxplvl {
            Rxplvl::from_bits(val)
        }
    }
    impl From<Rxplvl> for u8 {
        #[inline(always)]
        fn from(val: Rxplvl) -> u8 {
            Rxplvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxwne {
        #[doc = "Less than 32-bit data frame received"]
        LESS_THAN32 = 0x0,
        #[doc = "At least 32-bit data frame received"]
        AT_LEAST32 = 0x01,
    }
    impl Rxwne {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxwne {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxwne {
        #[inline(always)]
        fn from(val: u8) -> Rxwne {
            Rxwne::from_bits(val)
        }
    }
    impl From<Rxwne> for u8 {
        #[inline(always)]
        fn from(val: Rxwne) -> u8 {
            Rxwne::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sp {
        #[doc = "Motorola SPI protocol"]
        MOTOROLA = 0x0,
        #[doc = "TI SPI protocol"]
        TI = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sp {
        #[inline(always)]
        fn from(val: u8) -> Sp {
            Sp::from_bits(val)
        }
    }
    impl From<Sp> for u8 {
        #[inline(always)]
        fn from(val: Sp) -> u8 {
            Sp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ssiop {
        #[doc = "Low level is active for SS signal"]
        ACTIVE_LOW = 0x0,
        #[doc = "High level is active for SS signal"]
        ACTIVE_HIGH = 0x01,
    }
    impl Ssiop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ssiop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ssiop {
        #[inline(always)]
        fn from(val: u8) -> Ssiop {
            Ssiop::from_bits(val)
        }
    }
    impl From<Ssiop> for u8 {
        #[inline(always)]
        fn from(val: Ssiop) -> u8 {
            Ssiop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ssom {
        #[doc = "SS is asserted until data transfer complete"]
        ASSERTED = 0x0,
        #[doc = "Data frames interleaved with SS not asserted during MIDI"]
        NOT_ASSERTED = 0x01,
    }
    impl Ssom {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ssom {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ssom {
        #[inline(always)]
        fn from(val: u8) -> Ssom {
            Ssom::from_bits(val)
        }
    }
    impl From<Ssom> for u8 {
        #[inline(always)]
        fn from(val: Ssom) -> u8 {
            Ssom::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tcrcini {
        #[doc = "All zeros TX CRC initialization pattern"]
        ALL_ZEROS = 0x0,
        #[doc = "All ones TX CRC initialization pattern"]
        ALL_ONES = 0x01,
    }
    impl Tcrcini {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tcrcini {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tcrcini {
        #[inline(always)]
        fn from(val: u8) -> Tcrcini {
            Tcrcini::from_bits(val)
        }
    }
    impl From<Tcrcini> for u8 {
        #[inline(always)]
        fn from(val: Tcrcini) -> u8 {
            Tcrcini::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Udrcfg {
        #[doc = "Slave sends a constant underrun pattern"]
        CONSTANT = 0x0,
        #[doc = "Slave repeats last received data frame from master"]
        REPEAT_RECEIVED = 0x01,
        #[doc = "Slave repeats last transmitted data frame"]
        REPEAT_TRANSMITTED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Udrcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Udrcfg {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Udrcfg {
        #[inline(always)]
        fn from(val: u8) -> Udrcfg {
            Udrcfg::from_bits(val)
        }
    }
    impl From<Udrcfg> for u8 {
        #[inline(always)]
        fn from(val: Udrcfg) -> u8 {
            Udrcfg::to_bits(val)
        }
    }
}
