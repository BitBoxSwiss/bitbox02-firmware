///Register `SECCFGR3` reader
pub type R = crate::R<SECCFGR3rs>;
///Register `SECCFGR3` writer
pub type W = crate::W<SECCFGR3rs>;
///Field `MDF1SEC` reader - secure access mode for MDF1
pub type MDF1SEC_R = crate::BitReader;
///Field `MDF1SEC` writer - secure access mode for MDF1
pub type MDF1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICSEC` reader - secure access mode for CORDIC
pub type CORDICSEC_R = crate::BitReader;
///Field `CORDICSEC` writer - secure access mode for CORDIC
pub type CORDICSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACSEC` reader - secure access mode for FMAC
pub type FMACSEC_R = crate::BitReader;
///Field `FMACSEC` writer - secure access mode for FMAC
pub type FMACSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSEC` reader - secure access mode for CRC
pub type CRCSEC_R = crate::BitReader;
///Field `CRCSEC` writer - secure access mode for CRC
pub type CRCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCSEC` reader - secure access mode for TSC
pub type TSCSEC_R = crate::BitReader;
///Field `TSCSEC` writer - secure access mode for TSC
pub type TSCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_REGSEC` reader - secure access mode for ICACHE registers
pub type ICACHE_REGSEC_R = crate::BitReader;
///Field `ICACHE_REGSEC` writer - secure access mode for ICACHE registers
pub type ICACHE_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE1_REGSEC` reader - secure access mode for DCACHE1 registers
pub type DCACHE1_REGSEC_R = crate::BitReader;
///Field `DCACHE1_REGSEC` writer - secure access mode for DCACHE1 registers
pub type DCACHE1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1SEC` reader - secure access mode for ADC1
pub type ADC1SEC_R = crate::BitReader;
///Field `ADC1SEC` writer - secure access mode for ADC1
pub type ADC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMISEC` reader - secure access mode for DCMI
pub type DCMISEC_R = crate::BitReader;
///Field `DCMISEC` writer - secure access mode for DCMI
pub type DCMISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHSEC` reader - secure access mode for HASH
pub type HASHSEC_R = crate::BitReader;
///Field `HASHSEC` writer - secure access mode for HASH
pub type HASHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEC` reader - secure access mode for RNG
pub type RNGSEC_R = crate::BitReader;
///Field `RNGSEC` writer - secure access mode for RNG
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1SEC` reader - secure access mode
pub type SDMMC1SEC_R = crate::BitReader;
///Field `SDMMC1SEC` writer - secure access mode
pub type SDMMC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1_REGSEC` reader - secure access mode for OCTOSPI1 registers
pub type OCTOSPI1_REGSEC_R = crate::BitReader;
///Field `OCTOSPI1_REGSEC` writer - secure access mode for OCTOSPI1 registers
pub type OCTOSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGSEC` reader - secure access mode for RAMCFG
pub type RAMCFGSEC_R = crate::BitReader;
///Field `RAMCFGSEC` writer - secure access mode for RAMCFG
pub type RAMCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPU2DSEC` reader - GPU2DSEC
pub type GPU2DSEC_R = crate::BitReader;
///Field `GPU2DSEC` writer - GPU2DSEC
pub type GPU2DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPI1_REGSEC` reader - HSPI1_REGSEC
pub type HSPI1_REGSEC_R = crate::BitReader;
///Field `HSPI1_REGSEC` writer - HSPI1_REGSEC
pub type HSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for MDF1
    #[inline(always)]
    pub fn mdf1sec(&self) -> MDF1SEC_R {
        MDF1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for CORDIC
    #[inline(always)]
    pub fn cordicsec(&self) -> CORDICSEC_R {
        CORDICSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for FMAC
    #[inline(always)]
    pub fn fmacsec(&self) -> FMACSEC_R {
        FMACSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TSC
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - secure access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regsec(&self) -> ICACHE_REGSEC_R {
        ICACHE_REGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for DCACHE1 registers
    #[inline(always)]
    pub fn dcache1_regsec(&self) -> DCACHE1_REGSEC_R {
        DCACHE1_REGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for ADC1
    #[inline(always)]
    pub fn adc1sec(&self) -> ADC1SEC_R {
        ADC1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure access mode for DCMI
    #[inline(always)]
    pub fn dcmisec(&self) -> DCMISEC_R {
        DCMISEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - secure access mode
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - secure access mode for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - secure access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GPU2DSEC
    #[inline(always)]
    pub fn gpu2dsec(&self) -> GPU2DSEC_R {
        GPU2DSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 26 - HSPI1_REGSEC
    #[inline(always)]
    pub fn hspi1_regsec(&self) -> HSPI1_REGSEC_R {
        HSPI1_REGSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR3")
            .field("mdf1sec", &self.mdf1sec())
            .field("cordicsec", &self.cordicsec())
            .field("fmacsec", &self.fmacsec())
            .field("crcsec", &self.crcsec())
            .field("tscsec", &self.tscsec())
            .field("icache_regsec", &self.icache_regsec())
            .field("dcache1_regsec", &self.dcache1_regsec())
            .field("adc1sec", &self.adc1sec())
            .field("dcmisec", &self.dcmisec())
            .field("hashsec", &self.hashsec())
            .field("rngsec", &self.rngsec())
            .field("sdmmc1sec", &self.sdmmc1sec())
            .field("octospi1_regsec", &self.octospi1_regsec())
            .field("ramcfgsec", &self.ramcfgsec())
            .field("gpu2dsec", &self.gpu2dsec())
            .field("hspi1_regsec", &self.hspi1_regsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for MDF1
    #[inline(always)]
    pub fn mdf1sec(&mut self) -> MDF1SEC_W<SECCFGR3rs> {
        MDF1SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for CORDIC
    #[inline(always)]
    pub fn cordicsec(&mut self) -> CORDICSEC_W<SECCFGR3rs> {
        CORDICSEC_W::new(self, 1)
    }
    ///Bit 2 - secure access mode for FMAC
    #[inline(always)]
    pub fn fmacsec(&mut self) -> FMACSEC_W<SECCFGR3rs> {
        FMACSEC_W::new(self, 2)
    }
    ///Bit 3 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&mut self) -> CRCSEC_W<SECCFGR3rs> {
        CRCSEC_W::new(self, 3)
    }
    ///Bit 4 - secure access mode for TSC
    #[inline(always)]
    pub fn tscsec(&mut self) -> TSCSEC_W<SECCFGR3rs> {
        TSCSEC_W::new(self, 4)
    }
    ///Bit 6 - secure access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regsec(&mut self) -> ICACHE_REGSEC_W<SECCFGR3rs> {
        ICACHE_REGSEC_W::new(self, 6)
    }
    ///Bit 7 - secure access mode for DCACHE1 registers
    #[inline(always)]
    pub fn dcache1_regsec(&mut self) -> DCACHE1_REGSEC_W<SECCFGR3rs> {
        DCACHE1_REGSEC_W::new(self, 7)
    }
    ///Bit 8 - secure access mode for ADC1
    #[inline(always)]
    pub fn adc1sec(&mut self) -> ADC1SEC_W<SECCFGR3rs> {
        ADC1SEC_W::new(self, 8)
    }
    ///Bit 9 - secure access mode for DCMI
    #[inline(always)]
    pub fn dcmisec(&mut self) -> DCMISEC_W<SECCFGR3rs> {
        DCMISEC_W::new(self, 9)
    }
    ///Bit 12 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&mut self) -> HASHSEC_W<SECCFGR3rs> {
        HASHSEC_W::new(self, 12)
    }
    ///Bit 13 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W<SECCFGR3rs> {
        RNGSEC_W::new(self, 13)
    }
    ///Bit 17 - secure access mode
    #[inline(always)]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<SECCFGR3rs> {
        SDMMC1SEC_W::new(self, 17)
    }
    ///Bit 20 - secure access mode for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W<SECCFGR3rs> {
        OCTOSPI1_REGSEC_W::new(self, 20)
    }
    ///Bit 22 - secure access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<SECCFGR3rs> {
        RAMCFGSEC_W::new(self, 22)
    }
    ///Bit 23 - GPU2DSEC
    #[inline(always)]
    pub fn gpu2dsec(&mut self) -> GPU2DSEC_W<SECCFGR3rs> {
        GPU2DSEC_W::new(self, 23)
    }
    ///Bit 26 - HSPI1_REGSEC
    #[inline(always)]
    pub fn hspi1_regsec(&mut self) -> HSPI1_REGSEC_W<SECCFGR3rs> {
        HSPI1_REGSEC_W::new(self, 26)
    }
}
/**TZSC secure configuration register 3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GTZC1_TZSC:SECCFGR3)*/
pub struct SECCFGR3rs;
impl crate::RegisterSpec for SECCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr3::R`](R) reader structure
impl crate::Readable for SECCFGR3rs {}
///`write(|w| ..)` method takes [`seccfgr3::W`](W) writer structure
impl crate::Writable for SECCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR3 to value 0
impl crate::Resettable for SECCFGR3rs {}
