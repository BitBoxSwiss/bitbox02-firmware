///Register `CSR13` reader
pub type R = crate::R<CSR13rs>;
///Register `CSR13` writer
pub type W = crate::W<CSR13rs>;
///Field `CSR13` reader - CSR13
pub type CSR13_R = crate::FieldReader<u32>;
///Field `CSR13` writer - CSR13
pub type CSR13_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR13
    #[inline(always)]
    pub fn csr13(&self) -> CSR13_R {
        CSR13_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR13")
            .field("csr13", &self.csr13())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR13
    #[inline(always)]
    pub fn csr13(&mut self) -> CSR13_W<CSR13rs> {
        CSR13_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR13)*/
pub struct CSR13rs;
impl crate::RegisterSpec for CSR13rs {
    type Ux = u32;
}
///`read()` method returns [`csr13::R`](R) reader structure
impl crate::Readable for CSR13rs {}
///`write(|w| ..)` method takes [`csr13::W`](W) writer structure
impl crate::Writable for CSR13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR13 to value 0
impl crate::Resettable for CSR13rs {}
