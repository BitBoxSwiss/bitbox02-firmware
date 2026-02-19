///Register `HLCR` reader
pub type R = crate::R<HLCRrs>;
///Register `HLCR` writer
pub type W = crate::W<HLCRrs>;
/**Latency mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LM {
    ///0: Variable initial latency
    Variable = 0,
    ///1: Fixed latency
    Fixed = 1,
}
impl From<LM> for bool {
    #[inline(always)]
    fn from(variant: LM) -> Self {
        variant as u8 != 0
    }
}
///Field `LM` reader - Latency mode
pub type LM_R = crate::BitReader<LM>;
impl LM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LM {
        match self.bits {
            false => LM::Variable,
            true => LM::Fixed,
        }
    }
    ///Variable initial latency
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        *self == LM::Variable
    }
    ///Fixed latency
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == LM::Fixed
    }
}
///Field `LM` writer - Latency mode
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG, LM>;
impl<'a, REG> LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Variable initial latency
    #[inline(always)]
    pub fn variable(self) -> &'a mut crate::W<REG> {
        self.variant(LM::Variable)
    }
    ///Fixed latency
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(LM::Fixed)
    }
}
/**Write zero latency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WZL {
    ///0: Latency on write accesses
    Disabled = 0,
    ///1: No latency on write accesses
    Enabled = 1,
}
impl From<WZL> for bool {
    #[inline(always)]
    fn from(variant: WZL) -> Self {
        variant as u8 != 0
    }
}
///Field `WZL` reader - Write zero latency
pub type WZL_R = crate::BitReader<WZL>;
impl WZL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WZL {
        match self.bits {
            false => WZL::Disabled,
            true => WZL::Enabled,
        }
    }
    ///Latency on write accesses
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WZL::Disabled
    }
    ///No latency on write accesses
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WZL::Enabled
    }
}
///Field `WZL` writer - Write zero latency
pub type WZL_W<'a, REG> = crate::BitWriter<'a, REG, WZL>;
impl<'a, REG> WZL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Latency on write accesses
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WZL::Disabled)
    }
    ///No latency on write accesses
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WZL::Enabled)
    }
}
///Field `TACC` reader - Access time
pub type TACC_R = crate::FieldReader;
///Field `TACC` writer - Access time
pub type TACC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `TRWR` reader - Read write recovery time
pub type TRWR_R = crate::FieldReader;
///Field `TRWR` writer - Read write recovery time
pub type TRWR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bit 0 - Latency mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HLCR")
            .field("lm", &self.lm())
            .field("wzl", &self.wzl())
            .field("tacc", &self.tacc())
            .field("trwr", &self.trwr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Latency mode
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<HLCRrs> {
        LM_W::new(self, 0)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    pub fn wzl(&mut self) -> WZL_W<HLCRrs> {
        WZL_W::new(self, 1)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    pub fn tacc(&mut self) -> TACC_W<HLCRrs> {
        TACC_W::new(self, 8)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    pub fn trwr(&mut self) -> TRWR_W<HLCRrs> {
        TRWR_W::new(self, 16)
    }
}
/**HyperBus latency configuration register

You can [`read`](crate::Reg::read) this register and get [`hlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OCTOSPI1:HLCR)*/
pub struct HLCRrs;
impl crate::RegisterSpec for HLCRrs {
    type Ux = u32;
}
///`read()` method returns [`hlcr::R`](R) reader structure
impl crate::Readable for HLCRrs {}
///`write(|w| ..)` method takes [`hlcr::W`](W) writer structure
impl crate::Writable for HLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HLCR to value 0
impl crate::Resettable for HLCRrs {}
