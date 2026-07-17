///Register `CSR8` reader
pub type R = crate::R<CSR8rs>;
///Register `CSR8` writer
pub type W = crate::W<CSR8rs>;
///Field `CSR8` reader - CSR8
pub type CSR8_R = crate::FieldReader<u32>;
///Field `CSR8` writer - CSR8
pub type CSR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR8
    #[inline(always)]
    pub fn csr8(&self) -> CSR8_R {
        CSR8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR8").field("csr8", &self.csr8()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR8
    #[inline(always)]
    pub fn csr8(&mut self) -> CSR8_W<CSR8rs> {
        CSR8_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR8)*/
pub struct CSR8rs;
impl crate::RegisterSpec for CSR8rs {
    type Ux = u32;
}
///`read()` method returns [`csr8::R`](R) reader structure
impl crate::Readable for CSR8rs {}
///`write(|w| ..)` method takes [`csr8::W`](W) writer structure
impl crate::Writable for CSR8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR8 to value 0
impl crate::Resettable for CSR8rs {}
