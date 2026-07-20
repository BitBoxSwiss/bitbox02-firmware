///Register `SR4` reader
pub type R = crate::R<SR4rs>;
///Field `GPDMA1F` reader - illegal access flag for GPDMA1
pub type GPDMA1F_R = crate::BitReader;
///Field `FLASH_REGF` reader - illegal access flag for FLASH registers
pub type FLASH_REGF_R = crate::BitReader;
///Field `FLASHF` reader - illegal access flag for FLASH memory
pub type FLASHF_R = crate::BitReader;
///Field `TZSC1F` reader - illegal access flag for GTZC1 TZSC registers
pub type TZSC1F_R = crate::BitReader;
///Field `TZIC1F` reader - illegal access flag for GTZC1 TZIC registers
pub type TZIC1F_R = crate::BitReader;
///Field `OCTOSPI1_MEMF` reader - illegal access flag for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMF_R = crate::BitReader;
///Field `FSMC_MEMF` reader - illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3 (FSMC NOR)
pub type FSMC_MEMF_R = crate::BitReader;
///Field `BKPSRAMF` reader - illegal access flag for MPCWM3 (BKPSRAM) memory bank
pub type BKPSRAMF_R = crate::BitReader;
///Field `OCTOSPI2_MEMF` reader - illegal access flag for OCTOSPI2 memory bank
pub type OCTOSPI2_MEMF_R = crate::BitReader;
///Field `HSPI1_MEMF` reader - illegal access flag for HSPI1 memory bank
pub type HSPI1_MEMF_R = crate::BitReader;
///Field `SRAM1F` reader - illegal access flag for SRAM1
pub type SRAM1F_R = crate::BitReader;
///Field `MPCBB1_REGF` reader - illegal access flag for MPCBB1 registers
pub type MPCBB1_REGF_R = crate::BitReader;
///Field `SRAM2F` reader - illegal access flag for SRAM2
pub type SRAM2F_R = crate::BitReader;
///Field `MPCBB2_REGF` reader - illegal access flag for MPCBB2 registers
pub type MPCBB2_REGF_R = crate::BitReader;
///Field `SRAM3F` reader - illegal access flag for SRAM3
pub type SRAM3F_R = crate::BitReader;
///Field `MPCBB3_REGF` reader - illegal access flag for MPCBB3 registers
pub type MPCBB3_REGF_R = crate::BitReader;
///Field `SRAM5F` reader - illegal access flag for SRAM5
pub type SRAM5F_R = crate::BitReader;
///Field `MPCBB5_REGF` reader - illegal access flag for MPCBB5 registers
pub type MPCBB5_REGF_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for GPDMA1
    #[inline(always)]
    pub fn gpdma1f(&self) -> GPDMA1F_R {
        GPDMA1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for FLASH registers
    #[inline(always)]
    pub fn flash_regf(&self) -> FLASH_REGF_R {
        FLASH_REGF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for FLASH memory
    #[inline(always)]
    pub fn flashf(&self) -> FLASHF_R {
        FLASHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for GTZC1 TZSC registers
    #[inline(always)]
    pub fn tzsc1f(&self) -> TZSC1F_R {
        TZSC1F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for GTZC1 TZIC registers
    #[inline(always)]
    pub fn tzic1f(&self) -> TZIC1F_R {
        TZIC1F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn octospi1_memf(&self) -> OCTOSPI1_MEMF_R {
        OCTOSPI1_MEMF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3 (FSMC NOR)
    #[inline(always)]
    pub fn fsmc_memf(&self) -> FSMC_MEMF_R {
        FSMC_MEMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn bkpsramf(&self) -> BKPSRAMF_R {
        BKPSRAMF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for OCTOSPI2 memory bank
    #[inline(always)]
    pub fn octospi2_memf(&self) -> OCTOSPI2_MEMF_R {
        OCTOSPI2_MEMF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access flag for HSPI1 memory bank
    #[inline(always)]
    pub fn hspi1_memf(&self) -> HSPI1_MEMF_R {
        HSPI1_MEMF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SRAM1
    #[inline(always)]
    pub fn sram1f(&self) -> SRAM1F_R {
        SRAM1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for MPCBB1 registers
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for SRAM2
    #[inline(always)]
    pub fn sram2f(&self) -> SRAM2F_R {
        SRAM2F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access flag for MPCBB2 registers
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access flag for SRAM3
    #[inline(always)]
    pub fn sram3f(&self) -> SRAM3F_R {
        SRAM3F_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access flag for MPCBB3 registers
    #[inline(always)]
    pub fn mpcbb3_regf(&self) -> MPCBB3_REGF_R {
        MPCBB3_REGF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access flag for SRAM5
    #[inline(always)]
    pub fn sram5f(&self) -> SRAM5F_R {
        SRAM5F_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access flag for MPCBB5 registers
    #[inline(always)]
    pub fn mpcbb5_regf(&self) -> MPCBB5_REGF_R {
        MPCBB5_REGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR4")
            .field("gpdma1f", &self.gpdma1f())
            .field("flash_regf", &self.flash_regf())
            .field("flashf", &self.flashf())
            .field("tzsc1f", &self.tzsc1f())
            .field("tzic1f", &self.tzic1f())
            .field("octospi1_memf", &self.octospi1_memf())
            .field("fsmc_memf", &self.fsmc_memf())
            .field("bkpsramf", &self.bkpsramf())
            .field("octospi2_memf", &self.octospi2_memf())
            .field("hspi1_memf", &self.hspi1_memf())
            .field("sram1f", &self.sram1f())
            .field("mpcbb1_regf", &self.mpcbb1_regf())
            .field("sram2f", &self.sram2f())
            .field("mpcbb2_regf", &self.mpcbb2_regf())
            .field("sram3f", &self.sram3f())
            .field("mpcbb3_regf", &self.mpcbb3_regf())
            .field("sram5f", &self.sram5f())
            .field("mpcbb5_regf", &self.mpcbb5_regf())
            .finish()
    }
}
/**TZIC status register 4

You can [`read`](crate::Reg::read) this register and get [`sr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GTZC1_TZIC:SR4)*/
pub struct SR4rs;
impl crate::RegisterSpec for SR4rs {
    type Ux = u32;
}
///`read()` method returns [`sr4::R`](R) reader structure
impl crate::Readable for SR4rs {}
///`reset()` method sets SR4 to value 0
impl crate::Resettable for SR4rs {}
