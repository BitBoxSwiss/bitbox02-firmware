///Register `CSR29` reader
pub type R = crate::R<CSR29rs>;
///Register `CSR29` writer
pub type W = crate::W<CSR29rs>;
///Field `CS29` reader - CS29
pub type CS29_R = crate::FieldReader<u32>;
///Field `CS29` writer - CS29
pub type CS29_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS29
    #[inline(always)]
    pub fn cs29(&self) -> CS29_R {
        CS29_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR29").field("cs29", &self.cs29()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS29
    #[inline(always)]
    pub fn cs29(&mut self) -> CS29_W<CSR29rs> {
        CS29_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#HASH:CSR29)*/
pub struct CSR29rs;
impl crate::RegisterSpec for CSR29rs {
    type Ux = u32;
}
///`read()` method returns [`csr29::R`](R) reader structure
impl crate::Readable for CSR29rs {}
///`write(|w| ..)` method takes [`csr29::W`](W) writer structure
impl crate::Writable for CSR29rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR29 to value 0
impl crate::Resettable for CSR29rs {}
