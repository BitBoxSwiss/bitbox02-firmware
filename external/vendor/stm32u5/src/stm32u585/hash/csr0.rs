///Register `CSR0` reader
pub type R = crate::R<CSR0rs>;
///Register `CSR0` writer
pub type W = crate::W<CSR0rs>;
///Field `CSR0` reader - CSR0
pub type CSR0_R = crate::FieldReader<u32>;
///Field `CSR0` writer - CSR0
pub type CSR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr0(&self) -> CSR0_R {
        CSR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR0").field("csr0", &self.csr0()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr0(&mut self) -> CSR0_W<CSR0rs> {
        CSR0_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:CSR0)*/
pub struct CSR0rs;
impl crate::RegisterSpec for CSR0rs {
    type Ux = u32;
}
///`read()` method returns [`csr0::R`](R) reader structure
impl crate::Readable for CSR0rs {}
///`write(|w| ..)` method takes [`csr0::W`](W) writer structure
impl crate::Writable for CSR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR0 to value 0
impl crate::Resettable for CSR0rs {}
