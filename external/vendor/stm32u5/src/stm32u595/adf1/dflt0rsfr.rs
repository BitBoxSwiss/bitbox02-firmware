///Register `DFLT0RSFR` reader
pub type R = crate::R<DFLT0RSFRrs>;
///Register `DFLT0RSFR` writer
pub type W = crate::W<DFLT0RSFRrs>;
///Field `RSFLTBYP` reader - Reshaper filter bypass
pub type RSFLTBYP_R = crate::BitReader;
///Field `RSFLTBYP` writer - Reshaper filter bypass
pub type RSFLTBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSFLTD` reader - Reshaper filter decimation ratio
pub type RSFLTD_R = crate::BitReader;
///Field `RSFLTD` writer - Reshaper filter decimation ratio
pub type RSFLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPFBYP` reader - High-pass filter bypass
pub type HPFBYP_R = crate::BitReader;
///Field `HPFBYP` writer - High-pass filter bypass
pub type HPFBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPFC` reader - High-pass filter cut-off frequency
pub type HPFC_R = crate::FieldReader;
///Field `HPFC` writer - High-pass filter cut-off frequency
pub type HPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Reshaper filter bypass
    #[inline(always)]
    pub fn rsfltbyp(&self) -> RSFLTBYP_R {
        RSFLTBYP_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio
    #[inline(always)]
    pub fn rsfltd(&self) -> RSFLTD_R {
        RSFLTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - High-pass filter bypass
    #[inline(always)]
    pub fn hpfbyp(&self) -> HPFBYP_R {
        HPFBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency
    #[inline(always)]
    pub fn hpfc(&self) -> HPFC_R {
        HPFC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0RSFR")
            .field("hpfc", &self.hpfc())
            .field("hpfbyp", &self.hpfbyp())
            .field("rsfltd", &self.rsfltd())
            .field("rsfltbyp", &self.rsfltbyp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reshaper filter bypass
    #[inline(always)]
    pub fn rsfltbyp(&mut self) -> RSFLTBYP_W<DFLT0RSFRrs> {
        RSFLTBYP_W::new(self, 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio
    #[inline(always)]
    pub fn rsfltd(&mut self) -> RSFLTD_W<DFLT0RSFRrs> {
        RSFLTD_W::new(self, 4)
    }
    ///Bit 7 - High-pass filter bypass
    #[inline(always)]
    pub fn hpfbyp(&mut self) -> HPFBYP_W<DFLT0RSFRrs> {
        HPFBYP_W::new(self, 7)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency
    #[inline(always)]
    pub fn hpfc(&mut self) -> HPFC_W<DFLT0RSFRrs> {
        HPFC_W::new(self, 8)
    }
}
/**ADF reshape filter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0rsfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0rsfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADF1:DFLT0RSFR)*/
pub struct DFLT0RSFRrs;
impl crate::RegisterSpec for DFLT0RSFRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0rsfr::R`](R) reader structure
impl crate::Readable for DFLT0RSFRrs {}
///`write(|w| ..)` method takes [`dflt0rsfr::W`](W) writer structure
impl crate::Writable for DFLT0RSFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0RSFR to value 0
impl crate::Resettable for DFLT0RSFRrs {}
