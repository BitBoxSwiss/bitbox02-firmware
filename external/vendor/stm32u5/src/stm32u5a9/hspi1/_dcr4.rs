///Register `_DCR4` reader
pub type R = crate::R<_DCR4rs>;
///Register `_DCR4` writer
pub type W = crate::W<_DCR4rs>;
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
        f.debug_struct("_DCR4")
            .field("refresh", &self.refresh())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles.
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W<_DCR4rs> {
        REFRESH_W::new(self, 0)
    }
}
/**HSPI device configuration register 4

You can [`read`](crate::Reg::read) this register and get [`_dcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DCR4)*/
pub struct _DCR4rs;
impl crate::RegisterSpec for _DCR4rs {
    type Ux = u32;
}
///`read()` method returns [`_dcr4::R`](R) reader structure
impl crate::Readable for _DCR4rs {}
///`write(|w| ..)` method takes [`_dcr4::W`](W) writer structure
impl crate::Writable for _DCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _DCR4 to value 0
impl crate::Resettable for _DCR4rs {}
