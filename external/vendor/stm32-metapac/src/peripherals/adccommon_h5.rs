#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC common registers"]
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
    #[doc = "common status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "common regular data register for dual mode"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "hardware configuration register"]
    #[inline(always)]
    pub const fn hwcfgr0(self) -> crate::common::Reg<regs::Hwcfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "version register"]
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<regs::Verr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "identification register"]
    #[inline(always)]
    pub const fn ipdr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "size identification register"]
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Dual ADC mode selection These bits are written by software to select the operating mode. 0 value means Independent Mode. Values 00001 to 01001 means Dual mode, master and slave ADCs are working together. All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn dual(&self) -> super::vals::Dual {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Dual::from_bits(val as u8)
        }
        #[doc = "Dual ADC mode selection These bits are written by software to select the operating mode. 0 value means Independent Mode. Values 00001 to 01001 means Dual mode, master and slave ADCs are working together. All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_dual(&mut self, val: super::vals::Dual) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay between 2 sampling phases These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Delay between 2 sampling phases These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DMA configuration (for dual ADC mode) This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "DMA configuration (for dual ADC mode) This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Direct memory access mode for dual ADC mode This bitfield is set and cleared by software. Refer to the DMA controller section for more details. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
        #[inline(always)]
        pub const fn mdma(&self) -> super::vals::Mdma {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Mdma::from_bits(val as u8)
        }
        #[doc = "Direct memory access mode for dual ADC mode This bitfield is set and cleared by software. Refer to the DMA controller section for more details. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
        #[inline(always)]
        pub fn set_mdma(&mut self, val: super::vals::Mdma) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn ckmode(&self) -> super::vals::Ckmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ckmode::from_bits(val as u8)
        }
        #[doc = "ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: super::vals::Ckmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
        #[inline(always)]
        pub fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "VSENSE enable This bit is set and cleared by software to control VSENSE"]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "VSENSE enable This bit is set and cleared by software to control VSENSE"]
        #[inline(always)]
        pub fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VBAT enable This bit is set and cleared by software to control"]
        #[inline(always)]
        pub const fn vbaten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT enable This bit is set and cleared by software to control"]
        #[inline(always)]
        pub fn set_vbaten(&mut self, val: bool) {
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
                .field("dual", &self.dual())
                .field("delay", &self.delay())
                .field("dmacfg", &self.dmacfg())
                .field("mdma", &self.mdma())
                .field("ckmode", &self.ckmode())
                .field("presc", &self.presc())
                .field("vrefen", &self.vrefen())
                .field("tsen", &self.tsen())
                .field("vbaten", &self.vbaten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccr {{ dual: {:?}, delay: {=u8:?}, dmacfg: {:?}, mdma: {:?}, ckmode: {:?}, presc: {:?}, vrefen: {=bool:?}, tsen: {=bool:?}, vbaten: {=bool:?} }}" , self . dual () , self . delay () , self . dmacfg () , self . mdma () , self . ckmode () , self . presc () , self . vrefen () , self . tsen () , self . vbaten ())
        }
    }
    #[doc = "common regular data register for dual mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdr(pub u32);
    impl Cdr {
        #[doc = "Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN)) In MDMA = 0b11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
        #[inline(always)]
        pub const fn rdata_mst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN)) In MDMA = 0b11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
        #[inline(always)]
        pub fn set_rdata_mst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN))."]
        #[inline(always)]
        pub const fn rdata_slv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN))."]
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
    #[doc = "common status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Master ADC ready This bit is a copy of the ADRDY bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn adrdy_mst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master ADC ready This bit is a copy of the ADRDY bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_adrdy_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of Sampling phase flag of the master ADC This bit is a copy of the EOSMP bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn eosmp_mst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of Sampling phase flag of the master ADC This bit is a copy of the EOSMP bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_eosmp_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of regular conversion of the master ADC This bit is a copy of the EOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn eoc_mst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion of the master ADC This bit is a copy of the EOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_eoc_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular sequence flag of the master ADC This bit is a copy of the EOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn eos_mst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence flag of the master ADC This bit is a copy of the EOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_eos_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overrun flag of the master ADC This bit is a copy of the OVR bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn ovr_mst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag of the master ADC This bit is a copy of the OVR bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_ovr_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of injected conversion flag of the master ADC This bit is a copy of the JEOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn jeoc_mst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag of the master ADC This bit is a copy of the JEOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_jeoc_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "End of injected sequence flag of the master ADC This bit is a copy of the JEOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn jeos_mst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence flag of the master ADC This bit is a copy of the JEOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_jeos_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog 1 flag of the master ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn awd_mst(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 flag of the master ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_awd_mst(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected Context Queue Overflow flag of the master ADC This bit is a copy of the JQOVF bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn jqovf_mst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Injected Context Queue Overflow flag of the master ADC This bit is a copy of the JQOVF bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_jqovf_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Slave ADC ready This bit is a copy of the ADRDY bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn adrdy_slv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Slave ADC ready This bit is a copy of the ADRDY bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_adrdy_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "End of Sampling phase flag of the slave ADC This bit is a copy of the EOSMP2 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn eosmp_slv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "End of Sampling phase flag of the slave ADC This bit is a copy of the EOSMP2 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_eosmp_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "End of regular conversion of the slave ADC This bit is a copy of the EOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn eoc_slv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion of the slave ADC This bit is a copy of the EOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_eoc_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End of regular sequence flag of the slave ADC. This bit is a copy of the EOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn eos_slv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence flag of the slave ADC. This bit is a copy of the EOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_eos_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Overrun flag of the slave ADC This bit is a copy of the OVR bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn ovr_slv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag of the slave ADC This bit is a copy of the OVR bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_ovr_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "End of injected conversion flag of the slave ADC This bit is a copy of the JEOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn jeoc_slv(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag of the slave ADC This bit is a copy of the JEOC bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_jeoc_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "End of injected sequence flag of the slave ADC This bit is a copy of the JEOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn jeos_slv(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence flag of the slave ADC This bit is a copy of the JEOS bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_jeos_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn awd_slv(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 23usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_awd_slv(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 23usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected Context Queue Overflow flag of the slave ADC This bit is a copy of the JQOVF bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn jqovf_slv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Injected Context Queue Overflow flag of the slave ADC This bit is a copy of the JQOVF bit in the corresponding ADC_ISR register."]
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
                .field("adrdy_mst", &self.adrdy_mst())
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
            defmt :: write ! (f , "Csr {{ adrdy_mst: {=bool:?}, eosmp_mst: {=bool:?}, eoc_mst: {=bool:?}, eos_mst: {=bool:?}, ovr_mst: {=bool:?}, jeoc_mst: {=bool:?}, jeos_mst: {=bool:?}, awd_mst[0]: {=bool:?}, awd_mst[1]: {=bool:?}, awd_mst[2]: {=bool:?}, jqovf_mst: {=bool:?}, adrdy_slv: {=bool:?}, eosmp_slv: {=bool:?}, eoc_slv: {=bool:?}, eos_slv: {=bool:?}, ovr_slv: {=bool:?}, jeoc_slv: {=bool:?}, jeos_slv: {=bool:?}, awd_slv[0]: {=bool:?}, awd_slv[1]: {=bool:?}, awd_slv[2]: {=bool:?}, jqovf_slv: {=bool:?} }}" , self . adrdy_mst () , self . eosmp_mst () , self . eoc_mst () , self . eos_mst () , self . ovr_mst () , self . jeoc_mst () , self . jeos_mst () , self . awd_mst (0usize) , self . awd_mst (1usize) , self . awd_mst (2usize) , self . jqovf_mst () , self . adrdy_slv () , self . eosmp_slv () , self . eoc_slv () , self . eos_slv () , self . ovr_slv () , self . jeoc_slv () , self . jeos_slv () , self . awd_slv (0usize) , self . awd_slv (1usize) , self . awd_slv (2usize) , self . jqovf_slv ())
        }
    }
    #[doc = "hardware configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr0(pub u32);
    impl Hwcfgr0 {
        #[doc = "Number of ADCs implemented"]
        #[inline(always)]
        pub const fn adcnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of ADCs implemented"]
        #[inline(always)]
        pub fn set_adcnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Number of pipeline stages"]
        #[inline(always)]
        pub const fn mulpipe(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of pipeline stages"]
        #[inline(always)]
        pub fn set_mulpipe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Number of option bits 0002: 2 option bits implemented in the ADC option register (ADC_OR) at address offset 0xC8."]
        #[inline(always)]
        pub const fn opbits(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of option bits 0002: 2 option bits implemented in the ADC option register (ADC_OR) at address offset 0xC8."]
        #[inline(always)]
        pub fn set_opbits(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Idle value for non-selected channels"]
        #[inline(always)]
        pub const fn idlevalue(&self) -> super::vals::Idlevalue {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Idlevalue::from_bits(val as u8)
        }
        #[doc = "Idle value for non-selected channels"]
        #[inline(always)]
        pub fn set_idlevalue(&mut self, val: super::vals::Idlevalue) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Hwcfgr0 {
        #[inline(always)]
        fn default() -> Hwcfgr0 {
            Hwcfgr0(0)
        }
    }
    impl core::fmt::Debug for Hwcfgr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwcfgr0")
                .field("adcnum", &self.adcnum())
                .field("mulpipe", &self.mulpipe())
                .field("opbits", &self.opbits())
                .field("idlevalue", &self.idlevalue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwcfgr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hwcfgr0 {{ adcnum: {=u8:?}, mulpipe: {=u8:?}, opbits: {=u8:?}, idlevalue: {:?} }}",
                self.adcnum(),
                self.mulpipe(),
                self.opbits(),
                self.idlevalue()
            )
        }
    }
    #[doc = "version register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Verr(pub u32);
    impl Verr {
        #[doc = "Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2."]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2."]
        #[inline(always)]
        pub fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Major revision These bits returns the ADC IP major revision"]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Major revision These bits returns the ADC IP major revision"]
        #[inline(always)]
        pub fn set_majrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Verr {
        #[inline(always)]
        fn default() -> Verr {
            Verr(0)
        }
    }
    impl core::fmt::Debug for Verr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Verr")
                .field("minrev", &self.minrev())
                .field("majrev", &self.majrev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Verr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Verr {{ minrev: {=u8:?}, majrev: {=u8:?} }}",
                self.minrev(),
                self.majrev()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckmode {
        #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
        ASYNCHRONOUS = 0x0,
        #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
        SYNC_DIV1 = 0x01,
        #[doc = "Use AHB clock rcc_hclk3 divided by 2"]
        SYNC_DIV2 = 0x02,
        #[doc = "Use AHB clock rcc_hclk3 divided by 4"]
        SYNC_DIV4 = 0x03,
    }
    impl Ckmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckmode {
        #[inline(always)]
        fn from(val: u8) -> Ckmode {
            Ckmode::from_bits(val)
        }
    }
    impl From<Ckmode> for u8 {
        #[inline(always)]
        fn from(val: Ckmode) -> u8 {
            Ckmode::to_bits(val)
        }
    }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dual {
        #[doc = "Independent mode"]
        INDEPENDENT = 0x0,
        #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
        DUAL_RJ = 0x01,
        #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
        DUAL_RA = 0x02,
        #[doc = "Dual, combined interleaved mode + injected simultaneous mode"]
        DUAL_IJ = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "Dual, injected simultaneous mode only"]
        DUAL_J = 0x05,
        #[doc = "Dual, regular simultaneous mode only"]
        DUAL_R = 0x06,
        #[doc = "Dual, interleaved mode only"]
        DUAL_I = 0x07,
        _RESERVED_8 = 0x08,
        #[doc = "Dual, alternate trigger mode only"]
        DUAL_A = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Dual {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dual {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dual {
        #[inline(always)]
        fn from(val: u8) -> Dual {
            Dual::from_bits(val)
        }
    }
    impl From<Dual> for u8 {
        #[inline(always)]
        fn from(val: Dual) -> u8 {
            Dual::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Idlevalue {
        #[doc = "Dummy channel selection is 0x13"]
        H13 = 0x0,
        #[doc = "Dummy channel selection is 0x1F"]
        H1F = 0x01,
        _RESERVED_2 = 0x02,
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
    impl Idlevalue {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Idlevalue {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Idlevalue {
        #[inline(always)]
        fn from(val: u8) -> Idlevalue {
            Idlevalue::from_bits(val)
        }
    }
    impl From<Idlevalue> for u8 {
        #[inline(always)]
        fn from(val: Idlevalue) -> u8 {
            Idlevalue::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mdma {
        #[doc = "Without data packing, CDR/CDR2 not used"]
        NO_PACK = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "CDR formatted for 32-bit down to 10-bit resolution"]
        FORMAT32TO10 = 0x02,
        #[doc = "CDR formatted for 8-bit resolution"]
        FORMAT8 = 0x03,
    }
    impl Mdma {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mdma {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mdma {
        #[inline(always)]
        fn from(val: u8) -> Mdma {
            Mdma::from_bits(val)
        }
    }
    impl From<Mdma> for u8 {
        #[inline(always)]
        fn from(val: Mdma) -> u8 {
            Mdma::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Presc {
        #[doc = "adc_ker_ck_input not divided"]
        DIV1 = 0x0,
        #[doc = "adc_ker_ck_input divided by 2"]
        DIV2 = 0x01,
        #[doc = "adc_ker_ck_input divided by 4"]
        DIV4 = 0x02,
        #[doc = "adc_ker_ck_input divided by 6"]
        DIV6 = 0x03,
        #[doc = "adc_ker_ck_input divided by 8"]
        DIV8 = 0x04,
        #[doc = "adc_ker_ck_input divided by 10"]
        DIV10 = 0x05,
        #[doc = "adc_ker_ck_input divided by 12"]
        DIV12 = 0x06,
        #[doc = "adc_ker_ck_input divided by 16"]
        DIV16 = 0x07,
        #[doc = "adc_ker_ck_input divided by 32"]
        DIV32 = 0x08,
        #[doc = "adc_ker_ck_input divided by 64"]
        DIV64 = 0x09,
        #[doc = "adc_ker_ck_input divided by 128"]
        DIV128 = 0x0a,
        #[doc = "adc_ker_ck_input divided by 256"]
        DIV256 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Presc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Presc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Presc {
        #[inline(always)]
        fn from(val: u8) -> Presc {
            Presc::from_bits(val)
        }
    }
    impl From<Presc> for u8 {
        #[inline(always)]
        fn from(val: Presc) -> u8 {
            Presc::to_bits(val)
        }
    }
}
