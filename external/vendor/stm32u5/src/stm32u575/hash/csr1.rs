///Register `CSR1` reader
pub type R = crate::R<CSR1rs>;
///Register `CSR1` writer
pub type W = crate::W<CSR1rs>;
///Field `CSR1` reader - CSR1
pub type CSR1_R = crate::FieldReader<u32>;
///Field `CSR1` writer - CSR1
pub type CSR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR1
    #[inline(always)]
    pub fn csr1(&self) -> CSR1_R {
        CSR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR1").field("csr1", &self.csr1()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR1
    #[inline(always)]
    pub fn csr1(&mut self) -> CSR1_W<CSR1rs> {
        CSR1_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR1)*/
pub struct CSR1rs;
impl crate::RegisterSpec for CSR1rs {
    type Ux = u32;
}
///`read()` method returns [`csr1::R`](R) reader structure
impl crate::Readable for CSR1rs {}
///`write(|w| ..)` method takes [`csr1::W`](W) writer structure
impl crate::Writable for CSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR1 to value 0
impl crate::Resettable for CSR1rs {}
