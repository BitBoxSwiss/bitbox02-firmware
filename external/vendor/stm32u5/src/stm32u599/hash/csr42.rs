///Register `CSR42` reader
pub type R = crate::R<CSR42rs>;
///Register `CSR42` writer
pub type W = crate::W<CSR42rs>;
///Field `CS42` reader - CS42
pub type CS42_R = crate::FieldReader<u32>;
///Field `CS42` writer - CS42
pub type CS42_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS42
    #[inline(always)]
    pub fn cs42(&self) -> CS42_R {
        CS42_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR42").field("cs42", &self.cs42()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS42
    #[inline(always)]
    pub fn cs42(&mut self) -> CS42_W<CSR42rs> {
        CS42_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HASH:CSR42)*/
pub struct CSR42rs;
impl crate::RegisterSpec for CSR42rs {
    type Ux = u32;
}
///`read()` method returns [`csr42::R`](R) reader structure
impl crate::Readable for CSR42rs {}
///`write(|w| ..)` method takes [`csr42::W`](W) writer structure
impl crate::Writable for CSR42rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR42 to value 0
impl crate::Resettable for CSR42rs {}
