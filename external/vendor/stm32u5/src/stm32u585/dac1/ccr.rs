///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `OTRIM(1-2)` reader - DAC channel%s offset trimming value
pub type OTRIM_R = crate::FieldReader;
///Field `OTRIM(1-2)` writer - DAC channel%s offset trimming value
pub type OTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///DAC channel(1-2) offset trimming value
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OTRIM1` field.</div>
    #[inline(always)]
    pub fn otrim(&self, n: u8) -> OTRIM_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OTRIM_R::new(((self.bits >> (n * 16)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) offset trimming value
    #[inline(always)]
    pub fn otrim_iter(&self) -> impl Iterator<Item = OTRIM_R> + '_ {
        (0..2).map(move |n| OTRIM_R::new(((self.bits >> (n * 16)) & 0x1f) as u8))
    }
    ///Bits 0:4 - DAC channel1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM_R {
        OTRIM_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 16:20 - DAC channel2 offset trimming value
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM_R {
        OTRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("otrim1", &self.otrim1())
            .field("otrim2", &self.otrim2())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) offset trimming value
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OTRIM1` field.</div>
    #[inline(always)]
    pub fn otrim(&mut self, n: u8) -> OTRIM_W<CCRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OTRIM_W::new(self, n * 16)
    }
    ///Bits 0:4 - DAC channel1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&mut self) -> OTRIM_W<CCRrs> {
        OTRIM_W::new(self, 0)
    }
    ///Bits 16:20 - DAC channel2 offset trimming value
    #[inline(always)]
    pub fn otrim2(&mut self) -> OTRIM_W<CCRrs> {
        OTRIM_W::new(self, 16)
    }
}
/**DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DAC1:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
