///Register `CSR6` reader
pub type R = crate::R<CSR6rs>;
///Register `CSR6` writer
pub type W = crate::W<CSR6rs>;
///Field `CS6` reader - CS6
pub type CS6_R = crate::FieldReader<u32>;
///Field `CS6` writer - CS6
pub type CS6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS6
    #[inline(always)]
    pub fn cs6(&self) -> CS6_R {
        CS6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR6").field("cs6", &self.cs6()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS6
    #[inline(always)]
    pub fn cs6(&mut self) -> CS6_W<CSR6rs> {
        CS6_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:CSR6)*/
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
