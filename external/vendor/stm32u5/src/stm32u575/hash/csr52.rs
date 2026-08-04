///Register `CSR52` reader
pub type R = crate::R<CSR52rs>;
///Register `CSR52` writer
pub type W = crate::W<CSR52rs>;
///Field `CSR52` reader - CSR52
pub type CSR52_R = crate::FieldReader<u32>;
///Field `CSR52` writer - CSR52
pub type CSR52_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR52
    #[inline(always)]
    pub fn csr52(&self) -> CSR52_R {
        CSR52_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR52")
            .field("csr52", &self.csr52())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR52
    #[inline(always)]
    pub fn csr52(&mut self) -> CSR52_W<CSR52rs> {
        CSR52_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR52)*/
pub struct CSR52rs;
impl crate::RegisterSpec for CSR52rs {
    type Ux = u32;
}
///`read()` method returns [`csr52::R`](R) reader structure
impl crate::Readable for CSR52rs {}
///`write(|w| ..)` method takes [`csr52::W`](W) writer structure
impl crate::Writable for CSR52rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR52 to value 0
impl crate::Resettable for CSR52rs {}
