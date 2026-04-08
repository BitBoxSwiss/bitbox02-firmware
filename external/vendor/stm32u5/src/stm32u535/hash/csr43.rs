///Register `CSR43` reader
pub type R = crate::R<CSR43rs>;
///Register `CSR43` writer
pub type W = crate::W<CSR43rs>;
///Field `CS43` reader - CS43
pub type CS43_R = crate::FieldReader<u32>;
///Field `CS43` writer - CS43
pub type CS43_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS43
    #[inline(always)]
    pub fn cs43(&self) -> CS43_R {
        CS43_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR43").field("cs43", &self.cs43()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS43
    #[inline(always)]
    pub fn cs43(&mut self) -> CS43_W<CSR43rs> {
        CS43_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#HASH:CSR43)*/
pub struct CSR43rs;
impl crate::RegisterSpec for CSR43rs {
    type Ux = u32;
}
///`read()` method returns [`csr43::R`](R) reader structure
impl crate::Readable for CSR43rs {}
///`write(|w| ..)` method takes [`csr43::W`](W) writer structure
impl crate::Writable for CSR43rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR43 to value 0
impl crate::Resettable for CSR43rs {}
