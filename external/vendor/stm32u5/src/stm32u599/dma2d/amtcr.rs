///Register `AMTCR` reader
pub type R = crate::R<AMTCRrs>;
///Register `AMTCR` writer
pub type W = crate::W<AMTCRrs>;
/**Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Disabled AHB/AXI dead-time functionality
    Disabled = 0,
    ///1: Enabled AHB/AXI dead-time functionality
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Disabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Enabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Enabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
///Field `DT` reader - Dead Time
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Dead Time
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Dead Time
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMTCR")
            .field("dt", &self.dt())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<AMTCRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 8:15 - Dead Time
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<AMTCRrs> {
        DT_W::new(self, 8)
    }
}
/**AHB master timer configuration register

You can [`read`](crate::Reg::read) this register and get [`amtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DMA2D:AMTCR)*/
pub struct AMTCRrs;
impl crate::RegisterSpec for AMTCRrs {
    type Ux = u32;
}
///`read()` method returns [`amtcr::R`](R) reader structure
impl crate::Readable for AMTCRrs {}
///`write(|w| ..)` method takes [`amtcr::W`](W) writer structure
impl crate::Writable for AMTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AMTCR to value 0
impl crate::Resettable for AMTCRrs {}
