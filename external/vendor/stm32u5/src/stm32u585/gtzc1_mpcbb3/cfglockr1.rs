///Register `CFGLOCKR1` reader
pub type R = crate::R<CFGLOCKR1rs>;
///Register `CFGLOCKR1` writer
pub type W = crate::W<CFGLOCKR1rs>;
///Field `SPLCK(0-31)` reader - SPLCK%s
pub type SPLCK_R = crate::BitReader;
///Field `SPLCK(0-31)` writer - SPLCK%s
pub type SPLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///SPLCK(0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK0` field.</div>
    #[inline(always)]
    pub fn splck(&self, n: u8) -> SPLCK_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SPLCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///SPLCK(0-31)
    #[inline(always)]
    pub fn splck_iter(&self) -> impl Iterator<Item = SPLCK_R> + '_ {
        (0..32).map(move |n| SPLCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - SPLCK0
    #[inline(always)]
    pub fn splck0(&self) -> SPLCK_R {
        SPLCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPLCK1
    #[inline(always)]
    pub fn splck1(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPLCK2
    #[inline(always)]
    pub fn splck2(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SPLCK3
    #[inline(always)]
    pub fn splck3(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPLCK4
    #[inline(always)]
    pub fn splck4(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPLCK5
    #[inline(always)]
    pub fn splck5(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPLCK6
    #[inline(always)]
    pub fn splck6(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPLCK7
    #[inline(always)]
    pub fn splck7(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPLCK8
    #[inline(always)]
    pub fn splck8(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPLCK9
    #[inline(always)]
    pub fn splck9(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPLCK10
    #[inline(always)]
    pub fn splck10(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SPLCK11
    #[inline(always)]
    pub fn splck11(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPLCK12
    #[inline(always)]
    pub fn splck12(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPLCK13
    #[inline(always)]
    pub fn splck13(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPLCK14
    #[inline(always)]
    pub fn splck14(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPLCK15
    #[inline(always)]
    pub fn splck15(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPLCK16
    #[inline(always)]
    pub fn splck16(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SPLCK17
    #[inline(always)]
    pub fn splck17(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SPLCK18
    #[inline(always)]
    pub fn splck18(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPLCK19
    #[inline(always)]
    pub fn splck19(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPLCK20
    #[inline(always)]
    pub fn splck20(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SPLCK21
    #[inline(always)]
    pub fn splck21(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SPLCK22
    #[inline(always)]
    pub fn splck22(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SPLCK23
    #[inline(always)]
    pub fn splck23(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SPLCK24
    #[inline(always)]
    pub fn splck24(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SPLCK25
    #[inline(always)]
    pub fn splck25(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SPLCK26
    #[inline(always)]
    pub fn splck26(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SPLCK27
    #[inline(always)]
    pub fn splck27(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SPLCK28
    #[inline(always)]
    pub fn splck28(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SPLCK29
    #[inline(always)]
    pub fn splck29(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SPLCK30
    #[inline(always)]
    pub fn splck30(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SPLCK31
    #[inline(always)]
    pub fn splck31(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGLOCKR1")
            .field("splck0", &self.splck0())
            .field("splck1", &self.splck1())
            .field("splck2", &self.splck2())
            .field("splck3", &self.splck3())
            .field("splck4", &self.splck4())
            .field("splck5", &self.splck5())
            .field("splck6", &self.splck6())
            .field("splck7", &self.splck7())
            .field("splck8", &self.splck8())
            .field("splck9", &self.splck9())
            .field("splck10", &self.splck10())
            .field("splck11", &self.splck11())
            .field("splck12", &self.splck12())
            .field("splck13", &self.splck13())
            .field("splck14", &self.splck14())
            .field("splck15", &self.splck15())
            .field("splck16", &self.splck16())
            .field("splck17", &self.splck17())
            .field("splck18", &self.splck18())
            .field("splck19", &self.splck19())
            .field("splck20", &self.splck20())
            .field("splck21", &self.splck21())
            .field("splck22", &self.splck22())
            .field("splck23", &self.splck23())
            .field("splck24", &self.splck24())
            .field("splck25", &self.splck25())
            .field("splck26", &self.splck26())
            .field("splck27", &self.splck27())
            .field("splck28", &self.splck28())
            .field("splck29", &self.splck29())
            .field("splck30", &self.splck30())
            .field("splck31", &self.splck31())
            .finish()
    }
}
impl W {
    ///SPLCK(0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK0` field.</div>
    #[inline(always)]
    pub fn splck(&mut self, n: u8) -> SPLCK_W<CFGLOCKR1rs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SPLCK_W::new(self, n)
    }
    ///Bit 0 - SPLCK0
    #[inline(always)]
    pub fn splck0(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 0)
    }
    ///Bit 1 - SPLCK1
    #[inline(always)]
    pub fn splck1(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 1)
    }
    ///Bit 2 - SPLCK2
    #[inline(always)]
    pub fn splck2(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 2)
    }
    ///Bit 3 - SPLCK3
    #[inline(always)]
    pub fn splck3(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 3)
    }
    ///Bit 4 - SPLCK4
    #[inline(always)]
    pub fn splck4(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 4)
    }
    ///Bit 5 - SPLCK5
    #[inline(always)]
    pub fn splck5(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 5)
    }
    ///Bit 6 - SPLCK6
    #[inline(always)]
    pub fn splck6(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 6)
    }
    ///Bit 7 - SPLCK7
    #[inline(always)]
    pub fn splck7(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 7)
    }
    ///Bit 8 - SPLCK8
    #[inline(always)]
    pub fn splck8(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 8)
    }
    ///Bit 9 - SPLCK9
    #[inline(always)]
    pub fn splck9(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 9)
    }
    ///Bit 10 - SPLCK10
    #[inline(always)]
    pub fn splck10(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 10)
    }
    ///Bit 11 - SPLCK11
    #[inline(always)]
    pub fn splck11(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 11)
    }
    ///Bit 12 - SPLCK12
    #[inline(always)]
    pub fn splck12(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 12)
    }
    ///Bit 13 - SPLCK13
    #[inline(always)]
    pub fn splck13(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 13)
    }
    ///Bit 14 - SPLCK14
    #[inline(always)]
    pub fn splck14(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 14)
    }
    ///Bit 15 - SPLCK15
    #[inline(always)]
    pub fn splck15(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 15)
    }
    ///Bit 16 - SPLCK16
    #[inline(always)]
    pub fn splck16(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 16)
    }
    ///Bit 17 - SPLCK17
    #[inline(always)]
    pub fn splck17(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 17)
    }
    ///Bit 18 - SPLCK18
    #[inline(always)]
    pub fn splck18(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 18)
    }
    ///Bit 19 - SPLCK19
    #[inline(always)]
    pub fn splck19(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 19)
    }
    ///Bit 20 - SPLCK20
    #[inline(always)]
    pub fn splck20(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 20)
    }
    ///Bit 21 - SPLCK21
    #[inline(always)]
    pub fn splck21(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 21)
    }
    ///Bit 22 - SPLCK22
    #[inline(always)]
    pub fn splck22(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 22)
    }
    ///Bit 23 - SPLCK23
    #[inline(always)]
    pub fn splck23(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 23)
    }
    ///Bit 24 - SPLCK24
    #[inline(always)]
    pub fn splck24(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 24)
    }
    ///Bit 25 - SPLCK25
    #[inline(always)]
    pub fn splck25(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 25)
    }
    ///Bit 26 - SPLCK26
    #[inline(always)]
    pub fn splck26(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 26)
    }
    ///Bit 27 - SPLCK27
    #[inline(always)]
    pub fn splck27(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 27)
    }
    ///Bit 28 - SPLCK28
    #[inline(always)]
    pub fn splck28(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 28)
    }
    ///Bit 29 - SPLCK29
    #[inline(always)]
    pub fn splck29(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 29)
    }
    ///Bit 30 - SPLCK30
    #[inline(always)]
    pub fn splck30(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 30)
    }
    ///Bit 31 - SPLCK31
    #[inline(always)]
    pub fn splck31(&mut self) -> SPLCK_W<CFGLOCKR1rs> {
        SPLCK_W::new(self, 31)
    }
}
/**GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`cfglockr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglockr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_MPCBB3:CFGLOCKR1)*/
pub struct CFGLOCKR1rs;
impl crate::RegisterSpec for CFGLOCKR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfglockr1::R`](R) reader structure
impl crate::Readable for CFGLOCKR1rs {}
///`write(|w| ..)` method takes [`cfglockr1::W`](W) writer structure
impl crate::Writable for CFGLOCKR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGLOCKR1 to value 0
impl crate::Resettable for CFGLOCKR1rs {}
