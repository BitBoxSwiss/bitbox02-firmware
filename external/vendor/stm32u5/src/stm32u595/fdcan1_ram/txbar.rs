///Register `TXBAR` reader
pub type R = crate::R<TXBARrs>;
///Register `TXBAR` writer
pub type W = crate::W<TXBARrs>;
///Field `AR` reader - Add Request
pub type AR_R = crate::FieldReader;
///Field `AR` writer - Add Request
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Add Request
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBAR").field("ar", &self.ar()).finish()
    }
}
impl W {
    ///Bits 0:2 - Add Request
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<TXBARrs> {
        AR_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Add Request Register

You can [`read`](crate::Reg::read) this register and get [`txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:TXBAR)*/
pub struct TXBARrs;
impl crate::RegisterSpec for TXBARrs {
    type Ux = u32;
}
///`read()` method returns [`txbar::R`](R) reader structure
impl crate::Readable for TXBARrs {}
///`write(|w| ..)` method takes [`txbar::W`](W) writer structure
impl crate::Writable for TXBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBAR to value 0
impl crate::Resettable for TXBARrs {}
