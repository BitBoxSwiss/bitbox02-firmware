///Register `SRCR` reader
pub type R = crate::R<SRCRrs>;
///Register `SRCR` writer
pub type W = crate::W<SRCRrs>;
/**immediate reload This bit is set by software and cleared only by hardware after reload.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMR {
    ///0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    NoEffect = 0,
    ///1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
    Reload = 1,
}
impl From<IMR> for bool {
    #[inline(always)]
    fn from(variant: IMR) -> Self {
        variant as u8 != 0
    }
}
///Field `IMR` reader - immediate reload This bit is set by software and cleared only by hardware after reload.
pub type IMR_R = crate::BitReader<IMR>;
impl IMR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IMR {
        match self.bits {
            false => IMR::NoEffect,
            true => IMR::Reload,
        }
    }
    ///This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IMR::NoEffect
    }
    ///The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == IMR::Reload
    }
}
///Field `IMR` writer - immediate reload This bit is set by software and cleared only by hardware after reload.
pub type IMR_W<'a, REG> = crate::BitWriter<'a, REG, IMR>;
impl<'a, REG> IMR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IMR::NoEffect)
    }
    ///The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(IMR::Reload)
    }
}
/**vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBR {
    ///0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    NoEffect = 0,
    ///1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
    Reload = 1,
}
impl From<VBR> for bool {
    #[inline(always)]
    fn from(variant: VBR) -> Self {
        variant as u8 != 0
    }
}
///Field `VBR` reader - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
pub type VBR_R = crate::BitReader<VBR>;
impl VBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBR {
        match self.bits {
            false => VBR::NoEffect,
            true => VBR::Reload,
        }
    }
    ///This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == VBR::NoEffect
    }
    ///The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == VBR::Reload
    }
}
///Field `VBR` writer - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
pub type VBR_W<'a, REG> = crate::BitWriter<'a, REG, VBR>;
impl<'a, REG> VBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(VBR::NoEffect)
    }
    ///The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(VBR::Reload)
    }
}
impl R {
    ///Bit 0 - immediate reload This bit is set by software and cleared only by hardware after reload.
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRCR")
            .field("imr", &self.imr())
            .field("vbr", &self.vbr())
            .finish()
    }
}
impl W {
    ///Bit 0 - immediate reload This bit is set by software and cleared only by hardware after reload.
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W<SRCRrs> {
        IMR_W::new(self, 0)
    }
    ///Bit 1 - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
    #[inline(always)]
    pub fn vbr(&mut self) -> VBR_W<SRCRrs> {
        VBR_W::new(self, 1)
    }
}
/**LTDC shadow reload configuration register

You can [`read`](crate::Reg::read) this register and get [`srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:SRCR)*/
pub struct SRCRrs;
impl crate::RegisterSpec for SRCRrs {
    type Ux = u32;
}
///`read()` method returns [`srcr::R`](R) reader structure
impl crate::Readable for SRCRrs {}
///`write(|w| ..)` method takes [`srcr::W`](W) writer structure
impl crate::Writable for SRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCR to value 0
impl crate::Resettable for SRCRrs {}
