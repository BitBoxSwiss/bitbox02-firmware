#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-Digital Converter."]
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
    #[doc = "ADC common status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC_CCR system control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC common regular data register for dual mode."]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC common regular data register for 32-bit dual mode."]
    #[inline(always)]
    pub const fn cdr2(self) -> crate::common::Reg<regs::Cdr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC_CCR system control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn dual(&self) -> super::vals::Dual {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Dual::from_bits(val as u8)
        }
        #[doc = "Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_dual(&mut self, val: super::vals::Dual) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register CDR. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
        #[inline(always)]
        pub const fn damdf(&self) -> super::vals::Damdf {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Damdf::from_bits(val as u8)
        }
        #[doc = "Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register CDR. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
        #[inline(always)]
        pub fn set_damdf(&mut self, val: super::vals::Damdf) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn vsenseen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_vsenseen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn vbaten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
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
                .field("damdf", &self.damdf())
                .field("presc", &self.presc())
                .field("vrefen", &self.vrefen())
                .field("vsenseen", &self.vsenseen())
                .field("vbaten", &self.vbaten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccr {{ dual: {:?}, delay: {=u8:?}, damdf: {:?}, presc: {:?}, vrefen: {=bool:?}, vsenseen: {=bool:?}, vbaten: {=bool:?} }}" , self . dual () , self . delay () , self . damdf () , self . presc () , self . vrefen () , self . vsenseen () , self . vbaten ())
        }
    }
    #[doc = "ADC common regular data register for dual mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdr(pub u32);
    impl Cdr {
        #[doc = "Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF\\[1:0\\]
= 11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
        #[inline(always)]
        pub const fn rdata_mst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF\\[1:0\\]
= 11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
        #[inline(always)]
        pub fn set_rdata_mst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT))."]
        #[inline(always)]
        pub const fn rdata_slv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT))."]
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
    #[doc = "ADC common regular data register for 32-bit dual mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdr2(pub u32);
    impl Cdr2 {
        #[doc = "Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)."]
        #[inline(always)]
        pub const fn rdata_alt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)."]
        #[inline(always)]
        pub fn set_rdata_alt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cdr2 {
        #[inline(always)]
        fn default() -> Cdr2 {
            Cdr2(0)
        }
    }
    impl core::fmt::Debug for Cdr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdr2").field("rdata_alt", &self.rdata_alt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdr2 {{ rdata_alt: {=u32:?} }}", self.rdata_alt())
        }
    }
    #[doc = "ADC common status register."]
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
        #[doc = "Analog watchdog flags of the master ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn awd_mst(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flags of the master ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_awd_mst(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC voltage regulator ready flag of the master ADC This bit is a copy of the LDORDY bit of the corresponding ADC_ISR register."]
        #[inline(always)]
        pub const fn ldordy_mst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADC voltage regulator ready flag of the master ADC This bit is a copy of the LDORDY bit of the corresponding ADC_ISR register."]
        #[inline(always)]
        pub fn set_ldordy_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Slave ADC ready This bit is a copy of the ADRDY bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn adrdy_slv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Slave ADC ready This bit is a copy of the ADRDY bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_adrdy_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "End of Sampling phase flag of the slave ADC This bit is a copy of the EOSMP2 bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn eosmp_slv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "End of Sampling phase flag of the slave ADC This bit is a copy of the EOSMP2 bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_eosmp_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "End of regular conversion of the slave ADC This bit is a copy of the EOC bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn eoc_slv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion of the slave ADC This bit is a copy of the EOC bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_eoc_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End of regular sequence flag of the slave ADC This bit is a copy of the EOS bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn eos_slv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence flag of the slave ADC This bit is a copy of the EOS bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_eos_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Overrun flag of the slave ADC This bit is a copy of the OVR bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn ovr_slv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag of the slave ADC This bit is a copy of the OVR bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_ovr_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "End of injected conversion flag of the slave ADC This bit is a copy of the JEOC bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn jeoc_slv(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag of the slave ADC This bit is a copy of the JEOC bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_jeoc_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "End of injected sequence flag of the slave ADC This bit is a copy of the JEOS bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn jeos_slv(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence flag of the slave ADC This bit is a copy of the JEOS bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_jeos_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC This bit is a copy of the AWD1 bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn awd1_slv(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 23usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC This bit is a copy of the AWD1 bit in the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_awd1_slv(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 23usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC voltage regulator ready flag of the slave ADC This bit is a copy of the LDORDY bit of the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub const fn ldordy_slv(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC voltage regulator ready flag of the slave ADC This bit is a copy of the LDORDY bit of the corresponding ADCx+1_ISR register."]
        #[inline(always)]
        pub fn set_ldordy_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("ldordy_mst", &self.ldordy_mst())
                .field("adrdy_slv", &self.adrdy_slv())
                .field("eosmp_slv", &self.eosmp_slv())
                .field("eoc_slv", &self.eoc_slv())
                .field("eos_slv", &self.eos_slv())
                .field("ovr_slv", &self.ovr_slv())
                .field("jeoc_slv", &self.jeoc_slv())
                .field("jeos_slv", &self.jeos_slv())
                .field("awd1_slv[0]", &self.awd1_slv(0usize))
                .field("awd1_slv[1]", &self.awd1_slv(1usize))
                .field("awd1_slv[2]", &self.awd1_slv(2usize))
                .field("ldordy_slv", &self.ldordy_slv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ adrdy_mst: {=bool:?}, eosmp_mst: {=bool:?}, eoc_mst: {=bool:?}, eos_mst: {=bool:?}, ovr_mst: {=bool:?}, jeoc_mst: {=bool:?}, jeos_mst: {=bool:?}, awd_mst[0]: {=bool:?}, awd_mst[1]: {=bool:?}, awd_mst[2]: {=bool:?}, ldordy_mst: {=bool:?}, adrdy_slv: {=bool:?}, eosmp_slv: {=bool:?}, eoc_slv: {=bool:?}, eos_slv: {=bool:?}, ovr_slv: {=bool:?}, jeoc_slv: {=bool:?}, jeos_slv: {=bool:?}, awd1_slv[0]: {=bool:?}, awd1_slv[1]: {=bool:?}, awd1_slv[2]: {=bool:?}, ldordy_slv: {=bool:?} }}" , self . adrdy_mst () , self . eosmp_mst () , self . eoc_mst () , self . eos_mst () , self . ovr_mst () , self . jeoc_mst () , self . jeos_mst () , self . awd_mst (0usize) , self . awd_mst (1usize) , self . awd_mst (2usize) , self . ldordy_mst () , self . adrdy_slv () , self . eosmp_slv () , self . eoc_slv () , self . eos_slv () , self . ovr_slv () , self . jeoc_slv () , self . jeos_slv () , self . awd1_slv (0usize) , self . awd1_slv (1usize) , self . awd1_slv (2usize) , self . ldordy_slv ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Damdf {
        #[doc = "Without data packing, CDR/CDR2 not used"]
        NO_PACK = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "CDR formatted for 32-bit down to 10-bit resolution"]
        FORMAT32TO10 = 0x02,
        #[doc = "CDR formatted for 8-bit resolution"]
        FORMAT8 = 0x03,
    }
    impl Damdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Damdf {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Damdf {
        #[inline(always)]
        fn from(val: u8) -> Damdf {
            Damdf::from_bits(val)
        }
    }
    impl From<Damdf> for u8 {
        #[inline(always)]
        fn from(val: Damdf) -> u8 {
            Damdf::to_bits(val)
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
