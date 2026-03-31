///Register `CSR33` reader
pub type R = crate::R<CSR33rs>;
///Register `CSR33` writer
pub type W = crate::W<CSR33rs>;
///Field `CS33` reader - CS33
pub type CS33_R = crate::FieldReader<u32>;
///Field `CS33` writer - CS33
pub type CS33_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS33
    #[inline(always)]
    pub fn cs33(&self) -> CS33_R {
        CS33_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR33").field("cs33", &self.cs33()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS33
    #[inline(always)]
    pub fn cs33(&mut self) -> CS33_W<CSR33rs> {
        CS33_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:CSR33)*/
pub struct CSR33rs;
impl crate::RegisterSpec for CSR33rs {
    type Ux = u32;
}
///`read()` method returns [`csr33::R`](R) reader structure
impl crate::Readable for CSR33rs {}
///`write(|w| ..)` method takes [`csr33::W`](W) writer structure
impl crate::Writable for CSR33rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR33 to value 0
impl crate::Resettable for CSR33rs {}
