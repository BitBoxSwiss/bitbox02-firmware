///Register `CSR51` reader
pub type R = crate::R<CSR51rs>;
///Register `CSR51` writer
pub type W = crate::W<CSR51rs>;
///Field `CSR51` reader - CSR51
pub type CSR51_R = crate::FieldReader<u32>;
///Field `CSR51` writer - CSR51
pub type CSR51_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR51
    #[inline(always)]
    pub fn csr51(&self) -> CSR51_R {
        CSR51_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR51")
            .field("csr51", &self.csr51())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR51
    #[inline(always)]
    pub fn csr51(&mut self) -> CSR51_W<CSR51rs> {
        CSR51_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR51)*/
pub struct CSR51rs;
impl crate::RegisterSpec for CSR51rs {
    type Ux = u32;
}
///`read()` method returns [`csr51::R`](R) reader structure
impl crate::Readable for CSR51rs {}
///`write(|w| ..)` method takes [`csr51::W`](W) writer structure
impl crate::Writable for CSR51rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR51 to value 0
impl crate::Resettable for CSR51rs {}
