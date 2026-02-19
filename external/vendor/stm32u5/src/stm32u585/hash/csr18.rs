///Register `CSR18` reader
pub type R = crate::R<CSR18rs>;
///Register `CSR18` writer
pub type W = crate::W<CSR18rs>;
///Field `CSR18` reader - CSR18
pub type CSR18_R = crate::FieldReader<u32>;
///Field `CSR18` writer - CSR18
pub type CSR18_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR18
    #[inline(always)]
    pub fn csr18(&self) -> CSR18_R {
        CSR18_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR18")
            .field("csr18", &self.csr18())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR18
    #[inline(always)]
    pub fn csr18(&mut self) -> CSR18_W<CSR18rs> {
        CSR18_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR18)*/
pub struct CSR18rs;
impl crate::RegisterSpec for CSR18rs {
    type Ux = u32;
}
///`read()` method returns [`csr18::R`](R) reader structure
impl crate::Readable for CSR18rs {}
///`write(|w| ..)` method takes [`csr18::W`](W) writer structure
impl crate::Writable for CSR18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR18 to value 0
impl crate::Resettable for CSR18rs {}
