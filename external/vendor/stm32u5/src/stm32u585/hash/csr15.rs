///Register `CSR15` reader
pub type R = crate::R<CSR15rs>;
///Register `CSR15` writer
pub type W = crate::W<CSR15rs>;
///Field `CSR15` reader - CSR15
pub type CSR15_R = crate::FieldReader<u32>;
///Field `CSR15` writer - CSR15
pub type CSR15_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR15
    #[inline(always)]
    pub fn csr15(&self) -> CSR15_R {
        CSR15_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR15")
            .field("csr15", &self.csr15())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR15
    #[inline(always)]
    pub fn csr15(&mut self) -> CSR15_W<CSR15rs> {
        CSR15_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR15)*/
pub struct CSR15rs;
impl crate::RegisterSpec for CSR15rs {
    type Ux = u32;
}
///`read()` method returns [`csr15::R`](R) reader structure
impl crate::Readable for CSR15rs {}
///`write(|w| ..)` method takes [`csr15::W`](W) writer structure
impl crate::Writable for CSR15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR15 to value 0
impl crate::Resettable for CSR15rs {}
