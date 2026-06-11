///Register `AHB2RSTR2` reader
pub type R = crate::R<AHB2RSTR2rs>;
///Register `AHB2RSTR2` writer
pub type W = crate::W<AHB2RSTR2rs>;
/**Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<FSMCRST> for bool {
    #[inline(always)]
    fn from(variant: FSMCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `FSMCRST` reader - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type FSMCRST_R = crate::BitReader<FSMCRST>;
impl FSMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSMCRST {
        match self.bits {
            false => FSMCRST::NoEffect,
            true => FSMCRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FSMCRST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMCRST::Reset
    }
}
///Field `FSMCRST` writer - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type FSMCRST_W<'a, REG> = crate::BitWriter<'a, REG, FSMCRST>;
impl<'a, REG> FSMCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCRST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCRST::Reset)
    }
}
///Field `OCTOSPI1RST` reader - OCTOSPI1 reset This bit is set and cleared by software.
pub use FSMCRST_R as OCTOSPI1RST_R;
///Field `OCTOSPI2RST` reader - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCRST_R as OCTOSPI2RST_R;
///Field `HSPI1RST` reader - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCRST_R as HSPI1RST_R;
///Field `OCTOSPI1RST` writer - OCTOSPI1 reset This bit is set and cleared by software.
pub use FSMCRST_W as OCTOSPI1RST_W;
///Field `OCTOSPI2RST` writer - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCRST_W as OCTOSPI2RST_W;
///Field `HSPI1RST` writer - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCRST_W as HSPI1RST_W;
impl R {
    ///Bit 0 - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - OCTOSPI1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospi2rst(&self) -> OCTOSPI2RST_R {
        OCTOSPI2RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1rst(&self) -> HSPI1RST_R {
        HSPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR2")
            .field("fsmcrst", &self.fsmcrst())
            .field("octospi1rst", &self.octospi1rst())
            .field("octospi2rst", &self.octospi2rst())
            .field("hspi1rst", &self.hspi1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<AHB2RSTR2rs> {
        FSMCRST_W::new(self, 0)
    }
    ///Bit 4 - OCTOSPI1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<AHB2RSTR2rs> {
        OCTOSPI1RST_W::new(self, 4)
    }
    ///Bit 8 - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospi2rst(&mut self) -> OCTOSPI2RST_W<AHB2RSTR2rs> {
        OCTOSPI2RST_W::new(self, 8)
    }
    ///Bit 12 - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1rst(&mut self) -> HSPI1RST_W<AHB2RSTR2rs> {
        HSPI1RST_W::new(self, 12)
    }
}
/**RCC AHB2 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:AHB2RSTR2)*/
pub struct AHB2RSTR2rs;
impl crate::RegisterSpec for AHB2RSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr2::R`](R) reader structure
impl crate::Readable for AHB2RSTR2rs {}
///`write(|w| ..)` method takes [`ahb2rstr2::W`](W) writer structure
impl crate::Writable for AHB2RSTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR2 to value 0
impl crate::Resettable for AHB2RSTR2rs {}
