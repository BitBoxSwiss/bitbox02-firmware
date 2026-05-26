///Register `CSR53` reader
pub type R = crate::R<CSR53rs>;
///Register `CSR53` writer
pub type W = crate::W<CSR53rs>;
///Field `CSR53` reader - CSR53
pub type CSR53_R = crate::FieldReader<u32>;
///Field `CSR53` writer - CSR53
pub type CSR53_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR53
    #[inline(always)]
    pub fn csr53(&self) -> CSR53_R {
        CSR53_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR53")
            .field("csr53", &self.csr53())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR53
    #[inline(always)]
    pub fn csr53(&mut self) -> CSR53_W<CSR53rs> {
        CSR53_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR53)*/
pub struct CSR53rs;
impl crate::RegisterSpec for CSR53rs {
    type Ux = u32;
}
///`read()` method returns [`csr53::R`](R) reader structure
impl crate::Readable for CSR53rs {}
///`write(|w| ..)` method takes [`csr53::W`](W) writer structure
impl crate::Writable for CSR53rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR53 to value 0
impl crate::Resettable for CSR53rs {}
