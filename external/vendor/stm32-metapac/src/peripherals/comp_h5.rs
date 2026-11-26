#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp {
    ptr: *mut u8,
}
unsafe impl Send for Comp {}
unsafe impl Sync for Comp {}
impl Comp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Comparator status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[inline(always)]
    pub const fn icfr(self) -> crate::common::Reg<regs::Icfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Comparator configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Comparator configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Comparator configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP-Channel1."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP-Channel1."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4-V<sub>REF_COMP</sub>, 1/2-V<sub>REF_COMP</sub> and 1/4-V<sub>REF_COMP</sub> levels, respectively."]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4-V<sub>REF_COMP</sub>, 1/2-V<sub>REF_COMP</sub> and 1/4-V<sub>REF_COMP</sub> levels, respectively."]
        #[inline(always)]
        pub fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels."]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels."]
        #[inline(always)]
        pub fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
        #[inline(always)]
        pub const fn polarity(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
        #[inline(always)]
        pub fn set_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
        #[inline(always)]
        pub const fn iten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
        #[inline(always)]
        pub fn set_iten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::Pwrmode {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pwrmode::from_bits(val as u8)
        }
        #[doc = "Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
        #[inline(always)]
        pub fn set_pwrmode(&mut self, val: super::vals::Pwrmode) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table-146: COMP1 inverting input assignment for more details."]
        #[inline(always)]
        pub const fn inmsel(&self) -> super::vals::Inmsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Inmsel::from_bits(val as u8)
        }
        #[doc = "COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table-146: COMP1 inverting input assignment for more details."]
        #[inline(always)]
        pub fn set_inmsel(&mut self, val: super::vals::Inmsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table-145: COMP1 noninverting input assignment for more details."]
        #[inline(always)]
        pub const fn inpsel1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table-145: COMP1 noninverting input assignment for more details."]
        #[inline(always)]
        pub fn set_inpsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table-145: COMP1 noninverting input assignment for more details."]
        #[inline(always)]
        pub const fn inpsel2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table-145: COMP1 noninverting input assignment for more details."]
        #[inline(always)]
        pub fn set_inpsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved."]
        #[inline(always)]
        pub const fn blanking(&self) -> super::vals::Blanking {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Blanking::from_bits(val as u8)
        }
        #[doc = "COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved."]
        #[inline(always)]
        pub fn set_blanking(&mut self, val: super::vals::Blanking) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("en", &self.en())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("polarity", &self.polarity())
                .field("iten", &self.iten())
                .field("hyst", &self.hyst())
                .field("pwrmode", &self.pwrmode())
                .field("inmsel", &self.inmsel())
                .field("inpsel1", &self.inpsel1())
                .field("inpsel2", &self.inpsel2())
                .field("blanking", &self.blanking())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ en: {=bool:?}, brgen: {=bool:?}, scalen: {=bool:?}, polarity: {=bool:?}, iten: {=bool:?}, hyst: {:?}, pwrmode: {:?}, inmsel: {:?}, inpsel1: {=bool:?}, inpsel2: {=bool:?}, blanking: {:?}, lock: {=bool:?} }}" , self . en () , self . brgen () , self . scalen () , self . polarity () , self . iten () , self . hyst () , self . pwrmode () , self . inmsel () , self . inpsel1 () , self . inpsel2 () , self . blanking () , self . lock ())
        }
    }
    #[doc = "Comparator configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table-145: COMP1 noninverting input assignment for more details."]
        #[inline(always)]
        pub const fn inpsel0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table-145: COMP1 noninverting input assignment for more details."]
        #[inline(always)]
        pub fn set_inpsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("inpsel0", &self.inpsel0())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ inpsel0: {=bool:?}, lock: {=bool:?} }}",
                self.inpsel0(),
                self.lock()
            )
        }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icfr(pub u32);
    impl Icfr {
        #[doc = "Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Icfr {
        #[inline(always)]
        fn default() -> Icfr {
            Icfr(0)
        }
    }
    impl core::fmt::Debug for Icfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icfr").field("ccif[0]", &self.ccif(0usize)).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Icfr {{ ccif[0]: {=bool:?} }}", self.ccif(0usize))
        }
    }
    #[doc = "Comparator status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect."]
        #[inline(always)]
        pub const fn cval(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect."]
        #[inline(always)]
        pub fn set_cval(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register."]
        #[inline(always)]
        pub const fn cif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register."]
        #[inline(always)]
        pub fn set_cif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("cval[0]", &self.cval(0usize))
                .field("cif[0]", &self.cif(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ cval[0]: {=bool:?}, cif[0]: {=bool:?} }}",
                self.cval(0usize),
                self.cif(0usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Blanking {
        NO_BLANKING = 0x0,
        TIM1OC5 = 0x01,
        TIM2OC3 = 0x02,
        TIM3OC3 = 0x03,
        TIM3OC4 = 0x04,
        LPTIM1CH2 = 0x05,
        LPTIM2CH2 = 0x06,
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
    impl Blanking {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Blanking {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Blanking {
        #[inline(always)]
        fn from(val: u8) -> Blanking {
            Blanking::from_bits(val)
        }
    }
    impl From<Blanking> for u8 {
        #[inline(always)]
        fn from(val: Blanking) -> u8 {
            Blanking::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hyst {
        NONE = 0x0,
        LOW = 0x01,
        MEDIUM = 0x02,
        HIGH = 0x03,
    }
    impl Hyst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hyst {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hyst {
        #[inline(always)]
        fn from(val: u8) -> Hyst {
            Hyst::from_bits(val)
        }
    }
    impl From<Hyst> for u8 {
        #[inline(always)]
        fn from(val: Hyst) -> u8 {
            Hyst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Inmsel {
        VREF_1OVER4 = 0x0,
        VREF_1OVER2 = 0x01,
        VREF_3OVER4 = 0x02,
        VREF = 0x03,
        DAC1OUT1 = 0x04,
        INM1 = 0x05,
        INM2 = 0x06,
        INM3 = 0x07,
        VSENSE = 0x08,
        VBAT_1OVER4 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Inmsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Inmsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Inmsel {
        #[inline(always)]
        fn from(val: u8) -> Inmsel {
            Inmsel::from_bits(val)
        }
    }
    impl From<Inmsel> for u8 {
        #[inline(always)]
        fn from(val: Inmsel) -> u8 {
            Inmsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwrmode {
        #[doc = "High speed / full power"]
        HIGH = 0x0,
        #[doc = "Medium speed / medium power"]
        MEDIUM = 0x01,
        #[doc = "Medium speed / medium power"]
        MEDIUM_EITHER = 0x02,
        #[doc = "Ultra low power / ultra-low-power"]
        LOW = 0x03,
    }
    impl Pwrmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwrmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwrmode {
        #[inline(always)]
        fn from(val: u8) -> Pwrmode {
            Pwrmode::from_bits(val)
        }
    }
    impl From<Pwrmode> for u8 {
        #[inline(always)]
        fn from(val: Pwrmode) -> u8 {
            Pwrmode::to_bits(val)
        }
    }
}
