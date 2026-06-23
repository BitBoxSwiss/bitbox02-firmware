///Register `CSR16` reader
pub type R = crate::R<CSR16rs>;
///Register `CSR16` writer
pub type W = crate::W<CSR16rs>;
///Field `CS16` reader - CS16
pub type CS16_R = crate::FieldReader<u32>;
///Field `CS16` writer - CS16
pub type CS16_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS16
    #[inline(always)]
    pub fn cs16(&self) -> CS16_R {
        CS16_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR16").field("cs16", &self.cs16()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS16
    #[inline(always)]
    pub fn cs16(&mut self) -> CS16_W<CSR16rs> {
        CS16_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#HASH:CSR16)*/
pub struct CSR16rs;
impl crate::RegisterSpec for CSR16rs {
    type Ux = u32;
}
///`read()` method returns [`csr16::R`](R) reader structure
impl crate::Readable for CSR16rs {}
///`write(|w| ..)` method takes [`csr16::W`](W) writer structure
impl crate::Writable for CSR16rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR16 to value 0
impl crate::Resettable for CSR16rs {}
