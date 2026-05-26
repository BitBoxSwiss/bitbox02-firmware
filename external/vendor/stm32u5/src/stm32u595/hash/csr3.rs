///Register `CSR3` reader
pub type R = crate::R<CSR3rs>;
///Register `CSR3` writer
pub type W = crate::W<CSR3rs>;
///Field `CS3` reader - CS3
pub type CS3_R = crate::FieldReader<u32>;
///Field `CS3` writer - CS3
pub type CS3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS3
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR3").field("cs3", &self.cs3()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS3
    #[inline(always)]
    pub fn cs3(&mut self) -> CS3_W<CSR3rs> {
        CS3_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:CSR3)*/
pub struct CSR3rs;
impl crate::RegisterSpec for CSR3rs {
    type Ux = u32;
}
///`read()` method returns [`csr3::R`](R) reader structure
impl crate::Readable for CSR3rs {}
///`write(|w| ..)` method takes [`csr3::W`](W) writer structure
impl crate::Writable for CSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR3 to value 0
impl crate::Resettable for CSR3rs {}
