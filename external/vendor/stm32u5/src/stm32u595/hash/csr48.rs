///Register `CSR48` reader
pub type R = crate::R<CSR48rs>;
///Register `CSR48` writer
pub type W = crate::W<CSR48rs>;
///Field `CS48` reader - CS48
pub type CS48_R = crate::FieldReader<u32>;
///Field `CS48` writer - CS48
pub type CS48_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS48
    #[inline(always)]
    pub fn cs48(&self) -> CS48_R {
        CS48_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR48").field("cs48", &self.cs48()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS48
    #[inline(always)]
    pub fn cs48(&mut self) -> CS48_W<CSR48rs> {
        CS48_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:CSR48)*/
pub struct CSR48rs;
impl crate::RegisterSpec for CSR48rs {
    type Ux = u32;
}
///`read()` method returns [`csr48::R`](R) reader structure
impl crate::Readable for CSR48rs {}
///`write(|w| ..)` method takes [`csr48::W`](W) writer structure
impl crate::Writable for CSR48rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR48 to value 0
impl crate::Resettable for CSR48rs {}
