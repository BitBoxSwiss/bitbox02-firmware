///Register `SQR1` reader
pub type R = crate::R<SQR1rs>;
///Register `SQR1` writer
pub type W = crate::W<SQR1rs>;
///Field `L` reader - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type L_R = crate::FieldReader;
///Field `L` writer - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `SQ(1-4)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(1-4)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new((self.bits & 0x0f) as u8)
    }
    ///(1-4) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ1` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SQ_R::new(((self.bits >> (n * 6 + 6)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(1-4) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..4).map(move |n| SQ_R::new(((self.bits >> (n * 6 + 6)) & 0x1f) as u8))
    }
    ///Bits 6:10 - 1 conversion in regular sequence
    #[inline(always)]
    pub fn sq1(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - 2 conversion in regular sequence
    #[inline(always)]
    pub fn sq2(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - 3 conversion in regular sequence
    #[inline(always)]
    pub fn sq3(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - 4 conversion in regular sequence
    #[inline(always)]
    pub fn sq4(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR1")
            .field("l", &self.l())
            .field("sq1", &self.sq1())
            .field("sq2", &self.sq2())
            .field("sq3", &self.sq3())
            .field("sq4", &self.sq4())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn l(&mut self) -> L_W<SQR1rs> {
        L_W::new(self, 0)
    }
    ///(1-4) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ1` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<SQR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SQ_W::new(self, n * 6 + 6)
    }
    ///Bits 6:10 - 1 conversion in regular sequence
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ_W<SQR1rs> {
        SQ_W::new(self, 6)
    }
    ///Bits 12:16 - 2 conversion in regular sequence
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ_W<SQR1rs> {
        SQ_W::new(self, 12)
    }
    ///Bits 18:22 - 3 conversion in regular sequence
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ_W<SQR1rs> {
        SQ_W::new(self, 18)
    }
    ///Bits 24:28 - 4 conversion in regular sequence
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ_W<SQR1rs> {
        SQ_W::new(self, 24)
    }
}
/**ADC regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC1:SQR1)*/
pub struct SQR1rs;
impl crate::RegisterSpec for SQR1rs {
    type Ux = u32;
}
///`read()` method returns [`sqr1::R`](R) reader structure
impl crate::Readable for SQR1rs {}
///`write(|w| ..)` method takes [`sqr1::W`](W) writer structure
impl crate::Writable for SQR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR1 to value 0
impl crate::Resettable for SQR1rs {}
