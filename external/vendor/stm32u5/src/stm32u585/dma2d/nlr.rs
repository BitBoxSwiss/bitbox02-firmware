///Register `NLR` reader
pub type R = crate::R<NLRrs>;
///Register `NLR` writer
pub type W = crate::W<NLRrs>;
///Field `NL` reader - Number of lines
pub type NL_R = crate::FieldReader<u16>;
///Field `NL` writer - Number of lines
pub type NL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `PL` reader - Pixel per lines
pub type PL_R = crate::FieldReader<u16>;
///Field `PL` writer - Pixel per lines
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Number of lines
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:29 - Pixel per lines
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NLR")
            .field("pl", &self.pl())
            .field("nl", &self.nl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Number of lines
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W<NLRrs> {
        NL_W::new(self, 0)
    }
    ///Bits 16:29 - Pixel per lines
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<NLRrs> {
        PL_W::new(self, 16)
    }
}
/**number of line register

You can [`read`](crate::Reg::read) this register and get [`nlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DMA2D:NLR)*/
pub struct NLRrs;
impl crate::RegisterSpec for NLRrs {
    type Ux = u32;
}
///`read()` method returns [`nlr::R`](R) reader structure
impl crate::Readable for NLRrs {}
///`write(|w| ..)` method takes [`nlr::W`](W) writer structure
impl crate::Writable for NLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NLR to value 0
impl crate::Resettable for NLRrs {}
