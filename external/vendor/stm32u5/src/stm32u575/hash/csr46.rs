///Register `CSR46` reader
pub type R = crate::R<CSR46rs>;
///Register `CSR46` writer
pub type W = crate::W<CSR46rs>;
///Field `CSR46` reader - CSR46
pub type CSR46_R = crate::FieldReader<u32>;
///Field `CSR46` writer - CSR46
pub type CSR46_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR46
    #[inline(always)]
    pub fn csr46(&self) -> CSR46_R {
        CSR46_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR46")
            .field("csr46", &self.csr46())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR46
    #[inline(always)]
    pub fn csr46(&mut self) -> CSR46_W<CSR46rs> {
        CSR46_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR46)*/
pub struct CSR46rs;
impl crate::RegisterSpec for CSR46rs {
    type Ux = u32;
}
///`read()` method returns [`csr46::R`](R) reader structure
impl crate::Readable for CSR46rs {}
///`write(|w| ..)` method takes [`csr46::W`](W) writer structure
impl crate::Writable for CSR46rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR46 to value 0
impl crate::Resettable for CSR46rs {}
