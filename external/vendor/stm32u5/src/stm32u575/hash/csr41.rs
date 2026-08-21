///Register `CSR41` reader
pub type R = crate::R<CSR41rs>;
///Register `CSR41` writer
pub type W = crate::W<CSR41rs>;
///Field `CSR41` reader - CSR41
pub type CSR41_R = crate::FieldReader<u32>;
///Field `CSR41` writer - CSR41
pub type CSR41_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR41
    #[inline(always)]
    pub fn csr41(&self) -> CSR41_R {
        CSR41_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR41")
            .field("csr41", &self.csr41())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR41
    #[inline(always)]
    pub fn csr41(&mut self) -> CSR41_W<CSR41rs> {
        CSR41_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR41)*/
pub struct CSR41rs;
impl crate::RegisterSpec for CSR41rs {
    type Ux = u32;
}
///`read()` method returns [`csr41::R`](R) reader structure
impl crate::Readable for CSR41rs {}
///`write(|w| ..)` method takes [`csr41::W`](W) writer structure
impl crate::Writable for CSR41rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR41 to value 0
impl crate::Resettable for CSR41rs {}
