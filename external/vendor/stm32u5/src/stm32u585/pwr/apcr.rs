///Register `APCR` reader
pub type R = crate::R<APCRrs>;
///Register `APCR` writer
pub type W = crate::W<APCRrs>;
/**Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC {
    ///0: PWR_PUCRx and PWR_PDCRx are not applied to the I/Os
    Disabled = 0,
    ///1: I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied
    Enabled = 1,
}
impl From<APC> for bool {
    #[inline(always)]
    fn from(variant: APC) -> Self {
        variant as u8 != 0
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
pub type APC_R = crate::BitReader<APC>;
impl APC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APC {
        match self.bits {
            false => APC::Disabled,
            true => APC::Enabled,
        }
    }
    ///PWR_PUCRx and PWR_PDCRx are not applied to the I/Os
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC::Disabled
    }
    ///I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC::Enabled
    }
}
///Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG, APC>;
impl<'a, REG> APC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PWR_PUCRx and PWR_PDCRx are not applied to the I/Os
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Disabled)
    }
    ///I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Enabled)
    }
}
impl R {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APCR").field("apc", &self.apc()).finish()
    }
}
impl W {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<APCRrs> {
        APC_W::new(self, 0)
    }
}
/**PWR apply pull configuration register

You can [`read`](crate::Reg::read) this register and get [`apcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:APCR)*/
pub struct APCRrs;
impl crate::RegisterSpec for APCRrs {
    type Ux = u32;
}
///`read()` method returns [`apcr::R`](R) reader structure
impl crate::Readable for APCRrs {}
///`write(|w| ..)` method takes [`apcr::W`](W) writer structure
impl crate::Writable for APCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APCR to value 0
impl crate::Resettable for APCRrs {}
