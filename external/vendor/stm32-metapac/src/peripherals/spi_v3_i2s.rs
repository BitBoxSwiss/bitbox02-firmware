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
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "data register - half-word sized"]
    #[inline(always)]
    pub const fn dr16(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "data register - byte sized"]
    #[inline(always)]
    pub const fn dr8(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CRC polynomial register"]
    #[inline(always)]
    pub const fn crcpr(self) -> crate::common::Reg<regs::Crcpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RX CRC register"]
    #[inline(always)]
    pub const fn rxcrcr(self) -> crate::common::Reg<regs::Rxcrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TX CRC register"]
    #[inline(always)]
    pub const fn txcrcr(self) -> crate::common::Reg<regs::Txcrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "I2S configuration register"]
    #[inline(always)]
    pub const fn i2scfgr(self) -> crate::common::Reg<regs::I2scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "I2S prescaler register"]
    #[inline(always)]
    pub const fn i2spr(self) -> crate::common::Reg<regs::I2spr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Clock phase"]
        #[inline(always)]
        pub const fn cpha(&self) -> super::vals::Cpha {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Cpha::from_bits(val as u8)
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub fn set_cpha(&mut self, val: super::vals::Cpha) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub const fn cpol(&self) -> super::vals::Cpol {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cpol::from_bits(val as u8)
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub fn set_cpol(&mut self, val: super::vals::Cpol) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Master selection"]
        #[inline(always)]
        pub const fn mstr(&self) -> super::vals::Mstr {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Mstr::from_bits(val as u8)
        }
        #[doc = "Master selection"]
        #[inline(always)]
        pub fn set_mstr(&mut self, val: super::vals::Mstr) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Baud rate control"]
        #[inline(always)]
        pub const fn br(&self) -> super::vals::Br {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Br::from_bits(val as u8)
        }
        #[doc = "Baud rate control"]
        #[inline(always)]
        pub fn set_br(&mut self, val: super::vals::Br) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "SPI enable"]
        #[inline(always)]
        pub const fn spe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI enable"]
        #[inline(always)]
        pub fn set_spe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Frame format"]
        #[inline(always)]
        pub const fn lsbfirst(&self) -> super::vals::Lsbfirst {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Lsbfirst::from_bits(val as u8)
        }
        #[doc = "Frame format"]
        #[inline(always)]
        pub fn set_lsbfirst(&mut self, val: super::vals::Lsbfirst) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Internal slave select"]
        #[inline(always)]
        pub const fn ssi(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Internal slave select"]
        #[inline(always)]
        pub fn set_ssi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Software slave management"]
        #[inline(always)]
        pub const fn ssm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Software slave management"]
        #[inline(always)]
        pub fn set_ssm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receive only"]
        #[inline(always)]
        pub const fn rxonly(&self) -> super::vals::Rxonly {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Rxonly::from_bits(val as u8)
        }
        #[doc = "Receive only"]
        #[inline(always)]
        pub fn set_rxonly(&mut self, val: super::vals::Rxonly) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC length"]
        #[inline(always)]
        pub const fn crcl(&self) -> super::vals::Crcl {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Crcl::from_bits(val as u8)
        }
        #[doc = "CRC length"]
        #[inline(always)]
        pub fn set_crcl(&mut self, val: super::vals::Crcl) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "CRC transfer next"]
        #[inline(always)]
        pub const fn crcnext(&self) -> super::vals::Crcnext {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Crcnext::from_bits(val as u8)
        }
        #[doc = "CRC transfer next"]
        #[inline(always)]
        pub fn set_crcnext(&mut self, val: super::vals::Crcnext) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Hardware CRC calculation enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware CRC calculation enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Select the direction of transfer in bidirectional mode"]
        #[inline(always)]
        pub const fn bidioe(&self) -> super::vals::Bidioe {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Bidioe::from_bits(val as u8)
        }
        #[doc = "Select the direction of transfer in bidirectional mode"]
        #[inline(always)]
        pub fn set_bidioe(&mut self, val: super::vals::Bidioe) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Bidirectional data mode enable"]
        #[inline(always)]
        pub const fn bidimode(&self) -> super::vals::Bidimode {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Bidimode::from_bits(val as u8)
        }
        #[doc = "Bidirectional data mode enable"]
        #[inline(always)]
        pub fn set_bidimode(&mut self, val: super::vals::Bidimode) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("mstr", &self.mstr())
                .field("br", &self.br())
                .field("spe", &self.spe())
                .field("lsbfirst", &self.lsbfirst())
                .field("ssi", &self.ssi())
                .field("ssm", &self.ssm())
                .field("rxonly", &self.rxonly())
                .field("crcl", &self.crcl())
                .field("crcnext", &self.crcnext())
                .field("crcen", &self.crcen())
                .field("bidioe", &self.bidioe())
                .field("bidimode", &self.bidimode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ cpha: {:?}, cpol: {:?}, mstr: {:?}, br: {:?}, spe: {=bool:?}, lsbfirst: {:?}, ssi: {=bool:?}, ssm: {=bool:?}, rxonly: {:?}, crcl: {:?}, crcnext: {:?}, crcen: {=bool:?}, bidioe: {:?}, bidimode: {:?} }}" , self . cpha () , self . cpol () , self . mstr () , self . br () , self . spe () , self . lsbfirst () , self . ssi () , self . ssm () , self . rxonly () , self . crcl () , self . crcnext () , self . crcen () , self . bidioe () , self . bidimode ())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Rx buffer DMA enable"]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx buffer DMA enable"]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tx buffer DMA enable"]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tx buffer DMA enable"]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SS output enable"]
        #[inline(always)]
        pub const fn ssoe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SS output enable"]
        #[inline(always)]
        pub fn set_ssoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "NSS pulse management"]
        #[inline(always)]
        pub const fn nssp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "NSS pulse management"]
        #[inline(always)]
        pub fn set_nssp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Frame format"]
        #[inline(always)]
        pub const fn frf(&self) -> super::vals::Frf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Frf::from_bits(val as u8)
        }
        #[doc = "Frame format"]
        #[inline(always)]
        pub fn set_frf(&mut self, val: super::vals::Frf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX buffer not empty interrupt enable"]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX buffer not empty interrupt enable"]
        #[inline(always)]
        pub fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Tx buffer empty interrupt enable"]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Tx buffer empty interrupt enable"]
        #[inline(always)]
        pub fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data size"]
        #[inline(always)]
        pub const fn ds(&self) -> super::vals::Ds {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Ds::from_bits(val as u8)
        }
        #[doc = "Data size"]
        #[inline(always)]
        pub fn set_ds(&mut self, val: super::vals::Ds) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "FIFO reception threshold"]
        #[inline(always)]
        pub const fn frxth(&self) -> super::vals::Frxth {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Frxth::from_bits(val as u8)
        }
        #[doc = "FIFO reception threshold"]
        #[inline(always)]
        pub fn set_frxth(&mut self, val: super::vals::Frxth) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Last DMA transfer for reception"]
        #[inline(always)]
        pub const fn ldma_rx(&self) -> super::vals::LdmaRx {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::LdmaRx::from_bits(val as u8)
        }
        #[doc = "Last DMA transfer for reception"]
        #[inline(always)]
        pub fn set_ldma_rx(&mut self, val: super::vals::LdmaRx) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Last DMA transfer for transmission"]
        #[inline(always)]
        pub const fn ldma_tx(&self) -> super::vals::LdmaTx {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::LdmaTx::from_bits(val as u8)
        }
        #[doc = "Last DMA transfer for transmission"]
        #[inline(always)]
        pub fn set_ldma_tx(&mut self, val: super::vals::LdmaTx) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
                .field("rxdmaen", &self.rxdmaen())
                .field("txdmaen", &self.txdmaen())
                .field("ssoe", &self.ssoe())
                .field("nssp", &self.nssp())
                .field("frf", &self.frf())
                .field("errie", &self.errie())
                .field("rxneie", &self.rxneie())
                .field("txeie", &self.txeie())
                .field("ds", &self.ds())
                .field("frxth", &self.frxth())
                .field("ldma_rx", &self.ldma_rx())
                .field("ldma_tx", &self.ldma_tx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ rxdmaen: {=bool:?}, txdmaen: {=bool:?}, ssoe: {=bool:?}, nssp: {=bool:?}, frf: {:?}, errie: {=bool:?}, rxneie: {=bool:?}, txeie: {=bool:?}, ds: {:?}, frxth: {:?}, ldma_rx: {:?}, ldma_tx: {:?} }}" , self . rxdmaen () , self . txdmaen () , self . ssoe () , self . nssp () , self . frf () , self . errie () , self . rxneie () , self . txeie () , self . ds () , self . frxth () , self . ldma_rx () , self . ldma_tx ())
        }
    }
    #[doc = "CRC polynomial register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcpr(pub u32);
    impl Crcpr {
        #[doc = "CRC polynomial register"]
        #[inline(always)]
        pub const fn crcpoly(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "CRC polynomial register"]
        #[inline(always)]
        pub fn set_crcpoly(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Crcpr {
        #[inline(always)]
        fn default() -> Crcpr {
            Crcpr(0)
        }
    }
    impl core::fmt::Debug for Crcpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcpr").field("crcpoly", &self.crcpoly()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crcpr {{ crcpoly: {=u16:?} }}", self.crcpoly())
        }
    }
    #[doc = "I2S configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2scfgr(pub u32);
    impl I2scfgr {
        #[doc = "Channel length (number of bits per audio channel)"]
        #[inline(always)]
        pub const fn chlen(&self) -> super::vals::Chlen {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Chlen::from_bits(val as u8)
        }
        #[doc = "Channel length (number of bits per audio channel)"]
        #[inline(always)]
        pub fn set_chlen(&mut self, val: super::vals::Chlen) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Data length to be transferred"]
        #[inline(always)]
        pub const fn datlen(&self) -> super::vals::Datlen {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Datlen::from_bits(val as u8)
        }
        #[doc = "Data length to be transferred"]
        #[inline(always)]
        pub fn set_datlen(&mut self, val: super::vals::Datlen) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Steady state clock polarity"]
        #[inline(always)]
        pub const fn ckpol(&self) -> super::vals::Ckpol {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ckpol::from_bits(val as u8)
        }
        #[doc = "Steady state clock polarity"]
        #[inline(always)]
        pub fn set_ckpol(&mut self, val: super::vals::Ckpol) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
        #[doc = "I2S configuration mode"]
        #[inline(always)]
        pub const fn i2scfg(&self) -> super::vals::I2scfg {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2scfg::from_bits(val as u8)
        }
        #[doc = "I2S configuration mode"]
        #[inline(always)]
        pub fn set_i2scfg(&mut self, val: super::vals::I2scfg) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2S Enabled"]
        #[inline(always)]
        pub const fn i2se(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "I2S Enabled"]
        #[inline(always)]
        pub fn set_i2se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I2S mode selection"]
        #[inline(always)]
        pub const fn i2smod(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "I2S mode selection"]
        #[inline(always)]
        pub fn set_i2smod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Asynchronous start enable"]
        #[inline(always)]
        pub const fn astrten(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous start enable"]
        #[inline(always)]
        pub fn set_astrten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("chlen", &self.chlen())
                .field("datlen", &self.datlen())
                .field("ckpol", &self.ckpol())
                .field("i2sstd", &self.i2sstd())
                .field("pcmsync", &self.pcmsync())
                .field("i2scfg", &self.i2scfg())
                .field("i2se", &self.i2se())
                .field("i2smod", &self.i2smod())
                .field("astrten", &self.astrten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2scfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "I2scfgr {{ chlen: {:?}, datlen: {:?}, ckpol: {:?}, i2sstd: {:?}, pcmsync: {:?}, i2scfg: {:?}, i2se: {=bool:?}, i2smod: {=bool:?}, astrten: {=bool:?} }}" , self . chlen () , self . datlen () , self . ckpol () , self . i2sstd () , self . pcmsync () , self . i2scfg () , self . i2se () , self . i2smod () , self . astrten ())
        }
    }
    #[doc = "I2S prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2spr(pub u32);
    impl I2spr {
        #[doc = "I2S Linear prescaler"]
        #[inline(always)]
        pub const fn i2sdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "I2S Linear prescaler"]
        #[inline(always)]
        pub fn set_i2sdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Odd factor for the prescaler"]
        #[inline(always)]
        pub const fn odd(&self) -> super::vals::Odd {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Odd::from_bits(val as u8)
        }
        #[doc = "Odd factor for the prescaler"]
        #[inline(always)]
        pub fn set_odd(&mut self, val: super::vals::Odd) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Master clock output enable"]
        #[inline(always)]
        pub const fn mckoe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock output enable"]
        #[inline(always)]
        pub fn set_mckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for I2spr {
        #[inline(always)]
        fn default() -> I2spr {
            I2spr(0)
        }
    }
    impl core::fmt::Debug for I2spr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2spr")
                .field("i2sdiv", &self.i2sdiv())
                .field("odd", &self.odd())
                .field("mckoe", &self.mckoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2spr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2spr {{ i2sdiv: {=u8:?}, odd: {:?}, mckoe: {=bool:?} }}",
                self.i2sdiv(),
                self.odd(),
                self.mckoe()
            )
        }
    }
    #[doc = "RX CRC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcrcr(pub u32);
    impl Rxcrcr {
        #[doc = "Rx CRC register"]
        #[inline(always)]
        pub const fn rx_crc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Rx CRC register"]
        #[inline(always)]
        pub fn set_rx_crc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rxcrcr {
        #[inline(always)]
        fn default() -> Rxcrcr {
            Rxcrcr(0)
        }
    }
    impl core::fmt::Debug for Rxcrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxcrcr").field("rx_crc", &self.rx_crc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxcrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxcrcr {{ rx_crc: {=u16:?} }}", self.rx_crc())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Receive buffer not empty"]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer not empty"]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit buffer empty"]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer empty"]
        #[inline(always)]
        pub fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel side"]
        #[inline(always)]
        pub const fn chside(&self) -> super::vals::Chside {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Chside::from_bits(val as u8)
        }
        #[doc = "Channel side"]
        #[inline(always)]
        pub fn set_chside(&mut self, val: super::vals::Chside) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Underrun flag"]
        #[inline(always)]
        pub const fn udr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Underrun flag"]
        #[inline(always)]
        pub fn set_udr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC error flag"]
        #[inline(always)]
        pub const fn crcerr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error flag"]
        #[inline(always)]
        pub fn set_crcerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Mode fault"]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mode fault"]
        #[inline(always)]
        pub fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Overrun flag"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "frame format error"]
        #[inline(always)]
        pub const fn fre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "frame format error"]
        #[inline(always)]
        pub fn set_fre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FIFO reception level"]
        #[inline(always)]
        pub const fn frlvl(&self) -> super::vals::Frlvl {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Frlvl::from_bits(val as u8)
        }
        #[doc = "FIFO reception level"]
        #[inline(always)]
        pub fn set_frlvl(&mut self, val: super::vals::Frlvl) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "FIFO Transmission Level"]
        #[inline(always)]
        pub const fn ftlvl(&self) -> super::vals::Ftlvl {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Ftlvl::from_bits(val as u8)
        }
        #[doc = "FIFO Transmission Level"]
        #[inline(always)]
        pub fn set_ftlvl(&mut self, val: super::vals::Ftlvl) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
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
                .field("rxne", &self.rxne())
                .field("txe", &self.txe())
                .field("chside", &self.chside())
                .field("udr", &self.udr())
                .field("crcerr", &self.crcerr())
                .field("modf", &self.modf())
                .field("ovr", &self.ovr())
                .field("bsy", &self.bsy())
                .field("fre", &self.fre())
                .field("frlvl", &self.frlvl())
                .field("ftlvl", &self.ftlvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ rxne: {=bool:?}, txe: {=bool:?}, chside: {:?}, udr: {=bool:?}, crcerr: {=bool:?}, modf: {=bool:?}, ovr: {=bool:?}, bsy: {=bool:?}, fre: {=bool:?}, frlvl: {:?}, ftlvl: {:?} }}" , self . rxne () , self . txe () , self . chside () , self . udr () , self . crcerr () , self . modf () , self . ovr () , self . bsy () , self . fre () , self . frlvl () , self . ftlvl ())
        }
    }
    #[doc = "TX CRC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txcrcr(pub u32);
    impl Txcrcr {
        #[doc = "Tx CRC register"]
        #[inline(always)]
        pub const fn tx_crc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Tx CRC register"]
        #[inline(always)]
        pub fn set_tx_crc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Txcrcr {
        #[inline(always)]
        fn default() -> Txcrcr {
            Txcrcr(0)
        }
    }
    impl core::fmt::Debug for Txcrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txcrcr").field("tx_crc", &self.tx_crc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txcrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txcrcr {{ tx_crc: {=u16:?} }}", self.tx_crc())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bidimode {
        #[doc = "2-line unidirectional data mode selected"]
        UNIDIRECTIONAL = 0x0,
        #[doc = "1-line bidirectional data mode selected"]
        BIDIRECTIONAL = 0x01,
    }
    impl Bidimode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bidimode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bidimode {
        #[inline(always)]
        fn from(val: u8) -> Bidimode {
            Bidimode::from_bits(val)
        }
    }
    impl From<Bidimode> for u8 {
        #[inline(always)]
        fn from(val: Bidimode) -> u8 {
            Bidimode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bidioe {
        #[doc = "Output disabled (receive-only mode)"]
        RECEIVE = 0x0,
        #[doc = "Output enabled (transmit-only mode)"]
        TRANSMIT = 0x01,
    }
    impl Bidioe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bidioe {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bidioe {
        #[inline(always)]
        fn from(val: u8) -> Bidioe {
            Bidioe::from_bits(val)
        }
    }
    impl From<Bidioe> for u8 {
        #[inline(always)]
        fn from(val: Bidioe) -> u8 {
            Bidioe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Br {
        #[doc = "f_PCLK / 2"]
        DIV2 = 0x0,
        #[doc = "f_PCLK / 4"]
        DIV4 = 0x01,
        #[doc = "f_PCLK / 8"]
        DIV8 = 0x02,
        #[doc = "f_PCLK / 16"]
        DIV16 = 0x03,
        #[doc = "f_PCLK / 32"]
        DIV32 = 0x04,
        #[doc = "f_PCLK / 64"]
        DIV64 = 0x05,
        #[doc = "f_PCLK / 128"]
        DIV128 = 0x06,
        #[doc = "f_PCLK / 256"]
        DIV256 = 0x07,
    }
    impl Br {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Br {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Br {
        #[inline(always)]
        fn from(val: u8) -> Br {
            Br::from_bits(val)
        }
    }
    impl From<Br> for u8 {
        #[inline(always)]
        fn from(val: Br) -> u8 {
            Br::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chlen {
        #[doc = "16-bit wide"]
        BITS16 = 0x0,
        #[doc = "32-bit wide"]
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
    pub enum Chside {
        #[doc = "Channel left has to be transmitted or has been received"]
        LEFT = 0x0,
        #[doc = "Channel right has to be transmitted or has been received"]
        RIGHT = 0x01,
    }
    impl Chside {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chside {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chside {
        #[inline(always)]
        fn from(val: u8) -> Chside {
            Chside::from_bits(val)
        }
    }
    impl From<Chside> for u8 {
        #[inline(always)]
        fn from(val: Chside) -> u8 {
            Chside::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckpol {
        #[doc = "I2S clock inactive state is low level"]
        IDLE_LOW = 0x0,
        #[doc = "I2S clock inactive state is high level"]
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
        #[doc = "CK to 0 when idle"]
        IDLE_LOW = 0x0,
        #[doc = "CK to 1 when idle"]
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
    pub enum Crcl {
        #[doc = "8-bit CRC length"]
        BITS8 = 0x0,
        #[doc = "16-bit CRC length"]
        BITS16 = 0x01,
    }
    impl Crcl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crcl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crcl {
        #[inline(always)]
        fn from(val: u8) -> Crcl {
            Crcl::from_bits(val)
        }
    }
    impl From<Crcl> for u8 {
        #[inline(always)]
        fn from(val: Crcl) -> u8 {
            Crcl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Crcnext {
        #[doc = "Next transmit value is from Tx buffer"]
        TX_BUFFER = 0x0,
        #[doc = "Next transmit value is from Tx CRC register"]
        CRC = 0x01,
    }
    impl Crcnext {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crcnext {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crcnext {
        #[inline(always)]
        fn from(val: u8) -> Crcnext {
            Crcnext::from_bits(val)
        }
    }
    impl From<Crcnext> for u8 {
        #[inline(always)]
        fn from(val: Crcnext) -> u8 {
            Crcnext::to_bits(val)
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
    pub enum Ds {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "4-bit"]
        BITS4 = 0x03,
        #[doc = "5-bit"]
        BITS5 = 0x04,
        #[doc = "6-bit"]
        BITS6 = 0x05,
        #[doc = "7-bit"]
        BITS7 = 0x06,
        #[doc = "8-bit"]
        BITS8 = 0x07,
        #[doc = "9-bit"]
        BITS9 = 0x08,
        #[doc = "10-bit"]
        BITS10 = 0x09,
        #[doc = "11-bit"]
        BITS11 = 0x0a,
        #[doc = "12-bit"]
        BITS12 = 0x0b,
        #[doc = "13-bit"]
        BITS13 = 0x0c,
        #[doc = "14-bit"]
        BITS14 = 0x0d,
        #[doc = "15-bit"]
        BITS15 = 0x0e,
        #[doc = "16-bit"]
        BITS16 = 0x0f,
    }
    impl Ds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ds {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ds {
        #[inline(always)]
        fn from(val: u8) -> Ds {
            Ds::from_bits(val)
        }
    }
    impl From<Ds> for u8 {
        #[inline(always)]
        fn from(val: Ds) -> u8 {
            Ds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Frf {
        #[doc = "SPI Motorola mode"]
        MOTOROLA = 0x0,
        #[doc = "SPI TI mode"]
        TI = 0x01,
    }
    impl Frf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Frf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Frf {
        #[inline(always)]
        fn from(val: u8) -> Frf {
            Frf::from_bits(val)
        }
    }
    impl From<Frf> for u8 {
        #[inline(always)]
        fn from(val: Frf) -> u8 {
            Frf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Frlvl {
        #[doc = "Rx FIFO Empty"]
        EMPTY = 0x0,
        #[doc = "Rx 1/4 FIFO"]
        QUARTER = 0x01,
        #[doc = "Rx 1/2 FIFO"]
        HALF = 0x02,
        #[doc = "Rx FIFO full"]
        FULL = 0x03,
    }
    impl Frlvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Frlvl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Frlvl {
        #[inline(always)]
        fn from(val: u8) -> Frlvl {
            Frlvl::from_bits(val)
        }
    }
    impl From<Frlvl> for u8 {
        #[inline(always)]
        fn from(val: Frlvl) -> u8 {
            Frlvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Frxth {
        #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
        HALF = 0x0,
        #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
        QUARTER = 0x01,
    }
    impl Frxth {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Frxth {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Frxth {
        #[inline(always)]
        fn from(val: u8) -> Frxth {
            Frxth::from_bits(val)
        }
    }
    impl From<Frxth> for u8 {
        #[inline(always)]
        fn from(val: Frxth) -> u8 {
            Frxth::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ftlvl {
        #[doc = "Tx FIFO Empty"]
        EMPTY = 0x0,
        #[doc = "Tx 1/4 FIFO"]
        QUARTER = 0x01,
        #[doc = "Tx 1/2 FIFO"]
        HALF = 0x02,
        #[doc = "Tx FIFO full"]
        FULL = 0x03,
    }
    impl Ftlvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ftlvl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ftlvl {
        #[inline(always)]
        fn from(val: u8) -> Ftlvl {
            Ftlvl::from_bits(val)
        }
    }
    impl From<Ftlvl> for u8 {
        #[inline(always)]
        fn from(val: Ftlvl) -> u8 {
            Ftlvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2scfg {
        #[doc = "Slave - transmit"]
        SLAVE_TX = 0x0,
        #[doc = "Slave - receive"]
        SLAVE_RX = 0x01,
        #[doc = "Master - transmit"]
        MASTER_TX = 0x02,
        #[doc = "Master - receive"]
        MASTER_RX = 0x03,
    }
    impl I2scfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2scfg {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "MSB justified standard"]
        MSB = 0x01,
        #[doc = "LSB justified standard"]
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
    pub enum LdmaRx {
        #[doc = "Number of data to transfer for receive is even"]
        EVEN = 0x0,
        #[doc = "Number of data to transfer for receive is odd"]
        ODD = 0x01,
    }
    impl LdmaRx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LdmaRx {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LdmaRx {
        #[inline(always)]
        fn from(val: u8) -> LdmaRx {
            LdmaRx::from_bits(val)
        }
    }
    impl From<LdmaRx> for u8 {
        #[inline(always)]
        fn from(val: LdmaRx) -> u8 {
            LdmaRx::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LdmaTx {
        #[doc = "Number of data to transfer for transmit is even"]
        EVEN = 0x0,
        #[doc = "Number of data to transfer for transmit is odd"]
        ODD = 0x01,
    }
    impl LdmaTx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LdmaTx {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LdmaTx {
        #[inline(always)]
        fn from(val: u8) -> LdmaTx {
            LdmaTx::from_bits(val)
        }
    }
    impl From<LdmaTx> for u8 {
        #[inline(always)]
        fn from(val: LdmaTx) -> u8 {
            LdmaTx::to_bits(val)
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
    pub enum Mstr {
        #[doc = "Slave configuration"]
        SLAVE = 0x0,
        #[doc = "Master configuration"]
        MASTER = 0x01,
    }
    impl Mstr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mstr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mstr {
        #[inline(always)]
        fn from(val: u8) -> Mstr {
            Mstr::from_bits(val)
        }
    }
    impl From<Mstr> for u8 {
        #[inline(always)]
        fn from(val: Mstr) -> u8 {
            Mstr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Odd {
        #[doc = "Real divider value is I2SDIV * 2"]
        EVEN = 0x0,
        #[doc = "Real divider value is (I2SDIV * 2) + 1"]
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
        #[doc = "Short frame synchronisation"]
        SHORT = 0x0,
        #[doc = "Long frame synchronisation"]
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
    pub enum Rxonly {
        #[doc = "Full duplex (Transmit and receive)"]
        FULL_DUPLEX = 0x0,
        #[doc = "Output disabled (Receive-only mode)"]
        OUTPUT_DISABLED = 0x01,
    }
    impl Rxonly {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxonly {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxonly {
        #[inline(always)]
        fn from(val: u8) -> Rxonly {
            Rxonly::from_bits(val)
        }
    }
    impl From<Rxonly> for u8 {
        #[inline(always)]
        fn from(val: Rxonly) -> u8 {
            Rxonly::to_bits(val)
        }
    }
}
