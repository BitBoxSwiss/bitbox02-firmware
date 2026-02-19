///Register `CSR40` reader
pub type R = crate::R<CSR40rs>;
///Register `CSR40` writer
pub type W = crate::W<CSR40rs>;
///Field `CSR40` reader - CSR40
pub type CSR40_R = crate::FieldReader<u32>;
///Field `CSR40` writer - CSR40
pub type CSR40_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR40
    #[inline(always)]
    pub fn csr40(&self) -> CSR40_R {
        CSR40_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR40")
            .field("csr40", &self.csr40())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR40
    #[inline(always)]
    pub fn csr40(&mut self) -> CSR40_W<CSR40rs> {
        CSR40_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR40)*/
pub struct CSR40rs;
impl crate::RegisterSpec for CSR40rs {
    type Ux = u32;
}
///`read()` method returns [`csr40::R`](R) reader structure
impl crate::Readable for CSR40rs {}
///`write(|w| ..)` method takes [`csr40::W`](W) writer structure
impl crate::Writable for CSR40rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR40 to value 0
impl crate::Resettable for CSR40rs {}
