///Register `CSR51` reader
pub type R = crate::R<CSR51rs>;
///Register `CSR51` writer
pub type W = crate::W<CSR51rs>;
///Field `CS51` reader - CS51
pub type CS51_R = crate::FieldReader<u32>;
///Field `CS51` writer - CS51
pub type CS51_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS51
    #[inline(always)]
    pub fn cs51(&self) -> CS51_R {
        CS51_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR51").field("cs51", &self.cs51()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS51
    #[inline(always)]
    pub fn cs51(&mut self) -> CS51_W<CSR51rs> {
        CS51_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR51)*/
pub struct CSR51rs;
impl crate::RegisterSpec for CSR51rs {
    type Ux = u32;
}
///`read()` method returns [`csr51::R`](R) reader structure
impl crate::Readable for CSR51rs {}
///`write(|w| ..)` method takes [`csr51::W`](W) writer structure
impl crate::Writable for CSR51rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR51 to value 0
impl crate::Resettable for CSR51rs {}
