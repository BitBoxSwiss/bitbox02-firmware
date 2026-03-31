///Register `CSR28` reader
pub type R = crate::R<CSR28rs>;
///Register `CSR28` writer
pub type W = crate::W<CSR28rs>;
///Field `CSR28` reader - CSR28
pub type CSR28_R = crate::FieldReader<u32>;
///Field `CSR28` writer - CSR28
pub type CSR28_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR28
    #[inline(always)]
    pub fn csr28(&self) -> CSR28_R {
        CSR28_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR28")
            .field("csr28", &self.csr28())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR28
    #[inline(always)]
    pub fn csr28(&mut self) -> CSR28_W<CSR28rs> {
        CSR28_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR28)*/
pub struct CSR28rs;
impl crate::RegisterSpec for CSR28rs {
    type Ux = u32;
}
///`read()` method returns [`csr28::R`](R) reader structure
impl crate::Readable for CSR28rs {}
///`write(|w| ..)` method takes [`csr28::W`](W) writer structure
impl crate::Writable for CSR28rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR28 to value 0
impl crate::Resettable for CSR28rs {}
