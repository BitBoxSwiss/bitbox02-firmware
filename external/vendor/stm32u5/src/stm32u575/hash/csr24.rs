///Register `CSR24` reader
pub type R = crate::R<CSR24rs>;
///Register `CSR24` writer
pub type W = crate::W<CSR24rs>;
///Field `CSR24` reader - CSR24
pub type CSR24_R = crate::FieldReader<u32>;
///Field `CSR24` writer - CSR24
pub type CSR24_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR24
    #[inline(always)]
    pub fn csr24(&self) -> CSR24_R {
        CSR24_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR24")
            .field("csr24", &self.csr24())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR24
    #[inline(always)]
    pub fn csr24(&mut self) -> CSR24_W<CSR24rs> {
        CSR24_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR24)*/
pub struct CSR24rs;
impl crate::RegisterSpec for CSR24rs {
    type Ux = u32;
}
///`read()` method returns [`csr24::R`](R) reader structure
impl crate::Readable for CSR24rs {}
///`write(|w| ..)` method takes [`csr24::W`](W) writer structure
impl crate::Writable for CSR24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR24 to value 0
impl crate::Resettable for CSR24rs {}
