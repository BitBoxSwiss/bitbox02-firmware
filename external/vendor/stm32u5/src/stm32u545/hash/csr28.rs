///Register `CSR28` reader
pub type R = crate::R<CSR28rs>;
///Register `CSR28` writer
pub type W = crate::W<CSR28rs>;
///Field `CS28` reader - CS28
pub type CS28_R = crate::FieldReader<u32>;
///Field `CS28` writer - CS28
pub type CS28_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS28
    #[inline(always)]
    pub fn cs28(&self) -> CS28_R {
        CS28_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR28").field("cs28", &self.cs28()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS28
    #[inline(always)]
    pub fn cs28(&mut self) -> CS28_W<CSR28rs> {
        CS28_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR28)*/
pub struct CSR28rs;
impl crate::RegisterSpec for CSR28rs {
    type Ux = u32;
}
///`read()` method returns [`csr28::R`](R) reader structure
impl crate::Readable for CSR28rs {}
///`write(|w| ..)` method takes [`csr28::W`](W) writer structure
impl crate::Writable for CSR28rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR28 to value 0
impl crate::Resettable for CSR28rs {}
