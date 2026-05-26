///Register `WABR` reader
pub type R = crate::R<WABRrs>;
///Register `WABR` writer
pub type W = crate::W<WABRrs>;
///Field `ALTERNATE` reader - ALTERNATE
pub type ALTERNATE_R = crate::FieldReader<u32>;
///Field `ALTERNATE` writer - ALTERNATE
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WABR")
            .field("alternate", &self.alternate())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W<WABRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
/**write alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`wabr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wabr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OCTOSPI1:WABR)*/
pub struct WABRrs;
impl crate::RegisterSpec for WABRrs {
    type Ux = u32;
}
///`read()` method returns [`wabr::R`](R) reader structure
impl crate::Readable for WABRrs {}
///`write(|w| ..)` method takes [`wabr::W`](W) writer structure
impl crate::Writable for WABRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets WABR to value 0
impl crate::Resettable for WABRrs {}
