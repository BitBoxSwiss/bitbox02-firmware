///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
///Field `TRGO` reader - TRGO
pub type TRGO_R = crate::BitReader;
///Field `TRGO` writer - TRGO
pub type TRGO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILVNB` reader - ILVNB
pub type ILVNB_R = crate::FieldReader;
///Field `ILVNB` writer - ILVNB
pub type ILVNB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - TRGO
    #[inline(always)]
    pub fn trgo(&self) -> TRGO_R {
        TRGO_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - ILVNB
    #[inline(always)]
    pub fn ilvnb(&self) -> ILVNB_R {
        ILVNB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("trgo", &self.trgo())
            .field("ilvnb", &self.ilvnb())
            .finish()
    }
}
impl W {
    ///Bit 0 - TRGO
    #[inline(always)]
    pub fn trgo(&mut self) -> TRGO_W<GCRrs> {
        TRGO_W::new(self, 0)
    }
    ///Bits 4:7 - ILVNB
    #[inline(always)]
    pub fn ilvnb(&mut self) -> ILVNB_W<GCRrs> {
        ILVNB_W::new(self, 4)
    }
}
/**MDF global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#MDF1:GCR)*/
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GCRrs {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCRrs {}
