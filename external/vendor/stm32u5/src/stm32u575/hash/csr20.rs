///Register `CSR20` reader
pub type R = crate::R<CSR20rs>;
///Register `CSR20` writer
pub type W = crate::W<CSR20rs>;
///Field `CSR20` reader - CSR20
pub type CSR20_R = crate::FieldReader<u32>;
///Field `CSR20` writer - CSR20
pub type CSR20_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR20
    #[inline(always)]
    pub fn csr20(&self) -> CSR20_R {
        CSR20_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR20")
            .field("csr20", &self.csr20())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR20
    #[inline(always)]
    pub fn csr20(&mut self) -> CSR20_W<CSR20rs> {
        CSR20_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR20)*/
pub struct CSR20rs;
impl crate::RegisterSpec for CSR20rs {
    type Ux = u32;
}
///`read()` method returns [`csr20::R`](R) reader structure
impl crate::Readable for CSR20rs {}
///`write(|w| ..)` method takes [`csr20::W`](W) writer structure
impl crate::Writable for CSR20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR20 to value 0
impl crate::Resettable for CSR20rs {}
