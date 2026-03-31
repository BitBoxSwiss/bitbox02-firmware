///Register `DOR%s` reader
pub type R = crate::R<DORrs>;
///Field `DACCDOR` reader - DAC channel1 data output
pub type DACCDOR_R = crate::FieldReader<u16>;
///Field `DACC1DORB` reader - DAC channel1 data output
pub type DACC1DORB_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - DAC channel1 data output
    #[inline(always)]
    pub fn daccdor(&self) -> DACCDOR_R {
        DACCDOR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel1 data output
    #[inline(always)]
    pub fn dacc1dorb(&self) -> DACC1DORB_R {
        DACC1DORB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOR")
            .field("daccdor", &self.daccdor())
            .field("dacc1dorb", &self.dacc1dorb())
            .finish()
    }
}
/**channel%s data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DAC1:DOR[1])*/
pub struct DORrs;
impl crate::RegisterSpec for DORrs {
    type Ux = u32;
}
///`read()` method returns [`dor::R`](R) reader structure
impl crate::Readable for DORrs {}
///`reset()` method sets DOR%s to value 0
impl crate::Resettable for DORrs {}
