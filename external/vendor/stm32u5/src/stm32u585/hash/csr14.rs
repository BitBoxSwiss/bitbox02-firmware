///Register `CSR14` reader
pub type R = crate::R<CSR14rs>;
///Register `CSR14` writer
pub type W = crate::W<CSR14rs>;
///Field `CSR14` reader - CSR14
pub type CSR14_R = crate::FieldReader<u32>;
///Field `CSR14` writer - CSR14
pub type CSR14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR14
    #[inline(always)]
    pub fn csr14(&self) -> CSR14_R {
        CSR14_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR14")
            .field("csr14", &self.csr14())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR14
    #[inline(always)]
    pub fn csr14(&mut self) -> CSR14_W<CSR14rs> {
        CSR14_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR14)*/
pub struct CSR14rs;
impl crate::RegisterSpec for CSR14rs {
    type Ux = u32;
}
///`read()` method returns [`csr14::R`](R) reader structure
impl crate::Readable for CSR14rs {}
///`write(|w| ..)` method takes [`csr14::W`](W) writer structure
impl crate::Writable for CSR14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR14 to value 0
impl crate::Resettable for CSR14rs {}
