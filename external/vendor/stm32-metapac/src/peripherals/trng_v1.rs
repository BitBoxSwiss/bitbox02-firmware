#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng {
    ptr: *mut u8,
}
unsafe impl Send for Trng {}
unsafe impl Sync for Trng {}
impl Trng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CR register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SR register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "VAL register."]
    #[inline(always)]
    pub const fn val(self) -> crate::common::Reg<regs::Val, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "OSCS_CR register."]
    #[inline(always)]
    pub const fn oscs_cr(self) -> crate::common::Reg<regs::OscsCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "POSTP_CR register."]
    #[inline(always)]
    pub const fn postp_cr(self) -> crate::common::Reg<regs::PostpCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "POSTP_SR register."]
    #[inline(always)]
    pub const fn postp_sr(self) -> crate::common::Reg<regs::PostpSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "DEFKEY0 register."]
    #[inline(always)]
    pub const fn defkey0(self) -> crate::common::Reg<regs::Defkey0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DEFKEY1 register."]
    #[inline(always)]
    pub const fn defkey1(self) -> crate::common::Reg<regs::Defkey1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DEFKEY2 register."]
    #[inline(always)]
    pub const fn defkey2(self) -> crate::common::Reg<regs::Defkey2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DEFKEY3 register."]
    #[inline(always)]
    pub const fn defkey3(self) -> crate::common::Reg<regs::Defkey3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "HEALTH_CR register."]
    #[inline(always)]
    pub const fn health_cr(self) -> crate::common::Reg<regs::HealthCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "HEALTH_OSC1_CR register."]
    #[inline(always)]
    pub const fn health_osc1_cr(self) -> crate::common::Reg<regs::HealthOsc1Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "HEALTH_OSC2_CR register."]
    #[inline(always)]
    pub const fn health_osc2_cr(self) -> crate::common::Reg<regs::HealthOsc2Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "HEALTH_OSC3_CR register."]
    #[inline(always)]
    pub const fn health_osc3_cr(self) -> crate::common::Reg<regs::HealthOsc3Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "HEALTH_OSC1_SR register."]
    #[inline(always)]
    pub const fn health_osc1_sr(self) -> crate::common::Reg<regs::HealthOsc1Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "HEALTH_OSC2_SR register."]
    #[inline(always)]
    pub const fn health_osc2_sr(self) -> crate::common::Reg<regs::HealthOsc2Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "HEALTH_OSC3_SR register."]
    #[inline(always)]
    pub const fn health_osc3_sr(self) -> crate::common::Reg<regs::HealthOsc3Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "IRQ_CR register."]
    #[inline(always)]
    pub const fn irq_cr(self) -> crate::common::Reg<regs::IrqCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "IRQ_SR register."]
    #[inline(always)]
    pub const fn irq_sr(self) -> crate::common::Reg<regs::IrqSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
}
pub mod regs {
    #[doc = "CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written."]
        #[inline(always)]
        pub const fn disable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written."]
        #[inline(always)]
        pub fn set_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset."]
        #[inline(always)]
        pub const fn clr_revclk_flag(&self) -> super::vals::ClrRevclkFlag {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::ClrRevclkFlag::from_bits(val as u8)
        }
        #[doc = "Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset."]
        #[inline(always)]
        pub fn set_clr_revclk_flag(&mut self, val: super::vals::ClrRevclkFlag) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset."]
        #[inline(always)]
        pub const fn rst_health_flags(&self) -> super::vals::RstHealthFlags {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::RstHealthFlags::from_bits(val as u8)
        }
        #[doc = "Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset."]
        #[inline(always)]
        pub fn set_rst_health_flags(&mut self, val: super::vals::RstHealthFlags) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Sampling Clock Enable Divider. CLKDIV\\[15:0\\]
control the sampling clock enable divider, dividing by a factor equal to CLKDIV\\[15:0\\]
+ 1, values being in the range of 1 to 65536."]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0xffff;
            val as u16
        }
        #[doc = "Sampling Clock Enable Divider. CLKDIV\\[15:0\\]
