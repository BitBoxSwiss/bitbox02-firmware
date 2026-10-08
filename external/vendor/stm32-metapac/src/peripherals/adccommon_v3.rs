#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-Digital Converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCommon {
    ptr: *mut u8,
}
unsafe impl Send for AdcCommon {}
unsafe impl Sync for AdcCommon {}
impl AdcCommon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC Common status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC common regular data register for dual and triple modes"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Multi ADC mode selection"]
        #[inline(always)]
        pub const fn mult(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Multi ADC mode selection"]
        #[inline(always)]
        pub fn set_mult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay between 2 sampling phases"]
        #[inline(always)]
        pub const fn delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Delay between 2 sampling phases"]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Direct memory access mode for multi ADC mode"]
        #[inline(always)]
        pub const fn mdma(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Direct memory access mode for multi ADC mode"]
        #[inline(always)]
        pub fn set_mdma(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub const fn ckmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CH18 selection (Vbat)"]
        #[inline(always)]
        pub const fn ch18sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "CH18 selection (Vbat)"]
        #[inline(always)]
        pub fn set_ch18sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CH17 selection (temperature)"]
        #[inline(always)]
        pub const fn ch17sel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CH17 selection (temperature)"]
        #[inline(always)]
        pub fn set_ch17sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("mult", &self.mult())
                .field("delay", &self.delay())
                .field("dmacfg", &self.dmacfg())
                .field("mdma", &self.mdma())
                .field("ckmode", &self.ckmode())
                .field("vrefen", &self.vrefen())
                .field("ch18sel", &self.ch18sel())
                .field("ch17sel", &self.ch17sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccr {{ mult: {=u8:?}, delay: {=u8:?}, dmacfg: {:?}, mdma: {=u8:?}, ckmode: {=u8:?}, vrefen: {=bool:?}, ch18sel: {=bool:?}, ch17sel: {=bool:?} }}" , self . mult () , self . delay () , self . dmacfg () , self . mdma () , self . ckmode () , self . vrefen () , self . ch18sel () , self . ch17sel ())
        }
    }
    #[doc = "ADC common regular data register for dual and triple modes"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdr(pub u32);
    impl Cdr {
        #[doc = "Regular data of the master ADC"]
        #[inline(always)]
        pub const fn rdata_mst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the master ADC"]
        #[inline(always)]
        pub fn set_rdata_mst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Regular data of the slave ADC"]
        #[inline(always)]
        pub const fn rdata_slv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the slave ADC"]
        #[inline(always)]
        pub fn set_rdata_slv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cdr {
        #[inline(always)]
        fn default() -> Cdr {
            Cdr(0)
        }
    }
    impl core::fmt::Debug for Cdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdr")
                .field("rdata_mst", &self.rdata_mst())
                .field("rdata_slv", &self.rdata_slv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdr {{ rdata_mst: {=u16:?}, rdata_slv: {=u16:?} }}",
                self.rdata_mst(),
                self.rdata_slv()
            )
        }
    }
    #[doc = "ADC Common status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "ADDRDY_MST"]
        #[inline(always)]
        pub const fn addrdy_mst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADDRDY_MST"]
        #[inline(always)]
        pub fn set_addrdy_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMP_MST"]
        #[inline(always)]
        pub const fn eosmp_mst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMP_MST"]
        #[inline(always)]
        pub fn set_eosmp_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOC_MST"]
        #[inline(always)]
        pub const fn eoc_mst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOC_MST"]
        #[inline(always)]
        pub fn set_eoc_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOS_MST"]
        #[inline(always)]
        pub const fn eos_mst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOS_MST"]
        #[inline(always)]
        pub fn set_eos_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVR_MST"]
        #[inline(always)]
        pub const fn ovr_mst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVR_MST"]
        #[inline(always)]
        pub fn set_ovr_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JEOC_MST"]
        #[inline(always)]
        pub const fn jeoc_mst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JEOC_MST"]
        #[inline(always)]
        pub fn set_jeoc_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "JEOS_MST"]
        #[inline(always)]
        pub const fn jeos_mst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "JEOS_MST"]
        #[inline(always)]
        pub fn set_jeos_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog flag of the master ADC"]
        #[inline(always)]
        pub const fn awd_mst(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flag of the master ADC"]
        #[inline(always)]
        pub fn set_awd_mst(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "JQOVF_MST"]
        #[inline(always)]
        pub const fn jqovf_mst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "JQOVF_MST"]
        #[inline(always)]
        pub fn set_jqovf_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ADRDY_SLV"]
        #[inline(always)]
        pub const fn adrdy_slv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDY_SLV"]
        #[inline(always)]
        pub fn set_adrdy_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "EOSMP_SLV"]
        #[inline(always)]
        pub const fn eosmp_slv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMP_SLV"]
        #[inline(always)]
        pub fn set_eosmp_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "End of regular conversion of the slave ADC"]
        #[inline(always)]
        pub const fn eoc_slv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion of the slave ADC"]
        #[inline(always)]
        pub fn set_eoc_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End of regular sequence flag of the slave ADC"]
        #[inline(always)]
        pub const fn eos_slv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence flag of the slave ADC"]
        #[inline(always)]
        pub fn set_eos_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Overrun flag of the slave ADC"]
        #[inline(always)]
        pub const fn ovr_slv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag of the slave ADC"]
        #[inline(always)]
        pub fn set_ovr_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "End of injected conversion flag of the slave ADC"]
        #[inline(always)]
        pub const fn jeoc_slv(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag of the slave ADC"]
        #[inline(always)]
        pub fn set_jeoc_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "End of injected sequence flag of the slave ADC"]
        #[inline(always)]
        pub const fn jeos_slv(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence flag of the slave ADC"]
        #[inline(always)]
        pub fn set_jeos_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC"]
        #[inline(always)]
        pub const fn awd_slv(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 23usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC"]
        #[inline(always)]
        pub fn set_awd_slv(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 23usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected Context Queue Overflow flag of the slave ADC"]
        #[inline(always)]
        pub const fn jqovf_slv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Injected Context Queue Overflow flag of the slave ADC"]
        #[inline(always)]
        pub fn set_jqovf_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("addrdy_mst", &self.addrdy_mst())
                .field("eosmp_mst", &self.eosmp_mst())
                .field("eoc_mst", &self.eoc_mst())
                .field("eos_mst", &self.eos_mst())
                .field("ovr_mst", &self.ovr_mst())
                .field("jeoc_mst", &self.jeoc_mst())
                .field("jeos_mst", &self.jeos_mst())
                .field("awd_mst[0]", &self.awd_mst(0usize))
                .field("awd_mst[1]", &self.awd_mst(1usize))
                .field("awd_mst[2]", &self.awd_mst(2usize))
                .field("jqovf_mst", &self.jqovf_mst())
                .field("adrdy_slv", &self.adrdy_slv())
                .field("eosmp_slv", &self.eosmp_slv())
                .field("eoc_slv", &self.eoc_slv())
                .field("eos_slv", &self.eos_slv())
                .field("ovr_slv", &self.ovr_slv())
                .field("jeoc_slv", &self.jeoc_slv())
                .field("jeos_slv", &self.jeos_slv())
                .field("awd_slv[0]", &self.awd_slv(0usize))
                .field("awd_slv[1]", &self.awd_slv(1usize))
                .field("awd_slv[2]", &self.awd_slv(2usize))
                .field("jqovf_slv", &self.jqovf_slv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ addrdy_mst: {=bool:?}, eosmp_mst: {=bool:?}, eoc_mst: {=bool:?}, eos_mst: {=bool:?}, ovr_mst: {=bool:?}, jeoc_mst: {=bool:?}, jeos_mst: {=bool:?}, awd_mst[0]: {=bool:?}, awd_mst[1]: {=bool:?}, awd_mst[2]: {=bool:?}, jqovf_mst: {=bool:?}, adrdy_slv: {=bool:?}, eosmp_slv: {=bool:?}, eoc_slv: {=bool:?}, eos_slv: {=bool:?}, ovr_slv: {=bool:?}, jeoc_slv: {=bool:?}, jeos_slv: {=bool:?}, awd_slv[0]: {=bool:?}, awd_slv[1]: {=bool:?}, awd_slv[2]: {=bool:?}, jqovf_slv: {=bool:?} }}" , self . addrdy_mst () , self . eosmp_mst () , self . eoc_mst () , self . eos_mst () , self . ovr_mst () , self . jeoc_mst () , self . jeos_mst () , self . awd_mst (0usize) , self . awd_mst (1usize) , self . awd_mst (2usize) , self . jqovf_mst () , self . adrdy_slv () , self . eosmp_slv () , self . eoc_slv () , self . eos_slv () , self . ovr_slv () , self . jeoc_slv () , self . jeos_slv () , self . awd_slv (0usize) , self . awd_slv (1usize) , self . awd_slv (2usize) , self . jqovf_slv ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmacfg {
        #[doc = "DMA One Shot mode selected"]
        ONE_SHOT = 0x0,
        #[doc = "DMA Circular mode selected"]
        CIRCULAR = 0x01,
    }
    impl Dmacfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmacfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmacfg {
        #[inline(always)]
        fn from(val: u8) -> Dmacfg {
            Dmacfg::from_bits(val)
        }
    }
    impl From<Dmacfg> for u8 {
        #[inline(always)]
        fn from(val: Dmacfg) -> u8 {
            Dmacfg::to_bits(val)
        }
    }
}
