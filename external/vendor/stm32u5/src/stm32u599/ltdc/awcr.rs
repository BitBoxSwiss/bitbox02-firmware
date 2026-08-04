///Register `AWCR` reader
pub type R = crate::R<AWCRrs>;
///Register `AWCR` writer
pub type W = crate::W<AWCRrs>;
///Field `AAH` reader - accumulated active height (in units of horizontal scan line) These bits define the accumulated height which includes the vertical synchronization, vertical back porch and the active height lines minus 1. The active height is the number of active lines in the panel. Refer to device datasheet for maximum active height supported following maximum pixel clock.
pub type AAH_R = crate::FieldReader<u16>;
///Field `AAH` writer - accumulated active height (in units of horizontal scan line) These bits define the accumulated height which includes the vertical synchronization, vertical back porch and the active height lines minus 1. The active height is the number of active lines in the panel. Refer to device datasheet for maximum active height supported following maximum pixel clock.
pub type AAH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `AAW` reader - accumulated active width (in units of pixel clock period) These bits define the accumulated active width which includes the horizontal synchronization, horizontal back porch and active pixels minus 1. The active width is the number of pixels in active display area of the panel scan line. Refer to device datasheet for maximum active width supported following maximum pixel clock.
pub type AAW_R = crate::FieldReader<u16>;
///Field `AAW` writer - accumulated active width (in units of pixel clock period) These bits define the accumulated active width which includes the horizontal synchronization, horizontal back porch and active pixels minus 1. The active width is the number of pixels in active display area of the panel scan line. Refer to device datasheet for maximum active width supported following maximum pixel clock.
pub type AAW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - accumulated active height (in units of horizontal scan line) These bits define the accumulated height which includes the vertical synchronization, vertical back porch and the active height lines minus 1. The active height is the number of active lines in the panel. Refer to device datasheet for maximum active height supported following maximum pixel clock.
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - accumulated active width (in units of pixel clock period) These bits define the accumulated active width which includes the horizontal synchronization, horizontal back porch and active pixels minus 1. The active width is the number of pixels in active display area of the panel scan line. Refer to device datasheet for maximum active width supported following maximum pixel clock.
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWCR")
            .field("aah", &self.aah())
            .field("aaw", &self.aaw())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - accumulated active height (in units of horizontal scan line) These bits define the accumulated height which includes the vertical synchronization, vertical back porch and the active height lines minus 1. The active height is the number of active lines in the panel. Refer to device datasheet for maximum active height supported following maximum pixel clock.
    #[inline(always)]
    pub fn aah(&mut self) -> AAH_W<AWCRrs> {
        AAH_W::new(self, 0)
    }
    ///Bits 16:27 - accumulated active width (in units of pixel clock period) These bits define the accumulated active width which includes the horizontal synchronization, horizontal back porch and active pixels minus 1. The active width is the number of pixels in active display area of the panel scan line. Refer to device datasheet for maximum active width supported following maximum pixel clock.
    #[inline(always)]
    pub fn aaw(&mut self) -> AAW_W<AWCRrs> {
        AAW_W::new(self, 16)
    }
}
/**LTDC active width configuration register

You can [`read`](crate::Reg::read) this register and get [`awcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:AWCR)*/
pub struct AWCRrs;
impl crate::RegisterSpec for AWCRrs {
    type Ux = u32;
}
///`read()` method returns [`awcr::R`](R) reader structure
impl crate::Readable for AWCRrs {}
///`write(|w| ..)` method takes [`awcr::W`](W) writer structure
impl crate::Writable for AWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWCR to value 0
impl crate::Resettable for AWCRrs {}
