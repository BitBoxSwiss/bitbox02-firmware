///Register `CSR49` reader
pub type R = crate::R<CSR49rs>;
///Register `CSR49` writer
pub type W = crate::W<CSR49rs>;
///Field `CS49` reader - CS49
pub type CS49_R = crate::FieldReader<u32>;
///Field `CS49` writer - CS49
pub type CS49_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS49
    #[inline(always)]
    pub fn cs49(&self) -> CS49_R {
        CS49_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR49").field("cs49", &self.cs49()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS49
    #[inline(always)]
    pub fn cs49(&mut self) -> CS49_W<CSR49rs> {
        CS49_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR49)*/
pub struct CSR49rs;
impl crate::RegisterSpec for CSR49rs {
    type Ux = u32;
}
///`read()` method returns [`csr49::R`](R) reader structure
impl crate::Readable for CSR49rs {}
///`write(|w| ..)` method takes [`csr49::W`](W) writer structure
impl crate::Writable for CSR49rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR49 to value 0
impl crate::Resettable for CSR49rs {}
