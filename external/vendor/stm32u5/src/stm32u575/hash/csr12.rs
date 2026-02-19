///Register `CSR12` reader
pub type R = crate::R<CSR12rs>;
///Register `CSR12` writer
pub type W = crate::W<CSR12rs>;
///Field `CSR12` reader - CSR12
pub type CSR12_R = crate::FieldReader<u32>;
///Field `CSR12` writer - CSR12
pub type CSR12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR12
    #[inline(always)]
    pub fn csr12(&self) -> CSR12_R {
        CSR12_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR12")
            .field("csr12", &self.csr12())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR12
    #[inline(always)]
    pub fn csr12(&mut self) -> CSR12_W<CSR12rs> {
        CSR12_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR12)*/
pub struct CSR12rs;
impl crate::RegisterSpec for CSR12rs {
    type Ux = u32;
}
///`read()` method returns [`csr12::R`](R) reader structure
impl crate::Readable for CSR12rs {}
///`write(|w| ..)` method takes [`csr12::W`](W) writer structure
impl crate::Writable for CSR12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR12 to value 0
impl crate::Resettable for CSR12rs {}
