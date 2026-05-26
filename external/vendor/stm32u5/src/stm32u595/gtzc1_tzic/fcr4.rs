///Register `FCR4` writer
pub type W = crate::W<FCR4rs>;
///Field `CGPDMA1F` writer - clear the illegal access flag for GPDMA1
pub type CGPDMA1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFLASH_REGF` writer - clear the illegal access flag for FLASH registers
pub type CFLASH_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFLASHF` writer - clear the illegal access flag for FLASH memory
pub type CFLASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTZSC1F` writer - clear the illegal access flag for GTZC1 TZSC registers
pub type CTZSC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTZIC1F` writer - clear the illegal access flag for GTZC1 TZIC registers
pub type CTZIC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI1_MEMF` writer - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank
pub type COCTOSPI1_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFSMC_MEMF` writer - clear the illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3
pub type CFSMC_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBKPSRAMF` writer - clear the illegal access flag for MPCWM3 (BKPSRAM) memory bank
pub type CBKPSRAMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI2_MEMF` writer - clear the illegal access flag for OCTOSPI2 memory bank
pub type COCTOSPI2_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSPI1_MEMF` writer - clear the illegal access flag for HSPI1 memory bank
pub type CHSPI1_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM1F` writer - clear the illegal access flag for SRAM1
pub type CSRAM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB1_REGF` writer - clear the illegal access flag for MPCBB1 registers
pub type CMPCBB1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM2F` writer - clear the illegal access flag for SRAM2
pub type CSRAM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB2_REGF` writer - clear the illegal access flag for MPCBB2 registers
pub type CMPCBB2_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM3F` writer - clear the illegal access flag for SRAM3
pub type CSRAM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB3_REGF` writer - clear the illegal access flag for MPCBB3 registers
pub type CMPCBB3_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM5F` writer - clear the illegal access flag for SRAM5
pub type CSRAM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB5_REGF` writer - clear the illegal access flag for MPCBB5 registers
pub type CMPCBB5_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for GPDMA1
    #[inline(always)]
    pub fn cgpdma1f(&mut self) -> CGPDMA1F_W<FCR4rs> {
        CGPDMA1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for FLASH registers
    #[inline(always)]
    pub fn cflash_regf(&mut self) -> CFLASH_REGF_W<FCR4rs> {
        CFLASH_REGF_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for FLASH memory
    #[inline(always)]
    pub fn cflashf(&mut self) -> CFLASHF_W<FCR4rs> {
        CFLASHF_W::new(self, 2)
    }
    ///Bit 14 - clear the illegal access flag for GTZC1 TZSC registers
    #[inline(always)]
    pub fn ctzsc1f(&mut self) -> CTZSC1F_W<FCR4rs> {
        CTZSC1F_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for GTZC1 TZIC registers
    #[inline(always)]
    pub fn ctzic1f(&mut self) -> CTZIC1F_W<FCR4rs> {
        CTZIC1F_W::new(self, 15)
    }
    ///Bit 16 - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn coctospi1_memf(&mut self) -> COCTOSPI1_MEMF_W<FCR4rs> {
        COCTOSPI1_MEMF_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3
    #[inline(always)]
    pub fn cfsmc_memf(&mut self) -> CFSMC_MEMF_W<FCR4rs> {
        CFSMC_MEMF_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn cbkpsramf(&mut self) -> CBKPSRAMF_W<FCR4rs> {
        CBKPSRAMF_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for OCTOSPI2 memory bank
    #[inline(always)]
    pub fn coctospi2_memf(&mut self) -> COCTOSPI2_MEMF_W<FCR4rs> {
        COCTOSPI2_MEMF_W::new(self, 19)
    }
    ///Bit 20 - clear the illegal access flag for HSPI1 memory bank
    #[inline(always)]
    pub fn chspi1_memf(&mut self) -> CHSPI1_MEMF_W<FCR4rs> {
        CHSPI1_MEMF_W::new(self, 20)
    }
    ///Bit 24 - clear the illegal access flag for SRAM1
    #[inline(always)]
    pub fn csram1f(&mut self) -> CSRAM1F_W<FCR4rs> {
        CSRAM1F_W::new(self, 24)
    }
    ///Bit 25 - clear the illegal access flag for MPCBB1 registers
    #[inline(always)]
    pub fn cmpcbb1_regf(&mut self) -> CMPCBB1_REGF_W<FCR4rs> {
        CMPCBB1_REGF_W::new(self, 25)
    }
    ///Bit 26 - clear the illegal access flag for SRAM2
    #[inline(always)]
    pub fn csram2f(&mut self) -> CSRAM2F_W<FCR4rs> {
        CSRAM2F_W::new(self, 26)
    }
    ///Bit 27 - clear the illegal access flag for MPCBB2 registers
    #[inline(always)]
    pub fn cmpcbb2_regf(&mut self) -> CMPCBB2_REGF_W<FCR4rs> {
        CMPCBB2_REGF_W::new(self, 27)
    }
    ///Bit 28 - clear the illegal access flag for SRAM3
    #[inline(always)]
    pub fn csram3f(&mut self) -> CSRAM3F_W<FCR4rs> {
        CSRAM3F_W::new(self, 28)
    }
    ///Bit 29 - clear the illegal access flag for MPCBB3 registers
    #[inline(always)]
    pub fn cmpcbb3_regf(&mut self) -> CMPCBB3_REGF_W<FCR4rs> {
        CMPCBB3_REGF_W::new(self, 29)
    }
    ///Bit 30 - clear the illegal access flag for SRAM5
    #[inline(always)]
    pub fn csram5f(&mut self) -> CSRAM5F_W<FCR4rs> {
        CSRAM5F_W::new(self, 30)
    }
    ///Bit 31 - clear the illegal access flag for MPCBB5 registers
    #[inline(always)]
    pub fn cmpcbb5_regf(&mut self) -> CMPCBB5_REGF_W<FCR4rs> {
        CMPCBB5_REGF_W::new(self, 31)
    }
}
/**TZIC flag clear register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GTZC1_TZIC:FCR4)*/
pub struct FCR4rs;
impl crate::RegisterSpec for FCR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr4::W`](W) writer structure
impl crate::Writable for FCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR4 to value 0
impl crate::Resettable for FCR4rs {}
