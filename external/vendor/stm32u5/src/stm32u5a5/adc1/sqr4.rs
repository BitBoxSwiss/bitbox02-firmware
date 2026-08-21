///Register `SQR4` reader
pub type R = crate::R<SQR4rs>;
///Register `SQR4` writer
pub type W = crate::W<SQR4rs>;
///Field `SQ(15-16)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(15-16)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///(15-16) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ15` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SQ_R::new(((self.bits >> (n * 6)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(15-16) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..2).map(move |n| SQ_R::new(((self.bits >> (n * 6)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 15 conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&self) -> SQ_R {
        SQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - 16 conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR4")
            .field("sq15", &self.sq15())
            .field("sq16", &self.sq16())
            .finish()
    }
}
impl W {
    ///(15-16) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ15` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<SQR4rs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SQ_W::new(self, n * 6)
    }
    ///Bits 0:4 - 15 conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 6:10 - 16 conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 6)
    }
}
/**ADC regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:SQR4)*/
pub struct SQR4rs;
impl crate::RegisterSpec for SQR4rs {
    type Ux = u32;
}
///`read()` method returns [`sqr4::R`](R) reader structure
impl crate::Readable for SQR4rs {}
///`write(|w| ..)` method takes [`sqr4::W`](W) writer structure
impl crate::Writable for SQR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR4 to value 0
impl crate::Resettable for SQR4rs {}
