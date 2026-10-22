#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADF."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adf {
    ptr: *mut u8,
}
unsafe impl Send for Adf {}
unsafe impl Sync for Adf {}
impl Adf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADF Global Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADF clock generator control register."]
    #[inline(always)]
    pub const fn ckgcr(self) -> crate::common::Reg<regs::Ckgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADF serial interface control register 0."]
    #[inline(always)]
    pub const fn sitfcr(self) -> crate::common::Reg<regs::Sitfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "ADF bitstream matrix control register 0."]
    #[inline(always)]
    pub const fn bsmxcr(self) -> crate::common::Reg<regs::Bsmxcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "ADF digital filter control register 0."]
    #[inline(always)]
    pub const fn dfltcr(self) -> crate::common::Reg<regs::Dfltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "ADF digital filer configuration register 0."]
    #[inline(always)]
    pub const fn dfltcicr(self) -> crate::common::Reg<regs::Dfltcicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "ADF reshape filter configuration register 0."]
    #[inline(always)]
    pub const fn dfltrsfr(self) -> crate::common::Reg<regs::Dfltrsfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "ADF delay control register 0."]
    #[inline(always)]
    pub const fn dlycr(self) -> crate::common::Reg<regs::Dlycr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ADF DFLT0 interrupt enable register."]
    #[inline(always)]
    pub const fn dfltier(self) -> crate::common::Reg<regs::Dfltier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "ADF DFLT0 interrupt status register 0."]
    #[inline(always)]
    pub const fn dfltisr(self) -> crate::common::Reg<regs::Dfltisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "ADF SAD control register."]
    #[inline(always)]
    pub const fn sadcr(self) -> crate::common::Reg<regs::Sadcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "ADF SAD configuration register."]
    #[inline(always)]
    pub const fn sadcfgr(self) -> crate::common::Reg<regs::Sadcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "ADF SAD sound level register."]
    #[inline(always)]
    pub const fn sadsdlvr(self) -> crate::common::Reg<regs::Sadsdlvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "ADF SAD ambient noise level register."]
    #[inline(always)]
    pub const fn sadanlvr(self) -> crate::common::Reg<regs::Sadanlvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "ADF digital filter data register 0."]
    #[inline(always)]
    pub const fn dfltdr(self) -> crate::common::Reg<regs::Dfltdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADF bitstream matrix control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsmxcr(pub u32);
    impl Bsmxcr {
        #[doc = "Bitstream selection."]
        #[inline(always)]
        pub const fn bssel(&self) -> super::vals::Bssel {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Bssel::from_bits(val as u8)
        }
        #[doc = "Bitstream selection."]
        #[inline(always)]
        pub fn set_bssel(&mut self, val: super::vals::Bssel) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "BSMX active flag. This bit is set and cleared by hardware. It is used by the application to check if the BSMX is effectively enabled (active) or not. BSSEL\\[4:0\\]
can only be updated when BSMXACTIVE is set to 0. This BSMXACTIVE flag cannot go to 0 if DFLT0 is enabled."]
        #[inline(always)]
        pub const fn bsmxactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "BSMX active flag. This bit is set and cleared by hardware. It is used by the application to check if the BSMX is effectively enabled (active) or not. BSSEL\\[4:0\\]
can only be updated when BSMXACTIVE is set to 0. This BSMXACTIVE flag cannot go to 0 if DFLT0 is enabled."]
        #[inline(always)]
        pub fn set_bsmxactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bsmxcr {
        #[inline(always)]
        fn default() -> Bsmxcr {
            Bsmxcr(0)
        }
    }
    impl core::fmt::Debug for Bsmxcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsmxcr")
                .field("bssel", &self.bssel())
                .field("bsmxactive", &self.bsmxactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsmxcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bsmxcr {{ bssel: {:?}, bsmxactive: {=bool:?} }}",
                self.bssel(),
                self.bsmxactive()
            )
        }
    }
    #[doc = "ADF clock generator control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckgcr(pub u32);
    impl Ckgcr {
        #[doc = "Clock generator dividers enable."]
        #[inline(always)]
        pub const fn ckgden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clock generator dividers enable."]
        #[inline(always)]
        pub fn set_ckgden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CCK0 clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin."]
        #[inline(always)]
        pub const fn cck0en(&self) -> super::vals::Ccken {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Ccken::from_bits(val as u8)
        }
        #[doc = "CCK0 clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin."]
        #[inline(always)]
        pub fn set_cck0en(&mut self, val: super::vals::Ccken) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "CCK1 clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin."]
        #[inline(always)]
        pub const fn cck1en(&self) -> super::vals::Ccken {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ccken::from_bits(val as u8)
        }
        #[doc = "CCK1 clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin."]
        #[inline(always)]
        pub fn set_cck1en(&mut self, val: super::vals::Ccken) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock generator mode. This bit is set and reset by software. It is used to define the way the clock generator is enabled. This bit must not be changed if the filter is enabled (DFTEN = 1)."]
        #[inline(always)]
        pub const fn ckgmod(&self) -> super::vals::Ckgmod {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Ckgmod::from_bits(val as u8)
        }
        #[doc = "Clock generator mode. This bit is set and reset by software. It is used to define the way the clock generator is enabled. This bit must not be changed if the filter is enabled (DFTEN = 1)."]
        #[inline(always)]
        pub fn set_ckgmod(&mut self, val: super::vals::Ckgmod) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "CCK0 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK0 pin."]
        #[inline(always)]
        pub const fn cck0dir(&self) -> super::vals::Cckdir {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Cckdir::from_bits(val as u8)
        }
        #[doc = "CCK0 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK0 pin."]
        #[inline(always)]
        pub fn set_cck0dir(&mut self, val: super::vals::Cckdir) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CCK1 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin."]
        #[inline(always)]
        pub const fn cck1dir(&self) -> super::vals::Cckdir {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cckdir::from_bits(val as u8)
        }
        #[doc = "CCK1 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin."]
        #[inline(always)]
        pub fn set_cck1dir(&mut self, val: super::vals::Cckdir) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "CKGEN trigger sensitivity selection. This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger signals. This bit is not significant if the CKGMOD = 0."]
        #[inline(always)]
        pub const fn trgsens(&self) -> super::vals::Trgsens {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Trgsens::from_bits(val as u8)
        }
        #[doc = "CKGEN trigger sensitivity selection. This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger signals. This bit is not significant if the CKGMOD = 0."]
        #[inline(always)]
        pub fn set_trgsens(&mut self, val: super::vals::Trgsens) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Digital filter trigger signal selection. This bit is set and cleared by software. It is used to select the trigger signal for the digital filter. This bit is not significant if the CKGMOD = 0."]
        #[inline(always)]
        pub const fn trgsrc(&self) -> super::vals::Trgsrc {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Trgsrc::from_bits(val as u8)
        }
        #[doc = "Digital filter trigger signal selection. This bit is set and cleared by software. It is used to select the trigger signal for the digital filter. This bit is not significant if the CKGMOD = 0."]
        #[inline(always)]
        pub fn set_trgsrc(&mut self, val: super::vals::Trgsrc) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Divider to control the CCK clock."]
        #[inline(always)]
        pub const fn cckdiv(&self) -> super::vals::Cckdiv {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Cckdiv::from_bits(val as u8)
        }
        #[doc = "Divider to control the CCK clock."]
        #[inline(always)]
        pub fn set_cckdiv(&mut self, val: super::vals::Cckdiv) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Divider to control the serial interface clock."]
        #[inline(always)]
        pub const fn procdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Divider to control the serial interface clock."]
        #[inline(always)]
        pub fn set_procdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "Clock generator active flag."]
        #[inline(always)]
        pub const fn ckgactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Clock generator active flag."]
        #[inline(always)]
        pub fn set_ckgactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ckgcr {
        #[inline(always)]
        fn default() -> Ckgcr {
            Ckgcr(0)
        }
    }
    impl core::fmt::Debug for Ckgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckgcr")
                .field("ckgden", &self.ckgden())
                .field("cck0en", &self.cck0en())
                .field("cck1en", &self.cck1en())
                .field("ckgmod", &self.ckgmod())
                .field("cck0dir", &self.cck0dir())
                .field("cck1dir", &self.cck1dir())
                .field("trgsens", &self.trgsens())
                .field("trgsrc", &self.trgsrc())
                .field("cckdiv", &self.cckdiv())
                .field("procdiv", &self.procdiv())
                .field("ckgactive", &self.ckgactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ckgcr {{ ckgden: {=bool:?}, cck0en: {:?}, cck1en: {:?}, ckgmod: {:?}, cck0dir: {:?}, cck1dir: {:?}, trgsens: {:?}, trgsrc: {:?}, cckdiv: {:?}, procdiv: {=u8:?}, ckgactive: {=bool:?} }}" , self . ckgden () , self . cck0en () , self . cck1en () , self . ckgmod () , self . cck0dir () , self . cck1dir () , self . trgsens () , self . trgsrc () , self . cckdiv () , self . procdiv () , self . ckgactive ())
        }
    }
    #[doc = "ADF digital filer configuration register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltcicr(pub u32);
    impl Dfltcicr {
        #[doc = "Source data for the digital filter."]
        #[inline(always)]
        pub const fn datsrc(&self) -> super::vals::Datsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Datsrc::from_bits(val as u8)
        }
        #[doc = "Source data for the digital filter."]
        #[inline(always)]
        pub fn set_datsrc(&mut self, val: super::vals::Datsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Select the CIC order."]
        #[inline(always)]
        pub const fn cicmod(&self) -> super::vals::Cicmod {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Cicmod::from_bits(val as u8)
        }
        #[doc = "Select the CIC order."]
        #[inline(always)]
        pub fn set_cicmod(&mut self, val: super::vals::Cicmod) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "CIC decimation ratio selection. This bitfield is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1)."]
        #[inline(always)]
        pub const fn mcicd(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[doc = "CIC decimation ratio selection. This bitfield is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1)."]
        #[inline(always)]
        pub fn set_mcicd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[doc = "Scaling factor selection. This bitfield is set and cleared by software. It is used to select the gain to be applied at CIC output. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this bitfield informs the application on the current gain value."]
        #[inline(always)]
        pub const fn scale(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "Scaling factor selection. This bitfield is set and cleared by software. It is used to select the gain to be applied at CIC output. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this bitfield informs the application on the current gain value."]
        #[inline(always)]
        pub fn set_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
    }
    impl Default for Dfltcicr {
        #[inline(always)]
        fn default() -> Dfltcicr {
            Dfltcicr(0)
        }
    }
    impl core::fmt::Debug for Dfltcicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltcicr")
                .field("datsrc", &self.datsrc())
                .field("cicmod", &self.cicmod())
                .field("mcicd", &self.mcicd())
                .field("scale", &self.scale())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltcicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltcicr {{ datsrc: {:?}, cicmod: {:?}, mcicd: {=u16:?}, scale: {=u8:?} }}",
                self.datsrc(),
                self.cicmod(),
                self.mcicd(),
                self.scale()
            )
        }
    }
    #[doc = "ADF digital filter control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltcr(pub u32);
    impl Dfltcr {
        #[doc = "DFLT enable. This bit is set and reset by software. It is used to enable the digital filter."]
        #[inline(always)]
        pub const fn dflten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DFLT enable. This bit is set and reset by software. It is used to enable the digital filter."]
        #[inline(always)]
        pub fn set_dflten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA requests enable. This bit is set and reset by software. It is used to control the generation of DMA request to transfer the processed samples into the memory."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA requests enable. This bit is set and reset by software. It is used to control the generation of DMA request to transfer the processed samples into the memory."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RXFIFO threshold selection."]
        #[inline(always)]
        pub const fn fth(&self) -> super::vals::Rxfifo {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Rxfifo::from_bits(val as u8)
        }
        #[doc = "RXFIFO threshold selection."]
        #[inline(always)]
        pub fn set_fth(&mut self, val: super::vals::Rxfifo) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "DFLT trigger mode."]
        #[inline(always)]
        pub const fn acqmod(&self) -> super::vals::Acqmod {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Acqmod::from_bits(val as u8)
        }
        #[doc = "DFLT trigger mode."]
        #[inline(always)]
        pub fn set_acqmod(&mut self, val: super::vals::Acqmod) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "DFLT trigger signal selection."]
        #[inline(always)]
        pub const fn trgsrc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "DFLT trigger signal selection."]
        #[inline(always)]
        pub fn set_trgsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Number of samples to be discarded."]
        #[inline(always)]
        pub const fn nbdis(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Number of samples to be discarded."]
        #[inline(always)]
        pub fn set_nbdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "DFLT run status flag."]
        #[inline(always)]
        pub const fn dfltrun(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DFLT run status flag."]
        #[inline(always)]
        pub fn set_dfltrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DFLT active flag."]
        #[inline(always)]
        pub const fn dfltactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DFLT active flag."]
        #[inline(always)]
        pub fn set_dfltactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dfltcr {
        #[inline(always)]
        fn default() -> Dfltcr {
            Dfltcr(0)
        }
    }
    impl core::fmt::Debug for Dfltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltcr")
                .field("dflten", &self.dflten())
                .field("dmaen", &self.dmaen())
                .field("fth", &self.fth())
                .field("acqmod", &self.acqmod())
                .field("trgsrc", &self.trgsrc())
                .field("nbdis", &self.nbdis())
                .field("dfltrun", &self.dfltrun())
                .field("dfltactive", &self.dfltactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfltcr {{ dflten: {=bool:?}, dmaen: {=bool:?}, fth: {:?}, acqmod: {:?}, trgsrc: {=u8:?}, nbdis: {=u8:?}, dfltrun: {=bool:?}, dfltactive: {=bool:?} }}" , self . dflten () , self . dmaen () , self . fth () , self . acqmod () , self . trgsrc () , self . nbdis () , self . dfltrun () , self . dfltactive ())
        }
    }
    #[doc = "ADF digital filter data register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltdr(pub u32);
    impl Dfltdr {
        #[doc = "DR. Data processed by DFT"]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "DR. Data processed by DFT"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Dfltdr {
        #[inline(always)]
        fn default() -> Dfltdr {
            Dfltdr(0)
        }
    }
    impl core::fmt::Debug for Dfltdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltdr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dfltdr {{ dr: {=u32:?} }}", self.dr())
        }
    }
    #[doc = "ADF DFLT interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltier(pub u32);
    impl Dfltier {
        #[doc = "RXFIFO threshold interrupt enable."]
        #[inline(always)]
        pub const fn fthie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RXFIFO threshold interrupt enable."]
        #[inline(always)]
        pub fn set_fthie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data overflow interrupt enable."]
        #[inline(always)]
        pub const fn dovrie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data overflow interrupt enable."]
        #[inline(always)]
        pub fn set_dovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Saturation detection interrupt enable."]
        #[inline(always)]
        pub const fn satie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Saturation detection interrupt enable."]
        #[inline(always)]
        pub fn set_satie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock absence detection interrupt enable."]
        #[inline(always)]
        pub const fn ckabie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence detection interrupt enable."]
        #[inline(always)]
        pub fn set_ckabie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Reshape filter overrun interrupt enable."]
        #[inline(always)]
        pub const fn rfovrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Reshape filter overrun interrupt enable."]
        #[inline(always)]
        pub fn set_rfovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Sound activity detection interrupt enable."]
        #[inline(always)]
        pub const fn sddetie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Sound activity detection interrupt enable."]
        #[inline(always)]
        pub fn set_sddetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SAD sound-level value ready enable."]
        #[inline(always)]
        pub const fn sdlvlie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SAD sound-level value ready enable."]
        #[inline(always)]
        pub fn set_sdlvlie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Dfltier {
        #[inline(always)]
        fn default() -> Dfltier {
            Dfltier(0)
        }
    }
    impl core::fmt::Debug for Dfltier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltier")
                .field("fthie", &self.fthie())
                .field("dovrie", &self.dovrie())
                .field("satie", &self.satie())
                .field("ckabie", &self.ckabie())
                .field("rfovrie", &self.rfovrie())
                .field("sddetie", &self.sddetie())
                .field("sdlvlie", &self.sdlvlie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfltier {{ fthie: {=bool:?}, dovrie: {=bool:?}, satie: {=bool:?}, ckabie: {=bool:?}, rfovrie: {=bool:?}, sddetie: {=bool:?}, sdlvlie: {=bool:?} }}" , self . fthie () , self . dovrie () , self . satie () , self . ckabie () , self . rfovrie () , self . sddetie () , self . sdlvlie ())
        }
    }
    #[doc = "ADF DFLT interrupt status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltisr(pub u32);
    impl Dfltisr {
        #[doc = "RXFIFO threshold flag."]
        #[inline(always)]
        pub const fn fthf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RXFIFO threshold flag."]
        #[inline(always)]
        pub fn set_fthf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data overflow flag."]
        #[inline(always)]
        pub const fn dovrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data overflow flag."]
        #[inline(always)]
        pub fn set_dovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RXFIFO not empty flag."]
        #[inline(always)]
        pub const fn rxnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RXFIFO not empty flag."]
        #[inline(always)]
        pub fn set_rxnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Saturation detection flag."]
        #[inline(always)]
        pub const fn satf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Saturation detection flag."]
        #[inline(always)]
        pub fn set_satf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock absence detection flag."]
        #[inline(always)]
        pub const fn ckabf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence detection flag."]
        #[inline(always)]
        pub fn set_ckabf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Reshape filter overrun detection flag."]
        #[inline(always)]
        pub const fn rfovrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Reshape filter overrun detection flag."]
        #[inline(always)]
        pub fn set_rfovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Sound activity detection flag."]
        #[inline(always)]
        pub const fn sddetf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Sound activity detection flag."]
        #[inline(always)]
        pub fn set_sddetf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Sound level value ready flag."]
        #[inline(always)]
        pub const fn sdlvlf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Sound level value ready flag."]
        #[inline(always)]
        pub fn set_sdlvlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Dfltisr {
        #[inline(always)]
        fn default() -> Dfltisr {
            Dfltisr(0)
        }
    }
    impl core::fmt::Debug for Dfltisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltisr")
                .field("fthf", &self.fthf())
                .field("dovrf", &self.dovrf())
                .field("rxnef", &self.rxnef())
                .field("satf", &self.satf())
                .field("ckabf", &self.ckabf())
                .field("rfovrf", &self.rfovrf())
                .field("sddetf", &self.sddetf())
                .field("sdlvlf", &self.sdlvlf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfltisr {{ fthf: {=bool:?}, dovrf: {=bool:?}, rxnef: {=bool:?}, satf: {=bool:?}, ckabf: {=bool:?}, rfovrf: {=bool:?}, sddetf: {=bool:?}, sdlvlf: {=bool:?} }}" , self . fthf () , self . dovrf () , self . rxnef () , self . satf () , self . ckabf () , self . rfovrf () , self . sddetf () , self . sdlvlf ())
        }
    }
    #[doc = "ADF reshape filter configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltrsfr(pub u32);
    impl Dfltrsfr {
        #[doc = "Reshaper filter bypass."]
        #[inline(always)]
        pub const fn rsfltbyp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reshaper filter bypass."]
        #[inline(always)]
        pub fn set_rsfltbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reshaper filter decimation ratio."]
        #[inline(always)]
        pub const fn rsfltd(&self) -> super::vals::Rsfltd {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Rsfltd::from_bits(val as u8)
        }
        #[doc = "Reshaper filter decimation ratio."]
        #[inline(always)]
        pub fn set_rsfltd(&mut self, val: super::vals::Rsfltd) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "High-pass filter bypass. This bit is set and cleared by software. It is used to bypass the high-pass filter."]
        #[inline(always)]
        pub const fn hpfbyp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "High-pass filter bypass. This bit is set and cleared by software. It is used to bypass the high-pass filter."]
        #[inline(always)]
        pub fn set_hpfbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High-pass filter cut-off frequency. This bitfield is set and cleared by software. it is used to select the cut-off frequency of the high-pass filter. F PCM represents the sampling frequency at HPF input."]
        #[inline(always)]
        pub const fn hpfc(&self) -> super::vals::Hpfc {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hpfc::from_bits(val as u8)
        }
        #[doc = "High-pass filter cut-off frequency. This bitfield is set and cleared by software. it is used to select the cut-off frequency of the high-pass filter. F PCM represents the sampling frequency at HPF input."]
        #[inline(always)]
        pub fn set_hpfc(&mut self, val: super::vals::Hpfc) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Dfltrsfr {
        #[inline(always)]
        fn default() -> Dfltrsfr {
            Dfltrsfr(0)
        }
    }
    impl core::fmt::Debug for Dfltrsfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltrsfr")
                .field("rsfltbyp", &self.rsfltbyp())
                .field("rsfltd", &self.rsfltd())
                .field("hpfbyp", &self.hpfbyp())
                .field("hpfc", &self.hpfc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltrsfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltrsfr {{ rsfltbyp: {=bool:?}, rsfltd: {:?}, hpfbyp: {=bool:?}, hpfc: {:?} }}",
                self.rsfltbyp(),
                self.rsfltd(),
                self.hpfbyp(),
                self.hpfc()
            )
        }
    }
    #[doc = "ADF delay control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlycr(pub u32);
    impl Dlycr {
        #[doc = "Delay to apply to a bitstream. This bitfield is set and cleared by software. It defines the number of input samples that are skipped. Skipping is applied immediately after writing to this bitfield, if SKPBF = 0 and DFLTEN = 1. If SKPBF = 1, the value written into the register is ignored by the delay state machine."]
        #[inline(always)]
        pub const fn skpdly(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Delay to apply to a bitstream. This bitfield is set and cleared by software. It defines the number of input samples that are skipped. Skipping is applied immediately after writing to this bitfield, if SKPBF = 0 and DFLTEN = 1. If SKPBF = 1, the value written into the register is ignored by the delay state machine."]
        #[inline(always)]
        pub fn set_skpdly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Skip busy flag."]
        #[inline(always)]
        pub const fn skpbf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Skip busy flag."]
        #[inline(always)]
        pub fn set_skpbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dlycr {
        #[inline(always)]
        fn default() -> Dlycr {
            Dlycr(0)
        }
    }
    impl core::fmt::Debug for Dlycr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlycr")
                .field("skpdly", &self.skpdly())
                .field("skpbf", &self.skpbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlycr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlycr {{ skpdly: {=u8:?}, skpbf: {=bool:?} }}",
                self.skpdly(),
                self.skpbf()
            )
        }
    }
    #[doc = "ADF Global Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "Trigger output control Set by software and reset by."]
        #[inline(always)]
        pub const fn trgo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger output control Set by software and reset by."]
        #[inline(always)]
        pub fn set_trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr").field("trgo", &self.trgo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gcr {{ trgo: {=bool:?} }}", self.trgo())
        }
    }
    #[doc = "ADF SAD ambient noise level register. This bitfield is set by hardware. It contains the latest ambient noise level computed by the SAD. To refresh this bitfield, the SDLVLF flag must be cleared."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sadanlvr(pub u32);
    impl Sadanlvr {
        #[doc = "ANLVL."]
        #[inline(always)]
        pub const fn anlvl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "ANLVL."]
        #[inline(always)]
        pub fn set_anlvl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Sadanlvr {
        #[inline(always)]
        fn default() -> Sadanlvr {
            Sadanlvr(0)
        }
    }
    impl core::fmt::Debug for Sadanlvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sadanlvr").field("anlvl", &self.anlvl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sadanlvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sadanlvr {{ anlvl: {=u16:?} }}", self.anlvl())
        }
    }
    #[doc = "ADF SAD configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sadcfgr(pub u32);
    impl Sadcfgr {
        #[doc = "SNTHR."]
        #[inline(always)]
        pub const fn snthr(&self) -> super::vals::Snthr {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Snthr::from_bits(val as u8)
        }
        #[doc = "SNTHR."]
        #[inline(always)]
        pub fn set_snthr(&mut self, val: super::vals::Snthr) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "ANSLP."]
        #[inline(always)]
        pub const fn anslp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "ANSLP."]
        #[inline(always)]
        pub fn set_anslp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "LFRNB."]
        #[inline(always)]
        pub const fn lfrnb(&self) -> super::vals::Lfrnb {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Lfrnb::from_bits(val as u8)
        }
        #[doc = "LFRNB."]
        #[inline(always)]
        pub fn set_lfrnb(&mut self, val: super::vals::Lfrnb) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Hangover time window."]
        #[inline(always)]
        pub const fn hgovr(&self) -> super::vals::Hgovr {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Hgovr::from_bits(val as u8)
        }
        #[doc = "Hangover time window."]
        #[inline(always)]
        pub fn set_hgovr(&mut self, val: super::vals::Hgovr) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "ANMIN."]
        #[inline(always)]
        pub const fn anmin(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "ANMIN."]
        #[inline(always)]
        pub fn set_anmin(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Sadcfgr {
        #[inline(always)]
        fn default() -> Sadcfgr {
            Sadcfgr(0)
        }
    }
    impl core::fmt::Debug for Sadcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sadcfgr")
                .field("snthr", &self.snthr())
                .field("anslp", &self.anslp())
                .field("lfrnb", &self.lfrnb())
                .field("hgovr", &self.hgovr())
                .field("anmin", &self.anmin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sadcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sadcfgr {{ snthr: {:?}, anslp: {=u8:?}, lfrnb: {:?}, hgovr: {:?}, anmin: {=u16:?} }}",
                self.snthr(),
                self.anslp(),
                self.lfrnb(),
                self.hgovr(),
                self.anmin()
            )
        }
    }
    #[doc = "ADF Sound activity detector (SAD) control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sadcr(pub u32);
    impl Sadcr {
        #[doc = "Sound activity detector enable."]
        #[inline(always)]
        pub const fn saden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sound activity detector enable."]
        #[inline(always)]
        pub fn set_saden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data capture mode."]
        #[inline(always)]
        pub const fn datcap(&self) -> super::vals::Datcap {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Datcap::from_bits(val as u8)
        }
        #[doc = "Data capture mode."]
        #[inline(always)]
        pub fn set_datcap(&mut self, val: super::vals::Datcap) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Sound trigger event configuration."]
        #[inline(always)]
        pub const fn detcfg(&self) -> super::vals::Detcfg {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Detcfg::from_bits(val as u8)
        }
        #[doc = "Sound trigger event configuration."]
        #[inline(always)]
        pub fn set_detcfg(&mut self, val: super::vals::Detcfg) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "SAD state."]
        #[inline(always)]
        pub const fn sadst(&self) -> super::vals::Sadst {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Sadst::from_bits(val as u8)
        }
        #[doc = "SAD state."]
        #[inline(always)]
        pub fn set_sadst(&mut self, val: super::vals::Sadst) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Hysteresis enable."]
        #[inline(always)]
        pub const fn hysten(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Hysteresis enable."]
        #[inline(always)]
        pub fn set_hysten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame size."]
        #[inline(always)]
        pub const fn frsize(&self) -> super::vals::Frsize {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Frsize::from_bits(val as u8)
        }
        #[doc = "Frame size."]
        #[inline(always)]
        pub fn set_frsize(&mut self, val: super::vals::Frsize) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Sound activity detector working mode."]
        #[inline(always)]
        pub const fn sadmod(&self) -> super::vals::Sadmod {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Sadmod::from_bits(val as u8)
        }
        #[doc = "Sound activity detector working mode."]
        #[inline(always)]
        pub fn set_sadmod(&mut self, val: super::vals::Sadmod) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "SAD Active flag."]
        #[inline(always)]
        pub const fn sadactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SAD Active flag."]
        #[inline(always)]
        pub fn set_sadactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sadcr {
        #[inline(always)]
        fn default() -> Sadcr {
            Sadcr(0)
        }
    }
    impl core::fmt::Debug for Sadcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sadcr")
                .field("saden", &self.saden())
                .field("datcap", &self.datcap())
                .field("detcfg", &self.detcfg())
                .field("sadst", &self.sadst())
                .field("hysten", &self.hysten())
                .field("frsize", &self.frsize())
                .field("sadmod", &self.sadmod())
                .field("sadactive", &self.sadactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sadcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sadcr {{ saden: {=bool:?}, datcap: {:?}, detcfg: {:?}, sadst: {:?}, hysten: {=bool:?}, frsize: {:?}, sadmod: {:?}, sadactive: {=bool:?} }}" , self . saden () , self . datcap () , self . detcfg () , self . sadst () , self . hysten () , self . frsize () , self . sadmod () , self . sadactive ())
        }
    }
    #[doc = "ADF SAD sound level register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sadsdlvr(pub u32);
    impl Sadsdlvr {
        #[doc = "Short term sound level. This bitfield is set by hardware. It contains the latest sound level computed by the SAD. To refresh this value, SDLVLF must be cleared."]
        #[inline(always)]
        pub const fn sdlvl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Short term sound level. This bitfield is set by hardware. It contains the latest sound level computed by the SAD. To refresh this value, SDLVLF must be cleared."]
        #[inline(always)]
        pub fn set_sdlvl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Sadsdlvr {
        #[inline(always)]
        fn default() -> Sadsdlvr {
            Sadsdlvr(0)
        }
    }
    impl core::fmt::Debug for Sadsdlvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sadsdlvr").field("sdlvl", &self.sdlvl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sadsdlvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sadsdlvr {{ sdlvl: {=u16:?} }}", self.sdlvl())
        }
    }
    #[doc = "ADF serial interface control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sitfcr(pub u32);
    impl Sitfcr {
        #[inline(always)]
        pub const fn sitfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sitfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn scksrc(&self) -> super::vals::Scksrc {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Scksrc::from_bits(val as u8)
        }
        #[inline(always)]
        pub fn set_scksrc(&mut self, val: super::vals::Scksrc) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn sitfmod(&self) -> super::vals::Sitfmod {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Sitfmod::from_bits(val as u8)
        }
        #[inline(always)]
        pub fn set_sitfmod(&mut self, val: super::vals::Sitfmod) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Manchester symbol threshold/SPI threshold. This bitfield is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this bitfield is used to define the timeout value for the clock absence detection in Normal SPI mode. STH\\[4:0\\]
values lower than four are invalid."]
        #[inline(always)]
        pub const fn sth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Manchester symbol threshold/SPI threshold. This bitfield is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this bitfield is used to define the timeout value for the clock absence detection in Normal SPI mode. STH\\[4:0\\]
values lower than four are invalid."]
        #[inline(always)]
        pub fn set_sth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "SITFACTIVE."]
        #[inline(always)]
        pub const fn sitfactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SITFACTIVE."]
        #[inline(always)]
        pub fn set_sitfactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sitfcr {
        #[inline(always)]
        fn default() -> Sitfcr {
            Sitfcr(0)
        }
    }
    impl core::fmt::Debug for Sitfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sitfcr")
                .field("sitfen", &self.sitfen())
                .field("scksrc", &self.scksrc())
                .field("sitfmod", &self.sitfmod())
                .field("sth", &self.sth())
                .field("sitfactive", &self.sitfactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sitfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sitfcr {{ sitfen: {=bool:?}, scksrc: {:?}, sitfmod: {:?}, sth: {=u8:?}, sitfactive: {=bool:?} }}",
                self.sitfen(),
                self.scksrc(),
                self.sitfmod(),
                self.sth(),
                self.sitfactive()
            )
        }
    }
}
pub mod vals {
    #[doc = "DFLT trigger mode. This bitfield is set and cleared by software. It is used to select the trigger mode of the DFLT0."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Acqmod {
        #[doc = "Asynchronous continuous acquisition mode."]
        ASYNCHRONOUS_CONTINUOUS = 0x0,
        #[doc = "Asynchronous single-shot acquisition mode"]
        ASYNCHRONOUS_SINGLE_SHOT = 0x01,
        #[doc = "Synchronous continuous acquisition mode."]
        SYNCRONOUS_CONTINUOUS = 0x02,
        #[doc = "Synchronous single-shot acquisition mode."]
        SYNCRONOUS_SINGLE_SHOT = 0x03,
        #[doc = "Window continuous acquisition mode."]
        WINDOW_CONTINUOUS = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Acqmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Acqmod {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Acqmod {
        #[inline(always)]
        fn from(val: u8) -> Acqmod {
            Acqmod::from_bits(val)
        }
    }
    impl From<Acqmod> for u8 {
        #[inline(always)]
        fn from(val: Acqmod) -> u8 {
            Acqmod::to_bits(val)
        }
    }
    #[doc = "Bitstream selection. This bitfield is set and cleared by software. It is used to select the bitstream to be used by the DFLT0."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bssel {
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS0_R = 0x0,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS0_F = 0x01,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS1_R = 0x02,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS1_F = 0x03,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS2_R = 0x04,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS2_F = 0x05,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS3_R = 0x06,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS3_F = 0x07,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS4_R = 0x08,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS4_F = 0x09,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS5_R = 0x0a,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS5_F = 0x0b,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS6_R = 0x0c,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS6_F = 0x0d,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS7_R = 0x0e,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS7_F = 0x0f,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS8_R = 0x10,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS8_F = 0x11,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS9_R = 0x12,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS9_F = 0x13,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS10_R = 0x14,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS10_F = 0x15,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS11_R = 0x16,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS11_F = 0x17,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS12_R = 0x18,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS12_F = 0x19,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS13_R = 0x1a,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS13_F = 0x1b,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS14_R = 0x1c,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS14_F = 0x1d,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        BS15_R = 0x1e,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        BS15_F = 0x1f,
    }
    impl Bssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bssel {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bssel {
        #[inline(always)]
        fn from(val: u8) -> Bssel {
            Bssel::from_bits(val)
        }
    }
    impl From<Bssel> for u8 {
        #[inline(always)]
        fn from(val: Bssel) -> u8 {
            Bssel::to_bits(val)
        }
    }
    #[doc = "CCK1 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cckdir {
        #[doc = "CCK is an input."]
        INPUT = 0x0,
        #[doc = "CCK is an output."]
        OUTPUT = 0x01,
    }
    impl Cckdir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cckdir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cckdir {
        #[inline(always)]
        fn from(val: u8) -> Cckdir {
            Cckdir::from_bits(val)
        }
    }
    impl From<Cckdir> for u8 {
        #[inline(always)]
        fn from(val: Cckdir) -> u8 {
            Cckdir::to_bits(val)
        }
    }
    #[doc = "Divider to control the CCK clock. This bit is set and reset by software. It is used to control the frequency of the bitstream clock on the CCK pin."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cckdiv {
        #[doc = "The ADF_CCK clock is adf_proc_ck."]
        DIV1 = 0x0,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 2."]
        DIV2 = 0x01,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 3."]
        DIV3 = 0x02,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 4."]
        DIV4 = 0x03,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 5."]
        DIV5 = 0x04,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 6."]
        DIV6 = 0x05,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 7."]
        DIV7 = 0x06,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 8."]
        DIV8 = 0x07,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 9."]
        DIV9 = 0x08,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 10."]
        DIV10 = 0x09,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 11."]
        DIV11 = 0x0a,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 12."]
        DIV12 = 0x0b,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 13."]
        DIV13 = 0x0c,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 14."]
        DIV14 = 0x0d,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 15."]
        DIV15 = 0x0e,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 16."]
        DIV16 = 0x0f,
    }
    impl Cckdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cckdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cckdiv {
        #[inline(always)]
        fn from(val: u8) -> Cckdiv {
            Cckdiv::from_bits(val)
        }
    }
    impl From<Cckdiv> for u8 {
        #[inline(always)]
        fn from(val: Cckdiv) -> u8 {
            Cckdiv::to_bits(val)
        }
    }
    #[doc = "CCK clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ccken {
        #[doc = "Bitstream clock not generated."]
        NOT_GENERATED = 0x0,
        #[doc = "Bitstream clock generated on the CCK pin."]
        GENERATED = 0x01,
    }
    impl Ccken {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccken {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccken {
        #[inline(always)]
        fn from(val: u8) -> Ccken {
            Ccken::from_bits(val)
        }
    }
    impl From<Ccken> for u8 {
        #[inline(always)]
        fn from(val: Ccken) -> u8 {
            Ccken::to_bits(val)
        }
    }
    #[doc = "Select the CIC order. This bitfield is set and cleared by software. It is used to select the MCIC order."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cicmod {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "MCIC configured in single Sinc4 filter."]
        SINC4 = 0x04,
        #[doc = "MCIC configured in single Sinc5 filter."]
        SINC5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cicmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cicmod {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cicmod {
        #[inline(always)]
        fn from(val: u8) -> Cicmod {
            Cicmod::from_bits(val)
        }
    }
    impl From<Cicmod> for u8 {
        #[inline(always)]
        fn from(val: Cicmod) -> u8 {
            Cicmod::to_bits(val)
        }
    }
    #[doc = "Clock generator mode. This bit is set and reset by software. It is used to define the way the clock generator is enabled. This bit must not be changed if the filter is enabled (DFTEN = 1)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckgmod {
        #[doc = "The kernel clock is provided to the dividers as soon as CKGDEN is set to 1."]
        IMMEDIATE = 0x0,
        #[doc = "The kernel clock is provided to the dividers when CKGDEN is set to 1 and the trigger condition met."]
        TRIGGER = 0x01,
    }
    impl Ckgmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckgmod {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckgmod {
        #[inline(always)]
        fn from(val: u8) -> Ckgmod {
            Ckgmod::from_bits(val)
        }
    }
    impl From<Ckgmod> for u8 {
        #[inline(always)]
        fn from(val: Ckgmod) -> u8 {
            Ckgmod::to_bits(val)
        }
    }
    #[doc = "Data capture mode. This bitfield is set and cleared by software. It is used to define in which conditions, the samples provided by DLFT0 are stored into the memory."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datcap {
        #[doc = "Samples from DFLT0 not transfered into the memory."]
        DISABLED = 0x0,
        #[doc = "Samples from DFLT0 transfered into the memory when SAD is in DETECT state."]
        ON_DETECTED = 0x01,
        #[doc = "Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled."]
        ENABLED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Datcap {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datcap {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datcap {
        #[inline(always)]
        fn from(val: u8) -> Datcap {
            Datcap::from_bits(val)
        }
    }
    impl From<Datcap> for u8 {
        #[inline(always)]
        fn from(val: Datcap) -> u8 {
            Datcap::to_bits(val)
        }
    }
    #[doc = "Source data for the digital filter."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datsrc {
        #[doc = "Stream coming from the BSMX selected"]
        BSMX = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Stream coming from the ADCITF1 selected"]
        ADCITF1 = 0x02,
        #[doc = "Stream coming from the ADCITF2 selected"]
        ADCITF2 = 0x03,
    }
    impl Datsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datsrc {
        #[inline(always)]
        fn from(val: u8) -> Datsrc {
            Datsrc::from_bits(val)
        }
    }
    impl From<Datsrc> for u8 {
        #[inline(always)]
        fn from(val: Datsrc) -> u8 {
            Datsrc::to_bits(val)
        }
    }
    #[doc = "Sound trigger event configuration. This bit is set and cleared by software. It is used to define if the sddet_evt event is generated only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT state."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Detcfg {
        #[doc = "sddet_evt generated when SAD enters the MONITOR state."]
        MONITOR = 0x0,
        #[doc = "sddet_evt generated when SAD enters or exits the DETECT state."]
        DETECT = 0x01,
    }
    impl Detcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Detcfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Detcfg {
        #[inline(always)]
        fn from(val: u8) -> Detcfg {
            Detcfg::from_bits(val)
        }
    }
    impl From<Detcfg> for u8 {
        #[inline(always)]
        fn from(val: Detcfg) -> u8 {
            Detcfg::to_bits(val)
        }
    }
    #[doc = "Frame size. This bitfield is set and cleared by software. it is used to define the size of one frame and also to define how many samples are taken into account to compute the short-term signal level."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Frsize {
        #[doc = "8 sample."]
        SAMPLES8 = 0x0,
        #[doc = "16 samples."]
        SAMPLES16 = 0x01,
        #[doc = "32 samples."]
        SAMPLES32 = 0x02,
        #[doc = "64 samples."]
        SAMPLES64 = 0x03,
        #[doc = "128 samples."]
        SAMPLES128 = 0x04,
        #[doc = "256 samples."]
        SAMPLES256 = 0x05,
        #[doc = "512 samples."]
        SAMPLES512 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Frsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Frsize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Frsize {
        #[inline(always)]
        fn from(val: u8) -> Frsize {
            Frsize::from_bits(val)
        }
    }
    impl From<Frsize> for u8 {
        #[inline(always)]
        fn from(val: Frsize) -> u8 {
            Frsize::to_bits(val)
        }
    }
    #[doc = "Hangover time window. This bitfield is set and cleared by software. It is used to select the hangover time window."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hgovr {
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_4 = 0x0,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_8 = 0x01,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_16 = 0x02,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_32 = 0x03,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_64 = 0x04,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_128 = 0x05,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_256 = 0x06,
        #[doc = "SAD back to MONITOR state if sound is below threshold for 4 frames."]
        FRAMES_512 = 0x07,
    }
    impl Hgovr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hgovr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hgovr {
        #[inline(always)]
        fn from(val: u8) -> Hgovr {
            Hgovr::from_bits(val)
        }
    }
    impl From<Hgovr> for u8 {
        #[inline(always)]
        fn from(val: Hgovr) -> u8 {
            Hgovr::to_bits(val)
        }
    }
    #[doc = "High-pass filter cut-off frequency. This bitfield is set and cleared by software. it is used to select the cut-off frequency of the high-pass filter. F PCM represents the sampling frequency at HPF input."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpfc {
        #[doc = "Cut-off frequency = 0.000625 x FPCM."]
        LOW = 0x0,
        #[doc = "Cut-off frequency = 0.00125 x FPCM."]
        MEDIUM = 0x01,
        #[doc = "Cut-off frequency = 0.00250 x FPCM"]
        HIGH = 0x02,
        #[doc = "Cut-off frequency = 0.00950 x FPCM"]
        MAXIMUM = 0x03,
    }
    impl Hpfc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpfc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpfc {
        #[inline(always)]
        fn from(val: u8) -> Hpfc {
            Hpfc::from_bits(val)
        }
    }
    impl From<Hpfc> for u8 {
        #[inline(always)]
        fn from(val: Hpfc) -> u8 {
            Hpfc::to_bits(val)
        }
    }
    #[doc = "LFRNB. This bitfield is set and cleared by software. It is used to define the number of learning frames to perform the first estimate of the noise level."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lfrnb {
        #[doc = "2 samples."]
        FRAMES_2 = 0x0,
        #[doc = "4 samples."]
        FRAMES_4 = 0x01,
        #[doc = "8 samples."]
        FRAMES_8 = 0x02,
        #[doc = "16 samples."]
        FRAMES_16 = 0x03,
        #[doc = "32 samples."]
        FRAMES_32 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lfrnb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lfrnb {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lfrnb {
        #[inline(always)]
        fn from(val: u8) -> Lfrnb {
            Lfrnb::from_bits(val)
        }
    }
    impl From<Lfrnb> for u8 {
        #[inline(always)]
        fn from(val: Lfrnb) -> u8 {
            Lfrnb::to_bits(val)
        }
    }
    #[doc = "Reshaper filter decimation ratio. This bitfield is set and cleared by software. It is used to select the decimation ratio of the reshaper filter."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rsfltd {
        #[doc = "Decimation ratio is 4 (default value)."]
        DECIMATION4 = 0x0,
        #[doc = "Decimation ratio is 1."]
        DECIMATION1 = 0x01,
    }
    impl Rsfltd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rsfltd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rsfltd {
        #[inline(always)]
        fn from(val: u8) -> Rsfltd {
            Rsfltd::from_bits(val)
        }
    }
    impl From<Rsfltd> for u8 {
        #[inline(always)]
        fn from(val: Rsfltd) -> u8 {
            Rsfltd::to_bits(val)
        }
    }
    #[doc = "RXFIFO threshold selection. This bitfield is set and cleared by software. It is used to select the RXFIFO threshold."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxfifo {
        #[doc = "RXFIFO threshold event generated when the RXFIFO is not empty"]
        NOT_EMPTY = 0x0,
        #[doc = "RXFIFO threshold event generated when the RXFIFO is half-full"]
        HALF_FULL = 0x01,
    }
    impl Rxfifo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxfifo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxfifo {
        #[inline(always)]
        fn from(val: u8) -> Rxfifo {
            Rxfifo::from_bits(val)
        }
    }
    impl From<Rxfifo> for u8 {
        #[inline(always)]
        fn from(val: Rxfifo) -> u8 {
            Rxfifo::to_bits(val)
        }
    }
    #[doc = "SAD working mode. This bitfield is set and cleared by software. It is used to define the way the SAD works"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sadmod {
        #[doc = "Threshold value computed according to the estimated ambient noise. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a voice activity detector."]
        THRESHOLD_ESTIMATED_AMBIENT_NOISE = 0x0,
        #[doc = "Threshold value equal to ANMIN\\[12:0\\], multiplied by the gain selected by SNTHR\\[3:0\\]
