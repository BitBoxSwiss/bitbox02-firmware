///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**OVSE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSE {
    ///0: Oversampler disabled
    Disabled = 0,
    ///1: Oversampler enabled
    Enabled = 1,
}
impl From<OVSE> for bool {
    #[inline(always)]
    fn from(variant: OVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVSE` reader - OVSE
pub type OVSE_R = crate::BitReader<OVSE>;
impl OVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVSE {
        match self.bits {
            false => OVSE::Disabled,
            true => OVSE::Enabled,
        }
    }
    ///Oversampler disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVSE::Disabled
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVSE::Enabled
    }
}
///Field `OVSE` writer - OVSE
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG, OVSE>;
impl<'a, REG> OVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampler disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::Disabled)
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::Enabled)
    }
}
///Field `OVSR` reader - OVSR
pub type OVSR_R = crate::FieldReader;
///Field `OVSR` writer - OVSR
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Field `OVSS` reader - OVSS
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - OVSS
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**TOVS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS {
    ///0: All oversampled conversions for a channel are done consecutively following a trigger
    Automatic = 0,
    ///1: Each oversampled conversion for a channel needs a new trigger
    Triggered = 1,
}
impl From<TOVS> for bool {
    #[inline(always)]
    fn from(variant: TOVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TOVS` reader - TOVS
pub type TOVS_R = crate::BitReader<TOVS>;
impl TOVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOVS {
        match self.bits {
            false => TOVS::Automatic,
            true => TOVS::Triggered,
        }
    }
    ///All oversampled conversions for a channel are done consecutively following a trigger
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TOVS::Automatic
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TOVS::Triggered
    }
}
///Field `TOVS` writer - TOVS
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG, TOVS>;
impl<'a, REG> TOVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are done consecutively following a trigger
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::Automatic)
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::Triggered)
    }
}
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
impl R {
    ///Bit 0 - OVSE
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - OVSR
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - TOVS
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - LFTRIG
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("lftrig", &self.lftrig())
            .field("tovs", &self.tovs())
            .field("ovss", &self.ovss())
            .field("ovsr", &self.ovsr())
            .field("ovse", &self.ovse())
            .finish()
    }
}
impl W {
    ///Bit 0 - OVSE
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W<CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    ///Bits 2:4 - OVSR
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - TOVS
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bit 29 - LFTRIG
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W<CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC4:CFGR2)*/
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
