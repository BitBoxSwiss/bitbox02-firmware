///Register `CSR45` reader
pub type R = crate::R<CSR45rs>;
///Register `CSR45` writer
pub type W = crate::W<CSR45rs>;
///Field `CS45` reader - CS45
pub type CS45_R = crate::FieldReader<u32>;
///Field `CS45` writer - CS45
pub type CS45_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS45
    #[inline(always)]
    pub fn cs45(&self) -> CS45_R {
        CS45_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR45").field("cs45", &self.cs45()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS45
    #[inline(always)]
    pub fn cs45(&mut self) -> CS45_W<CSR45rs> {
        CS45_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR45)*/
pub struct CSR45rs;
impl crate::RegisterSpec for CSR45rs {
    type Ux = u32;
}
///`read()` method returns [`csr45::R`](R) reader structure
impl crate::Readable for CSR45rs {}
///`write(|w| ..)` method takes [`csr45::W`](W) writer structure
impl crate::Writable for CSR45rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR45 to value 0
impl crate::Resettable for CSR45rs {}
