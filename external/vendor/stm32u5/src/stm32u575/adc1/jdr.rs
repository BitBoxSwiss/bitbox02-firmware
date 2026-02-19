///Register `JDR%s` reader
pub type R = crate::R<JDRrs>;
///Field `JDATA` reader - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in .
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in .
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR").field("jdata", &self.jdata()).finish()
    }
}
/**ADC injected data register

You can [`read`](crate::Reg::read) this register and get [`jdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:JDR[1])*/
pub struct JDRrs;
impl crate::RegisterSpec for JDRrs {
    type Ux = u32;
}
///`read()` method returns [`jdr::R`](R) reader structure
impl crate::Readable for JDRrs {}
///`reset()` method sets JDR%s to value 0
impl crate::Resettable for JDRrs {}
