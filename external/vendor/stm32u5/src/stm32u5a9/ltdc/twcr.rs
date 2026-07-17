///Register `TWCR` reader
pub type R = crate::R<TWCRrs>;
///Register `TWCR` writer
pub type W = crate::W<TWCRrs>;
///Field `TOTALH` reader - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1.
pub type TOTALH_R = crate::FieldReader<u16>;
///Field `TOTALH` writer - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1.
pub type TOTALH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `TOTALW` reader - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1.
pub type TOTALW_R = crate::FieldReader<u16>;
///Field `TOTALW` writer - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1.
pub type TOTALW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1.
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1.
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWCR")
            .field("totalh", &self.totalh())
            .field("totalw", &self.totalw())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - total height (in units of horizontal scan line) These bits defines the accumulated height which includes the vertical synchronization, vertical back porch, the active height and vertical front porch height lines minus 1.
    #[inline(always)]
    pub fn totalh(&mut self) -> TOTALH_W<TWCRrs> {
        TOTALH_W::new(self, 0)
    }
    ///Bits 16:27 - total width (in units of pixel clock period) These bits defines the accumulated total width which includes the horizontal synchronization, horizontal back porch, active width and horizontal front porch pixels minus 1.
    #[inline(always)]
    pub fn totalw(&mut self) -> TOTALW_W<TWCRrs> {
        TOTALW_W::new(self, 16)
    }
}
/**LTDC total width configuration register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:TWCR)*/
pub struct TWCRrs;
impl crate::RegisterSpec for TWCRrs {
    type Ux = u32;
}
///`read()` method returns [`twcr::R`](R) reader structure
impl crate::Readable for TWCRrs {}
///`write(|w| ..)` method takes [`twcr::W`](W) writer structure
impl crate::Writable for TWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TWCR to value 0
impl crate::Resettable for TWCRrs {}
