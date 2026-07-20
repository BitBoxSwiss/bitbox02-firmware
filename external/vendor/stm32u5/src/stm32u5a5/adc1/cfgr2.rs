///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**ROVSE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    ///0: Regular oversampling disabled
    Disabled = 0,
    ///1: Regular oversampling enabled
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSE` reader - ROVSE
pub type ROVSE_R = crate::BitReader<ROVSE>;
impl ROVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSE {
        match self.bits {
            false => ROVSE::Disabled,
            true => ROVSE::Enabled,
        }
    }
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
///Field `ROVSE` writer - ROVSE
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
/**JOVSE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    ///0: Injected oversampling disabled
    Disabled = 0,
    ///1: Injected oversampling enabled
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `JOVSE` reader - JOVSE
pub type JOVSE_R = crate::BitReader<JOVSE>;
impl JOVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JOVSE {
        match self.bits {
            false => JOVSE::Disabled,
            true => JOVSE::Enabled,
        }
    }
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
///Field `JOVSE` writer - JOVSE
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Enabled)
    }
}
///Field `OVSS` reader - OVSS
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - OVSS
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**TROVS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    ///0: All oversampled conversions for a channel are done consecutively following a trigger
    Automatic = 0,
    ///1: Each oversampled conversion for a channel needs a new trigger
    Triggered = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TROVS` reader - TROVS
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::Automatic,
            true => TROVS::Triggered,
        }
    }
    ///All oversampled conversions for a channel are done consecutively following a trigger
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS::Automatic
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS::Triggered
    }
}
///Field `TROVS` writer - TROVS
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are done consecutively following a trigger
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Automatic)
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Triggered)
    }
}
/**ROVSM

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    ///0: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)
    Continued = 0,
    ///1: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)
    Resumed = 1,
}
impl From<ROVSM> for bool {
    #[inline(always)]
    fn from(variant: ROVSM) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSM` reader - ROVSM
pub type ROVSM_R = crate::BitReader<ROVSM>;
impl ROVSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM {
        match self.bits {
            false => ROVSM::Continued,
            true => ROVSM::Resumed,
        }
    }
    ///When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM::Continued
    }
    ///When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM::Resumed
    }
}
///Field `ROVSM` writer - ROVSM
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)
    #[inline(always)]
    pub fn continued(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Continued)
    }
    ///When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Resumed)
    }
}
/**BULB

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB {
    ///0: Bulb sampling mode disabled
    Disabled = 0,
    ///1: Bulb sampling mode enabled. The sampling period starts just after the previous end of the conversion.
    Enabled = 1,
}
impl From<BULB> for bool {
    #[inline(always)]
    fn from(variant: BULB) -> Self {
        variant as u8 != 0
    }
}
///Field `BULB` reader - BULB
pub type BULB_R = crate::BitReader<BULB>;
impl BULB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BULB {
        match self.bits {
            false => BULB::Disabled,
            true => BULB::Enabled,
        }
    }
    ///Bulb sampling mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BULB::Disabled
    }
    ///Bulb sampling mode enabled. The sampling period starts just after the previous end of the conversion.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BULB::Enabled
    }
}
///Field `BULB` writer - BULB
pub type BULB_W<'a, REG> = crate::BitWriter<'a, REG, BULB>;
impl<'a, REG> BULB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bulb sampling mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BULB::Disabled)
    }
    ///Bulb sampling mode enabled. The sampling period starts just after the previous end of the conversion.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BULB::Enabled)
    }
}
/**SWTRIG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG {
    ///0: Software trigger starts the conversion for sampling time control trigger mode
    Disabled = 0,
    ///1: Software trigger starts the sampling for sampling time control trigger mode
    Enabled = 1,
}
impl From<SWTRIG> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG` reader - SWTRIG
pub type SWTRIG_R = crate::BitReader<SWTRIG>;
impl SWTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWTRIG {
        match self.bits {
            false => SWTRIG::Disabled,
            true => SWTRIG::Enabled,
        }
    }
    ///Software trigger starts the conversion for sampling time control trigger mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWTRIG::Disabled
    }
    ///Software trigger starts the sampling for sampling time control trigger mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWTRIG::Enabled
    }
}
///Field `SWTRIG` writer - SWTRIG
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG>;
impl<'a, REG> SWTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger starts the conversion for sampling time control trigger mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Disabled)
    }
    ///Software trigger starts the sampling for sampling time control trigger mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG::Enabled)
    }
}
/**SMPTRIG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPTRIG {
    ///0: Sampling time control trigger mode disabled
    Disabled = 0,
    ///1: Sampling time control trigger mode enabled
    Enabled = 1,
}
impl From<SMPTRIG> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPTRIG` reader - SMPTRIG
pub type SMPTRIG_R = crate::BitReader<SMPTRIG>;
impl SMPTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPTRIG {
        match self.bits {
            false => SMPTRIG::Disabled,
            true => SMPTRIG::Enabled,
        }
    }
    ///Sampling time control trigger mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPTRIG::Disabled
    }
    ///Sampling time control trigger mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPTRIG::Enabled
    }
}
///Field `SMPTRIG` writer - SMPTRIG
pub type SMPTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SMPTRIG>;
impl<'a, REG> SMPTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time control trigger mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG::Disabled)
    }
    ///Sampling time control trigger mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG::Enabled)
    }
}
///Field `OSR` reader - OSR
pub type OSR_R = crate::FieldReader<u16>;
///Field `OSR` writer - OSR
pub type OSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
/**LFTRIG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFTRIG {
    ///0: Low-frequency trigger mode disabled
    Disabled = 0,
    ///1: Low-frequency trigger mode enabled
    Enabled = 1,
}
impl From<LFTRIG> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `LFTRIG` reader - LFTRIG
pub type LFTRIG_R = crate::BitReader<LFTRIG>;
impl LFTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LFTRIG {
        match self.bits {
            false => LFTRIG::Disabled,
            true => LFTRIG::Enabled,
        }
    }
    ///Low-frequency trigger mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFTRIG::Disabled
    }
    ///Low-frequency trigger mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFTRIG::Enabled
    }
}
///Field `LFTRIG` writer - LFTRIG
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG, LFTRIG>;
impl<'a, REG> LFTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low-frequency trigger mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::Disabled)
    }
    ///Low-frequency trigger mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::Enabled)
    }
}
///Field `LSHIFT` reader - LSHIFT
pub type LSHIFT_R = crate::FieldReader;
///Field `LSHIFT` writer - LSHIFT
pub type LSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bit 0 - ROVSE
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - JOVSE
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - TROVS
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ROVSM
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - BULB
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SWTRIG
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SMPTRIG
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:25 - OSR
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 27 - LFTRIG
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - LSHIFT
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("lshift", &self.lshift())
            .field("lftrig", &self.lftrig())
            .field("osr", &self.osr())
            .field("smptrig", &self.smptrig())
            .field("swtrig", &self.swtrig())
            .field("bulb", &self.bulb())
            .field("rovsm", &self.rovsm())
            .field("trovs", &self.trovs())
            .field("ovss", &self.ovss())
            .field("jovse", &self.jovse())
            .field("rovse", &self.rovse())
            .finish()
    }
}
impl W {
    ///Bit 0 - ROVSE
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W<CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - JOVSE
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W<CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - TROVS
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W<CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - ROVSM
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W<CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    ///Bit 13 - BULB
    #[inline(always)]
    pub fn bulb(&mut self) -> BULB_W<CFGR2rs> {
        BULB_W::new(self, 13)
    }
    ///Bit 14 - SWTRIG
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W<CFGR2rs> {
        SWTRIG_W::new(self, 14)
    }
    ///Bit 15 - SMPTRIG
    #[inline(always)]
    pub fn smptrig(&mut self) -> SMPTRIG_W<CFGR2rs> {
        SMPTRIG_W::new(self, 15)
    }
    ///Bits 16:25 - OSR
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W<CFGR2rs> {
        OSR_W::new(self, 16)
    }
    ///Bit 27 - LFTRIG
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W<CFGR2rs> {
        LFTRIG_W::new(self, 27)
    }
    ///Bits 28:31 - LSHIFT
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W<CFGR2rs> {
        LSHIFT_W::new(self, 28)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
