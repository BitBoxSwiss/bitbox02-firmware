///Register `CSR32` reader
pub type R = crate::R<CSR32rs>;
///Register `CSR32` writer
pub type W = crate::W<CSR32rs>;
///Field `CS32` reader - CS32
pub type CS32_R = crate::FieldReader<u32>;
///Field `CS32` writer - CS32
pub type CS32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS32
    #[inline(always)]
    pub fn cs32(&self) -> CS32_R {
        CS32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR32").field("cs32", &self.cs32()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS32
    #[inline(always)]
    pub fn cs32(&mut self) -> CS32_W<CSR32rs> {
        CS32_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#HASH:CSR32)*/
pub struct CSR32rs;
impl crate::RegisterSpec for CSR32rs {
    type Ux = u32;
}
///`read()` method returns [`csr32::R`](R) reader structure
impl crate::Readable for CSR32rs {}
///`write(|w| ..)` method takes [`csr32::W`](W) writer structure
impl crate::Writable for CSR32rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR32 to value 0
impl crate::Resettable for CSR32rs {}
