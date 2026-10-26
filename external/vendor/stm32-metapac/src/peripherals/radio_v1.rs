#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Radio {
    ptr: *mut u8,
}
unsafe impl Send for Radio {}
unsafe impl Sync for Radio {}
impl Radio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AA0_DIG_USR register."]
    #[inline(always)]
    pub const fn aa0_dig_usr(self) -> crate::common::Reg<regs::Aa0DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "AA1_DIG_USR register."]
    #[inline(always)]
    pub const fn aa1_dig_usr(self) -> crate::common::Reg<regs::Aa1DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "AA2_DIG_USR register."]
    #[inline(always)]
    pub const fn aa2_dig_usr(self) -> crate::common::Reg<regs::Aa2DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "AA3_DIG_USR register."]
    #[inline(always)]
    pub const fn aa3_dig_usr(self) -> crate::common::Reg<regs::Aa3DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DEM_MOD_DIG_USR register."]
    #[inline(always)]
    pub const fn dem_mod_dig_usr(self) -> crate::common::Reg<regs::DemModDigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RADIO_FSM_USR register."]
    #[inline(always)]
    pub const fn radio_fsm_usr(self) -> crate::common::Reg<regs::RadioFsmUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PHYCTRL_DIG_USR register."]
    #[inline(always)]
    pub const fn phyctrl_dig_usr(self) -> crate::common::Reg<regs::PhyctrlDigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "AFC1_DIG_ENG register."]
    #[inline(always)]
    pub const fn afc1_dig_eng(self) -> crate::common::Reg<regs::Afc1DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "CR0_DIG_ENG register."]
    #[inline(always)]
    pub const fn cr0_dig_eng(self) -> crate::common::Reg<regs::Cr0DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "CR0_LR register."]
    #[inline(always)]
    pub const fn cr0_lr(self) -> crate::common::Reg<regs::Cr0Lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "VIT_CONF_DIG_ENG register."]
    #[inline(always)]
    pub const fn vit_conf_dig_eng(self) -> crate::common::Reg<regs::VitConfDigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "LR_PD_THR_DIG_ENG register."]
    #[inline(always)]
    pub const fn lr_pd_thr_dig_eng(self) -> crate::common::Reg<regs::LrPdThrDigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "LR_RSSI_THR_DIG_ENG register."]
    #[inline(always)]
    pub const fn lr_rssi_thr_dig_eng(self) -> crate::common::Reg<regs::LrRssiThrDigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "LR_AAC_THR_DIG_ENG register."]
    #[inline(always)]
    pub const fn lr_aac_thr_dig_eng(self) -> crate::common::Reg<regs::LrAacThrDigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "SYNTHCAL0_DIG_ENG register."]
    #[inline(always)]
    pub const fn synthcal0_dig_eng(self) -> crate::common::Reg<regs::Synthcal0DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "DTB5_DIG_ENG register."]
    #[inline(always)]
    pub const fn dtb5_dig_eng(self) -> crate::common::Reg<regs::Dtb5DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "RXADC_ANA_USR register."]
    #[inline(always)]
    pub const fn rxadc_ana_usr(self) -> crate::common::Reg<regs::RxadcAnaUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "LDO_ANA_ENG register."]
    #[inline(always)]
    pub const fn ldo_ana_eng(self) -> crate::common::Reg<regs::LdoAnaEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "CBIAS0_ANA_ENG register."]
    #[inline(always)]
    pub const fn cbias0_ana_eng(self) -> crate::common::Reg<regs::Cbias0AnaEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "CBIAS1_ANA_ENG register."]
    #[inline(always)]
    pub const fn cbias1_ana_eng(self) -> crate::common::Reg<regs::Cbias1AnaEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "SYNTHCAL0_DIG_OUT register."]
    #[inline(always)]
    pub const fn synthcal0_dig_out(self) -> crate::common::Reg<regs::Synthcal0DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "SYNTHCAL1_DIG_OUT register."]
    #[inline(always)]
    pub const fn synthcal1_dig_out(self) -> crate::common::Reg<regs::Synthcal1DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "SYNTHCAL2_DIG_OUT register."]
    #[inline(always)]
    pub const fn synthcal2_dig_out(self) -> crate::common::Reg<regs::Synthcal2DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "SYNTHCAL3_DIG_OUT register."]
    #[inline(always)]
    pub const fn synthcal3_dig_out(self) -> crate::common::Reg<regs::Synthcal3DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "SYNTHCAL4_DIG_OUT register."]
    #[inline(always)]
    pub const fn synthcal4_dig_out(self) -> crate::common::Reg<regs::Synthcal4DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "SYNTHCAL5_DIG_OUT register."]
    #[inline(always)]
    pub const fn synthcal5_dig_out(self) -> crate::common::Reg<regs::Synthcal5DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "FSM_STATUS_DIG_OUT register."]
    #[inline(always)]
    pub const fn fsm_status_dig_out(self) -> crate::common::Reg<regs::FsmStatusDigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "RSSI0_DIG_OUT register."]
    #[inline(always)]
    pub const fn rssi0_dig_out(self) -> crate::common::Reg<regs::Rssi0DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "RSSI1_DIG_OUT register."]
    #[inline(always)]
    pub const fn rssi1_dig_out(self) -> crate::common::Reg<regs::Rssi1DigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "AGC_DIG_OUT register."]
    #[inline(always)]
    pub const fn agc_dig_out(self) -> crate::common::Reg<regs::AgcDigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[doc = "DEMOD_DIG_OUT register."]
    #[inline(always)]
    pub const fn demod_dig_out(self) -> crate::common::Reg<regs::DemodDigOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "AGC2_ANA_TST register."]
    #[inline(always)]
    pub const fn agc2_ana_tst(self) -> crate::common::Reg<regs::Agc2AnaTst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "AGC0_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc0_dig_eng(self) -> crate::common::Reg<regs::Agc0DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "AGC1_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc1_dig_eng(self) -> crate::common::Reg<regs::Agc1DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "AGC10_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc10_dig_eng(self) -> crate::common::Reg<regs::Agc10DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "AGC11_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc11_dig_eng(self) -> crate::common::Reg<regs::Agc11DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "AGC12_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc12_dig_eng(self) -> crate::common::Reg<regs::Agc12DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "AGC13_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc13_dig_eng(self) -> crate::common::Reg<regs::Agc13DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[doc = "AGC14_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc14_dig_eng(self) -> crate::common::Reg<regs::Agc14DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "AGC15_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc15_dig_eng(self) -> crate::common::Reg<regs::Agc15DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "AGC16_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc16_dig_eng(self) -> crate::common::Reg<regs::Agc16DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "AGC17_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc17_dig_eng(self) -> crate::common::Reg<regs::Agc17DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "AGC18_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc18_dig_eng(self) -> crate::common::Reg<regs::Agc18DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "AGC19_DIG_ENG register."]
    #[inline(always)]
    pub const fn agc19_dig_eng(self) -> crate::common::Reg<regs::Agc19DigEng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "RXADC_HW_TRIM_OUT register."]
    #[inline(always)]
    pub const fn rxadc_hw_trim_out(self) -> crate::common::Reg<regs::RxadcHwTrimOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[doc = "CBIAS0_HW_TRIM_OUT register."]
    #[inline(always)]
    pub const fn cbias0_hw_trim_out(self) -> crate::common::Reg<regs::Cbias0HwTrimOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[doc = "AGC_HW_TRIM_OUT register."]
    #[inline(always)]
    pub const fn agc_hw_trim_out(self) -> crate::common::Reg<regs::AgcHwTrimOut, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "DEMOD_IQ2_DIG_TST register."]
    #[inline(always)]
    pub const fn demod_iq2_dig_tst(self) -> crate::common::Reg<regs::DemodIq2DigTst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[doc = "ANTSW0_DIG_USR register."]
    #[inline(always)]
    pub const fn antsw0_dig_usr(self) -> crate::common::Reg<regs::Antsw0DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "ANTSW1_DIG_USR register."]
    #[inline(always)]
    pub const fn antsw1_dig_usr(self) -> crate::common::Reg<regs::Antsw1DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[doc = "ANTSW2_DIG_USR register."]
    #[inline(always)]
    pub const fn antsw2_dig_usr(self) -> crate::common::Reg<regs::Antsw2DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[doc = "ANTSW3_DIG_USR register."]
    #[inline(always)]
    pub const fn antsw3_dig_usr(self) -> crate::common::Reg<regs::Antsw3DigUsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
}
pub mod regs {
    #[doc = "AA0_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aa0DigUsr(pub u32);
    impl Aa0DigUsr {
        #[doc = "Least significant byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub const fn aa_7_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Least significant byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub fn set_aa_7_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Aa0DigUsr {
        #[inline(always)]
        fn default() -> Aa0DigUsr {
            Aa0DigUsr(0)
        }
    }
    impl core::fmt::Debug for Aa0DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aa0DigUsr").field("aa_7_0", &self.aa_7_0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aa0DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aa0DigUsr {{ aa_7_0: {=u8:?} }}", self.aa_7_0())
        }
    }
    #[doc = "AA1_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aa1DigUsr(pub u32);
    impl Aa1DigUsr {
        #[doc = "Next byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub const fn aa_15_8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Next byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub fn set_aa_15_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Aa1DigUsr {
        #[inline(always)]
        fn default() -> Aa1DigUsr {
            Aa1DigUsr(0)
        }
    }
    impl core::fmt::Debug for Aa1DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aa1DigUsr").field("aa_15_8", &self.aa_15_8()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aa1DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aa1DigUsr {{ aa_15_8: {=u8:?} }}", self.aa_15_8())
        }
    }
    #[doc = "AA2_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aa2DigUsr(pub u32);
    impl Aa2DigUsr {
        #[doc = "Next byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub const fn aa_23_16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Next byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub fn set_aa_23_16(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Aa2DigUsr {
        #[inline(always)]
        fn default() -> Aa2DigUsr {
            Aa2DigUsr(0)
        }
    }
    impl core::fmt::Debug for Aa2DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aa2DigUsr").field("aa_23_16", &self.aa_23_16()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aa2DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aa2DigUsr {{ aa_23_16: {=u8:?} }}", self.aa_23_16())
        }
    }
    #[doc = "AA3_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aa3DigUsr(pub u32);
    impl Aa3DigUsr {
        #[doc = "Most significant byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub const fn aa_31_24(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Most significant byte of the Bluetooth LE Access Address code."]
        #[inline(always)]
        pub fn set_aa_31_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Aa3DigUsr {
        #[inline(always)]
        fn default() -> Aa3DigUsr {
            Aa3DigUsr(0)
        }
    }
    impl core::fmt::Debug for Aa3DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aa3DigUsr").field("aa_31_24", &self.aa_31_24()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aa3DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aa3DigUsr {{ aa_31_24: {=u8:?} }}", self.aa_31_24())
        }
    }
    #[doc = "AFC1_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afc1DigEng(pub u32);
    impl Afc1DigEng {
        #[doc = "Set the decay factor of the AFC loop after Access Address detection."]
        #[inline(always)]
        pub const fn afc_delay_after(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Set the decay factor of the AFC loop after Access Address detection."]
        #[inline(always)]
        pub fn set_afc_delay_after(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Set the decay factor of the AFC loop before Access Address detection."]
        #[inline(always)]
        pub const fn afc_delay_before(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Set the decay factor of the AFC loop before Access Address detection."]
        #[inline(always)]
        pub fn set_afc_delay_before(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Afc1DigEng {
        #[inline(always)]
        fn default() -> Afc1DigEng {
            Afc1DigEng(0)
        }
    }
    impl core::fmt::Debug for Afc1DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afc1DigEng")
                .field("afc_delay_after", &self.afc_delay_after())
                .field("afc_delay_before", &self.afc_delay_before())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afc1DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afc1DigEng {{ afc_delay_after: {=u8:?}, afc_delay_before: {=u8:?} }}",
                self.afc_delay_after(),
                self.afc_delay_before()
            )
        }
    }
    #[doc = "AGC0_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc0DigEng(pub u32);
    impl Agc0DigEng {
        #[doc = "High AGC threshold."]
        #[inline(always)]
        pub const fn agc_thr_high(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "High AGC threshold."]
        #[inline(always)]
        pub fn set_agc_thr_high(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Enable AGC."]
        #[inline(always)]
        pub const fn agc_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable AGC."]
        #[inline(always)]
        pub fn set_agc_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Agc0DigEng {
        #[inline(always)]
        fn default() -> Agc0DigEng {
            Agc0DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc0DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc0DigEng")
                .field("agc_thr_high", &self.agc_thr_high())
                .field("agc_enable", &self.agc_enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc0DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc0DigEng {{ agc_thr_high: {=u8:?}, agc_enable: {=bool:?} }}",
                self.agc_thr_high(),
                self.agc_enable()
            )
        }
    }
    #[doc = "AGC10_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc10DigEng(pub u32);
    impl Agc10DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 0:."]
        #[inline(always)]
        pub const fn att_if_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 0:."]
        #[inline(always)]
        pub fn set_att_if_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 0:."]
        #[inline(always)]
        pub const fn att_lna_0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 0:."]
        #[inline(always)]
        pub fn set_att_lna_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 0:."]
        #[inline(always)]
        pub const fn att_ant_0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 0:."]
        #[inline(always)]
        pub fn set_att_ant_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc10DigEng {
        #[inline(always)]
        fn default() -> Agc10DigEng {
            Agc10DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc10DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc10DigEng")
                .field("att_if_0", &self.att_if_0())
                .field("att_lna_0", &self.att_lna_0())
                .field("att_ant_0", &self.att_ant_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc10DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc10DigEng {{ att_if_0: {=u8:?}, att_lna_0: {=bool:?}, att_ant_0: {=u8:?} }}",
                self.att_if_0(),
                self.att_lna_0(),
                self.att_ant_0()
            )
        }
    }
    #[doc = "AGC11_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc11DigEng(pub u32);
    impl Agc11DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 1."]
        #[inline(always)]
        pub const fn att_if_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 1."]
        #[inline(always)]
        pub fn set_att_if_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 1."]
        #[inline(always)]
        pub const fn att_lna_1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 1."]
        #[inline(always)]
        pub fn set_att_lna_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 1."]
        #[inline(always)]
        pub const fn att_ant_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 1."]
        #[inline(always)]
        pub fn set_att_ant_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc11DigEng {
        #[inline(always)]
        fn default() -> Agc11DigEng {
            Agc11DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc11DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc11DigEng")
                .field("att_if_1", &self.att_if_1())
                .field("att_lna_1", &self.att_lna_1())
                .field("att_ant_1", &self.att_ant_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc11DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc11DigEng {{ att_if_1: {=u8:?}, att_lna_1: {=bool:?}, att_ant_1: {=u8:?} }}",
                self.att_if_1(),
                self.att_lna_1(),
                self.att_ant_1()
            )
        }
    }
    #[doc = "AGC12_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc12DigEng(pub u32);
    impl Agc12DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 2."]
        #[inline(always)]
        pub const fn att_if_2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 2."]
        #[inline(always)]
        pub fn set_att_if_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 2."]
        #[inline(always)]
        pub const fn att_lna_2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 2."]
        #[inline(always)]
        pub fn set_att_lna_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 2."]
        #[inline(always)]
        pub const fn att_ant_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 2."]
        #[inline(always)]
        pub fn set_att_ant_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc12DigEng {
        #[inline(always)]
        fn default() -> Agc12DigEng {
            Agc12DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc12DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc12DigEng")
                .field("att_if_2", &self.att_if_2())
                .field("att_lna_2", &self.att_lna_2())
                .field("att_ant_2", &self.att_ant_2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc12DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc12DigEng {{ att_if_2: {=u8:?}, att_lna_2: {=bool:?}, att_ant_2: {=u8:?} }}",
                self.att_if_2(),
                self.att_lna_2(),
                self.att_ant_2()
            )
        }
    }
    #[doc = "AGC13_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc13DigEng(pub u32);
    impl Agc13DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 3."]
        #[inline(always)]
        pub const fn att_if_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 3."]
        #[inline(always)]
        pub fn set_att_if_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 3."]
        #[inline(always)]
        pub const fn att_lna_3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 3."]
        #[inline(always)]
        pub fn set_att_lna_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 3."]
        #[inline(always)]
        pub const fn att_ant_3(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 3."]
        #[inline(always)]
        pub fn set_att_ant_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc13DigEng {
        #[inline(always)]
        fn default() -> Agc13DigEng {
            Agc13DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc13DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc13DigEng")
                .field("att_if_3", &self.att_if_3())
                .field("att_lna_3", &self.att_lna_3())
                .field("att_ant_3", &self.att_ant_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc13DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc13DigEng {{ att_if_3: {=u8:?}, att_lna_3: {=bool:?}, att_ant_3: {=u8:?} }}",
                self.att_if_3(),
                self.att_lna_3(),
                self.att_ant_3()
            )
        }
    }
    #[doc = "AGC14_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc14DigEng(pub u32);
    impl Agc14DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 4."]
        #[inline(always)]
        pub const fn att_if_4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 4."]
        #[inline(always)]
        pub fn set_att_if_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 4."]
        #[inline(always)]
        pub const fn att_lna_4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 4."]
        #[inline(always)]
        pub fn set_att_lna_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 4."]
        #[inline(always)]
        pub const fn att_ant_4(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 4."]
        #[inline(always)]
        pub fn set_att_ant_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc14DigEng {
        #[inline(always)]
        fn default() -> Agc14DigEng {
            Agc14DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc14DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc14DigEng")
                .field("att_if_4", &self.att_if_4())
                .field("att_lna_4", &self.att_lna_4())
                .field("att_ant_4", &self.att_ant_4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc14DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc14DigEng {{ att_if_4: {=u8:?}, att_lna_4: {=bool:?}, att_ant_4: {=u8:?} }}",
                self.att_if_4(),
                self.att_lna_4(),
                self.att_ant_4()
            )
        }
    }
    #[doc = "AGC15_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc15DigEng(pub u32);
    impl Agc15DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 5."]
        #[inline(always)]
        pub const fn att_if_5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 5."]
        #[inline(always)]
        pub fn set_att_if_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 5."]
        #[inline(always)]
        pub const fn att_lna_5(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 5."]
        #[inline(always)]
        pub fn set_att_lna_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 5."]
        #[inline(always)]
        pub const fn att_ant_5(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 5."]
        #[inline(always)]
        pub fn set_att_ant_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc15DigEng {
        #[inline(always)]
        fn default() -> Agc15DigEng {
            Agc15DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc15DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc15DigEng")
                .field("att_if_5", &self.att_if_5())
                .field("att_lna_5", &self.att_lna_5())
                .field("att_ant_5", &self.att_ant_5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc15DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc15DigEng {{ att_if_5: {=u8:?}, att_lna_5: {=bool:?}, att_ant_5: {=u8:?} }}",
                self.att_if_5(),
                self.att_lna_5(),
                self.att_ant_5()
            )
        }
    }
    #[doc = "AGC16_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc16DigEng(pub u32);
    impl Agc16DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 6."]
        #[inline(always)]
        pub const fn att_if_6(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 6."]
        #[inline(always)]
        pub fn set_att_if_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 6."]
        #[inline(always)]
        pub const fn att_lna_6(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 6."]
        #[inline(always)]
        pub fn set_att_lna_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 6."]
        #[inline(always)]
        pub const fn att_ant_6(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 6."]
        #[inline(always)]
        pub fn set_att_ant_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc16DigEng {
        #[inline(always)]
        fn default() -> Agc16DigEng {
            Agc16DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc16DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc16DigEng")
                .field("att_if_6", &self.att_if_6())
                .field("att_lna_6", &self.att_lna_6())
                .field("att_ant_6", &self.att_ant_6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc16DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc16DigEng {{ att_if_6: {=u8:?}, att_lna_6: {=bool:?}, att_ant_6: {=u8:?} }}",
                self.att_if_6(),
                self.att_lna_6(),
                self.att_ant_6()
            )
        }
    }
    #[doc = "AGC17_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc17DigEng(pub u32);
    impl Agc17DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 7."]
        #[inline(always)]
        pub const fn att_if_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 7."]
        #[inline(always)]
        pub fn set_att_if_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 7."]
        #[inline(always)]
        pub const fn att_lna_7(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 7."]
        #[inline(always)]
        pub fn set_att_lna_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 7."]
        #[inline(always)]
        pub const fn att_ant_7(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 7."]
        #[inline(always)]
        pub fn set_att_ant_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc17DigEng {
        #[inline(always)]
        fn default() -> Agc17DigEng {
            Agc17DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc17DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc17DigEng")
                .field("att_if_7", &self.att_if_7())
                .field("att_lna_7", &self.att_lna_7())
                .field("att_ant_7", &self.att_ant_7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc17DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc17DigEng {{ att_if_7: {=u8:?}, att_lna_7: {=bool:?}, att_ant_7: {=u8:?} }}",
                self.att_if_7(),
                self.att_lna_7(),
                self.att_ant_7()
            )
        }
    }
    #[doc = "AGC18_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc18DigEng(pub u32);
    impl Agc18DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 8."]
        #[inline(always)]
        pub const fn att_if_8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 8."]
        #[inline(always)]
        pub fn set_att_if_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 8."]
        #[inline(always)]
        pub const fn att_lna_8(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 8."]
        #[inline(always)]
        pub fn set_att_lna_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 8."]
        #[inline(always)]
        pub const fn att_ant_8(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 8."]
        #[inline(always)]
        pub fn set_att_ant_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc18DigEng {
        #[inline(always)]
        fn default() -> Agc18DigEng {
            Agc18DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc18DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc18DigEng")
                .field("att_if_8", &self.att_if_8())
                .field("att_lna_8", &self.att_lna_8())
                .field("att_ant_8", &self.att_ant_8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc18DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc18DigEng {{ att_if_8: {=u8:?}, att_lna_8: {=bool:?}, att_ant_8: {=u8:?} }}",
                self.att_if_8(),
                self.att_lna_8(),
                self.att_ant_8()
            )
        }
    }
    #[doc = "AGC19_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc19DigEng(pub u32);
    impl Agc19DigEng {
        #[doc = "Attenuation at IF Level for the AGC step 9."]
        #[inline(always)]
        pub const fn att_if_9(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Attenuation at IF Level for the AGC step 9."]
        #[inline(always)]
        pub fn set_att_if_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Attenuation at LNA Level for the AGC step 9."]
        #[inline(always)]
        pub const fn att_lna_9(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Attenuation at LNA Level for the AGC step 9."]
        #[inline(always)]
        pub fn set_att_lna_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 9."]
        #[inline(always)]
        pub const fn att_ant_9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Attenuation at Antenna Level for the AGC step 9."]
        #[inline(always)]
        pub fn set_att_ant_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Agc19DigEng {
        #[inline(always)]
        fn default() -> Agc19DigEng {
            Agc19DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc19DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc19DigEng")
                .field("att_if_9", &self.att_if_9())
                .field("att_lna_9", &self.att_lna_9())
                .field("att_ant_9", &self.att_ant_9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc19DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc19DigEng {{ att_if_9: {=u8:?}, att_lna_9: {=bool:?}, att_ant_9: {=u8:?} }}",
                self.att_if_9(),
                self.att_lna_9(),
                self.att_ant_9()
            )
        }
    }
    #[doc = "AGC1_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc1DigEng(pub u32);
    impl Agc1DigEng {
        #[doc = "Low threshold for 6dB steps."]
        #[inline(always)]
        pub const fn agc_thr_low_6(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Low threshold for 6dB steps."]
        #[inline(always)]
        pub fn set_agc_thr_low_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "AGC locks when level is steady between high threshold and lock threshold."]
        #[inline(always)]
        pub const fn agc_autolock(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AGC locks when level is steady between high threshold and lock threshold."]
        #[inline(always)]
        pub fn set_agc_autolock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AGC locks when Access Address is detected (recommended)."]
        #[inline(always)]
        pub const fn agc_lock_sync(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "AGC locks when Access Address is detected (recommended)."]
        #[inline(always)]
        pub fn set_agc_lock_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Agc1DigEng {
        #[inline(always)]
        fn default() -> Agc1DigEng {
            Agc1DigEng(0)
        }
    }
    impl core::fmt::Debug for Agc1DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc1DigEng")
                .field("agc_thr_low_6", &self.agc_thr_low_6())
                .field("agc_autolock", &self.agc_autolock())
                .field("agc_lock_sync", &self.agc_lock_sync())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc1DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc1DigEng {{ agc_thr_low_6: {=u8:?}, agc_autolock: {=bool:?}, agc_lock_sync: {=bool:?} }}",
                self.agc_thr_low_6(),
                self.agc_autolock(),
                self.agc_lock_sync()
            )
        }
    }
    #[doc = "AGC2_ANA_TST register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agc2AnaTst(pub u32);
    impl Agc2AnaTst {
        #[doc = "Selection:."]
        #[inline(always)]
        pub const fn agc2_ana_tst_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Selection:."]
        #[inline(always)]
        pub fn set_agc2_ana_tst_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1)."]
        #[inline(always)]
        pub const fn agc_antennae_usr_trim(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1)."]
        #[inline(always)]
        pub fn set_agc_antennae_usr_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
    }
    impl Default for Agc2AnaTst {
        #[inline(always)]
        fn default() -> Agc2AnaTst {
            Agc2AnaTst(0)
        }
    }
    impl core::fmt::Debug for Agc2AnaTst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agc2AnaTst")
                .field("agc2_ana_tst_sel", &self.agc2_ana_tst_sel())
                .field("agc_antennae_usr_trim", &self.agc_antennae_usr_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agc2AnaTst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Agc2AnaTst {{ agc2_ana_tst_sel: {=bool:?}, agc_antennae_usr_trim: {=u8:?} }}",
                self.agc2_ana_tst_sel(),
                self.agc_antennae_usr_trim()
            )
        }
    }
    #[doc = "AGC_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AgcDigOut(pub u32);
    impl AgcDigOut {
        #[doc = "AGC attenuation value."]
        #[inline(always)]
        pub const fn agc_att_out(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "AGC attenuation value."]
        #[inline(always)]
        pub fn set_agc_att_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AgcDigOut {
        #[inline(always)]
        fn default() -> AgcDigOut {
            AgcDigOut(0)
        }
    }
    impl core::fmt::Debug for AgcDigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AgcDigOut")
                .field("agc_att_out", &self.agc_att_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AgcDigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AgcDigOut {{ agc_att_out: {=u8:?} }}", self.agc_att_out())
        }
    }
    #[doc = "AGC_HW_TRIM_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AgcHwTrimOut(pub u32);
    impl AgcHwTrimOut {
        #[doc = "AGC trim value (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub const fn hw_agc_antennae_trim(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "AGC trim value (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub fn set_hw_agc_antennae_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
    }
    impl Default for AgcHwTrimOut {
        #[inline(always)]
        fn default() -> AgcHwTrimOut {
            AgcHwTrimOut(0)
        }
    }
    impl core::fmt::Debug for AgcHwTrimOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AgcHwTrimOut")
                .field("hw_agc_antennae_trim", &self.hw_agc_antennae_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AgcHwTrimOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AgcHwTrimOut {{ hw_agc_antennae_trim: {=u8:?} }}",
                self.hw_agc_antennae_trim()
            )
        }
    }
    #[doc = "ANTSW0_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Antsw0DigUsr(pub u32);
    impl Antsw0DigUsr {
        #[doc = "specifies the exact timing of the first I/Q sampling in the reference period."]
        #[inline(always)]
        pub const fn rx_time_to_sample(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "specifies the exact timing of the first I/Q sampling in the reference period."]
        #[inline(always)]
        pub fn set_rx_time_to_sample(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Antsw0DigUsr {
        #[inline(always)]
        fn default() -> Antsw0DigUsr {
            Antsw0DigUsr(0)
        }
    }
    impl core::fmt::Debug for Antsw0DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Antsw0DigUsr")
                .field("rx_time_to_sample", &self.rx_time_to_sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Antsw0DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Antsw0DigUsr {{ rx_time_to_sample: {=u8:?} }}",
                self.rx_time_to_sample()
            )
        }
    }
    #[doc = "ANTSW1_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Antsw1DigUsr(pub u32);
    impl Antsw1DigUsr {
        #[doc = "specifies the exact timing of the antenna switching at receiver level (in AoA)."]
        #[inline(always)]
        pub const fn rx_time_to_switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "specifies the exact timing of the antenna switching at receiver level (in AoA)."]
        #[inline(always)]
        pub fn set_rx_time_to_switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Antsw1DigUsr {
        #[inline(always)]
        fn default() -> Antsw1DigUsr {
            Antsw1DigUsr(0)
        }
    }
    impl core::fmt::Debug for Antsw1DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Antsw1DigUsr")
                .field("rx_time_to_switch", &self.rx_time_to_switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Antsw1DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Antsw1DigUsr {{ rx_time_to_switch: {=u8:?} }}",
                self.rx_time_to_switch()
            )
        }
    }
    #[doc = "ANTSW2_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Antsw2DigUsr(pub u32);
    impl Antsw2DigUsr {
        #[doc = "specifies the exact timing of the antenna switching during transmission at LE_1M baud rate (in AoD)."]
        #[inline(always)]
        pub const fn tx_time_to_switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "specifies the exact timing of the antenna switching during transmission at LE_1M baud rate (in AoD)."]
        #[inline(always)]
        pub fn set_tx_time_to_switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Antsw2DigUsr {
        #[inline(always)]
        fn default() -> Antsw2DigUsr {
            Antsw2DigUsr(0)
        }
    }
    impl core::fmt::Debug for Antsw2DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Antsw2DigUsr")
                .field("tx_time_to_switch", &self.tx_time_to_switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Antsw2DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Antsw2DigUsr {{ tx_time_to_switch: {=u8:?} }}",
                self.tx_time_to_switch()
            )
        }
    }
    #[doc = "ANTSW3_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Antsw3DigUsr(pub u32);
    impl Antsw3DigUsr {
        #[doc = "specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD)."]
        #[inline(always)]
        pub const fn tx_time_to_switch_2m(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD)."]
        #[inline(always)]
        pub fn set_tx_time_to_switch_2m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Antsw3DigUsr {
        #[inline(always)]
        fn default() -> Antsw3DigUsr {
            Antsw3DigUsr(0)
        }
    }
    impl core::fmt::Debug for Antsw3DigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Antsw3DigUsr")
                .field("tx_time_to_switch_2m", &self.tx_time_to_switch_2m())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Antsw3DigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Antsw3DigUsr {{ tx_time_to_switch_2m: {=u8:?} }}",
                self.tx_time_to_switch_2m()
            )
        }
    }
    #[doc = "CBIAS0_ANA_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cbias0AnaEng(pub u32);
    impl Cbias0AnaEng {
        #[doc = "overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)."]
        #[inline(always)]
        pub const fn rfd_cbias_ibias_trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)."]
        #[inline(always)]
        pub fn set_rfd_cbias_ibias_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)."]
        #[inline(always)]
        pub const fn rfd_cbias_iptat_trim(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)."]
        #[inline(always)]
        pub fn set_rfd_cbias_iptat_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cbias0AnaEng {
        #[inline(always)]
        fn default() -> Cbias0AnaEng {
            Cbias0AnaEng(0)
        }
    }
    impl core::fmt::Debug for Cbias0AnaEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cbias0AnaEng")
                .field("rfd_cbias_ibias_trim", &self.rfd_cbias_ibias_trim())
                .field("rfd_cbias_iptat_trim", &self.rfd_cbias_iptat_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cbias0AnaEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cbias0AnaEng {{ rfd_cbias_ibias_trim: {=u8:?}, rfd_cbias_iptat_trim: {=u8:?} }}",
                self.rfd_cbias_ibias_trim(),
                self.rfd_cbias_iptat_trim()
            )
        }
    }
    #[doc = "CBIAS0_HW_TRIM_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cbias0HwTrimOut(pub u32);
    impl Cbias0HwTrimOut {
        #[doc = "CBIAS current (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub const fn hw_cbias_ibias_trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CBIAS current (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub fn set_hw_cbias_ibias_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CBIAS current (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub const fn hw_cbias_iptat_trim(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "CBIAS current (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub fn set_hw_cbias_iptat_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cbias0HwTrimOut {
        #[inline(always)]
        fn default() -> Cbias0HwTrimOut {
            Cbias0HwTrimOut(0)
        }
    }
    impl core::fmt::Debug for Cbias0HwTrimOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cbias0HwTrimOut")
                .field("hw_cbias_ibias_trim", &self.hw_cbias_ibias_trim())
                .field("hw_cbias_iptat_trim", &self.hw_cbias_iptat_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cbias0HwTrimOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cbias0HwTrimOut {{ hw_cbias_ibias_trim: {=u8:?}, hw_cbias_iptat_trim: {=u8:?} }}",
                self.hw_cbias_ibias_trim(),
                self.hw_cbias_iptat_trim()
            )
        }
    }
    #[doc = "CBIAS1_ANA_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cbias1AnaEng(pub u32);
    impl Cbias1AnaEng {
        #[doc = "When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings."]
        #[inline(always)]
        pub const fn cbias0_trim_tst_sel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings."]
        #[inline(always)]
        pub fn set_cbias0_trim_tst_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cbias1AnaEng {
        #[inline(always)]
        fn default() -> Cbias1AnaEng {
            Cbias1AnaEng(0)
        }
    }
    impl core::fmt::Debug for Cbias1AnaEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cbias1AnaEng")
                .field("cbias0_trim_tst_sel", &self.cbias0_trim_tst_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cbias1AnaEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cbias1AnaEng {{ cbias0_trim_tst_sel: {=bool:?} }}",
                self.cbias0_trim_tst_sel()
            )
        }
    }
    #[doc = "CR0_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0DigEng(pub u32);
    impl Cr0DigEng {
        #[doc = "Set the gain of the clock recovery loop before Access Address detection to the value."]
        #[inline(always)]
        pub const fn cr_gain_after(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Set the gain of the clock recovery loop before Access Address detection to the value."]
        #[inline(always)]
        pub fn set_cr_gain_after(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Set the gain of the clock recovery loop before Access Address detection to the value."]
        #[inline(always)]
        pub const fn cr_gain_before(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Set the gain of the clock recovery loop before Access Address detection to the value."]
        #[inline(always)]
        pub fn set_cr_gain_before(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cr0DigEng {
        #[inline(always)]
        fn default() -> Cr0DigEng {
            Cr0DigEng(0)
        }
    }
    impl core::fmt::Debug for Cr0DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0DigEng")
                .field("cr_gain_after", &self.cr_gain_after())
                .field("cr_gain_before", &self.cr_gain_before())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0DigEng {{ cr_gain_after: {=u8:?}, cr_gain_before: {=u8:?} }}",
                self.cr_gain_after(),
                self.cr_gain_before()
            )
        }
    }
    #[doc = "CR0_LR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0Lr(pub u32);
    impl Cr0Lr {
        #[doc = "Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use."]
        #[inline(always)]
        pub const fn cr_lr_gain_after(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use."]
        #[inline(always)]
        pub fn set_cr_lr_gain_after(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use."]
        #[inline(always)]
        pub const fn cr_lr_gain_before(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use."]
        #[inline(always)]
        pub fn set_cr_lr_gain_before(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cr0Lr {
        #[inline(always)]
        fn default() -> Cr0Lr {
            Cr0Lr(0)
        }
    }
    impl core::fmt::Debug for Cr0Lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0Lr")
                .field("cr_lr_gain_after", &self.cr_lr_gain_after())
                .field("cr_lr_gain_before", &self.cr_lr_gain_before())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0Lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0Lr {{ cr_lr_gain_after: {=u8:?}, cr_lr_gain_before: {=u8:?} }}",
                self.cr_lr_gain_after(),
                self.cr_lr_gain_before()
            )
        }
    }
    #[doc = "DEM_MOD_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DemModDigUsr(pub u32);
    impl DemModDigUsr {
        #[doc = "Index for internal lock up table in which the synthesizer setup is contained."]
        #[inline(always)]
        pub const fn channel_num(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Index for internal lock up table in which the synthesizer setup is contained."]
        #[inline(always)]
        pub fn set_channel_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for DemModDigUsr {
        #[inline(always)]
        fn default() -> DemModDigUsr {
            DemModDigUsr(0)
        }
    }
    impl core::fmt::Debug for DemModDigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DemModDigUsr")
                .field("channel_num", &self.channel_num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DemModDigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DemModDigUsr {{ channel_num: {=u8:?} }}", self.channel_num())
        }
    }
    #[doc = "DEMOD_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DemodDigOut(pub u32);
    impl DemodDigOut {
        #[doc = "CI field."]
        #[inline(always)]
        pub const fn ci_field(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CI field."]
        #[inline(always)]
        pub fn set_ci_field(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "aac_found."]
        #[inline(always)]
        pub const fn aac_found(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "aac_found."]
        #[inline(always)]
        pub fn set_aac_found(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "pd_found."]
        #[inline(always)]
        pub const fn pd_found(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "pd_found."]
        #[inline(always)]
        pub fn set_pd_found(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "rx_end."]
        #[inline(always)]
        pub const fn rx_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "rx_end."]
        #[inline(always)]
        pub fn set_rx_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for DemodDigOut {
        #[inline(always)]
        fn default() -> DemodDigOut {
            DemodDigOut(0)
        }
    }
    impl core::fmt::Debug for DemodDigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DemodDigOut")
                .field("ci_field", &self.ci_field())
                .field("aac_found", &self.aac_found())
                .field("pd_found", &self.pd_found())
                .field("rx_end", &self.rx_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DemodDigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DemodDigOut {{ ci_field: {=u8:?}, aac_found: {=bool:?}, pd_found: {=bool:?}, rx_end: {=bool:?} }}",
                self.ci_field(),
                self.aac_found(),
                self.pd_found(),
                self.rx_end()
            )
        }
    }
    #[doc = "DEMOD_IQ2_DIG_TST register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DemodIq2DigTst(pub u32);
    impl DemodIq2DigTst {
        #[doc = "Defines the sampling time, when extended configuration is enabled:."]
        #[inline(always)]
        pub const fn extcfg_sampling_time(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Defines the sampling time, when extended configuration is enabled:."]
        #[inline(always)]
        pub fn set_extcfg_sampling_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:."]
        #[inline(always)]
        pub const fn extcfg_trig_selection(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:."]
        #[inline(always)]
        pub fn set_extcfg_trig_selection(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for DemodIq2DigTst {
        #[inline(always)]
        fn default() -> DemodIq2DigTst {
            DemodIq2DigTst(0)
        }
    }
    impl core::fmt::Debug for DemodIq2DigTst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DemodIq2DigTst")
                .field("extcfg_sampling_time", &self.extcfg_sampling_time())
                .field("extcfg_trig_selection", &self.extcfg_trig_selection())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DemodIq2DigTst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DemodIq2DigTst {{ extcfg_sampling_time: {=u8:?}, extcfg_trig_selection: {=u8:?} }}",
                self.extcfg_sampling_time(),
                self.extcfg_trig_selection()
            )
        }
    }
    #[doc = "DTB5_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtb5DigEng(pub u32);
    impl Dtb5DigEng {
        #[doc = "enable the possibility to control some signals by the other register bits instead of system design:."]
        #[inline(always)]
        pub const fn rxtx_start_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable the possibility to control some signals by the other register bits instead of system design:."]
        #[inline(always)]
        pub fn set_rxtx_start_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force TX_ACTIVE signal."]
        #[inline(always)]
        pub const fn tx_active(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force TX_ACTIVE signal."]
        #[inline(always)]
        pub fn set_tx_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force RX_ACTIVE signal."]
        #[inline(always)]
        pub const fn rx_active(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force RX_ACTIVE signal."]
        #[inline(always)]
        pub fn set_rx_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force INITIALIZE signal (emulate a token request of the IP_BLE)."]
        #[inline(always)]
        pub const fn initialize(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force INITIALIZE signal (emulate a token request of the IP_BLE)."]
        #[inline(always)]
        pub fn set_initialize(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable port selection."]
        #[inline(always)]
        pub const fn port_selected_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable port selection."]
        #[inline(always)]
        pub fn set_port_selected_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force port_selected\\[0\\]
signal."]
        #[inline(always)]
        pub const fn port_selected_0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "force port_selected\\[0\\]
signal."]
        #[inline(always)]
        pub fn set_port_selected_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Dtb5DigEng {
        #[inline(always)]
        fn default() -> Dtb5DigEng {
            Dtb5DigEng(0)
        }
    }
    impl core::fmt::Debug for Dtb5DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtb5DigEng")
                .field("rxtx_start_sel", &self.rxtx_start_sel())
                .field("tx_active", &self.tx_active())
                .field("rx_active", &self.rx_active())
                .field("initialize", &self.initialize())
                .field("port_selected_en", &self.port_selected_en())
                .field("port_selected_0", &self.port_selected_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtb5DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dtb5DigEng {{ rxtx_start_sel: {=bool:?}, tx_active: {=bool:?}, rx_active: {=bool:?}, initialize: {=bool:?}, port_selected_en: {=bool:?}, port_selected_0: {=bool:?} }}" , self . rxtx_start_sel () , self . tx_active () , self . rx_active () , self . initialize () , self . port_selected_en () , self . port_selected_0 ())
        }
    }
    #[doc = "FSM_STATUS_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FsmStatusDigOut(pub u32);
    impl FsmStatusDigOut {
        #[doc = "RF FSM state:."]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "RF FSM state:."]
        #[inline(always)]
        pub fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "PLL calibration error."]
        #[inline(always)]
        pub const fn synth_cal_error(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLL calibration error."]
        #[inline(always)]
        pub fn set_synth_cal_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for FsmStatusDigOut {
        #[inline(always)]
        fn default() -> FsmStatusDigOut {
            FsmStatusDigOut(0)
        }
    }
    impl core::fmt::Debug for FsmStatusDigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FsmStatusDigOut")
                .field("status", &self.status())
                .field("synth_cal_error", &self.synth_cal_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FsmStatusDigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FsmStatusDigOut {{ status: {=u8:?}, synth_cal_error: {=bool:?} }}",
                self.status(),
                self.synth_cal_error()
            )
        }
    }
    #[doc = "LDO_ANA_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LdoAnaEng(pub u32);
    impl LdoAnaEng {
        #[doc = "RF_REG Bypass mode:."]
        #[inline(always)]
        pub const fn rfd_rf_reg_bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RF_REG Bypass mode:."]
        #[inline(always)]
        pub fn set_rfd_rf_reg_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for LdoAnaEng {
        #[inline(always)]
        fn default() -> LdoAnaEng {
            LdoAnaEng(0)
        }
    }
    impl core::fmt::Debug for LdoAnaEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LdoAnaEng")
                .field("rfd_rf_reg_bypass", &self.rfd_rf_reg_bypass())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LdoAnaEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LdoAnaEng {{ rfd_rf_reg_bypass: {=bool:?} }}",
                self.rfd_rf_reg_bypass()
            )
        }
    }
    #[doc = "LR_AAC_THR_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LrAacThrDigEng(pub u32);
    impl LrAacThrDigEng {
        #[doc = "address coded correlation threshold."]
        #[inline(always)]
        pub const fn lr_aac_thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "address coded correlation threshold."]
        #[inline(always)]
        pub fn set_lr_aac_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for LrAacThrDigEng {
        #[inline(always)]
        fn default() -> LrAacThrDigEng {
            LrAacThrDigEng(0)
        }
    }
    impl core::fmt::Debug for LrAacThrDigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LrAacThrDigEng")
                .field("lr_aac_thr", &self.lr_aac_thr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LrAacThrDigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LrAacThrDigEng {{ lr_aac_thr: {=u8:?} }}", self.lr_aac_thr())
        }
    }
    #[doc = "LR_PD_THR_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LrPdThrDigEng(pub u32);
    impl LrPdThrDigEng {
        #[doc = "preamble detect threshold value."]
        #[inline(always)]
        pub const fn lr_pd_thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "preamble detect threshold value."]
        #[inline(always)]
        pub fn set_lr_pd_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for LrPdThrDigEng {
        #[inline(always)]
        fn default() -> LrPdThrDigEng {
            LrPdThrDigEng(0)
        }
    }
    impl core::fmt::Debug for LrPdThrDigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LrPdThrDigEng")
                .field("lr_pd_thr", &self.lr_pd_thr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LrPdThrDigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LrPdThrDigEng {{ lr_pd_thr: {=u8:?} }}", self.lr_pd_thr())
        }
    }
    #[doc = "LR_RSSI_THR_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LrRssiThrDigEng(pub u32);
    impl LrRssiThrDigEng {
        #[doc = "RSSI or peak threshold value."]
        #[inline(always)]
        pub const fn lr_rssi_thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RSSI or peak threshold value."]
        #[inline(always)]
        pub fn set_lr_rssi_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for LrRssiThrDigEng {
        #[inline(always)]
        fn default() -> LrRssiThrDigEng {
            LrRssiThrDigEng(0)
        }
    }
    impl core::fmt::Debug for LrRssiThrDigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LrRssiThrDigEng")
                .field("lr_rssi_thr", &self.lr_rssi_thr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LrRssiThrDigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LrRssiThrDigEng {{ lr_rssi_thr: {=u8:?} }}", self.lr_rssi_thr())
        }
    }
    #[doc = "PHYCTRL_DIG_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyctrlDigUsr(pub u32);
    impl PhyctrlDigUsr {
        #[doc = "RXTXPHY selection."]
        #[inline(always)]
        pub const fn rxtxphy(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RXTXPHY selection."]
        #[inline(always)]
        pub fn set_rxtxphy(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for PhyctrlDigUsr {
        #[inline(always)]
        fn default() -> PhyctrlDigUsr {
            PhyctrlDigUsr(0)
        }
    }
    impl core::fmt::Debug for PhyctrlDigUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyctrlDigUsr")
                .field("rxtxphy", &self.rxtxphy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyctrlDigUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PhyctrlDigUsr {{ rxtxphy: {=u8:?} }}", self.rxtxphy())
        }
    }
    #[doc = "RADIO_FSM_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RadioFsmUsr(pub u32);
    impl RadioFsmUsr {
        #[doc = "CBP calibration enable bit."]
        #[inline(always)]
        pub const fn en_calib_cbp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CBP calibration enable bit."]
        #[inline(always)]
        pub fn set_en_calib_cbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SYNTH calibration enable bit."]
        #[inline(always)]
        pub const fn en_calib_synth(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SYNTH calibration enable bit."]
        #[inline(always)]
        pub fn set_en_calib_synth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PA Power coefficient."]
        #[inline(always)]
        pub const fn pa_power(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "PA Power coefficient."]
        #[inline(always)]
        pub fn set_pa_power(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
    }
    impl Default for RadioFsmUsr {
        #[inline(always)]
        fn default() -> RadioFsmUsr {
            RadioFsmUsr(0)
        }
    }
    impl core::fmt::Debug for RadioFsmUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RadioFsmUsr")
                .field("en_calib_cbp", &self.en_calib_cbp())
                .field("en_calib_synth", &self.en_calib_synth())
                .field("pa_power", &self.pa_power())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RadioFsmUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RadioFsmUsr {{ en_calib_cbp: {=bool:?}, en_calib_synth: {=bool:?}, pa_power: {=u8:?} }}",
                self.en_calib_cbp(),
                self.en_calib_synth(),
                self.pa_power()
            )
        }
    }
    #[doc = "RSSI0_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rssi0DigOut(pub u32);
    impl Rssi0DigOut {
        #[doc = "Measure of the received signal strength."]
        #[inline(always)]
        pub const fn rssi_meas_out_7_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Measure of the received signal strength."]
        #[inline(always)]
        pub fn set_rssi_meas_out_7_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rssi0DigOut {
        #[inline(always)]
        fn default() -> Rssi0DigOut {
            Rssi0DigOut(0)
        }
    }
    impl core::fmt::Debug for Rssi0DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rssi0DigOut")
                .field("rssi_meas_out_7_0", &self.rssi_meas_out_7_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rssi0DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rssi0DigOut {{ rssi_meas_out_7_0: {=u8:?} }}",
                self.rssi_meas_out_7_0()
            )
        }
    }
    #[doc = "RSSI1_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rssi1DigOut(pub u32);
    impl Rssi1DigOut {
        #[doc = "Measure of the received signal strength."]
        #[inline(always)]
        pub const fn rssi_meas_out_15_8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Measure of the received signal strength."]
        #[inline(always)]
        pub fn set_rssi_meas_out_15_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rssi1DigOut {
        #[inline(always)]
        fn default() -> Rssi1DigOut {
            Rssi1DigOut(0)
        }
    }
    impl core::fmt::Debug for Rssi1DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rssi1DigOut")
                .field("rssi_meas_out_15_8", &self.rssi_meas_out_15_8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rssi1DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rssi1DigOut {{ rssi_meas_out_15_8: {=u8:?} }}",
                self.rssi_meas_out_15_8()
            )
        }
    }
    #[doc = "RXADC_ANA_USR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxadcAnaUsr(pub u32);
    impl RxadcAnaUsr {
        #[doc = "ADC loop delay control bits for I channel to apply when SW overload is enabled."]
        #[inline(always)]
        pub const fn rfd_rxadc_delaytrim_i(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "ADC loop delay control bits for I channel to apply when SW overload is enabled."]
        #[inline(always)]
        pub fn set_rfd_rxadc_delaytrim_i(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "ADC loop delay control bits for Q channel to apply when SW overload is enabled."]
        #[inline(always)]
        pub const fn rfd_rxadc_delaytrim_q(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "ADC loop delay control bits for Q channel to apply when SW overload is enabled."]
        #[inline(always)]
        pub fn set_rfd_rxadc_delaytrim_q(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "Enable the SW overload on RXADX delay trimming."]
        #[inline(always)]
        pub const fn rxadc_delaytrim_i_tst_sel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the SW overload on RXADX delay trimming."]
        #[inline(always)]
        pub fn set_rxadc_delaytrim_i_tst_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable the SW overload on RXADX delay trimming."]
        #[inline(always)]
        pub const fn rxadc_delaytrim_q_tst_sel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the SW overload on RXADX delay trimming."]
        #[inline(always)]
        pub fn set_rxadc_delaytrim_q_tst_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for RxadcAnaUsr {
        #[inline(always)]
        fn default() -> RxadcAnaUsr {
            RxadcAnaUsr(0)
        }
    }
    impl core::fmt::Debug for RxadcAnaUsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxadcAnaUsr")
                .field("rfd_rxadc_delaytrim_i", &self.rfd_rxadc_delaytrim_i())
                .field("rfd_rxadc_delaytrim_q", &self.rfd_rxadc_delaytrim_q())
                .field("rxadc_delaytrim_i_tst_sel", &self.rxadc_delaytrim_i_tst_sel())
                .field("rxadc_delaytrim_q_tst_sel", &self.rxadc_delaytrim_q_tst_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxadcAnaUsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RxadcAnaUsr {{ rfd_rxadc_delaytrim_i: {=u8:?}, rfd_rxadc_delaytrim_q: {=u8:?}, rxadc_delaytrim_i_tst_sel: {=bool:?}, rxadc_delaytrim_q_tst_sel: {=bool:?} }}" , self . rfd_rxadc_delaytrim_i () , self . rfd_rxadc_delaytrim_q () , self . rxadc_delaytrim_i_tst_sel () , self . rxadc_delaytrim_q_tst_sel ())
        }
    }
    #[doc = "RXADC_HW_TRIM_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxadcHwTrimOut(pub u32);
    impl RxadcHwTrimOut {
        #[doc = "control bits of the RX ADC loop delay for I channel (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub const fn hw_rxadc_delaytrim_i(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "control bits of the RX ADC loop delay for I channel (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub fn set_hw_rxadc_delaytrim_i(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "control bits of the RX ADC loop delay for Q channel (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub const fn hw_rxadc_delaytrim_q(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "control bits of the RX ADC loop delay for Q channel (provided by the HW trimming, automatically loaded on POR)."]
        #[inline(always)]
        pub fn set_hw_rxadc_delaytrim_q(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
    }
    impl Default for RxadcHwTrimOut {
        #[inline(always)]
        fn default() -> RxadcHwTrimOut {
            RxadcHwTrimOut(0)
        }
    }
    impl core::fmt::Debug for RxadcHwTrimOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxadcHwTrimOut")
                .field("hw_rxadc_delaytrim_i", &self.hw_rxadc_delaytrim_i())
                .field("hw_rxadc_delaytrim_q", &self.hw_rxadc_delaytrim_q())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxadcHwTrimOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RxadcHwTrimOut {{ hw_rxadc_delaytrim_i: {=u8:?}, hw_rxadc_delaytrim_q: {=u8:?} }}",
                self.hw_rxadc_delaytrim_i(),
                self.hw_rxadc_delaytrim_q()
            )
        }
    }
    #[doc = "SYNTHCAL0_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal0DigEng(pub u32);
    impl Synthcal0DigEng {
        #[doc = "for Debug purpose."]
        #[inline(always)]
        pub const fn synthcal_debug_bus_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "for Debug purpose."]
        #[inline(always)]
        pub fn set_synthcal_debug_bus_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Define the frequency applied on the PLL during calibration phase."]
        #[inline(always)]
        pub const fn synth_if_freq_cal(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Define the frequency applied on the PLL during calibration phase."]
        #[inline(always)]
        pub fn set_synth_if_freq_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Synthcal0DigEng {
        #[inline(always)]
        fn default() -> Synthcal0DigEng {
            Synthcal0DigEng(0)
        }
    }
    impl core::fmt::Debug for Synthcal0DigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal0DigEng")
                .field("synthcal_debug_bus_sel", &self.synthcal_debug_bus_sel())
                .field("synth_if_freq_cal", &self.synth_if_freq_cal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal0DigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal0DigEng {{ synthcal_debug_bus_sel: {=u8:?}, synth_if_freq_cal: {=u8:?} }}",
                self.synthcal_debug_bus_sel(),
                self.synth_if_freq_cal()
            )
        }
    }
    #[doc = "SYNTHCAL0_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal0DigOut(pub u32);
    impl Synthcal0DigOut {
        #[doc = "VCO CALAMP value."]
        #[inline(always)]
        pub const fn vco_calamp_out_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "VCO CALAMP value."]
        #[inline(always)]
        pub fn set_vco_calamp_out_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Synthcal0DigOut {
        #[inline(always)]
        fn default() -> Synthcal0DigOut {
            Synthcal0DigOut(0)
        }
    }
    impl core::fmt::Debug for Synthcal0DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal0DigOut")
                .field("vco_calamp_out_6_0", &self.vco_calamp_out_6_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal0DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal0DigOut {{ vco_calamp_out_6_0: {=u8:?} }}",
                self.vco_calamp_out_6_0()
            )
        }
    }
    #[doc = "SYNTHCAL1_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal1DigOut(pub u32);
    impl Synthcal1DigOut {
        #[doc = "VCO CALAMP value."]
        #[inline(always)]
        pub const fn vco_calamp_out_10_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "VCO CALAMP value."]
        #[inline(always)]
        pub fn set_vco_calamp_out_10_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Synthcal1DigOut {
        #[inline(always)]
        fn default() -> Synthcal1DigOut {
            Synthcal1DigOut(0)
        }
    }
    impl core::fmt::Debug for Synthcal1DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal1DigOut")
                .field("vco_calamp_out_10_7", &self.vco_calamp_out_10_7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal1DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal1DigOut {{ vco_calamp_out_10_7: {=u8:?} }}",
                self.vco_calamp_out_10_7()
            )
        }
    }
    #[doc = "SYNTHCAL2_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal2DigOut(pub u32);
    impl Synthcal2DigOut {
        #[doc = "VCO CALFREQ value."]
        #[inline(always)]
        pub const fn vco_calfreq_out(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "VCO CALFREQ value."]
        #[inline(always)]
        pub fn set_vco_calfreq_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Synthcal2DigOut {
        #[inline(always)]
        fn default() -> Synthcal2DigOut {
            Synthcal2DigOut(0)
        }
    }
    impl core::fmt::Debug for Synthcal2DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal2DigOut")
                .field("vco_calfreq_out", &self.vco_calfreq_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal2DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal2DigOut {{ vco_calfreq_out: {=u8:?} }}",
                self.vco_calfreq_out()
            )
        }
    }
    #[doc = "SYNTHCAL3_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal3DigOut(pub u32);
    impl Synthcal3DigOut {
        #[doc = "Calibration debug bus."]
        #[inline(always)]
        pub const fn synthcal_debug_bus(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Calibration debug bus."]
        #[inline(always)]
        pub fn set_synthcal_debug_bus(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Synthcal3DigOut {
        #[inline(always)]
        fn default() -> Synthcal3DigOut {
            Synthcal3DigOut(0)
        }
    }
    impl core::fmt::Debug for Synthcal3DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal3DigOut")
                .field("synthcal_debug_bus", &self.synthcal_debug_bus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal3DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal3DigOut {{ synthcal_debug_bus: {=u8:?} }}",
                self.synthcal_debug_bus()
            )
        }
    }
    #[doc = "SYNTHCAL4_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal4DigOut(pub u32);
    impl Synthcal4DigOut {
        #[doc = "Calibration word."]
        #[inline(always)]
        pub const fn mod_ref_dac_word_out(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Calibration word."]
        #[inline(always)]
        pub fn set_mod_ref_dac_word_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Synthcal4DigOut {
        #[inline(always)]
        fn default() -> Synthcal4DigOut {
            Synthcal4DigOut(0)
        }
    }
    impl core::fmt::Debug for Synthcal4DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal4DigOut")
                .field("mod_ref_dac_word_out", &self.mod_ref_dac_word_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal4DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal4DigOut {{ mod_ref_dac_word_out: {=u8:?} }}",
                self.mod_ref_dac_word_out()
            )
        }
    }
    #[doc = "SYNTHCAL5_DIG_OUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synthcal5DigOut(pub u32);
    impl Synthcal5DigOut {
        #[doc = "CBP Calibration word."]
        #[inline(always)]
        pub const fn cbp_calib_word(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CBP Calibration word."]
        #[inline(always)]
        pub fn set_cbp_calib_word(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Synthcal5DigOut {
        #[inline(always)]
        fn default() -> Synthcal5DigOut {
            Synthcal5DigOut(0)
        }
    }
    impl core::fmt::Debug for Synthcal5DigOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synthcal5DigOut")
                .field("cbp_calib_word", &self.cbp_calib_word())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synthcal5DigOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synthcal5DigOut {{ cbp_calib_word: {=u8:?} }}",
                self.cbp_calib_word()
            )
        }
    }
    #[doc = "VIT_CONF_DIG_ENG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VitConfDigEng(pub u32);
    impl VitConfDigEng {
        #[doc = "Viterbi enable."]
        #[inline(always)]
        pub const fn vit_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Viterbi enable."]
        #[inline(always)]
        pub fn set_vit_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "spare."]
        #[inline(always)]
        pub const fn spare(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[doc = "spare."]
        #[inline(always)]
        pub fn set_spare(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
    }
    impl Default for VitConfDigEng {
        #[inline(always)]
        fn default() -> VitConfDigEng {
            VitConfDigEng(0)
        }
    }
    impl core::fmt::Debug for VitConfDigEng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VitConfDigEng")
                .field("vit_en", &self.vit_en())
                .field("spare", &self.spare())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VitConfDigEng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VitConfDigEng {{ vit_en: {=bool:?}, spare: {=u8:?} }}",
                self.vit_en(),
                self.spare()
            )
        }
    }
}
