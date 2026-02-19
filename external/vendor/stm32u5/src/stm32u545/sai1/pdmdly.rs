///Register `PDMDLY` reader
pub type R = crate::R<PDMDLYrs>;
///Register `PDMDLY` writer
pub type W = crate::W<PDMDLYrs>;
///Field `DLYM1L` reader - Delay line adjust for first microphone of pair 1
pub type DLYM1L_R = crate::FieldReader;
///Field `DLYM1L` writer - Delay line adjust for first microphone of pair 1
pub type DLYM1L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM1R` reader - Delay line adjust for second microphone of pair 1
pub type DLYM1R_R = crate::FieldReader;
///Field `DLYM1R` writer - Delay line adjust for second microphone of pair 1
pub type DLYM1R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM2L` reader - Delay line for first microphone of pair 2
pub type DLYM2L_R = crate::FieldReader;
///Field `DLYM2L` writer - Delay line for first microphone of pair 2
pub type DLYM2L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM2R` reader - Delay line for second microphone of pair 2
pub type DLYM2R_R = crate::FieldReader;
///Field `DLYM2R` writer - Delay line for second microphone of pair 2
pub type DLYM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM3L` reader - DLYM3L
pub type DLYM3L_R = crate::FieldReader;
///Field `DLYM3L` writer - DLYM3L
pub type DLYM3L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM3R` reader - DLYM3R
pub type DLYM3R_R = crate::FieldReader;
///Field `DLYM3R` writer - DLYM3R
pub type DLYM3R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM4L` reader - DLYM4L
pub type DLYM4L_R = crate::FieldReader;
///Field `DLYM4L` writer - DLYM4L
pub type DLYM4L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYM4R` reader - DLYM4R
pub type DLYM4R_R = crate::FieldReader;
///Field `DLYM4R` writer - DLYM4R
pub type DLYM4R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Delay line adjust for first microphone of pair 1
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Delay line adjust for second microphone of pair 1
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Delay line for first microphone of pair 2
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Delay line for second microphone of pair 2
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - DLYM3L
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - DLYM3R
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - DLYM4L
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - DLYM4R
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMDLY")
            .field("dlym1l", &self.dlym1l())
            .field("dlym1r", &self.dlym1r())
            .field("dlym2l", &self.dlym2l())
            .field("dlym2r", &self.dlym2r())
            .field("dlym3l", &self.dlym3l())
            .field("dlym3r", &self.dlym3r())
            .field("dlym4l", &self.dlym4l())
            .field("dlym4r", &self.dlym4r())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Delay line adjust for first microphone of pair 1
    #[inline(always)]
    pub fn dlym1l(&mut self) -> DLYM1L_W<PDMDLYrs> {
        DLYM1L_W::new(self, 0)
    }
    ///Bits 4:6 - Delay line adjust for second microphone of pair 1
    #[inline(always)]
    pub fn dlym1r(&mut self) -> DLYM1R_W<PDMDLYrs> {
        DLYM1R_W::new(self, 4)
    }
    ///Bits 8:10 - Delay line for first microphone of pair 2
    #[inline(always)]
    pub fn dlym2l(&mut self) -> DLYM2L_W<PDMDLYrs> {
        DLYM2L_W::new(self, 8)
    }
    ///Bits 12:14 - Delay line for second microphone of pair 2
    #[inline(always)]
    pub fn dlym2r(&mut self) -> DLYM2R_W<PDMDLYrs> {
        DLYM2R_W::new(self, 12)
    }
    ///Bits 16:18 - DLYM3L
    #[inline(always)]
    pub fn dlym3l(&mut self) -> DLYM3L_W<PDMDLYrs> {
        DLYM3L_W::new(self, 16)
    }
    ///Bits 20:22 - DLYM3R
    #[inline(always)]
    pub fn dlym3r(&mut self) -> DLYM3R_W<PDMDLYrs> {
        DLYM3R_W::new(self, 20)
    }
    ///Bits 24:26 - DLYM4L
    #[inline(always)]
    pub fn dlym4l(&mut self) -> DLYM4L_W<PDMDLYrs> {
        DLYM4L_W::new(self, 24)
    }
    ///Bits 28:30 - DLYM4R
    #[inline(always)]
    pub fn dlym4r(&mut self) -> DLYM4R_W<PDMDLYrs> {
        DLYM4R_W::new(self, 28)
    }
}
/**PDM delay register

You can [`read`](crate::Reg::read) this register and get [`pdmdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SAI1:PDMDLY)*/
pub struct PDMDLYrs;
impl crate::RegisterSpec for PDMDLYrs {
    type Ux = u32;
}
///`read()` method returns [`pdmdly::R`](R) reader structure
impl crate::Readable for PDMDLYrs {}
///`write(|w| ..)` method takes [`pdmdly::W`](W) writer structure
impl crate::Writable for PDMDLYrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDMDLY to value 0
impl crate::Resettable for PDMDLYrs {}
