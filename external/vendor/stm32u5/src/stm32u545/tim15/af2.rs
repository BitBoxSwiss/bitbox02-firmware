///Register `AF2` reader
pub type R = crate::R<AF2rs>;
///Register `AF2` writer
pub type W = crate::W<AF2rs>;
///Field `OCRSEL` reader - ocref_clr source selection
pub type OCRSEL_R = crate::FieldReader;
///Field `OCRSEL` writer - ocref_clr source selection
pub type OCRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 16:18 - ocref_clr source selection
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("ocrsel", &self.ocrsel())
            .finish()
    }
}
impl W {
    ///Bits 16:18 - ocref_clr source selection
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OCRSEL_W<AF2rs> {
        OCRSEL_W::new(self, 16)
    }
}
/**alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#TIM15:AF2)*/
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
///`read()` method returns [`af2::R`](R) reader structure
impl crate::Readable for AF2rs {}
///`write(|w| ..)` method takes [`af2::W`](W) writer structure
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF2 to value 0x01
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
