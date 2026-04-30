///Register `CSR9` reader
pub type R = crate::R<CSR9rs>;
///Register `CSR9` writer
pub type W = crate::W<CSR9rs>;
///Field `CS9` reader - CS9
pub type CS9_R = crate::FieldReader<u32>;
///Field `CS9` writer - CS9
pub type CS9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS9
    #[inline(always)]
    pub fn cs9(&self) -> CS9_R {
        CS9_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR9").field("cs9", &self.cs9()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS9
    #[inline(always)]
    pub fn cs9(&mut self) -> CS9_W<CSR9rs> {
        CS9_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR9)*/
pub struct CSR9rs;
impl crate::RegisterSpec for CSR9rs {
    type Ux = u32;
}
///`read()` method returns [`csr9::R`](R) reader structure
impl crate::Readable for CSR9rs {}
///`write(|w| ..)` method takes [`csr9::W`](W) writer structure
impl crate::Writable for CSR9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR9 to value 0
impl crate::Resettable for CSR9rs {}
