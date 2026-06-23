///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `SYSCFGF` reader - illegal access flag for SYSCFG
pub type SYSCFGF_R = crate::BitReader;
///Field `RTCF` reader - illegal access flag for RTC
pub type RTCF_R = crate::BitReader;
///Field `TAMPF` reader - illegal access flag for TAMP
pub type TAMPF_R = crate::BitReader;
///Field `PWRF` reader - illegal access flag for PWRUSART1F
pub type PWRF_R = crate::BitReader;
///Field `RCCF` reader - illegal access flag for RCC
pub type RCCF_R = crate::BitReader;
///Field `LPDMA1F` reader - illegal access flag for LPDMA
pub type LPDMA1F_R = crate::BitReader;
///Field `EXTIF` reader - illegal access flag for EXTI
pub type EXTIF_R = crate::BitReader;
///Field `TZSC2F` reader - illegal access flag for GTZC2 TZSC registers
pub type TZSC2F_R = crate::BitReader;
///Field `TZIC2F` reader - illegal access flag for GTZC2 TZIC registers
pub type TZIC2F_R = crate::BitReader;
///Field `SRAM4F` reader - illegal access flag for SRAM4
pub type SRAM4F_R = crate::BitReader;
///Field `MPCBB4_REGF` reader - illegal access flag for MPCBB4 registers
pub type MPCBB4_REGF_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for SYSCFG
    #[inline(always)]
    pub fn syscfgf(&self) -> SYSCFGF_R {
        SYSCFGF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for RTC
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for TAMP
    #[inline(always)]
    pub fn tampf(&self) -> TAMPF_R {
        TAMPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for PWRUSART1F
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for RCC
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for LPDMA
    #[inline(always)]
    pub fn lpdma1f(&self) -> LPDMA1F_R {
        LPDMA1F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for EXTI
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for GTZC2 TZSC registers
    #[inline(always)]
    pub fn tzsc2f(&self) -> TZSC2F_R {
        TZSC2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for GTZC2 TZIC registers
    #[inline(always)]
    pub fn tzic2f(&self) -> TZIC2F_R {
        TZIC2F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SRAM4
    #[inline(always)]
    pub fn sram4f(&self) -> SRAM4F_R {
        SRAM4F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for MPCBB4 registers
    #[inline(always)]
    pub fn mpcbb4_regf(&self) -> MPCBB4_REGF_R {
        MPCBB4_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("syscfgf", &self.syscfgf())
            .field("rtcf", &self.rtcf())
            .field("tampf", &self.tampf())
            .field("pwrf", &self.pwrf())
            .field("rccf", &self.rccf())
            .field("lpdma1f", &self.lpdma1f())
            .field("extif", &self.extif())
            .field("tzsc2f", &self.tzsc2f())
            .field("tzic2f", &self.tzic2f())
            .field("sram4f", &self.sram4f())
            .field("mpcbb4_regf", &self.mpcbb4_regf())
            .finish()
    }
}
/**TZIC status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC2_TZIC:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
