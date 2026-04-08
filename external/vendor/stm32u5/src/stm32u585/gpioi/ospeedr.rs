///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
///Field `OSPEED(0-15)` reader - Port x configuration pin %s
pub use crate::stm32u585::gpioa::ospeedr::OSPEED_R;
///Field `OSPEED(0-15)` writer - Port x configuration pin %s
pub use crate::stm32u585::gpioa::ospeedr::OSPEED_W;
///Port x configuration pin %s
pub use crate::stm32u585::gpioa::ospeedr::OUTPUT_SPEED;
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEED0` field.</div>
    #[inline(always)]
    pub fn ospeed(&self, n: u8) -> OSPEED_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEED_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn ospeed_iter(&self) -> impl Iterator<Item = OSPEED_R> + '_ {
        (0..16).map(move |n| OSPEED_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED_R {
        OSPEED_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed0", &self.ospeed0())
            .field("ospeed1", &self.ospeed1())
            .field("ospeed2", &self.ospeed2())
            .field("ospeed3", &self.ospeed3())
            .field("ospeed4", &self.ospeed4())
            .field("ospeed5", &self.ospeed5())
            .field("ospeed6", &self.ospeed6())
            .field("ospeed7", &self.ospeed7())
            .field("ospeed8", &self.ospeed8())
            .field("ospeed9", &self.ospeed9())
            .field("ospeed10", &self.ospeed10())
            .field("ospeed11", &self.ospeed11())
            .field("ospeed12", &self.ospeed12())
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEED0` field.</div>
    #[inline(always)]
    pub fn ospeed(&mut self, n: u8) -> OSPEED_W<OSPEEDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEED_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GPIOI:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSPEEDR to value 0
impl crate::Resettable for OSPEEDRrs {}