The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a sound detector."]
        THRESHOLD_MINIMUM_NOISELEVEL = 0x01,
        #[doc = "Threshold value given by 4 x ANMIN\\[12:0\\]. The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected by SNTHR\\[3:0\\]
is bigger than the defined threshold. In this mode, the SAD is working like an ambient noise estimator. Hysteresis function cannot be used in this mode."]
        THRESHOLD_MINIMUM_NOISELEVELX4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Sadmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sadmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sadmod {
        #[inline(always)]
        fn from(val: u8) -> Sadmod {
            Sadmod::from_bits(val)
        }
    }
    impl From<Sadmod> for u8 {
        #[inline(always)]
        fn from(val: Sadmod) -> u8 {
            Sadmod::to_bits(val)
        }
    }
    #[doc = "SAD state. This bitfield is set and cleared by hardware. It indicates the SAD state and is meaningful only when SADEN = 1."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sadst {
        #[doc = "SAD in LEARN state."]
        LEARN = 0x0,
        #[doc = "SAD in MONITOR state."]
        MONITOR = 0x01,
        #[doc = "SAD in DETECT state."]
        DETECT = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Sadst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sadst {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sadst {
        #[inline(always)]
        fn from(val: u8) -> Sadst {
            Sadst::from_bits(val)
        }
    }
    impl From<Sadst> for u8 {
        #[inline(always)]
        fn from(val: Sadst) -> u8 {
            Sadst::to_bits(val)
        }
    }
    #[doc = "Serial clock source. This bitfield is set and cleared by software. It is used to select the clock source of the serial interface."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scksrc {
        #[doc = "Serial clock source is CCK0."]
        CCK0 = 0x0,
        #[doc = "Serial clock source is CCK1."]
        CCK1 = 0x01,
        #[doc = "Serial clock source is CCI0."]
        CKI0 = 0x02,
        #[doc = "Serial clock source is CCI1."]
        CKI1 = 0x03,
    }
    impl Scksrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scksrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scksrc {
        #[inline(always)]
        fn from(val: u8) -> Scksrc {
            Scksrc::from_bits(val)
        }
    }
    impl From<Scksrc> for u8 {
        #[inline(always)]
        fn from(val: Scksrc) -> u8 {
            Scksrc::to_bits(val)
        }
    }
    #[doc = "Serial interface mode. This bitfield is set and cleared by software. It is used to select the serial interface mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sitfmod {
        #[doc = "LF_MASTER SPI mode."]
        MASTER_SPI = 0x0,
        #[doc = "Normal SPI mode."]
        NORMAL_SPI = 0x01,
        #[doc = "Manchester mode rising edge = logic 0, falling edge = logic 1."]
        MANCHESTER_FALLING = 0x02,
        #[doc = "Manchester mode rising edge = logic 1, falling edge = logic 0."]
        MANCHESTER_RISING = 0x03,
    }
    impl Sitfmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sitfmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sitfmod {
        #[inline(always)]
        fn from(val: u8) -> Sitfmod {
            Sitfmod::from_bits(val)
        }
    }
    impl From<Sitfmod> for u8 {
        #[inline(always)]
        fn from(val: Sitfmod) -> u8 {
            Sitfmod::to_bits(val)
        }
    }
    #[doc = "SNTHR. This bitfield is set and cleared by software. It is used to select the gain to be applied at CIC output. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this bitfield informs the application on the current gain value."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Snthr {
        #[doc = "Threshold is 3.5 dB higher than ANLVL"]
        NOISE_PLUS_3_5 = 0x0,
        #[doc = "Threshold is 6.0 dB higher than ANLVL"]
        NOISE_PLUS_6_0 = 0x01,
        #[doc = "Threshold is 9.5 dB higher than ANLVL"]
        NOISE_PLUS_9_5 = 0x02,
        #[doc = "Threshold is 12 dB higher than ANLVL"]
        NOISE_PLUS_12 = 0x03,
        #[doc = "Threshold is 15.6 dB higher than ANLVL"]
        NOISE_PLUS_15_6 = 0x04,
        #[doc = "Threshold is 18 dB higher than ANLVL"]
        NOISE_PLUS_18 = 0x05,
        #[doc = "Threshold is 21.6 dB higher than ANLVL"]
        NOISE_PLUS_21_6 = 0x06,
        #[doc = "Threshold is 24.1 dB higher than ANLVL"]
        NOISE_PLUS_24_1 = 0x07,
        #[doc = "Threshold is 27.6 dB higher than ANLVL"]
        NOISE_PLUS_27_6 = 0x08,
        #[doc = "Threshold is 30.1 dB higher than ANLVL"]
        NOISE_PLUS_30_1 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Snthr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Snthr {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Snthr {
        #[inline(always)]
        fn from(val: u8) -> Snthr {
            Snthr::from_bits(val)
        }
    }
    impl From<Snthr> for u8 {
        #[inline(always)]
        fn from(val: Snthr) -> u8 {
            Snthr::to_bits(val)
        }
    }
    #[doc = "CKGEN trigger sensitivity selection. This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger signals. This bit is not significant if the CKGMOD = 0."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trgsens {
        #[doc = "A rising edge event triggers the activation of CKGEN dividers."]
        RISING_EDGE = 0x0,
        #[doc = "A falling edge even triggers the activation of CKGEN dividers."]
        FALLING_EDGE = 0x01,
    }
    impl Trgsens {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trgsens {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trgsens {
        #[inline(always)]
        fn from(val: u8) -> Trgsens {
            Trgsens::from_bits(val)
        }
    }
    impl From<Trgsens> for u8 {
        #[inline(always)]
        fn from(val: Trgsens) -> u8 {
            Trgsens::to_bits(val)
        }
    }
    #[doc = "Digital filter trigger signal selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trgsrc {
        #[doc = "TRGO Selected."]
        TRGO = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "adf_trg1 selected."]
        TRG1 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Trgsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trgsrc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trgsrc {
        #[inline(always)]
        fn from(val: u8) -> Trgsrc {
            Trgsrc::from_bits(val)
        }
    }
    impl From<Trgsrc> for u8 {
        #[inline(always)]
        fn from(val: Trgsrc) -> u8 {
            Trgsrc::to_bits(val)
        }
    }
}
