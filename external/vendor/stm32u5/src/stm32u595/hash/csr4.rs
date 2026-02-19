///Register `CSR4` reader
pub type R = crate::R<CSR4rs>;
///Register `CSR4` writer
pub type W = crate::W<CSR4rs>;
///Field `CS4` reader - CS4
pub type CS4_R = crate::FieldReader<u32>;
///Field `CS4` writer - CS4
pub type CS4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS4
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR4").field("cs4", &self.cs4()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS4
    #[inline(always)]
    pub fn cs4(&mut self) -> CS4_W<CSR4rs> {
        CS4_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:CSR4)*/
pub struct CSR4rs;
impl crate::RegisterSpec for CSR4rs {
    type Ux = u32;
}
///`read()` method returns [`csr4::R`](R) reader structure
impl crate::Readable for CSR4rs {}
///`write(|w| ..)` method takes [`csr4::W`](W) writer structure
impl crate::Writable for CSR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR4 to value 0
impl crate::Resettable for CSR4rs {}
