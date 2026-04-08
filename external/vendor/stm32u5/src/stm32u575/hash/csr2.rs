///Register `CSR2` reader
pub type R = crate::R<CSR2rs>;
///Register `CSR2` writer
pub type W = crate::W<CSR2rs>;
///Field `CSR2` reader - CSR2
pub type CSR2_R = crate::FieldReader<u32>;
///Field `CSR2` writer - CSR2
pub type CSR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR2
    #[inline(always)]
    pub fn csr2(&self) -> CSR2_R {
        CSR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR2").field("csr2", &self.csr2()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR2
    #[inline(always)]
    pub fn csr2(&mut self) -> CSR2_W<CSR2rs> {
        CSR2_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR2)*/
pub struct CSR2rs;
impl crate::RegisterSpec for CSR2rs {
    type Ux = u32;
}
///`read()` method returns [`csr2::R`](R) reader structure
impl crate::Readable for CSR2rs {}
///`write(|w| ..)` method takes [`csr2::W`](W) writer structure
impl crate::Writable for CSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR2 to value 0
impl crate::Resettable for CSR2rs {}
