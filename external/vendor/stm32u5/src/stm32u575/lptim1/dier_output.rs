///Register `DIER_output` reader
pub type R = crate::R<DIER_OUTPUTrs>;
///Register `DIER_output` writer
pub type W = crate::W<DIER_OUTPUTrs>;
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
///Field `CMP1OKIE` reader - Compare register 1 update OK Interrupt Enable
pub type CMP1OKIE_R = crate::BitReader;
///Field `CMP1OKIE` writer - Compare register 1 update OK Interrupt Enable
pub type CMP1OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `CMP2OKIE` reader - Compare register 2 update OK interrupt enable
pub type CMP2OKIE_R = crate::BitReader;
///Field `CMP2OKIE` writer - Compare register 2 update OK interrupt enable
pub type CMP2OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEDE` reader - Update event DMA request enable
pub type UEDE_R = crate::BitReader;
///Field `UEDE` writer - Update event DMA request enable
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - Compare register 1 update OK Interrupt Enable
    #[inline(always)]
    pub fn cmp1okie(&self) -> CMP1OKIE_R {
        CMP1OKIE_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bit 19 - Compare register 2 update OK interrupt enable
    #[inline(always)]
    pub fn cmp2okie(&self) -> CMP2OKIE_R {
        CMP2OKIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - Update event DMA request enable
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER_output")
            .field("uede", &self.uede())
            .field("cmp2okie", &self.cmp2okie())
            .field("cc2ie", &self.cc2ie())
            .field("repokie", &self.repokie())
            .field("ueie", &self.ueie())
            .field("downie", &self.downie())
            .field("upie", &self.upie())
            .field("arrokie", &self.arrokie())
            .field("cmp1okie", &self.cmp1okie())
            .field("exttrigie", &self.exttrigie())
            .field("arrmie", &self.arrmie())
            .field("cc1if", &self.cc1if())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 clear flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<DIER_OUTPUTrs> {
        CC1IF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIER_OUTPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIER_OUTPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 3 - Compare register 1 update OK Interrupt Enable
    #[inline(always)]
    pub fn cmp1okie(&mut self) -> CMP1OKIE_W<DIER_OUTPUTrs> {
        CMP1OKIE_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIER_OUTPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<DIER_OUTPUTrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<DIER_OUTPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<DIER_OUTPUTrs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - REPOKIE
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<DIER_OUTPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIER_OUTPUTrs> {
        CC2IE_W::new(self, 9)
    }
    ///Bit 19 - Compare register 2 update OK interrupt enable
    #[inline(always)]
    pub fn cmp2okie(&mut self) -> CMP2OKIE_W<DIER_OUTPUTrs> {
        CMP2OKIE_W::new(self, 19)
    }
    ///Bit 23 - Update event DMA request enable
    #[inline(always)]
    pub fn uede(&mut self) -> UEDE_W<DIER_OUTPUTrs> {
        UEDE_W::new(self, 23)
    }
}
/**LPTIM interrupt Enable Register (output mode)

You can [`read`](crate::Reg::read) this register and get [`dier_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPTIM1:DIER_output)*/
pub struct DIER_OUTPUTrs;
impl crate::RegisterSpec for DIER_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`dier_output::R`](R) reader structure
impl crate::Readable for DIER_OUTPUTrs {}
///`write(|w| ..)` method takes [`dier_output::W`](W) writer structure
impl crate::Writable for DIER_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER_output to value 0
impl crate::Resettable for DIER_OUTPUTrs {}
