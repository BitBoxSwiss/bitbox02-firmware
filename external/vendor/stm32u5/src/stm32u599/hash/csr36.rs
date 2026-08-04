///Register `CSR36` reader
pub type R = crate::R<CSR36rs>;
///Register `CSR36` writer
pub type W = crate::W<CSR36rs>;
///Field `CS36` reader - CS36
pub type CS36_R = crate::FieldReader<u32>;
///Field `CS36` writer - CS36
pub type CS36_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS36
    #[inline(always)]
    pub fn cs36(&self) -> CS36_R {
        CS36_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR36").field("cs36", &self.cs36()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS36
    #[inline(always)]
    pub fn cs36(&mut self) -> CS36_W<CSR36rs> {
        CS36_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HASH:CSR36)*/
pub struct CSR36rs;
impl crate::RegisterSpec for CSR36rs {
    type Ux = u32;
}
///`read()` method returns [`csr36::R`](R) reader structure
impl crate::Readable for CSR36rs {}
///`write(|w| ..)` method takes [`csr36::W`](W) writer structure
impl crate::Writable for CSR36rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR36 to value 0
impl crate::Resettable for CSR36rs {}
