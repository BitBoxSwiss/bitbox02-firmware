///Register `CSR39` reader
pub type R = crate::R<CSR39rs>;
///Register `CSR39` writer
pub type W = crate::W<CSR39rs>;
///Field `CS39` reader - CS39
pub type CS39_R = crate::FieldReader<u32>;
///Field `CS39` writer - CS39
pub type CS39_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS39
    #[inline(always)]
    pub fn cs39(&self) -> CS39_R {
        CS39_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR39").field("cs39", &self.cs39()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS39
    #[inline(always)]
    pub fn cs39(&mut self) -> CS39_W<CSR39rs> {
        CS39_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR39)*/
pub struct CSR39rs;
impl crate::RegisterSpec for CSR39rs {
    type Ux = u32;
}
///`read()` method returns [`csr39::R`](R) reader structure
impl crate::Readable for CSR39rs {}
///`write(|w| ..)` method takes [`csr39::W`](W) writer structure
impl crate::Writable for CSR39rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR39 to value 0
impl crate::Resettable for CSR39rs {}
