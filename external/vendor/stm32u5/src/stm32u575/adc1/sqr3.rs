///Register `SQR3` reader
pub type R = crate::R<SQR3rs>;
///Register `SQR3` writer
pub type W = crate::W<SQR3rs>;
///Field `SQ(10-14)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(10-14)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///(10-14) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ10` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SQ_R::new(((self.bits >> (n * 6)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(10-14) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..5).map(move |n| SQ_R::new(((self.bits >> (n * 6)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 10 conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&self) -> SQ_R {
        SQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - 11 conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - 12 conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - 13 conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - 14 conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR3")
            .field("sq10", &self.sq10())
            .field("sq11", &self.sq11())
            .field("sq12", &self.sq12())
            .field("sq13", &self.sq13())
            .field("sq14", &self.sq14())
            .finish()
    }
}
impl W {
    ///(10-14) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ10` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<SQR3rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SQ_W::new(self, n * 6)
    }
    ///Bits 0:4 - 10 conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 6:10 - 11 conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 6)
    }
    ///Bits 12:16 - 12 conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 12)
    }
    ///Bits 18:22 - 13 conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 18)
    }
    ///Bits 24:28 - 14 conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 24)
    }
}
/**ADC regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:SQR3)*/
pub struct SQR3rs;
impl crate::RegisterSpec for SQR3rs {
    type Ux = u32;
}
///`read()` method returns [`sqr3::R`](R) reader structure
impl crate::Readable for SQR3rs {}
///`write(|w| ..)` method takes [`sqr3::W`](W) writer structure
impl crate::Writable for SQR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR3 to value 0
impl crate::Resettable for SQR3rs {}
