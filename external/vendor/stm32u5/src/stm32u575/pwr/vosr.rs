///Register `VOSR` reader
pub type R = crate::R<VOSRrs>;
///Register `VOSR` writer
pub type W = crate::W<VOSRrs>;
/**EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTRDY {
    ///0: Power booster not ready
    NotReady = 0,
    ///1: Power booster ready
    Ready = 1,
}
impl From<BOOSTRDY> for bool {
    #[inline(always)]
    fn from(variant: BOOSTRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOSTRDY` reader - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.
pub type BOOSTRDY_R = crate::BitReader<BOOSTRDY>;
impl BOOSTRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTRDY {
        match self.bits {
            false => BOOSTRDY::NotReady,
            true => BOOSTRDY::Ready,
        }
    }
    ///Power booster not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == BOOSTRDY::NotReady
    }
    ///Power booster ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BOOSTRDY::Ready
    }
}
/**Ready bit for VCORE voltage scaling output selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSRDY {
    ///0: Not ready, voltage level < VOS selected level
    NotReady = 0,
    ///1: Ready, voltage level ≥ VOS selected level
    Ready = 1,
}
impl From<VOSRDY> for bool {
    #[inline(always)]
    fn from(variant: VOSRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSRDY` reader - Ready bit for VCORE voltage scaling output selection
pub type VOSRDY_R = crate::BitReader<VOSRDY>;
impl VOSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOSRDY {
        match self.bits {
            false => VOSRDY::NotReady,
            true => VOSRDY::Ready,
        }
    }
    ///Not ready, voltage level < VOS selected level
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSRDY::NotReady
    }
    ///Ready, voltage level ≥ VOS selected level
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSRDY::Ready
    }
}
/**Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///0: Range 4 (lowest power)
    Range4 = 0,
    ///1: Range 3
    Range3 = 1,
    ///2: Range 2
    Range2 = 2,
    ///3: Range 1 (highest frequency)
    Range1 = 3,
}
impl From<VOS> for u8 {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS {
    type Ux = u8;
}
impl crate::IsEnum for VOS {}
///Field `VOS` reader - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOS {
        match self.bits {
            0 => VOS::Range4,
            1 => VOS::Range3,
            2 => VOS::Range2,
            3 => VOS::Range1,
            _ => unreachable!(),
        }
    }
    ///Range 4 (lowest power)
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == VOS::Range4
    }
    ///Range 3
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == VOS::Range3
    }
    ///Range 2
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == VOS::Range2
    }
    ///Range 1 (highest frequency)
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == VOS::Range1
    }
}
///Field `VOS` writer - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS, crate::Safe>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Range 4 (lowest power)
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range4)
    }
    ///Range 3
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range3)
    }
    ///Range 2
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range2)
    }
    ///Range 1 (highest frequency)
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range1)
    }
}
/**EPOD booster enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN {
    ///0: Booster disabled
    Disabled = 0,
    ///1: Booster enabled
    Enabled = 1,
}
impl From<BOOSTEN> for bool {
    #[inline(always)]
    fn from(variant: BOOSTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOSTEN` reader - EPOD booster enable
pub type BOOSTEN_R = crate::BitReader<BOOSTEN>;
impl BOOSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTEN {
        match self.bits {
            false => BOOSTEN::Disabled,
            true => BOOSTEN::Enabled,
        }
    }
    ///Booster disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTEN::Disabled
    }
    ///Booster enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTEN::Enabled
    }
}
///Field `BOOSTEN` writer - EPOD booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTEN>;
impl<'a, REG> BOOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Booster disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Disabled)
    }
    ///Booster enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Enabled)
    }
}
impl R {
    ///Bit 14 - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.
    #[inline(always)]
    pub fn boostrdy(&self) -> BOOSTRDY_R {
        BOOSTRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ready bit for VCORE voltage scaling output selection
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - EPOD booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOSR")
            .field("boostrdy", &self.boostrdy())
            .field("vosrdy", &self.vosrdy())
            .field("vos", &self.vos())
            .field("boosten", &self.boosten())
            .finish()
    }
}
impl W {
    ///Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<VOSRrs> {
        VOS_W::new(self, 16)
    }
    ///Bit 18 - EPOD booster enable
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<VOSRrs> {
        BOOSTEN_W::new(self, 18)
    }
}
/**PWR voltage scaling register

You can [`read`](crate::Reg::read) this register and get [`vosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:VOSR)*/
pub struct VOSRrs;
impl crate::RegisterSpec for VOSRrs {
    type Ux = u32;
}
///`read()` method returns [`vosr::R`](R) reader structure
impl crate::Readable for VOSRrs {}
///`write(|w| ..)` method takes [`vosr::W`](W) writer structure
impl crate::Writable for VOSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VOSR to value 0x8000
impl crate::Resettable for VOSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
