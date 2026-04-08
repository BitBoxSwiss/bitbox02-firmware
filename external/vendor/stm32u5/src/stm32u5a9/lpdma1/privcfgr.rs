///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `PRIV(0-3)` reader - PRIV%s
pub type PRIV_R = crate::BitReader;
///Field `PRIV(0-3)` writer - PRIV%s
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///PRIV(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PRIV0` field.</div>
    #[inline(always)]
    pub fn priv_(&self, n: u8) -> PRIV_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PRIV_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///PRIV(0-3)
    #[inline(always)]
    pub fn priv__iter(&self) -> impl Iterator<Item = PRIV_R> + '_ {
        (0..4).map(move |n| PRIV_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - PRIV0
    #[inline(always)]
    pub fn priv0(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PRIV1
    #[inline(always)]
    pub fn priv1(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PRIV2
    #[inline(always)]
    pub fn priv2(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PRIV3
    #[inline(always)]
    pub fn priv3(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv0", &self.priv0())
            .field("priv1", &self.priv1())
            .field("priv2", &self.priv2())
            .field("priv3", &self.priv3())
            .finish()
    }
}
impl W {
    ///PRIV(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PRIV0` field.</div>
    #[inline(always)]
    pub fn priv_(&mut self, n: u8) -> PRIV_W<PRIVCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PRIV_W::new(self, n)
    }
    ///Bit 0 - PRIV0
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 0)
    }
    ///Bit 1 - PRIV1
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 1)
    }
    ///Bit 2 - PRIV2
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 2)
    }
    ///Bit 3 - PRIV3
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 3)
    }
}
/**LPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPDMA1:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
