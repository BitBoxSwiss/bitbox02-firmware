///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**RXFILTDIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFILTDIS {
    ///0: Rx pre-filter enabled
    Enabled = 0,
    ///1: Rx pre-filter disabled
    Disabled = 1,
}
impl From<RXFILTDIS> for bool {
    #[inline(always)]
    fn from(variant: RXFILTDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFILTDIS` reader - RXFILTDIS
pub type RXFILTDIS_R = crate::BitReader<RXFILTDIS>;
impl RXFILTDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFILTDIS {
        match self.bits {
            false => RXFILTDIS::Enabled,
            true => RXFILTDIS::Disabled,
        }
    }
    ///Rx pre-filter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFILTDIS::Enabled
    }
    ///Rx pre-filter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFILTDIS::Disabled
    }
}
///Field `RXFILTDIS` writer - RXFILTDIS
pub type RXFILTDIS_W<'a, REG> = crate::BitWriter<'a, REG, RXFILTDIS>;
impl<'a, REG> RXFILTDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx pre-filter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFILTDIS::Enabled)
    }
    ///Rx pre-filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFILTDIS::Disabled)
    }
}
/**RXFILT2N3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFILT2N3 {
    ///0: 3 samples
    Samp3 = 0,
    ///1: 2 samples
    Samp2 = 1,
}
impl From<RXFILT2N3> for bool {
    #[inline(always)]
    fn from(variant: RXFILT2N3) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFILT2N3` reader - RXFILT2N3
pub type RXFILT2N3_R = crate::BitReader<RXFILT2N3>;
impl RXFILT2N3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFILT2N3 {
        match self.bits {
            false => RXFILT2N3::Samp3,
            true => RXFILT2N3::Samp2,
        }
    }
    ///3 samples
    #[inline(always)]
    pub fn is_samp3(&self) -> bool {
        *self == RXFILT2N3::Samp3
    }
    ///2 samples
    #[inline(always)]
    pub fn is_samp2(&self) -> bool {
        *self == RXFILT2N3::Samp2
    }
}
///Field `RXFILT2N3` writer - RXFILT2N3
pub type RXFILT2N3_W<'a, REG> = crate::BitWriter<'a, REG, RXFILT2N3>;
impl<'a, REG> RXFILT2N3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///3 samples
    #[inline(always)]
    pub fn samp3(self) -> &'a mut crate::W<REG> {
        self.variant(RXFILT2N3::Samp3)
    }
    ///2 samples
    #[inline(always)]
    pub fn samp2(self) -> &'a mut crate::W<REG> {
        self.variant(RXFILT2N3::Samp2)
    }
}
/**FORCECLK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCECLK {
    ///0: Do not force clock request
    NoForce = 0,
    ///1: Force clock request
    Force = 1,
}
impl From<FORCECLK> for bool {
    #[inline(always)]
    fn from(variant: FORCECLK) -> Self {
        variant as u8 != 0
    }
}
///Field `FORCECLK` reader - FORCECLK
pub type FORCECLK_R = crate::BitReader<FORCECLK>;
impl FORCECLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORCECLK {
        match self.bits {
            false => FORCECLK::NoForce,
            true => FORCECLK::Force,
        }
    }
    ///Do not force clock request
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == FORCECLK::NoForce
    }
    ///Force clock request
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCECLK::Force
    }
}
///Field `FORCECLK` writer - FORCECLK
pub type FORCECLK_W<'a, REG> = crate::BitWriter<'a, REG, FORCECLK>;
impl<'a, REG> FORCECLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not force clock request
    #[inline(always)]
    pub fn no_force(self) -> &'a mut crate::W<REG> {
        self.variant(FORCECLK::NoForce)
    }
    ///Force clock request
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(FORCECLK::Force)
    }
}
/**WUPEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<WUPEN> for bool {
    #[inline(always)]
    fn from(variant: WUPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPEN` reader - WUPEN
pub type WUPEN_R = crate::BitReader<WUPEN>;
impl WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN {
        match self.bits {
            false => WUPEN::Disabled,
            true => WUPEN::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN::Enabled
    }
}
///Field `WUPEN` writer - WUPEN
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN>;
impl<'a, REG> WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN::Enabled)
    }
}
impl R {
    ///Bit 0 - RXFILTDIS
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RXFILT2N3
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FORCECLK
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUPEN
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("rxfiltdis", &self.rxfiltdis())
            .field("rxfilt2n3", &self.rxfilt2n3())
            .field("forceclk", &self.forceclk())
            .field("wupen", &self.wupen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXFILTDIS
    #[inline(always)]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W<CFGR2rs> {
        RXFILTDIS_W::new(self, 0)
    }
    ///Bit 1 - RXFILT2N3
    #[inline(always)]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W<CFGR2rs> {
        RXFILT2N3_W::new(self, 1)
    }
    ///Bit 2 - FORCECLK
    #[inline(always)]
    pub fn forceclk(&mut self) -> FORCECLK_W<CFGR2rs> {
        FORCECLK_W::new(self, 2)
    }
    ///Bit 3 - WUPEN
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W<CFGR2rs> {
        WUPEN_W::new(self, 3)
    }
}
/**UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#UCPD1:CFGR2)*/
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
