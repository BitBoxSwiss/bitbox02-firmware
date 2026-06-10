///Register `PWRR` reader
pub type R = crate::R<PWRRrs>;
///Register `PWRR` writer
pub type W = crate::W<PWRRrs>;
/**AUTOFF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF {
    ///0: Auto-off mode disabled
    Disabled = 0,
    ///1: Auto-off mode enabled
    Enabled = 1,
}
impl From<AUTOFF> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOFF` reader - AUTOFF
pub type AUTOFF_R = crate::BitReader<AUTOFF>;
impl AUTOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOFF {
        match self.bits {
            false => AUTOFF::Disabled,
            true => AUTOFF::Enabled,
        }
    }
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOFF::Disabled
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOFF::Enabled
    }
}
///Field `AUTOFF` writer - AUTOFF
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG, AUTOFF>;
impl<'a, REG> AUTOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::Disabled)
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::Enabled)
    }
}
/**DPD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPD {
    ///0: Deep-power-down mode disabled
    Disabled = 0,
    ///1: Deep-power-down mode enabled
    Enabled = 1,
}
impl From<DPD> for bool {
    #[inline(always)]
    fn from(variant: DPD) -> Self {
        variant as u8 != 0
    }
}
///Field `DPD` reader - DPD
pub type DPD_R = crate::BitReader<DPD>;
impl DPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPD {
        match self.bits {
            false => DPD::Disabled,
            true => DPD::Enabled,
        }
    }
    ///Deep-power-down mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DPD::Disabled
    }
    ///Deep-power-down mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DPD::Enabled
    }
}
///Field `DPD` writer - DPD
pub type DPD_W<'a, REG> = crate::BitWriter<'a, REG, DPD>;
impl<'a, REG> DPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deep-power-down mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DPD::Disabled)
    }
    ///Deep-power-down mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DPD::Enabled)
    }
}
/**VREFPROT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFPROT {
    ///0: VREF+ protection disabled
    Disabled = 0,
    ///1: VREF+ protection enabled
    Enabled = 1,
}
impl From<VREFPROT> for bool {
    #[inline(always)]
    fn from(variant: VREFPROT) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFPROT` reader - VREFPROT
pub type VREFPROT_R = crate::BitReader<VREFPROT>;
impl VREFPROT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFPROT {
        match self.bits {
            false => VREFPROT::Disabled,
            true => VREFPROT::Enabled,
        }
    }
    ///VREF+ protection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFPROT::Disabled
    }
    ///VREF+ protection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFPROT::Enabled
    }
}
///Field `VREFPROT` writer - VREFPROT
pub type VREFPROT_W<'a, REG> = crate::BitWriter<'a, REG, VREFPROT>;
impl<'a, REG> VREFPROT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VREF+ protection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFPROT::Disabled)
    }
    ///VREF+ protection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFPROT::Enabled)
    }
}
/**VREFSECSMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFSECSMP {
    ///0: VREF+ second sample disabled
    Disabled = 0,
    ///1: VREF+ second sample enabled
    Enabled = 1,
}
impl From<VREFSECSMP> for bool {
    #[inline(always)]
    fn from(variant: VREFSECSMP) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFSECSMP` reader - VREFSECSMP
pub type VREFSECSMP_R = crate::BitReader<VREFSECSMP>;
impl VREFSECSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFSECSMP {
        match self.bits {
            false => VREFSECSMP::Disabled,
            true => VREFSECSMP::Enabled,
        }
    }
    ///VREF+ second sample disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFSECSMP::Disabled
    }
    ///VREF+ second sample enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFSECSMP::Enabled
    }
}
///Field `VREFSECSMP` writer - VREFSECSMP
pub type VREFSECSMP_W<'a, REG> = crate::BitWriter<'a, REG, VREFSECSMP>;
impl<'a, REG> VREFSECSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VREF+ second sample disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSECSMP::Disabled)
    }
    ///VREF+ second sample enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSECSMP::Enabled)
    }
}
impl R {
    ///Bit 0 - AUTOFF
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DPD
    #[inline(always)]
    pub fn dpd(&self) -> DPD_R {
        DPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VREFPROT
    #[inline(always)]
    pub fn vrefprot(&self) -> VREFPROT_R {
        VREFPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VREFSECSMP
    #[inline(always)]
    pub fn vrefsecsmp(&self) -> VREFSECSMP_R {
        VREFSECSMP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRR")
            .field("vrefsecsmp", &self.vrefsecsmp())
            .field("vrefprot", &self.vrefprot())
            .field("dpd", &self.dpd())
            .field("autoff", &self.autoff())
            .finish()
    }
}
impl W {
    ///Bit 0 - AUTOFF
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W<PWRRrs> {
        AUTOFF_W::new(self, 0)
    }
    ///Bit 1 - DPD
    #[inline(always)]
    pub fn dpd(&mut self) -> DPD_W<PWRRrs> {
        DPD_W::new(self, 1)
    }
    ///Bit 2 - VREFPROT
    #[inline(always)]
    pub fn vrefprot(&mut self) -> VREFPROT_W<PWRRrs> {
        VREFPROT_W::new(self, 2)
    }
    ///Bit 3 - VREFSECSMP
    #[inline(always)]
    pub fn vrefsecsmp(&mut self) -> VREFSECSMP_W<PWRRrs> {
        VREFSECSMP_W::new(self, 3)
    }
}
/**ADC data register

You can [`read`](crate::Reg::read) this register and get [`pwrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC4:PWRR)*/
pub struct PWRRrs;
impl crate::RegisterSpec for PWRRrs {
    type Ux = u32;
}
///`read()` method returns [`pwrr::R`](R) reader structure
impl crate::Readable for PWRRrs {}
///`write(|w| ..)` method takes [`pwrr::W`](W) writer structure
impl crate::Writable for PWRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRR to value 0
impl crate::Resettable for PWRRrs {}
