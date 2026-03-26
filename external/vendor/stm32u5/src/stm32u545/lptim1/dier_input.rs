///Register `DIER_input` reader
pub type R = crate::R<DIER_INPUTrs>;
///Register `DIER_input` writer
pub type W = crate::W<DIER_INPUTrs>;
///Field `CC1IF` reader - Capture/compare 1 clear flag
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - Capture/compare 1 clear flag
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMIE` reader - Autoreload match Interrupt Enable
pub type ARRMIE_R = crate::BitReader;
///Field `ARRMIE` writer - Autoreload match Interrupt Enable
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_R = crate::BitReader;
///Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable
pub type ARROKIE_R = crate::BitReader;
///Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPIE` reader - Direction change to UP Interrupt Enable
pub type UPIE_R = crate::BitReader;
///Field `UPIE` writer - Direction change to UP Interrupt Enable
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNIE` reader - Direction change to down Interrupt Enable
pub type DOWNIE_R = crate::BitReader;
///Field `DOWNIE` writer - Direction change to down Interrupt Enable
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEIE` reader - Update event interrupt enable
pub type UEIE_R = crate::BitReader;
///Field `UEIE` writer - Update event interrupt enable
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKIE` reader - REPOKIE
pub type REPOKIE_R = crate::BitReader;
///Field `REPOKIE` writer - REPOKIE
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/compare 2 interrupt enable
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - Capture/compare 2 interrupt enable
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OIE` reader - Capture/compare 1 over-capture interrupt enable
pub type CC1OIE_R = crate::BitReader;
///Field `CC1OIE` writer - Capture/compare 1 over-capture interrupt enable
pub type CC1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OIE` reader - Capture/compare 2 over-capture interrupt enable
pub type CC2OIE_R = crate::BitReader;
///Field `CC2OIE` writer - Capture/compare 2 over-capture interrupt enable
pub type CC2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - Capture/compare 2 DMA request enable
pub type CC2DE_R = crate::BitReader;
///Field `CC2DE` writer - Capture/compare 2 DMA request enable
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare 1 clear flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - REPOKIE
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Capture/compare 1 over-capture interrupt enable
    #[inline(always)]
    pub fn cc1oie(&self) -> CC1OIE_R {
        CC1OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/compare 2 over-capture interrupt enable
    #[inline(always)]
    pub fn cc2oie(&self) -> CC2OIE_R {
        CC2OIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Capture/compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 25 - Capture/compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER_input")
            .field("cc2de", &self.cc2de())
            .field("cc1de", &self.cc1de())
            .field("cc2oie", &self.cc2oie())
            .field("cc1oie", &self.cc1oie())
            .field("cc2ie", &self.cc2ie())
            .field("repokie", &self.repokie())
            .field("ueie", &self.ueie())
            .field("downie", &self.downie())
            .field("upie", &self.upie())
            .field("arrokie", &self.arrokie())
            .field("exttrigie", &self.exttrigie())
            .field("arrmie", &self.arrmie())
            .field("cc1if", &self.cc1if())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 clear flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<DIER_INPUTrs> {
        CC1IF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIER_INPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIER_INPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIER_INPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<DIER_INPUTrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<DIER_INPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<DIER_INPUTrs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - REPOKIE
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<DIER_INPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIER_INPUTrs> {
        CC2IE_W::new(self, 9)
    }
    ///Bit 12 - Capture/compare 1 over-capture interrupt enable
    #[inline(always)]
    pub fn cc1oie(&mut self) -> CC1OIE_W<DIER_INPUTrs> {
        CC1OIE_W::new(self, 12)
    }
    ///Bit 13 - Capture/compare 2 over-capture interrupt enable
    #[inline(always)]
    pub fn cc2oie(&mut self) -> CC2OIE_W<DIER_INPUTrs> {
        CC2OIE_W::new(self, 13)
    }
    ///Bit 16 - Capture/compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<DIER_INPUTrs> {
        CC1DE_W::new(self, 16)
    }
    ///Bit 25 - Capture/compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<DIER_INPUTrs> {
        CC2DE_W::new(self, 25)
    }
}
/**LPTIM interrupt Enable Register (intput mode)

You can [`read`](crate::Reg::read) this register and get [`dier_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#LPTIM1:DIER_input)*/
pub struct DIER_INPUTrs;
impl crate::RegisterSpec for DIER_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`dier_input::R`](R) reader structure
impl crate::Readable for DIER_INPUTrs {}
///`write(|w| ..)` method takes [`dier_input::W`](W) writer structure
impl crate::Writable for DIER_INPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER_input to value 0
impl crate::Resettable for DIER_INPUTrs {}
