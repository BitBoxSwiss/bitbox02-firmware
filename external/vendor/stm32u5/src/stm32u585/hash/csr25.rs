///Register `CSR25` reader
pub type R = crate::R<CSR25rs>;
///Register `CSR25` writer
pub type W = crate::W<CSR25rs>;
///Field `CSR25` reader - CSR25
pub type CSR25_R = crate::FieldReader<u32>;
///Field `CSR25` writer - CSR25
pub type CSR25_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR25
    #[inline(always)]
    pub fn csr25(&self) -> CSR25_R {
        CSR25_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR25")
            .field("csr25", &self.csr25())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR25
    #[inline(always)]
    pub fn csr25(&mut self) -> CSR25_W<CSR25rs> {
        CSR25_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR25)*/
pub struct CSR25rs;
impl crate::RegisterSpec for CSR25rs {
    type Ux = u32;
}
///`read()` method returns [`csr25::R`](R) reader structure
impl crate::Readable for CSR25rs {}
///`write(|w| ..)` method takes [`csr25::W`](W) writer structure
impl crate::Writable for CSR25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR25 to value 0
impl crate::Resettable for CSR25rs {}
