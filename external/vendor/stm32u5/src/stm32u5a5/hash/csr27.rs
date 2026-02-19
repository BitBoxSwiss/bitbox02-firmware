///Register `CSR27` reader
pub type R = crate::R<CSR27rs>;
///Register `CSR27` writer
pub type W = crate::W<CSR27rs>;
///Field `CS27` reader - CS27
pub type CS27_R = crate::FieldReader<u32>;
///Field `CS27` writer - CS27
pub type CS27_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS27
    #[inline(always)]
    pub fn cs27(&self) -> CS27_R {
        CS27_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR27").field("cs27", &self.cs27()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS27
    #[inline(always)]
    pub fn cs27(&mut self) -> CS27_W<CSR27rs> {
        CS27_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#HASH:CSR27)*/
pub struct CSR27rs;
impl crate::RegisterSpec for CSR27rs {
    type Ux = u32;
}
///`read()` method returns [`csr27::R`](R) reader structure
impl crate::Readable for CSR27rs {}
///`write(|w| ..)` method takes [`csr27::W`](W) writer structure
impl crate::Writable for CSR27rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR27 to value 0
impl crate::Resettable for CSR27rs {}
