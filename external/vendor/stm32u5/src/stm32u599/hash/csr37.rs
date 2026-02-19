///Register `CSR37` reader
pub type R = crate::R<CSR37rs>;
///Register `CSR37` writer
pub type W = crate::W<CSR37rs>;
///Field `CS37` reader - CS37
pub type CS37_R = crate::FieldReader<u32>;
///Field `CS37` writer - CS37
pub type CS37_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS37
    #[inline(always)]
    pub fn cs37(&self) -> CS37_R {
        CS37_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR37").field("cs37", &self.cs37()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS37
    #[inline(always)]
    pub fn cs37(&mut self) -> CS37_W<CSR37rs> {
        CS37_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HASH:CSR37)*/
pub struct CSR37rs;
impl crate::RegisterSpec for CSR37rs {
    type Ux = u32;
}
///`read()` method returns [`csr37::R`](R) reader structure
impl crate::Readable for CSR37rs {}
///`write(|w| ..)` method takes [`csr37::W`](W) writer structure
impl crate::Writable for CSR37rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR37 to value 0
impl crate::Resettable for CSR37rs {}
