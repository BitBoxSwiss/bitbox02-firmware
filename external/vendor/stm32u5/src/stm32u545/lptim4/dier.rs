///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
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
    pub fn cc1if(&mut self) -> CC1IF_W<DIERrs> {
        CC1IF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIERrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIERrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 3 - Compare register 1 update OK Interrupt Enable
    #[inline(always)]
    pub fn cmp1okie(&mut self) -> CMP1OKIE_W<DIERrs> {
        CMP1OKIE_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIERrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<DIERrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<DIERrs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<DIERrs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - REPOKIE
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<DIERrs> {
        REPOKIE_W::new(self, 8)
    }
}
/**LPTIM interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#LPTIM4:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
