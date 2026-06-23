///Register `SQR2` reader
pub type R = crate::R<SQR2rs>;
///Register `SQR2` writer
pub type W = crate::W<SQR2rs>;
///Field `SQ(5-9)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(5-9)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///(5-9) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ5` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SQ_R::new(((self.bits >> (n * 6)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(5-9) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..5).map(move |n| SQ_R::new(((self.bits >> (n * 6)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 5 conversion in regular sequence
    #[inline(always)]
    pub fn sq5(&self) -> SQ_R {
        SQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - 6 conversion in regular sequence
    #[inline(always)]
    pub fn sq6(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - 7 conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - 8 conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - 9 conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR2")
            .field("sq5", &self.sq5())
            .field("sq6", &self.sq6())
            .field("sq7", &self.sq7())
            .field("sq8", &self.sq8())
            .field("sq9", &self.sq9())
            .finish()
    }
}
impl W {
    ///(5-9) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ5` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<SQR2rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SQ_W::new(self, n * 6)
    }
    ///Bits 0:4 - 5 conversion in regular sequence
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ_W<SQR2rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 6:10 - 6 conversion in regular sequence
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ_W<SQR2rs> {
        SQ_W::new(self, 6)
    }
    ///Bits 12:16 - 7 conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ_W<SQR2rs> {
        SQ_W::new(self, 12)
    }
    ///Bits 18:22 - 8 conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ_W<SQR2rs> {
        SQ_W::new(self, 18)
    }
    ///Bits 24:28 - 9 conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ_W<SQR2rs> {
        SQ_W::new(self, 24)
    }
}
/**ADC regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:SQR2)*/
pub struct SQR2rs;
impl crate::RegisterSpec for SQR2rs {
    type Ux = u32;
}
///`read()` method returns [`sqr2::R`](R) reader structure
impl crate::Readable for SQR2rs {}
///`write(|w| ..)` method takes [`sqr2::W`](W) writer structure
impl crate::Writable for SQR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR2 to value 0
impl crate::Resettable for SQR2rs {}
