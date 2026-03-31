///Register `CSR47` reader
pub type R = crate::R<CSR47rs>;
///Register `CSR47` writer
pub type W = crate::W<CSR47rs>;
///Field `CSR47` reader - CSR47
pub type CSR47_R = crate::FieldReader<u32>;
///Field `CSR47` writer - CSR47
pub type CSR47_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR47
    #[inline(always)]
    pub fn csr47(&self) -> CSR47_R {
        CSR47_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR47")
            .field("csr47", &self.csr47())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR47
    #[inline(always)]
    pub fn csr47(&mut self) -> CSR47_W<CSR47rs> {
        CSR47_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR47)*/
pub struct CSR47rs;
impl crate::RegisterSpec for CSR47rs {
    type Ux = u32;
}
///`read()` method returns [`csr47::R`](R) reader structure
impl crate::Readable for CSR47rs {}
///`write(|w| ..)` method takes [`csr47::W`](W) writer structure
impl crate::Writable for CSR47rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR47 to value 0
impl crate::Resettable for CSR47rs {}
