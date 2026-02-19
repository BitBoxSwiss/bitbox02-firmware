///Register `CSR34` reader
pub type R = crate::R<CSR34rs>;
///Register `CSR34` writer
pub type W = crate::W<CSR34rs>;
///Field `CSR34` reader - CSR34
pub type CSR34_R = crate::FieldReader<u32>;
///Field `CSR34` writer - CSR34
pub type CSR34_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR34
    #[inline(always)]
    pub fn csr34(&self) -> CSR34_R {
        CSR34_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR34")
            .field("csr34", &self.csr34())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR34
    #[inline(always)]
    pub fn csr34(&mut self) -> CSR34_W<CSR34rs> {
        CSR34_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#HASH:CSR34)*/
pub struct CSR34rs;
impl crate::RegisterSpec for CSR34rs {
    type Ux = u32;
}
///`read()` method returns [`csr34::R`](R) reader structure
impl crate::Readable for CSR34rs {}
///`write(|w| ..)` method takes [`csr34::W`](W) writer structure
impl crate::Writable for CSR34rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR34 to value 0
impl crate::Resettable for CSR34rs {}
