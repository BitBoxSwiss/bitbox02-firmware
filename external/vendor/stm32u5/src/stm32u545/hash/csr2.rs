///Register `CSR2` reader
pub type R = crate::R<CSR2rs>;
///Register `CSR2` writer
pub type W = crate::W<CSR2rs>;
///Field `CS2` reader - CS2
pub type CS2_R = crate::FieldReader<u32>;
///Field `CS2` writer - CS2
pub type CS2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS2
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR2").field("cs2", &self.cs2()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS2
    #[inline(always)]
    pub fn cs2(&mut self) -> CS2_W<CSR2rs> {
        CS2_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR2)*/
pub struct CSR2rs;
impl crate::RegisterSpec for CSR2rs {
    type Ux = u32;
}
///`read()` method returns [`csr2::R`](R) reader structure
impl crate::Readable for CSR2rs {}
///`write(|w| ..)` method takes [`csr2::W`](W) writer structure
impl crate::Writable for CSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR2 to value 0
impl crate::Resettable for CSR2rs {}
