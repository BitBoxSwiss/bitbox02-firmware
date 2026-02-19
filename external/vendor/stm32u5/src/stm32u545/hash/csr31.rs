///Register `CSR31` reader
pub type R = crate::R<CSR31rs>;
///Register `CSR31` writer
pub type W = crate::W<CSR31rs>;
///Field `CS31` reader - CS31
pub type CS31_R = crate::FieldReader<u32>;
///Field `CS31` writer - CS31
pub type CS31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS31
    #[inline(always)]
    pub fn cs31(&self) -> CS31_R {
        CS31_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR31").field("cs31", &self.cs31()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS31
    #[inline(always)]
    pub fn cs31(&mut self) -> CS31_W<CSR31rs> {
        CS31_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR31)*/
pub struct CSR31rs;
impl crate::RegisterSpec for CSR31rs {
    type Ux = u32;
}
///`read()` method returns [`csr31::R`](R) reader structure
impl crate::Readable for CSR31rs {}
///`write(|w| ..)` method takes [`csr31::W`](W) writer structure
impl crate::Writable for CSR31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR31 to value 0
impl crate::Resettable for CSR31rs {}
