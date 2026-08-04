///Register `CSR44` reader
pub type R = crate::R<CSR44rs>;
///Register `CSR44` writer
pub type W = crate::W<CSR44rs>;
///Field `CS44` reader - CS44
pub type CS44_R = crate::FieldReader<u32>;
///Field `CS44` writer - CS44
pub type CS44_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS44
    #[inline(always)]
    pub fn cs44(&self) -> CS44_R {
        CS44_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR44").field("cs44", &self.cs44()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS44
    #[inline(always)]
    pub fn cs44(&mut self) -> CS44_W<CSR44rs> {
        CS44_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR44)*/
pub struct CSR44rs;
impl crate::RegisterSpec for CSR44rs {
    type Ux = u32;
}
///`read()` method returns [`csr44::R`](R) reader structure
impl crate::Readable for CSR44rs {}
///`write(|w| ..)` method takes [`csr44::W`](W) writer structure
impl crate::Writable for CSR44rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR44 to value 0
impl crate::Resettable for CSR44rs {}
