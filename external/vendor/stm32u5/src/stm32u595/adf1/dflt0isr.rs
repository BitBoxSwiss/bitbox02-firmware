///Register `DFLT0ISR` reader
pub type R = crate::R<DFLT0ISRrs>;
///Register `DFLT0ISR` writer
pub type W = crate::W<DFLT0ISRrs>;
///Field `FTHF` reader - RXFIFO threshold flag
pub type FTHF_R = crate::BitReader;
///Field `DOVRF` reader - Data overflow flag
pub type DOVRF_R = crate::BitReader;
///Field `DOVRF` writer - Data overflow flag
pub type DOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEF` reader - RXFIFO not empty flag
pub type RXNEF_R = crate::BitReader;
///Field `SATF` reader - Saturation detection flag
pub type SATF_R = crate::BitReader;
///Field `SATF` writer - Saturation detection flag
pub type SATF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABF` reader - Clock absence detection flag
pub type CKABF_R = crate::BitReader;
///Field `CKABF` writer - Clock absence detection flag
pub type CKABF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOVRF` reader - Reshape filter overrun detection flag
pub type RFOVRF_R = crate::BitReader;
///Field `RFOVRF` writer - Reshape filter overrun detection flag
pub type RFOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDDETF` reader - Sound activity detection flag
pub type SDDETF_R = crate::BitReader;
///Field `SDDETF` writer - Sound activity detection flag
pub type SDDETF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDLVLF` reader - Sound level value ready flag
pub type SDLVLF_R = crate::BitReader;
///Field `SDLVLF` writer - Sound level value ready flag
pub type SDLVLF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXFIFO threshold flag
    #[inline(always)]
    pub fn fthf(&self) -> FTHF_R {
        FTHF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow flag
    #[inline(always)]
    pub fn dovrf(&self) -> DOVRF_R {
        DOVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - RXFIFO not empty flag
    #[inline(always)]
    pub fn rxnef(&self) -> RXNEF_R {
        RXNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - Saturation detection flag
    #[inline(always)]
    pub fn satf(&self) -> SATF_R {
        SATF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection flag
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape filter overrun detection flag
    #[inline(always)]
    pub fn rfovrf(&self) -> RFOVRF_R {
        RFOVRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Sound activity detection flag
    #[inline(always)]
    pub fn sddetf(&self) -> SDDETF_R {
        SDDETF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Sound level value ready flag
    #[inline(always)]
    pub fn sdlvlf(&self) -> SDLVLF_R {
        SDLVLF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0ISR")
            .field("sdlvlf", &self.sdlvlf())
            .field("sddetf", &self.sddetf())
            .field("rfovrf", &self.rfovrf())
            .field("ckabf", &self.ckabf())
            .field("satf", &self.satf())
            .field("rxnef", &self.rxnef())
            .field("dovrf", &self.dovrf())
            .field("fthf", &self.fthf())
            .finish()
    }
}
impl W {
    ///Bit 1 - Data overflow flag
    #[inline(always)]
    pub fn dovrf(&mut self) -> DOVRF_W<DFLT0ISRrs> {
        DOVRF_W::new(self, 1)
    }
    ///Bit 9 - Saturation detection flag
    #[inline(always)]
    pub fn satf(&mut self) -> SATF_W<DFLT0ISRrs> {
        SATF_W::new(self, 9)
    }
    ///Bit 10 - Clock absence detection flag
    #[inline(always)]
    pub fn ckabf(&mut self) -> CKABF_W<DFLT0ISRrs> {
        CKABF_W::new(self, 10)
    }
    ///Bit 11 - Reshape filter overrun detection flag
    #[inline(always)]
    pub fn rfovrf(&mut self) -> RFOVRF_W<DFLT0ISRrs> {
        RFOVRF_W::new(self, 11)
    }
    ///Bit 12 - Sound activity detection flag
    #[inline(always)]
    pub fn sddetf(&mut self) -> SDDETF_W<DFLT0ISRrs> {
        SDDETF_W::new(self, 12)
    }
    ///Bit 13 - Sound level value ready flag
    #[inline(always)]
    pub fn sdlvlf(&mut self) -> SDLVLF_W<DFLT0ISRrs> {
        SDLVLF_W::new(self, 13)
    }
}
/**ADF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADF1:DFLT0ISR)*/
pub struct DFLT0ISRrs;
impl crate::RegisterSpec for DFLT0ISRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0isr::R`](R) reader structure
impl crate::Readable for DFLT0ISRrs {}
///`write(|w| ..)` method takes [`dflt0isr::W`](W) writer structure
impl crate::Writable for DFLT0ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0ISR to value 0
impl crate::Resettable for DFLT0ISRrs {}
