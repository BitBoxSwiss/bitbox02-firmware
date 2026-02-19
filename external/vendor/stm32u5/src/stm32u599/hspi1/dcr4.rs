///Register `DCR4` reader
pub type R = crate::R<DCR4rs>;
///Register `DCR4` writer
pub type W = crate::W<DCR4rs>;
///Field `REFRESH` reader - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles.
pub type REFRESH_R = crate::FieldReader<u32>;
///Field `REFRESH` writer - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles.
pub type REFRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles.
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR4")
            .field("refresh", &self.refresh())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles.
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W<DCR4rs> {
        REFRESH_W::new(self, 0)
    }
}
/**HSPI device configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DCR4)*/
pub struct DCR4rs;
impl crate::RegisterSpec for DCR4rs {
    type Ux = u32;
}
///`read()` method returns [`dcr4::R`](R) reader structure
impl crate::Readable for DCR4rs {}
///`write(|w| ..)` method takes [`dcr4::W`](W) writer structure
impl crate::Writable for DCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR4 to value 0
impl crate::Resettable for DCR4rs {}
