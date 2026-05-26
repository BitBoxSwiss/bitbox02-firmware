///Register `CSR11` reader
pub type R = crate::R<CSR11rs>;
///Register `CSR11` writer
pub type W = crate::W<CSR11rs>;
///Field `CSR11` reader - CSR11
pub type CSR11_R = crate::FieldReader<u32>;
///Field `CSR11` writer - CSR11
pub type CSR11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR11
    #[inline(always)]
    pub fn csr11(&self) -> CSR11_R {
        CSR11_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR11")
            .field("csr11", &self.csr11())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR11
    #[inline(always)]
    pub fn csr11(&mut self) -> CSR11_W<CSR11rs> {
        CSR11_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR11)*/
pub struct CSR11rs;
impl crate::RegisterSpec for CSR11rs {
    type Ux = u32;
}
///`read()` method returns [`csr11::R`](R) reader structure
impl crate::Readable for CSR11rs {}
///`write(|w| ..)` method takes [`csr11::W`](W) writer structure
impl crate::Writable for CSR11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR11 to value 0
impl crate::Resettable for CSR11rs {}
