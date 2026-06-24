///Register `CSR27` reader
pub type R = crate::R<CSR27rs>;
///Register `CSR27` writer
pub type W = crate::W<CSR27rs>;
///Field `CSR27` reader - CSR27
pub type CSR27_R = crate::FieldReader<u32>;
///Field `CSR27` writer - CSR27
pub type CSR27_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR27
    #[inline(always)]
    pub fn csr27(&self) -> CSR27_R {
        CSR27_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR27")
            .field("csr27", &self.csr27())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR27
    #[inline(always)]
    pub fn csr27(&mut self) -> CSR27_W<CSR27rs> {
        CSR27_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR27)*/
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
