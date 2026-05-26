///Register `CSR14` reader
pub type R = crate::R<CSR14rs>;
///Register `CSR14` writer
pub type W = crate::W<CSR14rs>;
///Field `CS14` reader - CS14
pub type CS14_R = crate::FieldReader<u32>;
///Field `CS14` writer - CS14
pub type CS14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS14
    #[inline(always)]
    pub fn cs14(&self) -> CS14_R {
        CS14_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR14").field("cs14", &self.cs14()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS14
    #[inline(always)]
    pub fn cs14(&mut self) -> CS14_W<CSR14rs> {
        CS14_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR14)*/
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
