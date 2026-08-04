///Register `ESCR` reader
pub type R = crate::R<ESCRrs>;
///Register `ESCR` writer
pub type W = crate::W<ESCRrs>;
///Field `FSC` reader - Frame start delimiter code
pub type FSC_R = crate::FieldReader;
///Field `FSC` writer - Frame start delimiter code
pub type FSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LSC` reader - Line start delimiter code
pub type LSC_R = crate::FieldReader;
///Field `LSC` writer - Line start delimiter code
pub type LSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LEC` reader - Line end delimiter code
pub type LEC_R = crate::FieldReader;
///Field `LEC` writer - Line end delimiter code
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FEC` reader - Frame end delimiter code
pub type FEC_R = crate::FieldReader;
///Field `FEC` writer - Frame end delimiter code
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Frame start delimiter code
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Line start delimiter code
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Line end delimiter code
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Frame end delimiter code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESCR")
            .field("fec", &self.fec())
            .field("lec", &self.lec())
            .field("lsc", &self.lsc())
            .field("fsc", &self.fsc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Frame start delimiter code
    #[inline(always)]
    pub fn fsc(&mut self) -> FSC_W<ESCRrs> {
        FSC_W::new(self, 0)
    }
    ///Bits 8:15 - Line start delimiter code
    #[inline(always)]
    pub fn lsc(&mut self) -> LSC_W<ESCRrs> {
        LSC_W::new(self, 8)
    }
    ///Bits 16:23 - Line end delimiter code
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<ESCRrs> {
        LEC_W::new(self, 16)
    }
    ///Bits 24:31 - Frame end delimiter code
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<ESCRrs> {
        FEC_W::new(self, 24)
    }
}
/**background offset register

You can [`read`](crate::Reg::read) this register and get [`escr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`escr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCMI:ESCR)*/
pub struct ESCRrs;
impl crate::RegisterSpec for ESCRrs {
    type Ux = u32;
}
///`read()` method returns [`escr::R`](R) reader structure
impl crate::Readable for ESCRrs {}
///`write(|w| ..)` method takes [`escr::W`](W) writer structure
impl crate::Writable for ESCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESCR to value 0
impl crate::Resettable for ESCRrs {}
