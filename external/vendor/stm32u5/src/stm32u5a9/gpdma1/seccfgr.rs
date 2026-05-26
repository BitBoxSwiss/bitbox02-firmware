///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC(0-15)` reader - SEC%s
pub type SEC_R = crate::BitReader;
///Field `SEC(0-15)` writer - SEC%s
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///SEC(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&self, n: u8) -> SEC_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        SEC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///SEC(0-15)
    #[inline(always)]
    pub fn sec_iter(&self) -> impl Iterator<Item = SEC_R> + '_ {
        (0..16).map(move |n| SEC_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - SEC0
    #[inline(always)]
    pub fn sec0(&self) -> SEC_R {
        SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    pub fn sec1(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    pub fn sec2(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    pub fn sec3(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SEC4
    #[inline(always)]
    pub fn sec4(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    pub fn sec5(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    pub fn sec6(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    pub fn sec7(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SEC8
    #[inline(always)]
    pub fn sec8(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SEC9
    #[inline(always)]
    pub fn sec9(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SEC10
    #[inline(always)]
    pub fn sec10(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SEC11
    #[inline(always)]
    pub fn sec11(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SEC12
    #[inline(always)]
    pub fn sec12(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SEC13
    #[inline(always)]
    pub fn sec13(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SEC14
    #[inline(always)]
    pub fn sec14(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SEC15
    #[inline(always)]
    pub fn sec15(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sec0", &self.sec0())
            .field("sec1", &self.sec1())
            .field("sec2", &self.sec2())
            .field("sec3", &self.sec3())
            .field("sec4", &self.sec4())
            .field("sec5", &self.sec5())
            .field("sec6", &self.sec6())
            .field("sec7", &self.sec7())
            .field("sec8", &self.sec8())
            .field("sec9", &self.sec9())
            .field("sec10", &self.sec10())
            .field("sec11", &self.sec11())
            .field("sec12", &self.sec12())
            .field("sec13", &self.sec13())
            .field("sec14", &self.sec14())
            .field("sec15", &self.sec15())
            .finish()
    }
}
impl W {
    ///SEC(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&mut self, n: u8) -> SEC_W<SECCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        SEC_W::new(self, n)
    }
    ///Bit 0 - SEC0
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 1)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 2)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 3)
    }
    ///Bit 4 - SEC4
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 4)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 5)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 6)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 7)
    }
    ///Bit 8 - SEC8
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 8)
    }
    ///Bit 9 - SEC9
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 9)
    }
    ///Bit 10 - SEC10
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 10)
    }
    ///Bit 11 - SEC11
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 11)
    }
    ///Bit 12 - SEC12
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 12)
    }
    ///Bit 13 - SEC13
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 13)
    }
    ///Bit 14 - SEC14
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 14)
    }
    ///Bit 15 - SEC15
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 15)
    }
}
/**GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPDMA1:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
