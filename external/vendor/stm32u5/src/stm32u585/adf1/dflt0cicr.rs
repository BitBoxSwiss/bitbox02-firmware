///Register `DFLT0CICR` reader
pub type R = crate::R<DFLT0CICRrs>;
///Register `DFLT0CICR` writer
pub type W = crate::W<DFLT0CICRrs>;
///Field `DATSRC` reader - Source data for the digital filter
pub type DATSRC_R = crate::FieldReader;
///Field `DATSRC` writer - Source data for the digital filter
pub type DATSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CICMOD` reader - Select the CIC order
pub type CICMOD_R = crate::FieldReader;
///Field `CICMOD` writer - Select the CIC order
pub type CICMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCICD` reader - CIC decimation ratio selection
pub type MCICD_R = crate::FieldReader<u16>;
///Field `MCICD` writer - CIC decimation ratio selection
pub type MCICD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SCALE` reader - Scaling factor selection
pub type SCALE_R = crate::FieldReader;
///Field `SCALE` writer - Scaling factor selection
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1 - Source data for the digital filter
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Select the CIC order
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:16 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    ///Bits 20:25 - Scaling factor selection
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0CICR")
            .field("scale", &self.scale())
            .field("mcicd", &self.mcicd())
            .field("cicmod", &self.cicmod())
            .field("datsrc", &self.datsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source data for the digital filter
    #[inline(always)]
    pub fn datsrc(&mut self) -> DATSRC_W<DFLT0CICRrs> {
        DATSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Select the CIC order
    #[inline(always)]
    pub fn cicmod(&mut self) -> CICMOD_W<DFLT0CICRrs> {
        CICMOD_W::new(self, 4)
    }
    ///Bits 8:16 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd(&mut self) -> MCICD_W<DFLT0CICRrs> {
        MCICD_W::new(self, 8)
    }
    ///Bits 20:25 - Scaling factor selection
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<DFLT0CICRrs> {
        SCALE_W::new(self, 20)
    }
}
/**ADF digital filer configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADF1:DFLT0CICR)*/
pub struct DFLT0CICRrs;
impl crate::RegisterSpec for DFLT0CICRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0cicr::R`](R) reader structure
impl crate::Readable for DFLT0CICRrs {}
///`write(|w| ..)` method takes [`dflt0cicr::W`](W) writer structure
impl crate::Writable for DFLT0CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0CICR to value 0
impl crate::Resettable for DFLT0CICRrs {}
