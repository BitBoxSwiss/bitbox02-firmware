///Register `CSR10` reader
pub type R = crate::R<CSR10rs>;
///Register `CSR10` writer
pub type W = crate::W<CSR10rs>;
///Field `CS10` reader - CS10
pub type CS10_R = crate::FieldReader<u32>;
///Field `CS10` writer - CS10
pub type CS10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS10
    #[inline(always)]
    pub fn cs10(&self) -> CS10_R {
        CS10_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR10").field("cs10", &self.cs10()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS10
    #[inline(always)]
    pub fn cs10(&mut self) -> CS10_W<CSR10rs> {
        CS10_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR10)*/
pub struct CSR10rs;
impl crate::RegisterSpec for CSR10rs {
    type Ux = u32;
}
///`read()` method returns [`csr10::R`](R) reader structure
impl crate::Readable for CSR10rs {}
///`write(|w| ..)` method takes [`csr10::W`](W) writer structure
impl crate::Writable for CSR10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR10 to value 0
impl crate::Resettable for CSR10rs {}
