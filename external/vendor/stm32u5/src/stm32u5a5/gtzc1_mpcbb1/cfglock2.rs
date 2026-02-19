///Register `CFGLOCK2` reader
pub type R = crate::R<CFGLOCK2rs>;
///Register `CFGLOCK2` writer
pub type W = crate::W<CFGLOCK2rs>;
///Field `SPLCK(32-51)` reader - SPLCK%s
pub type SPLCK_R = crate::BitReader;
///Field `SPLCK(32-51)` writer - SPLCK%s
pub type SPLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///SPLCK(32-51)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK32` field.</div>
    #[inline(always)]
    pub fn splck(&self, n: u8) -> SPLCK_R {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        SPLCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///SPLCK(32-51)
    #[inline(always)]
    pub fn splck_iter(&self) -> impl Iterator<Item = SPLCK_R> + '_ {
        (0..20).map(move |n| SPLCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - SPLCK32
    #[inline(always)]
    pub fn splck32(&self) -> SPLCK_R {
        SPLCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPLCK33
    #[inline(always)]
    pub fn splck33(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPLCK34
    #[inline(always)]
    pub fn splck34(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SPLCK35
    #[inline(always)]
    pub fn splck35(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPLCK36
    #[inline(always)]
    pub fn splck36(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPLCK37
    #[inline(always)]
    pub fn splck37(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPLCK38
    #[inline(always)]
    pub fn splck38(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPLCK39
    #[inline(always)]
    pub fn splck39(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPLCK40
    #[inline(always)]
    pub fn splck40(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPLCK41
    #[inline(always)]
    pub fn splck41(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPLCK42
    #[inline(always)]
    pub fn splck42(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SPLCK43
    #[inline(always)]
    pub fn splck43(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPLCK44
    #[inline(always)]
    pub fn splck44(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPLCK45
    #[inline(always)]
    pub fn splck45(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPLCK46
    #[inline(always)]
    pub fn splck46(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPLCK47
    #[inline(always)]
    pub fn splck47(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPLCK48
    #[inline(always)]
    pub fn splck48(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SPLCK49
    #[inline(always)]
    pub fn splck49(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SPLCK50
    #[inline(always)]
    pub fn splck50(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPLCK51
    #[inline(always)]
    pub fn splck51(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGLOCK2")
            .field("splck32", &self.splck32())
            .field("splck33", &self.splck33())
            .field("splck34", &self.splck34())
            .field("splck35", &self.splck35())
            .field("splck36", &self.splck36())
            .field("splck37", &self.splck37())
            .field("splck38", &self.splck38())
            .field("splck39", &self.splck39())
            .field("splck40", &self.splck40())
            .field("splck41", &self.splck41())
            .field("splck42", &self.splck42())
            .field("splck43", &self.splck43())
            .field("splck44", &self.splck44())
            .field("splck45", &self.splck45())
            .field("splck46", &self.splck46())
            .field("splck47", &self.splck47())
            .field("splck48", &self.splck48())
            .field("splck49", &self.splck49())
            .field("splck50", &self.splck50())
            .field("splck51", &self.splck51())
            .finish()
    }
}
impl W {
    ///SPLCK(32-51)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK32` field.</div>
    #[inline(always)]
    pub fn splck(&mut self, n: u8) -> SPLCK_W<CFGLOCK2rs> {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        SPLCK_W::new(self, n)
    }
    ///Bit 0 - SPLCK32
    #[inline(always)]
    pub fn splck32(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 0)
    }
    ///Bit 1 - SPLCK33
    #[inline(always)]
    pub fn splck33(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 1)
    }
    ///Bit 2 - SPLCK34
    #[inline(always)]
    pub fn splck34(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 2)
    }
    ///Bit 3 - SPLCK35
    #[inline(always)]
    pub fn splck35(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 3)
    }
    ///Bit 4 - SPLCK36
    #[inline(always)]
    pub fn splck36(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 4)
    }
    ///Bit 5 - SPLCK37
    #[inline(always)]
    pub fn splck37(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 5)
    }
    ///Bit 6 - SPLCK38
    #[inline(always)]
    pub fn splck38(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 6)
    }
    ///Bit 7 - SPLCK39
    #[inline(always)]
    pub fn splck39(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 7)
    }
    ///Bit 8 - SPLCK40
    #[inline(always)]
    pub fn splck40(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 8)
    }
    ///Bit 9 - SPLCK41
    #[inline(always)]
    pub fn splck41(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 9)
    }
    ///Bit 10 - SPLCK42
    #[inline(always)]
    pub fn splck42(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 10)
    }
    ///Bit 11 - SPLCK43
    #[inline(always)]
    pub fn splck43(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 11)
    }
    ///Bit 12 - SPLCK44
    #[inline(always)]
    pub fn splck44(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 12)
    }
    ///Bit 13 - SPLCK45
    #[inline(always)]
    pub fn splck45(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 13)
    }
    ///Bit 14 - SPLCK46
    #[inline(always)]
    pub fn splck46(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 14)
    }
    ///Bit 15 - SPLCK47
    #[inline(always)]
    pub fn splck47(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 15)
    }
    ///Bit 16 - SPLCK48
    #[inline(always)]
    pub fn splck48(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 16)
    }
    ///Bit 17 - SPLCK49
    #[inline(always)]
    pub fn splck49(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 17)
    }
    ///Bit 18 - SPLCK50
    #[inline(always)]
    pub fn splck50(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 18)
    }
    ///Bit 19 - SPLCK51
    #[inline(always)]
    pub fn splck51(&mut self) -> SPLCK_W<CFGLOCK2rs> {
        SPLCK_W::new(self, 19)
    }
}
/**GTZC1 SRAMz MPCBB configuration lock register 2

You can [`read`](crate::Reg::read) this register and get [`cfglock2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglock2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GTZC1_MPCBB1:CFGLOCK2)*/
pub struct CFGLOCK2rs;
impl crate::RegisterSpec for CFGLOCK2rs {
    type Ux = u32;
}
///`read()` method returns [`cfglock2::R`](R) reader structure
impl crate::Readable for CFGLOCK2rs {}
///`write(|w| ..)` method takes [`cfglock2::W`](W) writer structure
impl crate::Writable for CFGLOCK2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGLOCK2 to value 0
impl crate::Resettable for CFGLOCK2rs {}
