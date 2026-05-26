///Register `SR3` reader
pub type R = crate::R<SR3rs>;
///Field `MDF1F` reader - illegal access flag for MDF1
pub type MDF1F_R = crate::BitReader;
///Field `CORDICF` reader - illegal access flag for CORDIC
pub type CORDICF_R = crate::BitReader;
///Field `FMACF` reader - illegal access flag for FMAC
pub type FMACF_R = crate::BitReader;
///Field `CRCF` reader - illegal access flag for CRC
pub type CRCF_R = crate::BitReader;
///Field `TSCF` reader - illegal access flag for TSC
pub type TSCF_R = crate::BitReader;
///Field `ICACHE_REGF` reader - illegal access flag for ICACHE registers
pub type ICACHE_REGF_R = crate::BitReader;
///Field `DCACHE1_REGF` reader - illegal access flag for DCACHE registers
pub type DCACHE1_REGF_R = crate::BitReader;
///Field `ADC12F` reader - illegal access flag for ADC1 and ADC2
pub type ADC12F_R = crate::BitReader;
///Field `DCMIF` reader - illegal access flag for DCMI
pub type DCMIF_R = crate::BitReader;
///Field `HASHF` reader - illegal access flag for HASH
pub type HASHF_R = crate::BitReader;
///Field `RNGF` reader - illegal access flag for RNG
pub type RNGF_R = crate::BitReader;
///Field `SDMMC1F` reader - illegal access flag
pub type SDMMC1F_R = crate::BitReader;
///Field `OCTOSPI1_REGF` reader - illegal access flag for OCTOSPI1 registers
pub type OCTOSPI1_REGF_R = crate::BitReader;
///Field `RAMCFGF` reader - illegal access flag for RAMCFG
pub type RAMCFGF_R = crate::BitReader;
///Field `GPU2DF` reader - illegal access flag for GPU2D
pub type GPU2DF_R = crate::BitReader;
///Field `HSPI1_REGF` reader - illegal access flag for HSPI1 registers
pub type HSPI1_REGF_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for MDF1
    #[inline(always)]
    pub fn mdf1f(&self) -> MDF1F_R {
        MDF1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for CORDIC
    #[inline(always)]
    pub fn cordicf(&self) -> CORDICF_R {
        CORDICF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for FMAC
    #[inline(always)]
    pub fn fmacf(&self) -> FMACF_R {
        FMACF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for CRC
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TSC
    #[inline(always)]
    pub fn tscf(&self) -> TSCF_R {
        TSCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for ICACHE registers
    #[inline(always)]
    pub fn icache_regf(&self) -> ICACHE_REGF_R {
        ICACHE_REGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for DCACHE registers
    #[inline(always)]
    pub fn dcache1_regf(&self) -> DCACHE1_REGF_R {
        DCACHE1_REGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12f(&self) -> ADC12F_R {
        ADC12F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for DCMI
    #[inline(always)]
    pub fn dcmif(&self) -> DCMIF_R {
        DCMIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for HASH
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for RNG
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - illegal access flag
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - illegal access flag for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1_regf(&self) -> OCTOSPI1_REGF_R {
        OCTOSPI1_REGF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for RAMCFG
    #[inline(always)]
    pub fn ramcfgf(&self) -> RAMCFGF_R {
        RAMCFGF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access flag for GPU2D
    #[inline(always)]
    pub fn gpu2df(&self) -> GPU2DF_R {
        GPU2DF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for HSPI1 registers
    #[inline(always)]
    pub fn hspi1_regf(&self) -> HSPI1_REGF_R {
        HSPI1_REGF_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR3")
            .field("mdf1f", &self.mdf1f())
            .field("cordicf", &self.cordicf())
            .field("fmacf", &self.fmacf())
            .field("crcf", &self.crcf())
            .field("tscf", &self.tscf())
            .field("icache_regf", &self.icache_regf())
            .field("dcache1_regf", &self.dcache1_regf())
            .field("adc12f", &self.adc12f())
            .field("dcmif", &self.dcmif())
            .field("hashf", &self.hashf())
            .field("rngf", &self.rngf())
            .field("sdmmc1f", &self.sdmmc1f())
            .field("octospi1_regf", &self.octospi1_regf())
            .field("ramcfgf", &self.ramcfgf())
            .field("gpu2df", &self.gpu2df())
            .field("hspi1_regf", &self.hspi1_regf())
            .finish()
    }
}
/**TZIC status register 3

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GTZC1_TZIC:SR3)*/
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
///`read()` method returns [`sr3::R`](R) reader structure
impl crate::Readable for SR3rs {}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3rs {}
