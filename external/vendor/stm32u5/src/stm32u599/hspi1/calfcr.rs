///Register `CALFCR` reader
pub type R = crate::R<CALFCRrs>;
///Field `FINE` reader - 6: 0\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
pub type FINE_R = crate::FieldReader;
///Field `COARSE` reader - 4: 0\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
pub type COARSE_R = crate::FieldReader;
///Field `CALMAX` reader - Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case HSPI_CALFCR and HSPI_CALSR are updated with the values for the maximum delay.
pub type CALMAX_R = crate::BitReader;
impl R {
    ///Bits 0:6 - 6: 0\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:20 - 4: 0\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 31 - Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case HSPI_CALFCR and HSPI_CALSR are updated with the values for the maximum delay.
    #[inline(always)]
    pub fn calmax(&self) -> CALMAX_R {
        CALMAX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFCR")
            .field("fine", &self.fine())
            .field("coarse", &self.coarse())
            .field("calmax", &self.calmax())
            .finish()
    }
}
/**HSPI full-cycle calibration configuration

You can [`read`](crate::Reg::read) this register and get [`calfcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CALFCR)*/
pub struct CALFCRrs;
impl crate::RegisterSpec for CALFCRrs {
    type Ux = u32;
}
///`read()` method returns [`calfcr::R`](R) reader structure
impl crate::Readable for CALFCRrs {}
///`reset()` method sets CALFCR to value 0
impl crate::Resettable for CALFCRrs {}
