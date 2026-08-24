///Register `SSCR` reader
pub type R = crate::R<SSCRrs>;
///Register `SSCR` writer
pub type W = crate::W<SSCRrs>;
///Field `VSH` reader - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
pub type VSH_R = crate::FieldReader<u16>;
///Field `VSH` writer - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
pub type VSH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `HSW` reader - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
pub type HSW_R = crate::FieldReader<u16>;
///Field `HSW` writer - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
pub type HSW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSCR")
            .field("vsh", &self.vsh())
            .field("hsw", &self.hsw())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
    #[inline(always)]
    pub fn vsh(&mut self) -> VSH_W<SSCRrs> {
        VSH_W::new(self, 0)
    }
    ///Bits 16:27 - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W<SSCRrs> {
        HSW_W::new(self, 16)
    }
}
/**LTDC synchronization size configuration register

You can [`read`](crate::Reg::read) this register and get [`sscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:SSCR)*/
pub struct SSCRrs;
impl crate::RegisterSpec for SSCRrs {
    type Ux = u32;
}
///`read()` method returns [`sscr::R`](R) reader structure
impl crate::Readable for SSCRrs {}
///`write(|w| ..)` method takes [`sscr::W`](W) writer structure
impl crate::Writable for SSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSCR to value 0
impl crate::Resettable for SSCRrs {}
