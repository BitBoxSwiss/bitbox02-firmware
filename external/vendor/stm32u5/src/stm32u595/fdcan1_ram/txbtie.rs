///Register `TXBTIE` reader
pub type R = crate::R<TXBTIErs>;
///Register `TXBTIE` writer
pub type W = crate::W<TXBTIErs>;
///Field `TIE` reader - Transmission Interrupt Enable
pub type TIE_R = crate::FieldReader;
///Field `TIE` writer - Transmission Interrupt Enable
pub type TIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Transmission Interrupt Enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTIE").field("tie", &self.tie()).finish()
    }
}
impl W {
    ///Bits 0:2 - Transmission Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<TXBTIErs> {
        TIE_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Transmission Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:TXBTIE)*/
pub struct TXBTIErs;
impl crate::RegisterSpec for TXBTIErs {
    type Ux = u32;
}
///`read()` method returns [`txbtie::R`](R) reader structure
impl crate::Readable for TXBTIErs {}
///`write(|w| ..)` method takes [`txbtie::W`](W) writer structure
impl crate::Writable for TXBTIErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBTIE to value 0
impl crate::Resettable for TXBTIErs {}
