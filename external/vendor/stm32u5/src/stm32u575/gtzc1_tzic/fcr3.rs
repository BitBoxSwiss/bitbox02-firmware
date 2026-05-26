///Register `FCR3` writer
pub type W = crate::W<FCR3rs>;
///Field `CMDF1F` writer - clear the illegal access flag for MDF1
pub type CMDF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCORDICF` writer - clear the illegal access flag for CORDIC
pub type CCORDICF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFMACF` writer - clear the illegal access flag for FMAC
pub type CFMACF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRCF` writer - clear the illegal access flag for CRC
pub type CCRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - clear the illegal access flag for TSC
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMA2DF` writer - clear the illegal access flag for register of DMA2D
pub type CDMA2DF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CICACHEF` writer - clear the illegal access flag for ICACHE registers
pub type CICACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDCACHEF` writer - clear the illegal access flag for DCACHE registers
pub type CDCACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CADC1F` writer - clear the illegal access flag for ADC1
pub type CADC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDCMIF` writer - clear the illegal access flag for DCMI
pub type CDCMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COTGFSF` writer - clear the illegal access flag for OTG_FS
pub type COTGFSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAESF` writer - clear the illegal access flag for AES
pub type CAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHASHF` writer - clear the illegal access flag for HASH
pub type CHASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRNGF` writer - clear the illegal access flag for RNG
pub type CRNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPKAF` writer - clear the illegal access flag for PKA
pub type CPKAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSAESF` writer - clear the illegal access flag for SAES
pub type CSAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPIMF` writer - clear the illegal access flag for OCTOSPIM
pub type COCTOSPIMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSDMMC1F` writer - clear the illegal access flag for SDMMC2
pub type CSDMMC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSDMMC2F` writer - clear the illegal access flag for SDMMC1
pub type CSDMMC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFSMCF` writer - clear the illegal access flag for FSMC registers
pub type CFSMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI1F` writer - clear the illegal access flag for OCTOSPI1 registers
pub type COCTOSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI2F` writer - clear the illegal access flag for OCTOSPI2 registers
pub type COCTOSPI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRAMCFGF` writer - clear the illegal access flag for RAMCFG
pub type CRAMCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for MDF1
    #[inline(always)]
    pub fn cmdf1f(&mut self) -> CMDF1F_W<FCR3rs> {
        CMDF1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for CORDIC
    #[inline(always)]
    pub fn ccordicf(&mut self) -> CCORDICF_W<FCR3rs> {
        CCORDICF_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for FMAC
    #[inline(always)]
    pub fn cfmacf(&mut self) -> CFMACF_W<FCR3rs> {
        CFMACF_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for CRC
    #[inline(always)]
    pub fn ccrcf(&mut self) -> CCRCF_W<FCR3rs> {
        CCRCF_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for TSC
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<FCR3rs> {
        CTSCF_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for register of DMA2D
    #[inline(always)]
    pub fn cdma2df(&mut self) -> CDMA2DF_W<FCR3rs> {
        CDMA2DF_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for ICACHE registers
    #[inline(always)]
    pub fn cicachef(&mut self) -> CICACHEF_W<FCR3rs> {
        CICACHEF_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for DCACHE registers
    #[inline(always)]
    pub fn cdcachef(&mut self) -> CDCACHEF_W<FCR3rs> {
        CDCACHEF_W::new(self, 7)
    }
    ///Bit 8 - clear the illegal access flag for ADC1
    #[inline(always)]
    pub fn cadc1f(&mut self) -> CADC1F_W<FCR3rs> {
        CADC1F_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for DCMI
    #[inline(always)]
    pub fn cdcmif(&mut self) -> CDCMIF_W<FCR3rs> {
        CDCMIF_W::new(self, 9)
    }
    ///Bit 10 - clear the illegal access flag for OTG_FS
    #[inline(always)]
    pub fn cotgfsf(&mut self) -> COTGFSF_W<FCR3rs> {
        COTGFSF_W::new(self, 10)
    }
    ///Bit 11 - clear the illegal access flag for AES
    #[inline(always)]
    pub fn caesf(&mut self) -> CAESF_W<FCR3rs> {
        CAESF_W::new(self, 11)
    }
    ///Bit 12 - clear the illegal access flag for HASH
    #[inline(always)]
    pub fn chashf(&mut self) -> CHASHF_W<FCR3rs> {
        CHASHF_W::new(self, 12)
    }
    ///Bit 13 - clear the illegal access flag for RNG
    #[inline(always)]
    pub fn crngf(&mut self) -> CRNGF_W<FCR3rs> {
        CRNGF_W::new(self, 13)
    }
    ///Bit 14 - clear the illegal access flag for PKA
    #[inline(always)]
    pub fn cpkaf(&mut self) -> CPKAF_W<FCR3rs> {
        CPKAF_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for SAES
    #[inline(always)]
    pub fn csaesf(&mut self) -> CSAESF_W<FCR3rs> {
        CSAESF_W::new(self, 15)
    }
    ///Bit 16 - clear the illegal access flag for OCTOSPIM
    #[inline(always)]
    pub fn coctospimf(&mut self) -> COCTOSPIMF_W<FCR3rs> {
        COCTOSPIMF_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for SDMMC2
    #[inline(always)]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<FCR3rs> {
        CSDMMC1F_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for SDMMC1
    #[inline(always)]
    pub fn csdmmc2f(&mut self) -> CSDMMC2F_W<FCR3rs> {
        CSDMMC2F_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for FSMC registers
    #[inline(always)]
    pub fn cfsmcf(&mut self) -> CFSMCF_W<FCR3rs> {
        CFSMCF_W::new(self, 19)
    }
    ///Bit 20 - clear the illegal access flag for OCTOSPI1 registers
    #[inline(always)]
    pub fn coctospi1f(&mut self) -> COCTOSPI1F_W<FCR3rs> {
        COCTOSPI1F_W::new(self, 20)
    }
    ///Bit 21 - clear the illegal access flag for OCTOSPI2 registers
    #[inline(always)]
    pub fn coctospi2f(&mut self) -> COCTOSPI2F_W<FCR3rs> {
        COCTOSPI2F_W::new(self, 21)
    }
    ///Bit 22 - clear the illegal access flag for RAMCFG
    #[inline(always)]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<FCR3rs> {
        CRAMCFGF_W::new(self, 22)
    }
}
/**TZIC flag clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_TZIC:FCR3)*/
pub struct FCR3rs;
impl crate::RegisterSpec for FCR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr3::W`](W) writer structure
impl crate::Writable for FCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR3 to value 0
impl crate::Resettable for FCR3rs {}
