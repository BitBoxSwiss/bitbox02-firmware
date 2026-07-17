///Register `CSR3` reader
pub type R = crate::R<CSR3rs>;
///Register `CSR3` writer
pub type W = crate::W<CSR3rs>;
///Field `CSR3` reader - CSR3
pub type CSR3_R = crate::FieldReader<u32>;
///Field `CSR3` writer - CSR3
pub type CSR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR3
    #[inline(always)]
    pub fn csr3(&self) -> CSR3_R {
        CSR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR3").field("csr3", &self.csr3()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR3
    #[inline(always)]
    pub fn csr3(&mut self) -> CSR3_W<CSR3rs> {
        CSR3_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR3)*/
pub struct CSR3rs;
impl crate::RegisterSpec for CSR3rs {
    type Ux = u32;
}
///`read()` method returns [`csr3::R`](R) reader structure
impl crate::Readable for CSR3rs {}
///`write(|w| ..)` method takes [`csr3::W`](W) writer structure
impl crate::Writable for CSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR3 to value 0
impl crate::Resettable for CSR3rs {}
