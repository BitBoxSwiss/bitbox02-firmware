///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**ADRDY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR {
    ///0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)
    NotReady = 0,
    ///1: ADC is ready to start conversion
    Ready = 1,
}
impl From<ADRDYR> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` reader - ADRDY
pub type ADRDY_R = crate::BitReader<ADRDYR>;
impl ADRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYR {
        match self.bits {
            false => ADRDYR::NotReady,
            true => ADRDYR::Ready,
        }
    }
    ///ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR::NotReady
    }
    ///ADC is ready to start conversion
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR::Ready
    }
}
/**ADRDY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW {
    ///1: Clear the ADC ready flag
    Clear = 1,
}
impl From<ADRDYW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` writer - ADRDY
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYW>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the ADC ready flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYW::Clear)
    }
}
/**EOSMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR {
    ///0: Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)
    NotAtEnd = 0,
    ///1: End of sampling phase reached
    AtEnd = 1,
}
impl From<EOSMPR> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` reader - EOSMP
pub type EOSMP_R = crate::BitReader<EOSMPR>;
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPR {
        match self.bits {
            false => EOSMPR::NotAtEnd,
            true => EOSMPR::AtEnd,
        }
    }
    ///Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_not_at_end(&self) -> bool {
        *self == EOSMPR::NotAtEnd
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        *self == EOSMPR::AtEnd
    }
}
/**EOSMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW {
    ///1: Clear the sampling phase flag
    Clear = 1,
}
impl From<EOSMPW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` writer - EOSMP
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPW>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the sampling phase flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPW::Clear)
    }
}
/**EOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    ///0: Regular channel conversion not complete (or the flag event was already acknowledged and cleared by software)
    NotComplete = 0,
    ///1: Regular channel conversion complete
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - EOC
pub type EOC_R = crate::BitReader<EOCR>;
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCR {
        match self.bits {
            false => EOCR::NotComplete,
            true => EOCR::Complete,
        }
    }
    ///Regular channel conversion not complete (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    ///Regular channel conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
/**EOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    ///1: Clear the regular channel conversion flag
    Clear = 1,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - EOC
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the regular channel conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
/**EOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR {
    ///0: Regular conversions sequence not complete (or the flag event was already acknowledged and cleared by software)
    NotComplete = 0,
    ///1: Regular conversions sequence complete
    Complete = 1,
}
impl From<EOSR> for bool {
    #[inline(always)]
    fn from(variant: EOSR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` reader - EOS
pub type EOS_R = crate::BitReader<EOSR>;
impl EOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSR {
        match self.bits {
            false => EOSR::NotComplete,
            true => EOSR::Complete,
        }
    }
    ///Regular conversions sequence not complete (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR::NotComplete
    }
    ///Regular conversions sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR::Complete
    }
}
/**EOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW {
    ///1: Clear the regular conversion sequence flag
    Clear = 1,
}
impl From<EOSW> for bool {
    #[inline(always)]
    fn from(variant: EOSW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` writer - EOS
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG, EOSW>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the regular conversion sequence flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSW::Clear)
    }
}
/**OVR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    ///0: No overrun occurred (or the flag event was already acknowledged and cleared by software)
    NoOverrun = 0,
    ///1: Overrun has occurred
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    ///No overrun occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    ///Overrun has occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
/**OVR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    ///1: Clear the overrun flag
    Clear = 1,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - OVR
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the overrun flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
/**Analog watchdog %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R {
    ///0: No analog watchdog x event occurred (or the flag event was already acknowledged and cleared by software)
    NoEvent = 0,
    ///1: Analog watchdog x event occurred
    Event = 1,
}
impl From<AWD1R> for bool {
    #[inline(always)]
    fn from(variant: AWD1R) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD(1-3)` reader - Analog watchdog %s flag
pub type AWD_R = crate::BitReader<AWD1R>;
impl AWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1R {
        match self.bits {
            false => AWD1R::NoEvent,
            true => AWD1R::Event,
        }
    }
    ///No analog watchdog x event occurred (or the flag event was already acknowledged and cleared by software)
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R::NoEvent
    }
    ///Analog watchdog x event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R::Event
    }
}
/**Analog watchdog %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W {
    ///1: Clear the analog watchdog x event flag
    Clear = 1,
}
impl From<AWD1W> for bool {
    #[inline(always)]
    fn from(variant: AWD1W) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD(1-3)` writer - Analog watchdog %s flag
pub type AWD_W<'a, REG> = crate::BitWriter<'a, REG, AWD1W>;
impl<'a, REG> AWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the analog watchdog x event flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1W::Clear)
    }
}
/**EOCAL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALR {
    ///0: Calibration is not complete
    NotComplete = 0,
    ///1: Calibration is complete
    Complete = 1,
}
impl From<EOCALR> for bool {
    #[inline(always)]
    fn from(variant: EOCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` reader - EOCAL
pub type EOCAL_R = crate::BitReader<EOCALR>;
impl EOCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCALR {
        match self.bits {
            false => EOCALR::NotComplete,
            true => EOCALR::Complete,
        }
    }
    ///Calibration is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCALR::NotComplete
    }
    ///Calibration is complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCALR::Complete
    }
}
/**EOCAL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALW {
    ///1: Clear the end of calibration flag
    Clear = 1,
}
impl From<EOCALW> for bool {
    #[inline(always)]
    fn from(variant: EOCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` writer - EOCAL
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG, EOCALW>;
impl<'a, REG> EOCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the end of calibration flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALW::Clear)
    }
}
/**LDORDY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDORDYR {
    ///0: ADC voltage regulator disabled
    Disabled = 0,
    ///1: ADC voltage regulator enabled
    Enabled = 1,
}
impl From<LDORDYR> for bool {
    #[inline(always)]
    fn from(variant: LDORDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LDORDY` reader - LDORDY
pub type LDORDY_R = crate::BitReader<LDORDYR>;
impl LDORDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDORDYR {
        match self.bits {
            false => LDORDYR::Disabled,
            true => LDORDYR::Enabled,
        }
    }
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LDORDYR::Disabled
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LDORDYR::Enabled
    }
}
///Field `LDORDY` writer - LDORDY
pub type LDORDY_W<'a, REG> = crate::BitWriter<'a, REG, LDORDYR>;
impl<'a, REG> LDORDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LDORDYR::Disabled)
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LDORDYR::Enabled)
    }
}
impl R {
    ///Bit 0 - ADRDY
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Analog watchdog (1-3) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1` field.</div>
    #[inline(always)]
    pub fn awd(&self, n: u8) -> AWD_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWD_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog (1-3) flag
    #[inline(always)]
    pub fn awd_iter(&self) -> impl Iterator<Item = AWD_R> + '_ {
        (0..3).map(move |n| AWD_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - EOCAL
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LDORDY
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("ldordy", &self.ldordy())
            .field("eocal", &self.eocal())
            .field("awd1", &self.awd1())
            .field("awd2", &self.awd2())
            .field("awd3", &self.awd3())
            .field("ovr", &self.ovr())
            .field("eos", &self.eos())
            .field("eoc", &self.eoc())
            .field("eosmp", &self.eosmp())
            .field("adrdy", &self.adrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDY
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Analog watchdog (1-3) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1` field.</div>
    #[inline(always)]
    pub fn awd(&mut self, n: u8) -> AWD_W<ISRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWD_W::new(self, n + 7)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD_W<ISRrs> {
        AWD_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD_W<ISRrs> {
        AWD_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD_W<ISRrs> {
        AWD_W::new(self, 9)
    }
    ///Bit 11 - EOCAL
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W<ISRrs> {
        EOCAL_W::new(self, 11)
    }
    ///Bit 12 - LDORDY
    #[inline(always)]
    pub fn ldordy(&mut self) -> LDORDY_W<ISRrs> {
        LDORDY_W::new(self, 12)
    }
}
/**ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC4:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
