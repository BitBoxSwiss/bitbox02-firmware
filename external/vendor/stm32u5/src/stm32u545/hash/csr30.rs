///Register `CSR30` reader
pub type R = crate::R<CSR30rs>;
///Register `CSR30` writer
pub type W = crate::W<CSR30rs>;
///Field `CS30` reader - CS30
pub type CS30_R = crate::FieldReader<u32>;
///Field `CS30` writer - CS30
pub type CS30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS30
    #[inline(always)]
    pub fn cs30(&self) -> CS30_R {
        CS30_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR30").field("cs30", &self.cs30()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS30
    #[inline(always)]
    pub fn cs30(&mut self) -> CS30_W<CSR30rs> {
        CS30_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR30)*/
pub struct CSR30rs;
impl crate::RegisterSpec for CSR30rs {
    type Ux = u32;
}
///`read()` method returns [`csr30::R`](R) reader structure
impl crate::Readable for CSR30rs {}
///`write(|w| ..)` method takes [`csr30::W`](W) writer structure
impl crate::Writable for CSR30rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR30 to value 0
impl crate::Resettable for CSR30rs {}
