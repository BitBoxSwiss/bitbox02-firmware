///Register `CSR5` reader
pub type R = crate::R<CSR5rs>;
///Register `CSR5` writer
pub type W = crate::W<CSR5rs>;
///Field `CS5` reader - CS5
pub type CS5_R = crate::FieldReader<u32>;
///Field `CS5` writer - CS5
pub type CS5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS5
    #[inline(always)]
    pub fn cs5(&self) -> CS5_R {
        CS5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR5").field("cs5", &self.cs5()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS5
    #[inline(always)]
    pub fn cs5(&mut self) -> CS5_W<CSR5rs> {
        CS5_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HASH:CSR5)*/
pub struct CSR5rs;
impl crate::RegisterSpec for CSR5rs {
    type Ux = u32;
}
///`read()` method returns [`csr5::R`](R) reader structure
impl crate::Readable for CSR5rs {}
///`write(|w| ..)` method takes [`csr5::W`](W) writer structure
impl crate::Writable for CSR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR5 to value 0
impl crate::Resettable for CSR5rs {}
