///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
///Field `MODE(0-15)` reader - MODE%s
pub type MODE_R = crate::BitReader;
///Field `MODE(0-15)` writer - MODE%s
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///MODE(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE0` field.</div>
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MODE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///MODE(0-15)
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..16).map(move |n| MODE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - MODE0
    #[inline(always)]
    pub fn mode0(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MODE1
    #[inline(always)]
    pub fn mode1(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MODE2
    #[inline(always)]
    pub fn mode2(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MODE3
    #[inline(always)]
    pub fn mode3(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MODE4
    #[inline(always)]
    pub fn mode4(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MODE5
    #[inline(always)]
    pub fn mode5(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MODE6
    #[inline(always)]
    pub fn mode6(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MODE7
    #[inline(always)]
    pub fn mode7(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MODE8
    #[inline(always)]
    pub fn mode8(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MODE9
    #[inline(always)]
    pub fn mode9(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MODE10
    #[inline(always)]
    pub fn mode10(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MODE11
    #[inline(always)]
    pub fn mode11(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - MODE12
    #[inline(always)]
    pub fn mode12(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MODE13
    #[inline(always)]
    pub fn mode13(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MODE14
    #[inline(always)]
    pub fn mode14(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MODE15
    #[inline(always)]
    pub fn mode15(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("mode0", &self.mode0())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("mode4", &self.mode4())
            .field("mode5", &self.mode5())
            .field("mode6", &self.mode6())
            .field("mode7", &self.mode7())
            .field("mode8", &self.mode8())
            .field("mode9", &self.mode9())
            .field("mode10", &self.mode10())
            .field("mode11", &self.mode11())
            .field("mode12", &self.mode12())
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .finish()
    }
}
impl W {
    ///MODE(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE0` field.</div>
    #[inline(always)]
    pub fn mode(&mut self, n: u8) -> MODE_W<MODERrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MODE_W::new(self, n)
    }
    ///Bit 0 - MODE0
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 0)
    }
    ///Bit 1 - MODE1
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 1)
    }
    ///Bit 2 - MODE2
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 2)
    }
    ///Bit 3 - MODE3
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 3)
    }
    ///Bit 4 - MODE4
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 4)
    }
    ///Bit 5 - MODE5
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 5)
    }
    ///Bit 6 - MODE6
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 6)
    }
    ///Bit 7 - MODE7
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 7)
    }
    ///Bit 8 - MODE8
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 8)
    }
    ///Bit 9 - MODE9
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 9)
    }
    ///Bit 10 - MODE10
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 10)
    }
    ///Bit 11 - MODE11
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 11)
    }
    ///Bit 12 - MODE12
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 12)
    }
    ///Bit 13 - MODE13
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 13)
    }
    ///Bit 14 - MODE14
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 14)
    }
    ///Bit 15 - MODE15
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE_W<MODERrs> {
        MODE_W::new(self, 15)
    }
}
/**LPGPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPGPIO1:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MODER to value 0
impl crate::Resettable for MODERrs {}
