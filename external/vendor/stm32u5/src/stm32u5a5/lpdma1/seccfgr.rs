///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC(0-3)` reader - SEC%s
pub type SEC_R = crate::BitReader;
///Field `SEC(0-3)` writer - SEC%s
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///SEC(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&self, n: u8) -> SEC_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SEC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///SEC(0-3)
    #[inline(always)]
    pub fn sec_iter(&self) -> impl Iterator<Item = SEC_R> + '_ {
        (0..4).map(move |n| SEC_R::new(((self.bits >> n) & 1) != 0))
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sec0", &self.sec0())
            .field("sec1", &self.sec1())
            .field("sec2", &self.sec2())
            .field("sec3", &self.sec3())
            .finish()
    }
}
impl W {
    ///SEC(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&mut self, n: u8) -> SEC_W<SECCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
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
}
/**LPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#LPDMA1:SECCFGR)*/
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
