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
///Field `DMA2DSEC` reader - secure access mode for register of DMA2D
pub type DMA2DSEC_R = crate::BitReader;
///Field `DMA2DSEC` writer - secure access mode for register of DMA2D
pub type DMA2DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `OTGFSSEC` reader - secure access mode for OTG_FS
pub type OTGFSSEC_R = crate::BitReader;
///Field `OTGFSSEC` writer - secure access mode for OTG_FS
pub type OTGFSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHSEC` reader - secure access mode for HASH
pub type HASHSEC_R = crate::BitReader;
///Field `HASHSEC` writer - secure access mode for HASH
pub type HASHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEC` reader - secure access mode for RNG
pub type RNGSEC_R = crate::BitReader;
///Field `RNGSEC` writer - secure access mode for RNG
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPIMSEC` reader - secure access mode for OCTOSPIM
pub type OCTOSPIMSEC_R = crate::BitReader;
///Field `OCTOSPIMSEC` writer - secure access mode for OCTOSPIM
pub type OCTOSPIMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1SEC` reader - secure access mode for SDMMC2
pub type SDMMC1SEC_R = crate::BitReader;
///Field `SDMMC1SEC` writer - secure access mode for SDMMC2
pub type SDMMC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2SEC` reader - secure access mode for SDMMC1
pub type SDMMC2SEC_R = crate::BitReader;
///Field `SDMMC2SEC` writer - secure access mode for SDMMC1
pub type SDMMC2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMC_REGSEC` reader - secure access mode for FSMC registers
pub type FSMC_REGSEC_R = crate::BitReader;
///Field `FSMC_REGSEC` writer - secure access mode for FSMC registers
pub type FSMC_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1_REGSEC` reader - secure access mode for OCTOSPI1 registers
pub type OCTOSPI1_REGSEC_R = crate::BitReader;
///Field `OCTOSPI1_REGSEC` writer - secure access mode for OCTOSPI1 registers
pub type OCTOSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI2_REGSEC` reader - secure access mode for OCTOSPI2 registers
pub type OCTOSPI2_REGSEC_R = crate::BitReader;
///Field `OCTOSPI2_REGSEC` writer - secure access mode for OCTOSPI2 registers
pub type OCTOSPI2_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGSEC` reader - secure access mode for RAMCFG
pub type RAMCFGSEC_R = crate::BitReader;
///Field `RAMCFGSEC` writer - secure access mode for RAMCFG
pub type RAMCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPU2DSEC` reader - GPU2DSEC
pub type GPU2DSEC_R = crate::BitReader;
///Field `GPU2DSEC` writer - GPU2DSEC
pub type GPU2DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUSEC` reader - GFXMMUSEC
pub type GFXMMUSEC_R = crate::BitReader;
///Field `GFXMMUSEC` writer - GFXMMUSEC
pub type GFXMMUSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMU_REGSEC` reader - GFXMMU_REGSEC
pub type GFXMMU_REGSEC_R = crate::BitReader;
///Field `GFXMMU_REGSEC` writer - GFXMMU_REGSEC
pub type GFXMMU_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPI1_REGSEC` reader - HSPI1_REGSEC
pub type HSPI1_REGSEC_R = crate::BitReader;
///Field `HSPI1_REGSEC` writer - HSPI1_REGSEC
pub type HSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE2_REGSEC` reader - DCACHE2_REGSEC
pub type DCACHE2_REGSEC_R = crate::BitReader;
///Field `DCACHE2_REGSEC` writer - DCACHE2_REGSEC
pub type DCACHE2_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - secure access mode for register of DMA2D
    #[inline(always)]
    pub fn dma2dsec(&self) -> DMA2DSEC_R {
        DMA2DSEC_R::new(((self.bits >> 5) & 1) != 0)
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
    ///Bit 10 - secure access mode for OTG_FS
    #[inline(always)]
    pub fn otgfssec(&self) -> OTGFSSEC_R {
        OTGFSSEC_R::new(((self.bits >> 10) & 1) != 0)
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
    ///Bit 16 - secure access mode for OCTOSPIM
    #[inline(always)]
    pub fn octospimsec(&self) -> OCTOSPIMSEC_R {
        OCTOSPIMSEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - secure access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2sec(&self) -> SDMMC2SEC_R {
        SDMMC2SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - secure access mode for FSMC registers
    #[inline(always)]
    pub fn fsmc_regsec(&self) -> FSMC_REGSEC_R {
        FSMC_REGSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - secure access mode for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - secure access mode for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2_regsec(&self) -> OCTOSPI2_REGSEC_R {
        OCTOSPI2_REGSEC_R::new(((self.bits >> 21) & 1) != 0)
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
    ///Bit 24 - GFXMMUSEC
    #[inline(always)]
    pub fn gfxmmusec(&self) -> GFXMMUSEC_R {
        GFXMMUSEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GFXMMU_REGSEC
    #[inline(always)]
    pub fn gfxmmu_regsec(&self) -> GFXMMU_REGSEC_R {
        GFXMMU_REGSEC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - HSPI1_REGSEC
    #[inline(always)]
    pub fn hspi1_regsec(&self) -> HSPI1_REGSEC_R {
        HSPI1_REGSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DCACHE2_REGSEC
    #[inline(always)]
    pub fn dcache2_regsec(&self) -> DCACHE2_REGSEC_R {
        DCACHE2_REGSEC_R::new(((self.bits >> 27) & 1) != 0)
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
            .field("dma2dsec", &self.dma2dsec())
            .field("icache_regsec", &self.icache_regsec())
            .field("dcache1_regsec", &self.dcache1_regsec())
            .field("adc1sec", &self.adc1sec())
            .field("dcmisec", &self.dcmisec())
            .field("otgfssec", &self.otgfssec())
            .field("hashsec", &self.hashsec())
            .field("rngsec", &self.rngsec())
            .field("octospimsec", &self.octospimsec())
            .field("sdmmc1sec", &self.sdmmc1sec())
            .field("sdmmc2sec", &self.sdmmc2sec())
            .field("fsmc_regsec", &self.fsmc_regsec())
            .field("octospi1_regsec", &self.octospi1_regsec())
            .field("octospi2_regsec", &self.octospi2_regsec())
            .field("ramcfgsec", &self.ramcfgsec())
            .field("gpu2dsec", &self.gpu2dsec())
            .field("gfxmmusec", &self.gfxmmusec())
            .field("gfxmmu_regsec", &self.gfxmmu_regsec())
            .field("hspi1_regsec", &self.hspi1_regsec())
            .field("dcache2_regsec", &self.dcache2_regsec())
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
    ///Bit 5 - secure access mode for register of DMA2D
    #[inline(always)]
    pub fn dma2dsec(&mut self) -> DMA2DSEC_W<SECCFGR3rs> {
        DMA2DSEC_W::new(self, 5)
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
    ///Bit 10 - secure access mode for OTG_FS
    #[inline(always)]
    pub fn otgfssec(&mut self) -> OTGFSSEC_W<SECCFGR3rs> {
        OTGFSSEC_W::new(self, 10)
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
    ///Bit 16 - secure access mode for OCTOSPIM
    #[inline(always)]
    pub fn octospimsec(&mut self) -> OCTOSPIMSEC_W<SECCFGR3rs> {
        OCTOSPIMSEC_W::new(self, 16)
    }
    ///Bit 17 - secure access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<SECCFGR3rs> {
        SDMMC1SEC_W::new(self, 17)
    }
    ///Bit 18 - secure access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2sec(&mut self) -> SDMMC2SEC_W<SECCFGR3rs> {
        SDMMC2SEC_W::new(self, 18)
    }
    ///Bit 19 - secure access mode for FSMC registers
    #[inline(always)]
    pub fn fsmc_regsec(&mut self) -> FSMC_REGSEC_W<SECCFGR3rs> {
        FSMC_REGSEC_W::new(self, 19)
    }
    ///Bit 20 - secure access mode for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W<SECCFGR3rs> {
        OCTOSPI1_REGSEC_W::new(self, 20)
    }
    ///Bit 21 - secure access mode for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2_regsec(&mut self) -> OCTOSPI2_REGSEC_W<SECCFGR3rs> {
        OCTOSPI2_REGSEC_W::new(self, 21)
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
    ///Bit 24 - GFXMMUSEC
    #[inline(always)]
    pub fn gfxmmusec(&mut self) -> GFXMMUSEC_W<SECCFGR3rs> {
        GFXMMUSEC_W::new(self, 24)
    }
    ///Bit 25 - GFXMMU_REGSEC
    #[inline(always)]
    pub fn gfxmmu_regsec(&mut self) -> GFXMMU_REGSEC_W<SECCFGR3rs> {
        GFXMMU_REGSEC_W::new(self, 25)
    }
    ///Bit 26 - HSPI1_REGSEC
    #[inline(always)]
    pub fn hspi1_regsec(&mut self) -> HSPI1_REGSEC_W<SECCFGR3rs> {
        HSPI1_REGSEC_W::new(self, 26)
    }
    ///Bit 27 - DCACHE2_REGSEC
    #[inline(always)]
    pub fn dcache2_regsec(&mut self) -> DCACHE2_REGSEC_W<SECCFGR3rs> {
        DCACHE2_REGSEC_W::new(self, 27)
    }
}
/**TZSC secure configuration register 3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZSC:SECCFGR3)*/
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
