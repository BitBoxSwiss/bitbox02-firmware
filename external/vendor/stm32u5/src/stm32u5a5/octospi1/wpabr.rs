///Register `WPABR` reader
pub type R = crate::R<WPABRrs>;
///Register `WPABR` writer
pub type W = crate::W<WPABRrs>;
///Field `ALTERNATE` reader - Alternate bytes
pub type ALTERNATE_R = crate::FieldReader<u32>;
///Field `ALTERNATE` writer - Alternate bytes
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Alternate bytes
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPABR")
            .field("alternate", &self.alternate())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Alternate bytes
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W<WPABRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
/**wrap alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`wpabr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpabr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OCTOSPI1:WPABR)*/
pub struct WPABRrs;
impl crate::RegisterSpec for WPABRrs {
    type Ux = u32;
}
///`read()` method returns [`wpabr::R`](R) reader structure
impl crate::Readable for WPABRrs {}
///`write(|w| ..)` method takes [`wpabr::W`](W) writer structure
impl crate::Writable for WPABRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets WPABR to value 0
impl crate::Resettable for WPABRrs {}
