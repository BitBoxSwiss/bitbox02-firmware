///Register `SECCFGR%s` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR%s` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC(0-31)` reader - SEC%s
pub type SEC_R = crate::BitReader;
///Field `SEC(0-31)` writer - SEC%s
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///SEC(0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&self, n: u8) -> SEC_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SEC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///SEC(0-31)
    #[inline(always)]
    pub fn sec_iter(&self) -> impl Iterator<Item = SEC_R> + '_ {
        (0..32).map(move |n| SEC_R::new(((self.bits >> n) & 1) != 0))
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
    ///Bit 16 - SEC16
    #[inline(always)]
    pub fn sec16(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SEC17
    #[inline(always)]
    pub fn sec17(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SEC18
    #[inline(always)]
    pub fn sec18(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SEC19
    #[inline(always)]
    pub fn sec19(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SEC20
    #[inline(always)]
    pub fn sec20(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SEC21
    #[inline(always)]
    pub fn sec21(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SEC22
    #[inline(always)]
    pub fn sec22(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SEC23
    #[inline(always)]
    pub fn sec23(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SEC24
    #[inline(always)]
    pub fn sec24(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SEC25
    #[inline(always)]
    pub fn sec25(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SEC26
    #[inline(always)]
    pub fn sec26(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SEC27
    #[inline(always)]
    pub fn sec27(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SEC28
    #[inline(always)]
    pub fn sec28(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SEC29
    #[inline(always)]
    pub fn sec29(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SEC30
    #[inline(always)]
    pub fn sec30(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SEC31
    #[inline(always)]
    pub fn sec31(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("sec16", &self.sec16())
            .field("sec17", &self.sec17())
            .field("sec18", &self.sec18())
            .field("sec19", &self.sec19())
            .field("sec20", &self.sec20())
            .field("sec21", &self.sec21())
            .field("sec22", &self.sec22())
            .field("sec23", &self.sec23())
            .field("sec24", &self.sec24())
            .field("sec25", &self.sec25())
            .field("sec26", &self.sec26())
            .field("sec27", &self.sec27())
            .field("sec28", &self.sec28())
            .field("sec29", &self.sec29())
            .field("sec30", &self.sec30())
            .field("sec31", &self.sec31())
            .finish()
    }
}
impl W {
    ///SEC(0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&mut self, n: u8) -> SEC_W<SECCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
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
    ///Bit 16 - SEC16
    #[inline(always)]
    pub fn sec16(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 16)
    }
    ///Bit 17 - SEC17
    #[inline(always)]
    pub fn sec17(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 17)
    }
    ///Bit 18 - SEC18
    #[inline(always)]
    pub fn sec18(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 18)
    }
    ///Bit 19 - SEC19
    #[inline(always)]
    pub fn sec19(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 19)
    }
    ///Bit 20 - SEC20
    #[inline(always)]
    pub fn sec20(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 20)
    }
    ///Bit 21 - SEC21
    #[inline(always)]
    pub fn sec21(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 21)
    }
    ///Bit 22 - SEC22
    #[inline(always)]
    pub fn sec22(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 22)
    }
    ///Bit 23 - SEC23
    #[inline(always)]
    pub fn sec23(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 23)
    }
    ///Bit 24 - SEC24
    #[inline(always)]
    pub fn sec24(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 24)
    }
    ///Bit 25 - SEC25
    #[inline(always)]
    pub fn sec25(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 25)
    }
    ///Bit 26 - SEC26
    #[inline(always)]
    pub fn sec26(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 26)
    }
    ///Bit 27 - SEC27
    #[inline(always)]
    pub fn sec27(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 27)
    }
    ///Bit 28 - SEC28
    #[inline(always)]
    pub fn sec28(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 28)
    }
    ///Bit 29 - SEC29
    #[inline(always)]
    pub fn sec29(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 29)
    }
    ///Bit 30 - SEC30
    #[inline(always)]
    pub fn sec30(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 30)
    }
    ///Bit 31 - SEC31
    #[inline(always)]
    pub fn sec31(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 31)
    }
}
/**MPCBBz security configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:SECCFGR[0])*/
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
///`reset()` method sets SECCFGR%s to value 0xffff_ffff
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