control the sampling clock enable divider, dividing by a factor equal to CLKDIV\\[15:0\\]
+ 1, values being in the range of 1 to 65536."]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("disable", &self.disable())
                .field("clr_revclk_flag", &self.clr_revclk_flag())
                .field("rst_health_flags", &self.rst_health_flags())
                .field("clkdiv", &self.clkdiv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ disable: {=bool:?}, clr_revclk_flag: {:?}, rst_health_flags: {:?}, clkdiv: {=u16:?} }}",
                self.disable(),
                self.clr_revclk_flag(),
                self.rst_health_flags(),
                self.clkdiv()
            )
        }
    }
    #[doc = "DEFKEY0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Defkey0(pub u32);
    impl Defkey0 {
        #[doc = "Bits 31 to 0 of AES 128-bit Default Key."]
        #[inline(always)]
        pub const fn defkey0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bits 31 to 0 of AES 128-bit Default Key."]
        #[inline(always)]
        pub fn set_defkey0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Defkey0 {
        #[inline(always)]
        fn default() -> Defkey0 {
            Defkey0(0)
        }
    }
    impl core::fmt::Debug for Defkey0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Defkey0").field("defkey0", &self.defkey0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Defkey0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Defkey0 {{ defkey0: {=u32:?} }}", self.defkey0())
        }
    }
    #[doc = "DEFKEY1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Defkey1(pub u32);
    impl Defkey1 {
        #[doc = "Bits 63 to 31 of AES 128-bit Default Key."]
        #[inline(always)]
        pub const fn defkey1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bits 63 to 31 of AES 128-bit Default Key."]
        #[inline(always)]
        pub fn set_defkey1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Defkey1 {
        #[inline(always)]
        fn default() -> Defkey1 {
            Defkey1(0)
        }
    }
    impl core::fmt::Debug for Defkey1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Defkey1").field("defkey1", &self.defkey1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Defkey1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Defkey1 {{ defkey1: {=u32:?} }}", self.defkey1())
        }
    }
    #[doc = "DEFKEY2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Defkey2(pub u32);
    impl Defkey2 {
        #[doc = "Bits 95 to 64 of AES 128-bit Default Key."]
        #[inline(always)]
        pub const fn defkey2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bits 95 to 64 of AES 128-bit Default Key."]
        #[inline(always)]
        pub fn set_defkey2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Defkey2 {
        #[inline(always)]
        fn default() -> Defkey2 {
            Defkey2(0)
        }
    }
    impl core::fmt::Debug for Defkey2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Defkey2").field("defkey2", &self.defkey2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Defkey2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Defkey2 {{ defkey2: {=u32:?} }}", self.defkey2())
        }
    }
    #[doc = "DEFKEY3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Defkey3(pub u32);
    impl Defkey3 {
        #[doc = "Bits 127 to 96 of AES 128-bit Default Key."]
        #[inline(always)]
        pub const fn defkey3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bits 127 to 96 of AES 128-bit Default Key."]
        #[inline(always)]
        pub fn set_defkey3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Defkey3 {
        #[inline(always)]
        fn default() -> Defkey3 {
            Defkey3(0)
        }
    }
    impl core::fmt::Debug for Defkey3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Defkey3").field("defkey3", &self.defkey3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Defkey3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Defkey3 {{ defkey3: {=u32:?} }}", self.defkey3())
        }
    }
    #[doc = "HEALTH_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthCr(pub u32);
    impl HealthCr {
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn repet_cutoff(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_repet_cutoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn adap_cutoff(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_adap_cutoff(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration."]
        #[inline(always)]
        pub const fn iter_adap(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration."]
        #[inline(always)]
        pub fn set_iter_adap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for HealthCr {
        #[inline(always)]
        fn default() -> HealthCr {
            HealthCr(0)
        }
    }
    impl core::fmt::Debug for HealthCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthCr")
                .field("repet_cutoff", &self.repet_cutoff())
                .field("adap_cutoff", &self.adap_cutoff())
                .field("iter_adap", &self.iter_adap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HealthCr {{ repet_cutoff: {=u8:?}, adap_cutoff: {=u16:?}, iter_adap: {=u8:?} }}",
                self.repet_cutoff(),
                self.adap_cutoff(),
                self.iter_adap()
            )
        }
    }
    #[doc = "HEALTH_OSC1_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthOsc1Cr(pub u32);
    impl HealthOsc1Cr {
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn repet_cutoff_osc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_repet_cutoff_osc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn adap_cutoff_osc1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_adap_cutoff_osc1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for HealthOsc1Cr {
        #[inline(always)]
        fn default() -> HealthOsc1Cr {
            HealthOsc1Cr(0)
        }
    }
    impl core::fmt::Debug for HealthOsc1Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthOsc1Cr")
                .field("repet_cutoff_osc1", &self.repet_cutoff_osc1())
                .field("adap_cutoff_osc1", &self.adap_cutoff_osc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthOsc1Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HealthOsc1Cr {{ repet_cutoff_osc1: {=u8:?}, adap_cutoff_osc1: {=u16:?} }}",
                self.repet_cutoff_osc1(),
                self.adap_cutoff_osc1()
            )
        }
    }
    #[doc = "HEALTH_OSC1_SR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthOsc1Sr(pub u32);
    impl HealthOsc1Sr {
        #[doc = "Repetition error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub const fn to1_repet_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to1_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Adaptive error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub const fn to1_adapt_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to1_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Repetition error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub const fn to2_repet_error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to2_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Adaptive error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub const fn to2_adapt_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to2_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub const fn to3_repet_error(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to3_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Adaptive error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub const fn to3_adapt_error(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to3_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for HealthOsc1Sr {
        #[inline(always)]
        fn default() -> HealthOsc1Sr {
            HealthOsc1Sr(0)
        }
    }
    impl core::fmt::Debug for HealthOsc1Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthOsc1Sr")
                .field("to1_repet_error", &self.to1_repet_error())
                .field("to1_adapt_error", &self.to1_adapt_error())
                .field("to2_repet_error", &self.to2_repet_error())
                .field("to2_adapt_error", &self.to2_adapt_error())
                .field("to3_repet_error", &self.to3_repet_error())
                .field("to3_adapt_error", &self.to3_adapt_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthOsc1Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HealthOsc1Sr {{ to1_repet_error: {=bool:?}, to1_adapt_error: {=bool:?}, to2_repet_error: {=bool:?}, to2_adapt_error: {=bool:?}, to3_repet_error: {=bool:?}, to3_adapt_error: {=bool:?} }}" , self . to1_repet_error () , self . to1_adapt_error () , self . to2_repet_error () , self . to2_adapt_error () , self . to3_repet_error () , self . to3_adapt_error ())
        }
    }
    #[doc = "HEALTH_OSC2_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthOsc2Cr(pub u32);
    impl HealthOsc2Cr {
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn repet_cutoff_osc2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_repet_cutoff_osc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn adap_cutoff_osc2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_adap_cutoff_osc2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for HealthOsc2Cr {
        #[inline(always)]
        fn default() -> HealthOsc2Cr {
            HealthOsc2Cr(0)
        }
    }
    impl core::fmt::Debug for HealthOsc2Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthOsc2Cr")
                .field("repet_cutoff_osc2", &self.repet_cutoff_osc2())
                .field("adap_cutoff_osc2", &self.adap_cutoff_osc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthOsc2Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HealthOsc2Cr {{ repet_cutoff_osc2: {=u8:?}, adap_cutoff_osc2: {=u16:?} }}",
                self.repet_cutoff_osc2(),
                self.adap_cutoff_osc2()
            )
        }
    }
    #[doc = "HEALTH_OSC2_SR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthOsc2Sr(pub u32);
    impl HealthOsc2Sr {
        #[doc = "Repetition error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub const fn to1_repet_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to1_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Adaptive error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub const fn to1_adapt_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to1_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Repetition error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub const fn to2_repet_error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to2_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Adaptive error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub const fn to2_adapt_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to2_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub const fn to3_repet_error(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to3_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Adaptive error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub const fn to3_adapt_error(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to3_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for HealthOsc2Sr {
        #[inline(always)]
        fn default() -> HealthOsc2Sr {
            HealthOsc2Sr(0)
        }
    }
    impl core::fmt::Debug for HealthOsc2Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthOsc2Sr")
                .field("to1_repet_error", &self.to1_repet_error())
                .field("to1_adapt_error", &self.to1_adapt_error())
                .field("to2_repet_error", &self.to2_repet_error())
                .field("to2_adapt_error", &self.to2_adapt_error())
                .field("to3_repet_error", &self.to3_repet_error())
                .field("to3_adapt_error", &self.to3_adapt_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthOsc2Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HealthOsc2Sr {{ to1_repet_error: {=bool:?}, to1_adapt_error: {=bool:?}, to2_repet_error: {=bool:?}, to2_adapt_error: {=bool:?}, to3_repet_error: {=bool:?}, to3_adapt_error: {=bool:?} }}" , self . to1_repet_error () , self . to1_adapt_error () , self . to2_repet_error () , self . to2_adapt_error () , self . to3_repet_error () , self . to3_adapt_error ())
        }
    }
    #[doc = "HEALTH_OSC3_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthOsc3Cr(pub u32);
    impl HealthOsc3Cr {
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn repet_cutoff_osc3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_repet_cutoff_osc3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub const fn adap_cutoff_osc3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG."]
        #[inline(always)]
        pub fn set_adap_cutoff_osc3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for HealthOsc3Cr {
        #[inline(always)]
        fn default() -> HealthOsc3Cr {
            HealthOsc3Cr(0)
        }
    }
    impl core::fmt::Debug for HealthOsc3Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthOsc3Cr")
                .field("repet_cutoff_osc3", &self.repet_cutoff_osc3())
                .field("adap_cutoff_osc3", &self.adap_cutoff_osc3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthOsc3Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HealthOsc3Cr {{ repet_cutoff_osc3: {=u8:?}, adap_cutoff_osc3: {=u16:?} }}",
                self.repet_cutoff_osc3(),
                self.adap_cutoff_osc3()
            )
        }
    }
    #[doc = "HEALTH_OSC3_SR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HealthOsc3Sr(pub u32);
    impl HealthOsc3Sr {
        #[doc = "Repetition error flag of third oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub const fn to1_repet_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of third oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to1_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Adaptive error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub const fn to1_adapt_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of first triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to1_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Repetition error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub const fn to2_repet_error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to2_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Adaptive error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub const fn to2_adapt_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of second triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to2_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub const fn to3_repet_error(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to3_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Adaptive error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub const fn to3_adapt_error(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptive error flag of first oscillator of third triple-oscillator cell."]
        #[inline(always)]
        pub fn set_to3_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for HealthOsc3Sr {
        #[inline(always)]
        fn default() -> HealthOsc3Sr {
            HealthOsc3Sr(0)
        }
    }
    impl core::fmt::Debug for HealthOsc3Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HealthOsc3Sr")
                .field("to1_repet_error", &self.to1_repet_error())
                .field("to1_adapt_error", &self.to1_adapt_error())
                .field("to2_repet_error", &self.to2_repet_error())
                .field("to2_adapt_error", &self.to2_adapt_error())
                .field("to3_repet_error", &self.to3_repet_error())
                .field("to3_adapt_error", &self.to3_adapt_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HealthOsc3Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HealthOsc3Sr {{ to1_repet_error: {=bool:?}, to1_adapt_error: {=bool:?}, to2_repet_error: {=bool:?}, to2_adapt_error: {=bool:?}, to3_repet_error: {=bool:?}, to3_adapt_error: {=bool:?} }}" , self . to1_repet_error () , self . to1_adapt_error () , self . to2_repet_error () , self . to2_adapt_error () , self . to3_repet_error () , self . to3_adapt_error ())
        }
    }
    #[doc = "IRQ_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqCr(pub u32);
    impl IrqCr {
        #[doc = "Enable the interrupt when the output fifo is full of new random."]
        #[inline(always)]
        pub const fn en_ff_full_irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the interrupt when the output fifo is full of new random."]
        #[inline(always)]
        pub fn set_en_ff_full_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable the interrupt when an error is reported by the health tests."]
        #[inline(always)]
        pub const fn en_error_irq(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the interrupt when an error is reported by the health tests."]
        #[inline(always)]
        pub fn set_en_error_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IrqCr {
        #[inline(always)]
        fn default() -> IrqCr {
            IrqCr(0)
        }
    }
    impl core::fmt::Debug for IrqCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqCr")
                .field("en_ff_full_irq", &self.en_ff_full_irq())
                .field("en_error_irq", &self.en_error_irq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IrqCr {{ en_ff_full_irq: {=bool:?}, en_error_irq: {=bool:?} }}",
                self.en_ff_full_irq(),
                self.en_error_irq()
            )
        }
    }
    #[doc = "IRQ_SR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqSr(pub u32);
    impl IrqSr {
        #[doc = "Set to 1 when the output fifo is full of new random. Flag is cleared by writing a 1."]
        #[inline(always)]
        pub const fn ff_full_irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Set to 1 when the output fifo is full of new random. Flag is cleared by writing a 1."]
        #[inline(always)]
        pub fn set_ff_full_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Set to 1 when an error is reported by the health tests. Flag is cleared by writing a 1."]
        #[inline(always)]
        pub const fn error_irq(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Set to 1 when an error is reported by the health tests. Flag is cleared by writing a 1."]
        #[inline(always)]
        pub fn set_error_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IrqSr {
        #[inline(always)]
        fn default() -> IrqSr {
            IrqSr(0)
        }
    }
    impl core::fmt::Debug for IrqSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqSr")
                .field("ff_full_irq", &self.ff_full_irq())
                .field("error_irq", &self.error_irq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IrqSr {{ ff_full_irq: {=bool:?}, error_irq: {=bool:?} }}",
                self.ff_full_irq(),
                self.error_irq()
            )
        }
    }
    #[doc = "OSCS_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OscsCr(pub u32);
    impl OscsCr {
        #[doc = "Power down of individual oscillators in triple-oscillator block number 1."]
        #[inline(always)]
        pub const fn pwrd1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Power down of individual oscillators in triple-oscillator block number 1."]
        #[inline(always)]
        pub fn set_pwrd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "Power down of individual oscillators in triple-oscillator block number 2."]
        #[inline(always)]
        pub const fn pwrd2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Power down of individual oscillators in triple-oscillator block number 2."]
        #[inline(always)]
        pub fn set_pwrd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Power down of individual oscillators in triple-oscillator block number 3."]
        #[inline(always)]
        pub const fn pwrd3(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[doc = "Power down of individual oscillators in triple-oscillator block number 3."]
        #[inline(always)]
        pub fn set_pwrd3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[doc = "When set, selection of resynchronized output of oscillators."]
        #[inline(always)]
        pub const fn sync_oscs(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "When set, selection of resynchronized output of oscillators."]
        #[inline(always)]
        pub fn set_sync_oscs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OscsCr {
        #[inline(always)]
        fn default() -> OscsCr {
            OscsCr(0)
        }
    }
    impl core::fmt::Debug for OscsCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OscsCr")
                .field("pwrd1", &self.pwrd1())
                .field("pwrd2", &self.pwrd2())
                .field("pwrd3", &self.pwrd3())
                .field("sync_oscs", &self.sync_oscs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OscsCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OscsCr {{ pwrd1: {=u8:?}, pwrd2: {=u8:?}, pwrd3: {=u8:?}, sync_oscs: {=bool:?} }}",
                self.pwrd1(),
                self.pwrd2(),
                self.pwrd3(),
                self.sync_oscs()
            )
        }
    }
    #[doc = "POSTP_CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PostpCr(pub u32);
    impl PostpCr {
        #[doc = "Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests."]
        #[inline(always)]
        pub const fn aes_reset(&self) -> super::vals::AesReset {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::AesReset::from_bits(val as u8)
        }
        #[doc = "Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests."]
        #[inline(always)]
        pub fn set_aes_reset(&mut self, val: super::vals::AesReset) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed."]
        #[inline(always)]
        pub const fn nb_loop_aes(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed."]
        #[inline(always)]
        pub fn set_nb_loop_aes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized."]
        #[inline(always)]
        pub const fn nb_rnd_reinit(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized."]
        #[inline(always)]
        pub fn set_nb_rnd_reinit(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PostpCr {
        #[inline(always)]
        fn default() -> PostpCr {
            PostpCr(0)
        }
    }
    impl core::fmt::Debug for PostpCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PostpCr")
                .field("aes_reset", &self.aes_reset())
                .field("nb_loop_aes", &self.nb_loop_aes())
                .field("nb_rnd_reinit", &self.nb_rnd_reinit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PostpCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PostpCr {{ aes_reset: {:?}, nb_loop_aes: {=u8:?}, nb_rnd_reinit: {=u16:?} }}",
                self.aes_reset(),
                self.nb_loop_aes(),
                self.nb_rnd_reinit()
            )
        }
    }
    #[doc = "POSTP_SR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PostpSr(pub u32);
    impl PostpSr {
        #[doc = "AES Post processing has been fully initialized (key and state) and is ready for generating 128-bit random words."]
        #[inline(always)]
        pub const fn aes_init(&self) -> super::vals::AesInit {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::AesInit::from_bits(val as u8)
        }
        #[doc = "AES Post processing has been fully initialized (key and state) and is ready for generating 128-bit random words."]
        #[inline(always)]
        pub fn set_aes_init(&mut self, val: super::vals::AesInit) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "AES random key has been generated and loaded in AES key register."]
        #[inline(always)]
        pub const fn aes_key_ld(&self) -> super::vals::AesKeyLd {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::AesKeyLd::from_bits(val as u8)
        }
        #[doc = "AES random key has been generated and loaded in AES key register."]
        #[inline(always)]
        pub fn set_aes_key_ld(&mut self, val: super::vals::AesKeyLd) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "AES core is busy, generating a random value."]
        #[inline(always)]
        pub const fn aes_busy(&self) -> super::vals::AesBusy {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::AesBusy::from_bits(val as u8)
        }
        #[doc = "AES core is busy, generating a random value."]
        #[inline(always)]
        pub fn set_aes_busy(&mut self, val: super::vals::AesBusy) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "AES-CMAC health test is completed."]
        #[inline(always)]
        pub const fn aes_health_done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AES-CMAC health test is completed."]
        #[inline(always)]
        pub fn set_aes_health_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Health test error on AES-CMAC sub-keys generation."]
        #[inline(always)]
        pub const fn aes_k12_error(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Health test error on AES-CMAC sub-keys generation."]
        #[inline(always)]
        pub fn set_aes_k12_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Health test error on AES-CMAC output generation."]
        #[inline(always)]
        pub const fn aes_dout_error(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Health test error on AES-CMAC output generation."]
        #[inline(always)]
        pub fn set_aes_dout_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for PostpSr {
        #[inline(always)]
        fn default() -> PostpSr {
            PostpSr(0)
        }
    }
    impl core::fmt::Debug for PostpSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PostpSr")
                .field("aes_init", &self.aes_init())
                .field("aes_key_ld", &self.aes_key_ld())
                .field("aes_busy", &self.aes_busy())
                .field("aes_health_done", &self.aes_health_done())
                .field("aes_k12_error", &self.aes_k12_error())
                .field("aes_dout_error", &self.aes_dout_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PostpSr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PostpSr {{ aes_init: {:?}, aes_key_ld: {:?}, aes_busy: {:?}, aes_health_done: {=bool:?}, aes_k12_error: {=bool:?}, aes_dout_error: {=bool:?} }}" , self . aes_init () , self . aes_key_ld () , self . aes_busy () , self . aes_health_done () , self . aes_k12_error () , self . aes_dout_error ())
        }
    }
    #[doc = "SR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "TRNG is disabled."]
        #[inline(always)]
        pub const fn disabled(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TRNG is disabled."]
        #[inline(always)]
        pub fn set_disabled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "All oscillators of the random source noise have been powered down. This can cause the rising of OEC3 flag."]
        #[inline(always)]
        pub const fn all_oscs_down(&self) -> super::vals::AllOscsDown {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::AllOscsDown::from_bits(val as u8)
        }
        #[doc = "All oscillators of the random source noise have been powered down. This can cause the rising of OEC3 flag."]
        #[inline(always)]
        pub fn set_all_oscs_down(&mut self, val: super::vals::AllOscsDown) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "The internal clock for the RNG core is not revealed."]
        #[inline(always)]
        pub const fn reveal_clk_err(&self) -> super::vals::RevealClkErr {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::RevealClkErr::from_bits(val as u8)
        }
        #[doc = "The internal clock for the RNG core is not revealed."]
        #[inline(always)]
        pub fn set_reveal_clk_err(&mut self, val: super::vals::RevealClkErr) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "The error refers to a fault in the bit sequence detected by the Entropy Monitor. Failed test is given by REPET_ERROR, and ADAPT_ERROR, OSCS_REPET_ERROR and OSCS_ADAPT_ERROR status flags."]
        #[inline(always)]
        pub const fn entropy_err(&self) -> super::vals::EntropyErr {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::EntropyErr::from_bits(val as u8)
        }
        #[doc = "The error refers to a fault in the bit sequence detected by the Entropy Monitor. Failed test is given by REPET_ERROR, and ADAPT_ERROR, OSCS_REPET_ERROR and OSCS_ADAPT_ERROR status flags."]
        #[inline(always)]
        pub fn set_entropy_err(&mut self, val: super::vals::EntropyErr) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "TRNG Value ready At least one 32-bit random value is available in the data FIFO. Note that application must ensure that a random is available in internal FIFO before starting a read otherwise a bus error will be generated."]
        #[inline(always)]
        pub const fn val_ready(&self) -> super::vals::ValReady {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::ValReady::from_bits(val as u8)
        }
        #[doc = "TRNG Value ready At least one 32-bit random value is available in the data FIFO. Note that application must ensure that a random is available in internal FIFO before starting a read otherwise a bus error will be generated."]
        #[inline(always)]
        pub fn set_val_ready(&mut self, val: super::vals::ValReady) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Indicates whether random data FIFO is full."]
        #[inline(always)]
        pub const fn fifo_full(&self) -> super::vals::FifoFull {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::FifoFull::from_bits(val as u8)
        }
        #[doc = "Indicates whether random data FIFO is full."]
        #[inline(always)]
        pub fn set_fifo_full(&mut self, val: super::vals::FifoFull) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "First run of noise source health test is completed."]
        #[inline(always)]
        pub const fn src_health_done(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "First run of noise source health test is completed."]
        #[inline(always)]
        pub fn set_src_health_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Noise source Repetition health test error."]
        #[inline(always)]
        pub const fn repet_error(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Noise source Repetition health test error."]
        #[inline(always)]
        pub fn set_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Noise source Adaptive 1024 health test error."]
        #[inline(always)]
        pub const fn adapt_error(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Noise source Adaptive 1024 health test error."]
        #[inline(always)]
        pub fn set_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "First run of source health tests of individual oscillators composing the noise source are completed.Reserved."]
        #[inline(always)]
        pub const fn oscs_health_done(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "First run of source health tests of individual oscillators composing the noise source are completed.Reserved."]
        #[inline(always)]
        pub fn set_oscs_health_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Logical OR of repetition health test errors of individual oscillators composing the noise source."]
        #[inline(always)]
        pub const fn oscs_repet_error(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Logical OR of repetition health test errors of individual oscillators composing the noise source."]
        #[inline(always)]
        pub fn set_oscs_repet_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Logical OR of adaptive health test errors of individual oscillators composing the noise source."]
        #[inline(always)]
        pub const fn oscs_adapt_error(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Logical OR of adaptive health test errors of individual oscillators composing the noise source."]
        #[inline(always)]
        pub fn set_oscs_adapt_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("disabled", &self.disabled())
                .field("all_oscs_down", &self.all_oscs_down())
                .field("reveal_clk_err", &self.reveal_clk_err())
                .field("entropy_err", &self.entropy_err())
                .field("val_ready", &self.val_ready())
                .field("fifo_full", &self.fifo_full())
                .field("src_health_done", &self.src_health_done())
                .field("repet_error", &self.repet_error())
                .field("adapt_error", &self.adapt_error())
                .field("oscs_health_done", &self.oscs_health_done())
                .field("oscs_repet_error", &self.oscs_repet_error())
                .field("oscs_adapt_error", &self.oscs_adapt_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ disabled: {=bool:?}, all_oscs_down: {:?}, reveal_clk_err: {:?}, entropy_err: {:?}, val_ready: {:?}, fifo_full: {:?}, src_health_done: {=bool:?}, repet_error: {=bool:?}, adapt_error: {=bool:?}, oscs_health_done: {=bool:?}, oscs_repet_error: {=bool:?}, oscs_adapt_error: {=bool:?} }}" , self . disabled () , self . all_oscs_down () , self . reveal_clk_err () , self . entropy_err () , self . val_ready () , self . fifo_full () , self . src_health_done () , self . repet_error () , self . adapt_error () , self . oscs_health_done () , self . oscs_repet_error () , self . oscs_adapt_error ())
        }
    }
    #[doc = "VAL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Val(pub u32);
    impl Val {
        #[doc = "RND_VAL is a 32-bit Random Value. This is the output of the internal four-word FIFO. Note that application must ensure that a random is available in FIFO by ready VAL_READY flag before starting a read otherwise a null value will be returned."]
        #[inline(always)]
        pub const fn rnd_val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RND_VAL is a 32-bit Random Value. This is the output of the internal four-word FIFO. Note that application must ensure that a random is available in FIFO by ready VAL_READY flag before starting a read otherwise a null value will be returned."]
        #[inline(always)]
        pub fn set_rnd_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Val {
        #[inline(always)]
        fn default() -> Val {
            Val(0)
        }
    }
    impl core::fmt::Debug for Val {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Val").field("rnd_val", &self.rnd_val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Val {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Val {{ rnd_val: {=u32:?} }}", self.rnd_val())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AesBusy {
        #[doc = "AES core is idle."]
        B_0X0 = 0x0,
        #[doc = "AES core is busy."]
        B_0X1 = 0x01,
    }
    impl AesBusy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AesBusy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AesBusy {
        #[inline(always)]
        fn from(val: u8) -> AesBusy {
            AesBusy::from_bits(val)
        }
    }
    impl From<AesBusy> for u8 {
        #[inline(always)]
        fn from(val: AesBusy) -> u8 {
            AesBusy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AesInit {
        #[doc = "AES core is not initialized (no key or state set)."]
        B_0X0 = 0x0,
        #[doc = "AES core is fully initialized."]
        B_0X1 = 0x01,
    }
    impl AesInit {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AesInit {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AesInit {
        #[inline(always)]
        fn from(val: u8) -> AesInit {
            AesInit::from_bits(val)
        }
    }
    impl From<AesInit> for u8 {
        #[inline(always)]
        fn from(val: AesInit) -> u8 {
            AesInit::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AesKeyLd {
        #[doc = "AES core is waiting for 128 random bits from the entropy sources for generating its key."]
        B_0X0 = 0x0,
        #[doc = "AES key register has been loaded with a random key."]
        B_0X1 = 0x01,
    }
    impl AesKeyLd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AesKeyLd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AesKeyLd {
        #[inline(always)]
        fn from(val: u8) -> AesKeyLd {
            AesKeyLd::from_bits(val)
        }
    }
    impl From<AesKeyLd> for u8 {
        #[inline(always)]
        fn from(val: AesKeyLd) -> u8 {
            AesKeyLd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AesReset {
        #[doc = "No effect."]
        B_0X0 = 0x0,
        #[doc = "Reset AES core."]
        B_0X1 = 0x01,
    }
    impl AesReset {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AesReset {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AesReset {
        #[inline(always)]
        fn from(val: u8) -> AesReset {
            AesReset::from_bits(val)
        }
    }
    impl From<AesReset> for u8 {
        #[inline(always)]
        fn from(val: AesReset) -> u8 {
            AesReset::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AllOscsDown {
        #[doc = "At least one oscillator is ON."]
        B_0X0 = 0x0,
        #[doc = "All oscillators are down."]
        B_0X1 = 0x01,
    }
    impl AllOscsDown {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AllOscsDown {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AllOscsDown {
        #[inline(always)]
        fn from(val: u8) -> AllOscsDown {
            AllOscsDown::from_bits(val)
        }
    }
    impl From<AllOscsDown> for u8 {
        #[inline(always)]
        fn from(val: AllOscsDown) -> u8 {
            AllOscsDown::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ClrRevclkFlag {
        #[doc = "no reset."]
        B_0X0 = 0x0,
        #[doc = "reset revclk flag."]
        B_0X1 = 0x01,
    }
    impl ClrRevclkFlag {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClrRevclkFlag {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ClrRevclkFlag {
        #[inline(always)]
        fn from(val: u8) -> ClrRevclkFlag {
            ClrRevclkFlag::from_bits(val)
        }
    }
    impl From<ClrRevclkFlag> for u8 {
        #[inline(always)]
        fn from(val: ClrRevclkFlag) -> u8 {
            ClrRevclkFlag::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EntropyErr {
        #[doc = "No fault detected."]
        B_0X0 = 0x0,
        #[doc = "Embedded heath monitor detects an error in bit stream quality."]
        B_0X1 = 0x01,
    }
    impl EntropyErr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EntropyErr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EntropyErr {
        #[inline(always)]
        fn from(val: u8) -> EntropyErr {
            EntropyErr::from_bits(val)
        }
    }
    impl From<EntropyErr> for u8 {
        #[inline(always)]
        fn from(val: EntropyErr) -> u8 {
            EntropyErr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FifoFull {
        #[doc = "FIFO is not full."]
        B_0X0 = 0x0,
        #[doc = "The internal data FIFO is full and four 32-bit random values can be read."]
        B_0X1 = 0x01,
    }
    impl FifoFull {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FifoFull {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FifoFull {
        #[inline(always)]
        fn from(val: u8) -> FifoFull {
            FifoFull::from_bits(val)
        }
    }
    impl From<FifoFull> for u8 {
        #[inline(always)]
        fn from(val: FifoFull) -> u8 {
            FifoFull::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RevealClkErr {
        #[doc = "Internal clock for RNG clock is present."]
        B_0X0 = 0x0,
        #[doc = "Internal RNG clock is not present."]
        B_0X1 = 0x01,
    }
    impl RevealClkErr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RevealClkErr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RevealClkErr {
        #[inline(always)]
        fn from(val: u8) -> RevealClkErr {
            RevealClkErr::from_bits(val)
        }
    }
    impl From<RevealClkErr> for u8 {
        #[inline(always)]
        fn from(val: RevealClkErr) -> u8 {
            RevealClkErr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RstHealthFlags {
        #[doc = "no reset."]
        B_0X0 = 0x0,
        #[doc = "reset health flag."]
        B_0X1 = 0x01,
    }
    impl RstHealthFlags {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RstHealthFlags {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RstHealthFlags {
        #[inline(always)]
        fn from(val: u8) -> RstHealthFlags {
            RstHealthFlags::from_bits(val)
        }
    }
    impl From<RstHealthFlags> for u8 {
        #[inline(always)]
        fn from(val: RstHealthFlags) -> u8 {
            RstHealthFlags::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ValReady {
        #[doc = "No value is ready in FIFO."]
        B_0X0 = 0x0,
        #[doc = "A 32-bit value is available in the internal FIFO."]
        B_0X1 = 0x01,
    }
    impl ValReady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ValReady {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ValReady {
        #[inline(always)]
        fn from(val: u8) -> ValReady {
            ValReady::from_bits(val)
        }
    }
    impl From<ValReady> for u8 {
        #[inline(always)]
        fn from(val: ValReady) -> u8 {
            ValReady::to_bits(val)
        }
    }
}
