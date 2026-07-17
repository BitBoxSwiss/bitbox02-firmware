///Register `PRIVCFGR3` reader
pub type R = crate::R<PRIVCFGR3rs>;
///Register `PRIVCFGR3` writer
pub type W = crate::W<PRIVCFGR3rs>;
///Field `MDF1PRIV` reader - privileged access mode for MDF1
pub type MDF1PRIV_R = crate::BitReader;
///Field `MDF1PRIV` writer - privileged access mode for MDF1
pub type MDF1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICPRIV` reader - privileged access mode for CORDIC
pub type CORDICPRIV_R = crate::BitReader;
///Field `CORDICPRIV` writer - privileged access mode for CORDIC
pub type CORDICPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACPRIV` reader - privileged access mode for FMAC
pub type FMACPRIV_R = crate::BitReader;
///Field `FMACPRIV` writer - privileged access mode for FMAC
pub type FMACPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCPRIV` reader - privileged access mode for CRC
pub type CRCPRIV_R = crate::BitReader;
///Field `CRCPRIV` writer - privileged access mode for CRC
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCPRIV` reader - privileged access mode for TSC
pub type TSCPRIV_R = crate::BitReader;
///Field `TSCPRIV` writer - privileged access mode for TSC
pub type TSCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DPRIV` reader - privileged access mode for register of DMA2D
pub type DMA2DPRIV_R = crate::BitReader;
///Field `DMA2DPRIV` writer - privileged access mode for register of DMA2D
pub type DMA2DPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_REGPRIV` reader - privileged access mode for ICACHE registers
pub type ICACHE_REGPRIV_R = crate::BitReader;
///Field `ICACHE_REGPRIV` writer - privileged access mode for ICACHE registers
pub type ICACHE_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_REGPRIV` reader - privileged access mode for DCACHE registers
pub type DCACHE_REGPRIV_R = crate::BitReader;
///Field `DCACHE_REGPRIV` writer - privileged access mode for DCACHE registers
pub type DCACHE_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1PRIV` reader - privileged access mode for ADC1
pub type ADC1PRIV_R = crate::BitReader;
///Field `ADC1PRIV` writer - privileged access mode for ADC1
pub type ADC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPRIV` reader - privileged access mode for DCMI
pub type DCMIPRIV_R = crate::BitReader;
///Field `DCMIPRIV` writer - privileged access mode for DCMI
pub type DCMIPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGFSPRIV` reader - privileged access mode for OTG_FS
pub type OTGFSPRIV_R = crate::BitReader;
///Field `OTGFSPRIV` writer - privileged access mode for OTG_FS
pub type OTGFSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESPRIV` reader - privileged access mode for AES
pub type AESPRIV_R = crate::BitReader;
///Field `AESPRIV` writer - privileged access mode for AES
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHPRIV` reader - privileged access mode for HASH
pub type HASHPRIV_R = crate::BitReader;
///Field `HASHPRIV` writer - privileged access mode for HASH
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGPRIV` reader - privileged access mode for RNG
pub type RNGPRIV_R = crate::BitReader;
///Field `RNGPRIV` writer - privileged access mode for RNG
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAPRIV` reader - privileged access mode for PKA
pub type PKAPRIV_R = crate::BitReader;
///Field `PKAPRIV` writer - privileged access mode for PKA
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESPRIV` reader - privileged access mode for SAES
pub type SAESPRIV_R = crate::BitReader;
///Field `SAESPRIV` writer - privileged access mode for SAES
pub type SAESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPIMPRIV` reader - privileged access mode for OCTOSPIM
pub type OCTOSPIMPRIV_R = crate::BitReader;
///Field `OCTOSPIMPRIV` writer - privileged access mode for OCTOSPIM
pub type OCTOSPIMPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1PRIV` reader - privileged access mode for SDMMC2
pub type SDMMC1PRIV_R = crate::BitReader;
///Field `SDMMC1PRIV` writer - privileged access mode for SDMMC2
pub type SDMMC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2PRIV` reader - privileged access mode for SDMMC1
pub type SDMMC2PRIV_R = crate::BitReader;
///Field `SDMMC2PRIV` writer - privileged access mode for SDMMC1
pub type SDMMC2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMC_REGPRIV` reader - privileged access mode for FSMC registers
pub type FSMC_REGPRIV_R = crate::BitReader;
///Field `FSMC_REGPRIV` writer - privileged access mode for FSMC registers
pub type FSMC_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1_REGPRIV` reader - privileged access mode for OCTOSPI1
pub type OCTOSPI1_REGPRIV_R = crate::BitReader;
///Field `OCTOSPI1_REGPRIV` writer - privileged access mode for OCTOSPI1
pub type OCTOSPI1_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI2_REGPRIV` reader - privileged access mode for OCTOSPI2
pub type OCTOSPI2_REGPRIV_R = crate::BitReader;
///Field `OCTOSPI2_REGPRIV` writer - privileged access mode for OCTOSPI2
pub type OCTOSPI2_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGPRIV` reader - privileged access mode for RAMCFG
pub type RAMCFGPRIV_R = crate::BitReader;
///Field `RAMCFGPRIV` writer - privileged access mode for RAMCFG
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for MDF1
    #[inline(always)]
    pub fn mdf1priv(&self) -> MDF1PRIV_R {
        MDF1PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for CORDIC
    #[inline(always)]
    pub fn cordicpriv(&self) -> CORDICPRIV_R {
        CORDICPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for FMAC
    #[inline(always)]
    pub fn fmacpriv(&self) -> FMACPRIV_R {
        FMACPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for TSC
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for register of DMA2D
    #[inline(always)]
    pub fn dma2dpriv(&self) -> DMA2DPRIV_R {
        DMA2DPRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regpriv(&self) -> ICACHE_REGPRIV_R {
        ICACHE_REGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for DCACHE registers
    #[inline(always)]
    pub fn dcache_regpriv(&self) -> DCACHE_REGPRIV_R {
        DCACHE_REGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for ADC1
    #[inline(always)]
    pub fn adc1priv(&self) -> ADC1PRIV_R {
        ADC1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for DCMI
    #[inline(always)]
    pub fn dcmipriv(&self) -> DCMIPRIV_R {
        DCMIPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - privileged access mode for OTG_FS
    #[inline(always)]
    pub fn otgfspriv(&self) -> OTGFSPRIV_R {
        OTGFSPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&self) -> SAESPRIV_R {
        SAESPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - privileged access mode for OCTOSPIM
    #[inline(always)]
    pub fn octospimpriv(&self) -> OCTOSPIMPRIV_R {
        OCTOSPIMPRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2priv(&self) -> SDMMC2PRIV_R {
        SDMMC2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for FSMC registers
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - privileged access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - privileged access mode for OCTOSPI2
    #[inline(always)]
    pub fn octospi2_regpriv(&self) -> OCTOSPI2_REGPRIV_R {
        OCTOSPI2_REGPRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - privileged access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR3")
            .field("mdf1priv", &self.mdf1priv())
            .field("cordicpriv", &self.cordicpriv())
            .field("fmacpriv", &self.fmacpriv())
            .field("crcpriv", &self.crcpriv())
            .field("tscpriv", &self.tscpriv())
            .field("dma2dpriv", &self.dma2dpriv())
            .field("icache_regpriv", &self.icache_regpriv())
            .field("dcache_regpriv", &self.dcache_regpriv())
            .field("adc1priv", &self.adc1priv())
            .field("dcmipriv", &self.dcmipriv())
            .field("otgfspriv", &self.otgfspriv())
            .field("aespriv", &self.aespriv())
            .field("hashpriv", &self.hashpriv())
            .field("rngpriv", &self.rngpriv())
            .field("pkapriv", &self.pkapriv())
            .field("saespriv", &self.saespriv())
            .field("octospimpriv", &self.octospimpriv())
            .field("sdmmc1priv", &self.sdmmc1priv())
            .field("sdmmc2priv", &self.sdmmc2priv())
            .field("fsmc_regpriv", &self.fsmc_regpriv())
            .field("octospi1_regpriv", &self.octospi1_regpriv())
            .field("octospi2_regpriv", &self.octospi2_regpriv())
            .field("ramcfgpriv", &self.ramcfgpriv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for MDF1
    #[inline(always)]
    pub fn mdf1priv(&mut self) -> MDF1PRIV_W<PRIVCFGR3rs> {
        MDF1PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for CORDIC
    #[inline(always)]
    pub fn cordicpriv(&mut self) -> CORDICPRIV_W<PRIVCFGR3rs> {
        CORDICPRIV_W::new(self, 1)
    }
    ///Bit 2 - privileged access mode for FMAC
    #[inline(always)]
    pub fn fmacpriv(&mut self) -> FMACPRIV_W<PRIVCFGR3rs> {
        FMACPRIV_W::new(self, 2)
    }
    ///Bit 3 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<PRIVCFGR3rs> {
        CRCPRIV_W::new(self, 3)
    }
    ///Bit 4 - privileged access mode for TSC
    #[inline(always)]
    pub fn tscpriv(&mut self) -> TSCPRIV_W<PRIVCFGR3rs> {
        TSCPRIV_W::new(self, 4)
    }
    ///Bit 5 - privileged access mode for register of DMA2D
    #[inline(always)]
    pub fn dma2dpriv(&mut self) -> DMA2DPRIV_W<PRIVCFGR3rs> {
        DMA2DPRIV_W::new(self, 5)
    }
    ///Bit 6 - privileged access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regpriv(&mut self) -> ICACHE_REGPRIV_W<PRIVCFGR3rs> {
        ICACHE_REGPRIV_W::new(self, 6)
    }
    ///Bit 7 - privileged access mode for DCACHE registers
    #[inline(always)]
    pub fn dcache_regpriv(&mut self) -> DCACHE_REGPRIV_W<PRIVCFGR3rs> {
        DCACHE_REGPRIV_W::new(self, 7)
    }
    ///Bit 8 - privileged access mode for ADC1
    #[inline(always)]
    pub fn adc1priv(&mut self) -> ADC1PRIV_W<PRIVCFGR3rs> {
        ADC1PRIV_W::new(self, 8)
    }
    ///Bit 9 - privileged access mode for DCMI
    #[inline(always)]
    pub fn dcmipriv(&mut self) -> DCMIPRIV_W<PRIVCFGR3rs> {
        DCMIPRIV_W::new(self, 9)
    }
    ///Bit 10 - privileged access mode for OTG_FS
    #[inline(always)]
    pub fn otgfspriv(&mut self) -> OTGFSPRIV_W<PRIVCFGR3rs> {
        OTGFSPRIV_W::new(self, 10)
    }
    ///Bit 11 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W<PRIVCFGR3rs> {
        AESPRIV_W::new(self, 11)
    }
    ///Bit 12 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<PRIVCFGR3rs> {
        HASHPRIV_W::new(self, 12)
    }
    ///Bit 13 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<PRIVCFGR3rs> {
        RNGPRIV_W::new(self, 13)
    }
    ///Bit 14 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<PRIVCFGR3rs> {
        PKAPRIV_W::new(self, 14)
    }
    ///Bit 15 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&mut self) -> SAESPRIV_W<PRIVCFGR3rs> {
        SAESPRIV_W::new(self, 15)
    }
    ///Bit 16 - privileged access mode for OCTOSPIM
    #[inline(always)]
    pub fn octospimpriv(&mut self) -> OCTOSPIMPRIV_W<PRIVCFGR3rs> {
        OCTOSPIMPRIV_W::new(self, 16)
    }
    ///Bit 17 - privileged access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<PRIVCFGR3rs> {
        SDMMC1PRIV_W::new(self, 17)
    }
    ///Bit 18 - privileged access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2priv(&mut self) -> SDMMC2PRIV_W<PRIVCFGR3rs> {
        SDMMC2PRIV_W::new(self, 18)
    }
    ///Bit 19 - privileged access mode for FSMC registers
    #[inline(always)]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W<PRIVCFGR3rs> {
        FSMC_REGPRIV_W::new(self, 19)
    }
    ///Bit 20 - privileged access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W<PRIVCFGR3rs> {
        OCTOSPI1_REGPRIV_W::new(self, 20)
    }
    ///Bit 21 - privileged access mode for OCTOSPI2
    #[inline(always)]
    pub fn octospi2_regpriv(&mut self) -> OCTOSPI2_REGPRIV_W<PRIVCFGR3rs> {
        OCTOSPI2_REGPRIV_W::new(self, 21)
    }
    ///Bit 22 - privileged access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<PRIVCFGR3rs> {
        RAMCFGPRIV_W::new(self, 22)
    }
}
/**TZSC privilege configuration register 3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_TZSC:PRIVCFGR3)*/
pub struct PRIVCFGR3rs;
impl crate::RegisterSpec for PRIVCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr3::R`](R) reader structure
impl crate::Readable for PRIVCFGR3rs {}
///`write(|w| ..)` method takes [`privcfgr3::W`](W) writer structure
impl crate::Writable for PRIVCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR3 to value 0
impl crate::Resettable for PRIVCFGR3rs {}
