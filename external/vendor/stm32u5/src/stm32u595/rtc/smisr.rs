///Register `SMISR` reader
pub type R = crate::R<SMISRrs>;
///Field `ALRAMF` reader - ALRAMF
pub type ALRAMF_R = crate::BitReader;
///Field `ALRBMF` reader - ALRBMF
pub type ALRBMF_R = crate::BitReader;
///Field `WUTMF` reader - WUTMF
pub type WUTMF_R = crate::BitReader;
///Field `TSMF` reader - TSMF
pub type TSMF_R = crate::BitReader;
///Field `TSOVMF` reader - TSOVMF
pub type TSOVMF_R = crate::BitReader;
///Field `ITSMF` reader - ITSMF
pub type ITSMF_R = crate::BitReader;
///Field `SSRUMF` reader - SSRUMF
pub type SSRUMF_R = crate::BitReader;
impl R {
    ///Bit 0 - ALRAMF
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ALRBMF
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUTMF
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSMF
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TSOVMF
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ITSMF
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSRUMF
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMISR")
            .field("alramf", &self.alramf())
            .field("alrbmf", &self.alrbmf())
            .field("wutmf", &self.wutmf())
            .field("tsmf", &self.tsmf())
            .field("tsovmf", &self.tsovmf())
            .field("itsmf", &self.itsmf())
            .field("ssrumf", &self.ssrumf())
            .finish()
    }
}
/**RTC secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RTC:SMISR)*/
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
///`read()` method returns [`smisr::R`](R) reader structure
impl crate::Readable for SMISRrs {}
///`reset()` method sets SMISR to value 0
impl crate::Resettable for SMISRrs {}
