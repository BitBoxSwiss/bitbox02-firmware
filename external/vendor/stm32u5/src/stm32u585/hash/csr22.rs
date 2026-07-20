///Register `CSR22` reader
pub type R = crate::R<CSR22rs>;
///Register `CSR22` writer
pub type W = crate::W<CSR22rs>;
///Field `CSR22` reader - CSR22
pub type CSR22_R = crate::FieldReader<u32>;
///Field `CSR22` writer - CSR22
pub type CSR22_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR22
    #[inline(always)]
    pub fn csr22(&self) -> CSR22_R {
        CSR22_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR22")
            .field("csr22", &self.csr22())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR22
    #[inline(always)]
    pub fn csr22(&mut self) -> CSR22_W<CSR22rs> {
        CSR22_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR22)*/
pub struct CSR22rs;
impl crate::RegisterSpec for CSR22rs {
    type Ux = u32;
}
///`read()` method returns [`csr22::R`](R) reader structure
impl crate::Readable for CSR22rs {}
///`write(|w| ..)` method takes [`csr22::W`](W) writer structure
impl crate::Writable for CSR22rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR22 to value 0
impl crate::Resettable for CSR22rs {}
