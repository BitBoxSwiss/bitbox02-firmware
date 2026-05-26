///Register `CSR6` reader
pub type R = crate::R<CSR6rs>;
///Register `CSR6` writer
pub type W = crate::W<CSR6rs>;
///Field `CSR6` reader - CSR6
pub type CSR6_R = crate::FieldReader<u32>;
///Field `CSR6` writer - CSR6
pub type CSR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR6
    #[inline(always)]
    pub fn csr6(&self) -> CSR6_R {
        CSR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR6").field("csr6", &self.csr6()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR6
    #[inline(always)]
    pub fn csr6(&mut self) -> CSR6_W<CSR6rs> {
        CSR6_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR6)*/
pub struct CSR6rs;
impl crate::RegisterSpec for CSR6rs {
    type Ux = u32;
}
///`read()` method returns [`csr6::R`](R) reader structure
impl crate::Readable for CSR6rs {}
///`write(|w| ..)` method takes [`csr6::W`](W) writer structure
impl crate::Writable for CSR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR6 to value 0
impl crate::Resettable for CSR6rs {}
