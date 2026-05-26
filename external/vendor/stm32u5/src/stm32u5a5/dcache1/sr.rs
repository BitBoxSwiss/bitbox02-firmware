///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `BUSYF` reader - BUSYF
pub type BUSYF_R = crate::BitReader;
///Field `BSYENDF` reader - BSYENDF
pub type BSYENDF_R = crate::BitReader;
///Field `ERRF` reader - ERRF
pub type ERRF_R = crate::BitReader;
///Field `BUSYCMDF` reader - BUSYCMDF
pub type BUSYCMDF_R = crate::BitReader;
///Field `CMDENDF` reader - CMDENDF
pub type CMDENDF_R = crate::BitReader;
impl R {
    ///Bit 0 - BUSYF
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BSYENDF
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRF
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BUSYCMDF
    #[inline(always)]
    pub fn busycmdf(&self) -> BUSYCMDF_R {
        BUSYCMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CMDENDF
    #[inline(always)]
    pub fn cmdendf(&self) -> CMDENDF_R {
        CMDENDF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busyf", &self.busyf())
            .field("bsyendf", &self.bsyendf())
            .field("errf", &self.errf())
            .field("busycmdf", &self.busycmdf())
            .field("cmdendf", &self.cmdendf())
            .finish()
    }
}
/**DCACHE status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
