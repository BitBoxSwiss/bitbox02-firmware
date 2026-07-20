///Register `CSR38` reader
pub type R = crate::R<CSR38rs>;
///Register `CSR38` writer
pub type W = crate::W<CSR38rs>;
///Field `CSR38` reader - CSR38
pub type CSR38_R = crate::FieldReader<u32>;
///Field `CSR38` writer - CSR38
pub type CSR38_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR38
    #[inline(always)]
    pub fn csr38(&self) -> CSR38_R {
        CSR38_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR38")
            .field("csr38", &self.csr38())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR38
    #[inline(always)]
    pub fn csr38(&mut self) -> CSR38_W<CSR38rs> {
        CSR38_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR38)*/
pub struct CSR38rs;
impl crate::RegisterSpec for CSR38rs {
    type Ux = u32;
}
///`read()` method returns [`csr38::R`](R) reader structure
impl crate::Readable for CSR38rs {}
///`write(|w| ..)` method takes [`csr38::W`](W) writer structure
impl crate::Writable for CSR38rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR38 to value 0
impl crate::Resettable for CSR38rs {}
