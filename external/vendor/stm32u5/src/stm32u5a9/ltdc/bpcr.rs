///Register `BPCR` reader
pub type R = crate::R<BPCRrs>;
///Register `BPCR` writer
pub type W = crate::W<BPCRrs>;
///Field `AVBP` reader - accumulated Vertical back porch (in units of horizontal scan line) These bits define the accumulated vertical back porch width that includes the vertical synchronization and vertical back porch lines minus 1. The vertical back porch is the number of horizontal scan lines at a start of frame to the start of the first active scan line of the next frame.
pub type AVBP_R = crate::FieldReader<u16>;
///Field `AVBP` writer - accumulated Vertical back porch (in units of horizontal scan line) These bits define the accumulated vertical back porch width that includes the vertical synchronization and vertical back porch lines minus 1. The vertical back porch is the number of horizontal scan lines at a start of frame to the start of the first active scan line of the next frame.
pub type AVBP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `AHBP` reader - accumulated horizontal back porch (in units of pixel clock period) These bits define the accumulated horizontal back porch width that includes the horizontal synchronization and horizontal back porch pixels minus 1. The horizontal back porch is the period between horizontal synchronization going inactive and the start of the active display part of the next scan line.
pub type AHBP_R = crate::FieldReader<u16>;
///Field `AHBP` writer - accumulated horizontal back porch (in units of pixel clock period) These bits define the accumulated horizontal back porch width that includes the horizontal synchronization and horizontal back porch pixels minus 1. The horizontal back porch is the period between horizontal synchronization going inactive and the start of the active display part of the next scan line.
pub type AHBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - accumulated Vertical back porch (in units of horizontal scan line) These bits define the accumulated vertical back porch width that includes the vertical synchronization and vertical back porch lines minus 1. The vertical back porch is the number of horizontal scan lines at a start of frame to the start of the first active scan line of the next frame.
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - accumulated horizontal back porch (in units of pixel clock period) These bits define the accumulated horizontal back porch width that includes the horizontal synchronization and horizontal back porch pixels minus 1. The horizontal back porch is the period between horizontal synchronization going inactive and the start of the active display part of the next scan line.
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPCR")
            .field("avbp", &self.avbp())
            .field("ahbp", &self.ahbp())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - accumulated Vertical back porch (in units of horizontal scan line) These bits define the accumulated vertical back porch width that includes the vertical synchronization and vertical back porch lines minus 1. The vertical back porch is the number of horizontal scan lines at a start of frame to the start of the first active scan line of the next frame.
    #[inline(always)]
    pub fn avbp(&mut self) -> AVBP_W<BPCRrs> {
        AVBP_W::new(self, 0)
    }
    ///Bits 16:27 - accumulated horizontal back porch (in units of pixel clock period) These bits define the accumulated horizontal back porch width that includes the horizontal synchronization and horizontal back porch pixels minus 1. The horizontal back porch is the period between horizontal synchronization going inactive and the start of the active display part of the next scan line.
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W<BPCRrs> {
        AHBP_W::new(self, 16)
    }
}
/**LTDC back porch configuration register

You can [`read`](crate::Reg::read) this register and get [`bpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:BPCR)*/
pub struct BPCRrs;
impl crate::RegisterSpec for BPCRrs {
    type Ux = u32;
}
///`read()` method returns [`bpcr::R`](R) reader structure
impl crate::Readable for BPCRrs {}
///`write(|w| ..)` method takes [`bpcr::W`](W) writer structure
impl crate::Writable for BPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BPCR to value 0
impl crate::Resettable for BPCRrs {}
