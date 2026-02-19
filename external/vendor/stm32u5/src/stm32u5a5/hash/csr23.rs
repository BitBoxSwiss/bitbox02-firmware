///Register `CSR23` reader
pub type R = crate::R<CSR23rs>;
///Register `CSR23` writer
pub type W = crate::W<CSR23rs>;
///Field `CS23` reader - CS23
pub type CS23_R = crate::FieldReader<u32>;
///Field `CS23` writer - CS23
pub type CS23_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS23
    #[inline(always)]
    pub fn cs23(&self) -> CS23_R {
        CS23_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR23").field("cs23", &self.cs23()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS23
    #[inline(always)]
    pub fn cs23(&mut self) -> CS23_W<CSR23rs> {
        CS23_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#HASH:CSR23)*/
pub struct CSR23rs;
impl crate::RegisterSpec for CSR23rs {
    type Ux = u32;
}
///`read()` method returns [`csr23::R`](R) reader structure
impl crate::Readable for CSR23rs {}
///`write(|w| ..)` method takes [`csr23::W`](W) writer structure
impl crate::Writable for CSR23rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR23 to value 0
impl crate::Resettable for CSR23rs {}
