///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `PRIV(0-15)` reader - PRIV%s
pub type PRIV_R = crate::BitReader;
///Field `PRIV(0-15)` writer - PRIV%s
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///PRIV(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PRIV0` field.</div>
    #[inline(always)]
    pub fn priv_(&self, n: u8) -> PRIV_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PRIV_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///PRIV(0-15)
    #[inline(always)]
    pub fn priv__iter(&self) -> impl Iterator<Item = PRIV_R> + '_ {
        (0..16).map(move |n| PRIV_R::new(((self.bits >> n) & 1) != 0))
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
    ///Bit 4 - PRIV4
    #[inline(always)]
    pub fn priv4(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PRIV5
    #[inline(always)]
    pub fn priv5(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRIV6
    #[inline(always)]
    pub fn priv6(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PRIV7
    #[inline(always)]
    pub fn priv7(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PRIV8
    #[inline(always)]
    pub fn priv8(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PRIV9
    #[inline(always)]
    pub fn priv9(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PRIV10
    #[inline(always)]
    pub fn priv10(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PRIV11
    #[inline(always)]
    pub fn priv11(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PRIV12
    #[inline(always)]
    pub fn priv12(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PRIV13
    #[inline(always)]
    pub fn priv13(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PRIV14
    #[inline(always)]
    pub fn priv14(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PRIV15
    #[inline(always)]
    pub fn priv15(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv0", &self.priv0())
            .field("priv1", &self.priv1())
            .field("priv2", &self.priv2())
            .field("priv3", &self.priv3())
            .field("priv4", &self.priv4())
            .field("priv5", &self.priv5())
            .field("priv6", &self.priv6())
            .field("priv7", &self.priv7())
            .field("priv8", &self.priv8())
            .field("priv9", &self.priv9())
            .field("priv10", &self.priv10())
            .field("priv11", &self.priv11())
            .field("priv12", &self.priv12())
            .field("priv13", &self.priv13())
            .field("priv14", &self.priv14())
            .field("priv15", &self.priv15())
            .finish()
    }
}
impl W {
    ///PRIV(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PRIV0` field.</div>
    #[inline(always)]
    pub fn priv_(&mut self, n: u8) -> PRIV_W<PRIVCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
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
    ///Bit 4 - PRIV4
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 4)
    }
    ///Bit 5 - PRIV5
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 5)
    }
    ///Bit 6 - PRIV6
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 6)
    }
    ///Bit 7 - PRIV7
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 7)
    }
    ///Bit 8 - PRIV8
    #[inline(always)]
    pub fn priv8(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 8)
    }
    ///Bit 9 - PRIV9
    #[inline(always)]
    pub fn priv9(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 9)
    }
    ///Bit 10 - PRIV10
    #[inline(always)]
    pub fn priv10(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 10)
    }
    ///Bit 11 - PRIV11
    #[inline(always)]
    pub fn priv11(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 11)
    }
    ///Bit 12 - PRIV12
    #[inline(always)]
    pub fn priv12(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 12)
    }
    ///Bit 13 - PRIV13
    #[inline(always)]
    pub fn priv13(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 13)
    }
    ///Bit 14 - PRIV14
    #[inline(always)]
    pub fn priv14(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 14)
    }
    ///Bit 15 - PRIV15
    #[inline(always)]
    pub fn priv15(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 15)
    }
}
/**GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPDMA1:PRIVCFGR)*/
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
