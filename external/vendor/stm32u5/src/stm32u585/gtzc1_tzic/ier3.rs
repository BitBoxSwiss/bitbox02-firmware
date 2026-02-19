///Register `IER3` reader
pub type R = crate::R<IER3rs>;
///Register `IER3` writer
pub type W = crate::W<IER3rs>;
///Field `MDF1IE` reader - illegal access interrupt enable for MDF1
pub type MDF1IE_R = crate::BitReader;
///Field `MDF1IE` writer - illegal access interrupt enable for MDF1
pub type MDF1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICIE` reader - illegal access interrupt enable for CORDIC
pub type CORDICIE_R = crate::BitReader;
///Field `CORDICIE` writer - illegal access interrupt enable for CORDIC
pub type CORDICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACIE` reader - illegal access interrupt enable for FMAC
pub type FMACIE_R = crate::BitReader;
///Field `FMACIE` writer - illegal access interrupt enable for FMAC
pub type FMACIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCIE` reader - illegal access interrupt enable for CRC
pub type CRCIE_R = crate::BitReader;
///Field `CRCIE` writer - illegal access interrupt enable for CRC
pub type CRCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCIE` reader - illegal access interrupt enable for TSC
pub type TSCIE_R = crate::BitReader;
///Field `TSCIE` writer - illegal access interrupt enable for TSC
pub type TSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DIE` reader - illegal access interrupt enable for register of DMA2D
pub type DMA2DIE_R = crate::BitReader;
///Field `DMA2DIE` writer - illegal access interrupt enable for register of DMA2D
pub type DMA2DIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHEIE` reader - illegal access interrupt enable for ICACHE registers
pub type ICACHEIE_R = crate::BitReader;
///Field `ICACHEIE` writer - illegal access interrupt enable for ICACHE registers
pub type ICACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHEIE` reader - illegal access interrupt enable for DCACHE registers
pub type DCACHEIE_R = crate::BitReader;
///Field `DCACHEIE` writer - illegal access interrupt enable for DCACHE registers
pub type DCACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1IE` reader - illegal access interrupt enable for ADC1
pub type ADC1IE_R = crate::BitReader;
///Field `ADC1IE` writer - illegal access interrupt enable for ADC1
pub type ADC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIIE` reader - illegal access interrupt enable for DCMI
pub type DCMIIE_R = crate::BitReader;
///Field `DCMIIE` writer - illegal access interrupt enable for DCMI
pub type DCMIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGFSIE` reader - illegal access interrupt enable for OTG_FS
pub type OTGFSIE_R = crate::BitReader;
///Field `OTGFSIE` writer - illegal access interrupt enable for OTG_FS
pub type OTGFSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESIE` reader - illegal access interrupt enable for AES
pub type AESIE_R = crate::BitReader;
///Field `AESIE` writer - illegal access interrupt enable for AES
pub type AESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHIE` reader - illegal access interrupt enable for HASH
pub type HASHIE_R = crate::BitReader;
///Field `HASHIE` writer - illegal access interrupt enable for HASH
pub type HASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGIE` reader - illegal access interrupt enable for RNG
pub type RNGIE_R = crate::BitReader;
///Field `RNGIE` writer - illegal access interrupt enable for RNG
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAIE` reader - illegal access interrupt enable for PKA
pub type PKAIE_R = crate::BitReader;
///Field `PKAIE` writer - illegal access interrupt enable for PKA
pub type PKAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESIE` reader - illegal access interrupt enable for SAES
pub type SAESIE_R = crate::BitReader;
///Field `SAESIE` writer - illegal access interrupt enable for SAES
pub type SAESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPIMIE` reader - illegal access interrupt enable for OCTOSPIM
pub type OCTOSPIMIE_R = crate::BitReader;
///Field `OCTOSPIMIE` writer - illegal access interrupt enable for OCTOSPIM
pub type OCTOSPIMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1IE` reader - illegal access interrupt enable for SDMMC2
pub type SDMMC1IE_R = crate::BitReader;
///Field `SDMMC1IE` writer - illegal access interrupt enable for SDMMC2
pub type SDMMC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2IE` reader - illegal access interrupt enable for SDMMC1
pub type SDMMC2IE_R = crate::BitReader;
///Field `SDMMC2IE` writer - illegal access interrupt enable for SDMMC1
pub type SDMMC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMCIE` reader - illegal access interrupt enable for FSMC registers
pub type FSMCIE_R = crate::BitReader;
///Field `FSMCIE` writer - illegal access interrupt enable for FSMC registers
pub type FSMCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1IE` reader - illegal access interrupt enable for OCTOSPI1 registers
pub type OCTOSPI1IE_R = crate::BitReader;
///Field `OCTOSPI1IE` writer - illegal access interrupt enable for OCTOSPI1 registers
pub type OCTOSPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI2IE` reader - illegal access interrupt enable for OCTOSPI2 registers
pub type OCTOSPI2IE_R = crate::BitReader;
///Field `OCTOSPI2IE` writer - illegal access interrupt enable for OCTOSPI2 registers
pub type OCTOSPI2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGIE` reader - illegal access interrupt enable for RAMCFG
pub type RAMCFGIE_R = crate::BitReader;
///Field `RAMCFGIE` writer - illegal access interrupt enable for RAMCFG
pub type RAMCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for MDF1
    #[inline(always)]
    pub fn mdf1ie(&self) -> MDF1IE_R {
        MDF1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for CORDIC
    #[inline(always)]
    pub fn cordicie(&self) -> CORDICIE_R {
        CORDICIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for FMAC
    #[inline(always)]
    pub fn fmacie(&self) -> FMACIE_R {
        FMACIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for TSC
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for register of DMA2D
    #[inline(always)]
    pub fn dma2die(&self) -> DMA2DIE_R {
        DMA2DIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for ICACHE registers
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for DCACHE registers
    #[inline(always)]
    pub fn dcacheie(&self) -> DCACHEIE_R {
        DCACHEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for ADC1
    #[inline(always)]
    pub fn adc1ie(&self) -> ADC1IE_R {
        ADC1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for DCMI
    #[inline(always)]
    pub fn dcmiie(&self) -> DCMIIE_R {
        DCMIIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for OTG_FS
    #[inline(always)]
    pub fn otgfsie(&self) -> OTGFSIE_R {
        OTGFSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&self) -> SAESIE_R {
        SAESIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for OCTOSPIM
    #[inline(always)]
    pub fn octospimie(&self) -> OCTOSPIMIE_R {
        OCTOSPIMIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for SDMMC2
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for SDMMC1
    #[inline(always)]
    pub fn sdmmc2ie(&self) -> SDMMC2IE_R {
        SDMMC2IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for FSMC registers
    #[inline(always)]
    pub fn fsmcie(&self) -> FSMCIE_R {
        FSMCIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1ie(&self) -> OCTOSPI1IE_R {
        OCTOSPI1IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2ie(&self) -> OCTOSPI2IE_R {
        OCTOSPI2IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for RAMCFG
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER3")
            .field("mdf1ie", &self.mdf1ie())
            .field("cordicie", &self.cordicie())
            .field("fmacie", &self.fmacie())
            .field("crcie", &self.crcie())
            .field("tscie", &self.tscie())
            .field("dma2die", &self.dma2die())
            .field("icacheie", &self.icacheie())
            .field("dcacheie", &self.dcacheie())
            .field("adc1ie", &self.adc1ie())
            .field("dcmiie", &self.dcmiie())
            .field("otgfsie", &self.otgfsie())
            .field("aesie", &self.aesie())
            .field("hashie", &self.hashie())
            .field("rngie", &self.rngie())
            .field("pkaie", &self.pkaie())
            .field("saesie", &self.saesie())
            .field("octospimie", &self.octospimie())
            .field("sdmmc1ie", &self.sdmmc1ie())
            .field("sdmmc2ie", &self.sdmmc2ie())
            .field("fsmcie", &self.fsmcie())
            .field("octospi1ie", &self.octospi1ie())
            .field("octospi2ie", &self.octospi2ie())
            .field("ramcfgie", &self.ramcfgie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for MDF1
    #[inline(always)]
    pub fn mdf1ie(&mut self) -> MDF1IE_W<IER3rs> {
        MDF1IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for CORDIC
    #[inline(always)]
    pub fn cordicie(&mut self) -> CORDICIE_W<IER3rs> {
        CORDICIE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for FMAC
    #[inline(always)]
    pub fn fmacie(&mut self) -> FMACIE_W<IER3rs> {
        FMACIE_W::new(self, 2)
    }
    ///Bit 3 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&mut self) -> CRCIE_W<IER3rs> {
        CRCIE_W::new(self, 3)
    }
    ///Bit 4 - illegal access interrupt enable for TSC
    #[inline(always)]
    pub fn tscie(&mut self) -> TSCIE_W<IER3rs> {
        TSCIE_W::new(self, 4)
    }
    ///Bit 5 - illegal access interrupt enable for register of DMA2D
    #[inline(always)]
    pub fn dma2die(&mut self) -> DMA2DIE_W<IER3rs> {
        DMA2DIE_W::new(self, 5)
    }
    ///Bit 6 - illegal access interrupt enable for ICACHE registers
    #[inline(always)]
    pub fn icacheie(&mut self) -> ICACHEIE_W<IER3rs> {
        ICACHEIE_W::new(self, 6)
    }
    ///Bit 7 - illegal access interrupt enable for DCACHE registers
    #[inline(always)]
    pub fn dcacheie(&mut self) -> DCACHEIE_W<IER3rs> {
        DCACHEIE_W::new(self, 7)
    }
    ///Bit 8 - illegal access interrupt enable for ADC1
    #[inline(always)]
    pub fn adc1ie(&mut self) -> ADC1IE_W<IER3rs> {
        ADC1IE_W::new(self, 8)
    }
    ///Bit 9 - illegal access interrupt enable for DCMI
    #[inline(always)]
    pub fn dcmiie(&mut self) -> DCMIIE_W<IER3rs> {
        DCMIIE_W::new(self, 9)
    }
    ///Bit 10 - illegal access interrupt enable for OTG_FS
    #[inline(always)]
    pub fn otgfsie(&mut self) -> OTGFSIE_W<IER3rs> {
        OTGFSIE_W::new(self, 10)
    }
    ///Bit 11 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W<IER3rs> {
        AESIE_W::new(self, 11)
    }
    ///Bit 12 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&mut self) -> HASHIE_W<IER3rs> {
        HASHIE_W::new(self, 12)
    }
    ///Bit 13 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W<IER3rs> {
        RNGIE_W::new(self, 13)
    }
    ///Bit 14 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W<IER3rs> {
        PKAIE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&mut self) -> SAESIE_W<IER3rs> {
        SAESIE_W::new(self, 15)
    }
    ///Bit 16 - illegal access interrupt enable for OCTOSPIM
    #[inline(always)]
    pub fn octospimie(&mut self) -> OCTOSPIMIE_W<IER3rs> {
        OCTOSPIMIE_W::new(self, 16)
    }
    ///Bit 17 - illegal access interrupt enable for SDMMC2
    #[inline(always)]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<IER3rs> {
        SDMMC1IE_W::new(self, 17)
    }
    ///Bit 18 - illegal access interrupt enable for SDMMC1
    #[inline(always)]
    pub fn sdmmc2ie(&mut self) -> SDMMC2IE_W<IER3rs> {
        SDMMC2IE_W::new(self, 18)
    }
    ///Bit 19 - illegal access interrupt enable for FSMC registers
    #[inline(always)]
    pub fn fsmcie(&mut self) -> FSMCIE_W<IER3rs> {
        FSMCIE_W::new(self, 19)
    }
    ///Bit 20 - illegal access interrupt enable for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1ie(&mut self) -> OCTOSPI1IE_W<IER3rs> {
        OCTOSPI1IE_W::new(self, 20)
    }
    ///Bit 21 - illegal access interrupt enable for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2ie(&mut self) -> OCTOSPI2IE_W<IER3rs> {
        OCTOSPI2IE_W::new(self, 21)
    }
    ///Bit 22 - illegal access interrupt enable for RAMCFG
    #[inline(always)]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<IER3rs> {
        RAMCFGIE_W::new(self, 22)
    }
}
/**TZIC interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`ier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_TZIC:IER3)*/
pub struct IER3rs;
impl crate::RegisterSpec for IER3rs {
    type Ux = u32;
}
///`read()` method returns [`ier3::R`](R) reader structure
impl crate::Readable for IER3rs {}
///`write(|w| ..)` method takes [`ier3::W`](W) writer structure
impl crate::Writable for IER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER3 to value 0
impl crate::Resettable for IER3rs {}
