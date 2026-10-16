#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital temperature sensor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dts {
    ptr: *mut u8,
}
unsafe impl Send for Dts {}
unsafe impl Sync for Dts {}
impl Dts {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Temperature sensor configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Temperature sensor T0 value register 1."]
    #[inline(always)]
    pub const fn t0valr1(self) -> crate::common::Reg<regs::T0valr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Temperature sensor ramp value register."]
    #[inline(always)]
    pub const fn rampvalr(self) -> crate::common::Reg<regs::Rampvalr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Temperature sensor interrupt threshold register 1."]
    #[inline(always)]
    pub const fn itr1(self) -> crate::common::Reg<regs::Itr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Temperature sensor data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Temperature sensor status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Temperature sensor interrupt enable register."]
    #[inline(always)]
    pub const fn itenr(self) -> crate::common::Reg<regs::Itenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Temperature sensor clear interrupt flag register."]
    #[inline(always)]
    pub const fn icifr(self) -> crate::common::Reg<regs::Icifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Temperature sensor option register."]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Temperature sensor configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
        #[inline(always)]
        pub const fn intrig_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
        #[inline(always)]
        pub fn set_intrig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
        #[inline(always)]
        pub const fn smp_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
        #[inline(always)]
        pub fn set_smp_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
        #[inline(always)]
        pub const fn refclk_sel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
        #[inline(always)]
        pub fn set_refclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
        #[inline(always)]
        pub const fn q_meas_opt(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
        #[inline(always)]
        pub fn set_q_meas_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
        #[inline(always)]
        pub const fn hsref_clk_div(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
        #[inline(always)]
        pub fn set_hsref_clk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
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
                .field("start", &self.start())
                .field("intrig_sel", &self.intrig_sel())
                .field("smp_time", &self.smp_time())
                .field("refclk_sel", &self.refclk_sel())
                .field("q_meas_opt", &self.q_meas_opt())
                .field("hsref_clk_div", &self.hsref_clk_div())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ en: {=bool:?}, start: {=bool:?}, intrig_sel: {=u8:?}, smp_time: {=u8:?}, refclk_sel: {=bool:?}, q_meas_opt: {=bool:?}, hsref_clk_div: {=u8:?} }}" , self . en () , self . start () , self . intrig_sel () , self . smp_time () , self . refclk_sel () , self . q_meas_opt () , self . hsref_clk_div ())
        }
    }
    #[doc = "Temperature sensor data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Value of the counter output value for temperature sensor 1."]
        #[inline(always)]
        pub const fn mfreq(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Value of the counter output value for temperature sensor 1."]
        #[inline(always)]
        pub fn set_mfreq(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("mfreq", &self.mfreq()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ mfreq: {=u16:?} }}", self.mfreq())
        }
    }
    #[doc = "Temperature sensor clear interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icifr(pub u32);
    impl Icifr {
        #[doc = "Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register."]
        #[inline(always)]
        pub const fn citef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register."]
        #[inline(always)]
        pub fn set_citef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register."]
        #[inline(always)]
        pub const fn citlf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register."]
        #[inline(always)]
        pub fn set_citlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register."]
        #[inline(always)]
        pub const fn cithf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register."]
        #[inline(always)]
        pub fn set_cithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register."]
        #[inline(always)]
        pub const fn caitef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register."]
        #[inline(always)]
        pub fn set_caitef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register."]
        #[inline(always)]
        pub const fn caitlf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register."]
        #[inline(always)]
        pub fn set_caitlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register."]
        #[inline(always)]
        pub const fn caithf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register."]
        #[inline(always)]
        pub fn set_caithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Icifr {
        #[inline(always)]
        fn default() -> Icifr {
            Icifr(0)
        }
    }
    impl core::fmt::Debug for Icifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icifr")
                .field("citef", &self.citef())
                .field("citlf", &self.citlf())
                .field("cithf", &self.cithf())
                .field("caitef", &self.caitef())
                .field("caitlf", &self.caitlf())
                .field("caithf", &self.caithf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icifr {{ citef: {=bool:?}, citlf: {=bool:?}, cithf: {=bool:?}, caitef: {=bool:?}, caitlf: {=bool:?}, caithf: {=bool:?} }}" , self . citef () , self . citlf () , self . cithf () , self . caitef () , self . caitlf () , self . caithf ())
        }
    }
    #[doc = "Temperature sensor interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itenr(pub u32);
    impl Itenr {
        #[doc = "Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
        #[inline(always)]
        pub const fn iteen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
        #[inline(always)]
        pub fn set_iteen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
        #[inline(always)]
        pub const fn itlen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
        #[inline(always)]
        pub fn set_itlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
        #[inline(always)]
        pub const fn ithen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
        #[inline(always)]
        pub fn set_ithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
        #[inline(always)]
        pub const fn aiteen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
        #[inline(always)]
        pub fn set_aiteen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)."]
        #[inline(always)]
        pub const fn aitlen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)."]
        #[inline(always)]
        pub fn set_aitlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)."]
        #[inline(always)]
        pub const fn aithen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)."]
        #[inline(always)]
        pub fn set_aithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Itenr {
        #[inline(always)]
        fn default() -> Itenr {
            Itenr(0)
        }
    }
    impl core::fmt::Debug for Itenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itenr")
                .field("iteen", &self.iteen())
                .field("itlen", &self.itlen())
                .field("ithen", &self.ithen())
                .field("aiteen", &self.aiteen())
                .field("aitlen", &self.aitlen())
                .field("aithen", &self.aithen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Itenr {{ iteen: {=bool:?}, itlen: {=bool:?}, ithen: {=bool:?}, aiteen: {=bool:?}, aitlen: {=bool:?}, aithen: {=bool:?} }}" , self . iteen () , self . itlen () , self . ithen () , self . aiteen () , self . aitlen () , self . aithen ())
        }
    }
    #[doc = "Temperature sensor interrupt threshold register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itr1(pub u32);
    impl Itr1 {
        #[doc = "Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
        #[inline(always)]
        pub const fn litthd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
        #[inline(always)]
        pub fn set_litthd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
        #[inline(always)]
        pub const fn hitthd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
        #[inline(always)]
        pub fn set_hitthd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Itr1 {
        #[inline(always)]
        fn default() -> Itr1 {
            Itr1(0)
        }
    }
    impl core::fmt::Debug for Itr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itr1")
                .field("litthd", &self.litthd())
                .field("hitthd", &self.hitthd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itr1 {{ litthd: {=u16:?}, hitthd: {=u16:?} }}",
                self.litthd(),
                self.hitthd()
            )
        }
    }
    #[doc = "Temperature sensor option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Or(pub u32);
    impl Or {
        #[doc = "general purpose option bits."]
        #[inline(always)]
        pub const fn op(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "general purpose option bits."]
        #[inline(always)]
        pub fn set_op(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Or {
        #[inline(always)]
        fn default() -> Or {
            Or(0)
        }
    }
    impl core::fmt::Debug for Or {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Or")
                .field("op[0]", &self.op(0usize))
                .field("op[1]", &self.op(1usize))
                .field("op[2]", &self.op(2usize))
                .field("op[3]", &self.op(3usize))
                .field("op[4]", &self.op(4usize))
                .field("op[5]", &self.op(5usize))
                .field("op[6]", &self.op(6usize))
                .field("op[7]", &self.op(7usize))
                .field("op[8]", &self.op(8usize))
                .field("op[9]", &self.op(9usize))
                .field("op[10]", &self.op(10usize))
                .field("op[11]", &self.op(11usize))
                .field("op[12]", &self.op(12usize))
                .field("op[13]", &self.op(13usize))
                .field("op[14]", &self.op(14usize))
                .field("op[15]", &self.op(15usize))
                .field("op[16]", &self.op(16usize))
                .field("op[17]", &self.op(17usize))
                .field("op[18]", &self.op(18usize))
                .field("op[19]", &self.op(19usize))
                .field("op[20]", &self.op(20usize))
                .field("op[21]", &self.op(21usize))
                .field("op[22]", &self.op(22usize))
                .field("op[23]", &self.op(23usize))
                .field("op[24]", &self.op(24usize))
                .field("op[25]", &self.op(25usize))
                .field("op[26]", &self.op(26usize))
                .field("op[27]", &self.op(27usize))
                .field("op[28]", &self.op(28usize))
                .field("op[29]", &self.op(29usize))
                .field("op[30]", &self.op(30usize))
                .field("op[31]", &self.op(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Or {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Or {{ op[0]: {=bool:?}, op[1]: {=bool:?}, op[2]: {=bool:?}, op[3]: {=bool:?}, op[4]: {=bool:?}, op[5]: {=bool:?}, op[6]: {=bool:?}, op[7]: {=bool:?}, op[8]: {=bool:?}, op[9]: {=bool:?}, op[10]: {=bool:?}, op[11]: {=bool:?}, op[12]: {=bool:?}, op[13]: {=bool:?}, op[14]: {=bool:?}, op[15]: {=bool:?}, op[16]: {=bool:?}, op[17]: {=bool:?}, op[18]: {=bool:?}, op[19]: {=bool:?}, op[20]: {=bool:?}, op[21]: {=bool:?}, op[22]: {=bool:?}, op[23]: {=bool:?}, op[24]: {=bool:?}, op[25]: {=bool:?}, op[26]: {=bool:?}, op[27]: {=bool:?}, op[28]: {=bool:?}, op[29]: {=bool:?}, op[30]: {=bool:?}, op[31]: {=bool:?} }}" , self . op (0usize) , self . op (1usize) , self . op (2usize) , self . op (3usize) , self . op (4usize) , self . op (5usize) , self . op (6usize) , self . op (7usize) , self . op (8usize) , self . op (9usize) , self . op (10usize) , self . op (11usize) , self . op (12usize) , self . op (13usize) , self . op (14usize) , self . op (15usize) , self . op (16usize) , self . op (17usize) , self . op (18usize) , self . op (19usize) , self . op (20usize) , self . op (21usize) , self . op (22usize) , self . op (23usize) , self . op (24usize) , self . op (25usize) , self . op (26usize) , self . op (27usize) , self . op (28usize) , self . op (29usize) , self . op (30usize) , self . op (31usize))
        }
    }
    #[doc = "Temperature sensor ramp value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rampvalr(pub u32);
    impl Rampvalr {
        #[doc = "Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C."]
        #[inline(always)]
        pub const fn ramp_coeff(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C."]
        #[inline(always)]
        pub fn set_ramp_coeff(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rampvalr {
        #[inline(always)]
        fn default() -> Rampvalr {
            Rampvalr(0)
        }
    }
    impl core::fmt::Debug for Rampvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rampvalr")
                .field("ramp_coeff", &self.ramp_coeff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rampvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rampvalr {{ ramp_coeff: {=u16:?} }}", self.ramp_coeff())
        }
    }
    #[doc = "Temperature sensor status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set."]
        #[inline(always)]
        pub const fn itef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set."]
        #[inline(always)]
        pub fn set_itef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set."]
        #[inline(always)]
        pub const fn itlf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set."]
        #[inline(always)]
        pub fn set_itlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set."]
        #[inline(always)]
        pub const fn ithf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set."]
        #[inline(always)]
        pub fn set_ithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set."]
        #[inline(always)]
        pub const fn aitef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set."]
        #[inline(always)]
        pub fn set_aitef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set."]
        #[inline(always)]
        pub const fn aitlf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set."]
        #[inline(always)]
        pub fn set_aitlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set."]
        #[inline(always)]
        pub const fn aithf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set."]
        #[inline(always)]
        pub fn set_aithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing."]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing."]
        #[inline(always)]
        pub fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("itef", &self.itef())
                .field("itlf", &self.itlf())
                .field("ithf", &self.ithf())
                .field("aitef", &self.aitef())
                .field("aitlf", &self.aitlf())
                .field("aithf", &self.aithf())
                .field("rdy", &self.rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ itef: {=bool:?}, itlf: {=bool:?}, ithf: {=bool:?}, aitef: {=bool:?}, aitlf: {=bool:?}, aithf: {=bool:?}, rdy: {=bool:?} }}" , self . itef () , self . itlf () , self . ithf () , self . aitef () , self . aitlf () , self . aithf () , self . rdy ())
        }
    }
    #[doc = "Temperature sensor T0 value register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct T0valr1(pub u32);
    impl T0valr1 {
        #[doc = "Engineering value of the frequency measured at T0 for. temperature sensor 1 This value is expressed in 0.1 kHz."]
        #[inline(always)]
        pub const fn fmt0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Engineering value of the frequency measured at T0 for. temperature sensor 1 This value is expressed in 0.1 kHz."]
        #[inline(always)]
        pub fn set_fmt0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used."]
        #[inline(always)]
        pub const fn t0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used."]
        #[inline(always)]
        pub fn set_t0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for T0valr1 {
        #[inline(always)]
        fn default() -> T0valr1 {
            T0valr1(0)
        }
    }
    impl core::fmt::Debug for T0valr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("T0valr1")
                .field("fmt0", &self.fmt0())
                .field("t0", &self.t0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for T0valr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "T0valr1 {{ fmt0: {=u16:?}, t0: {=u8:?} }}", self.fmt0(), self.t0())
        }
    }
}
