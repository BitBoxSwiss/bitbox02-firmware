///Register `WCR` reader
pub type R = crate::R<WCRrs>;
///Register `WCR` writer
pub type W = crate::W<WCRrs>;
///Field `COLM` reader - Color mode This bit controls the display color mode in video mode.
pub type COLM_R = crate::BitReader;
///Field `COLM` writer - Color mode This bit controls the display color mode in video mode.
pub type COLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHTDN` reader - Shutdown This bit controls the display shutdown in video mode.
pub type SHTDN_R = crate::BitReader;
///Field `SHTDN` writer - Shutdown This bit controls the display shutdown in video mode.
pub type SHTDN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LTDCEN` reader - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
pub type LTDCEN_R = crate::BitReader;
///Field `LTDCEN` writer - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSIEN` reader - DSI enable This bit enables the DSI Wrapper.
pub type DSIEN_R = crate::BitReader;
///Field `DSIEN` writer - DSI enable This bit enables the DSI Wrapper.
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Color mode This bit controls the display color mode in video mode.
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shutdown This bit controls the display shutdown in video mode.
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DSI enable This bit enables the DSI Wrapper.
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCR")
            .field("colm", &self.colm())
            .field("shtdn", &self.shtdn())
            .field("ltdcen", &self.ltdcen())
            .field("dsien", &self.dsien())
            .finish()
    }
}
impl W {
    ///Bit 0 - Color mode This bit controls the display color mode in video mode.
    #[inline(always)]
    pub fn colm(&mut self) -> COLM_W<WCRrs> {
        COLM_W::new(self, 0)
    }
    ///Bit 1 - Shutdown This bit controls the display shutdown in video mode.
    #[inline(always)]
    pub fn shtdn(&mut self) -> SHTDN_W<WCRrs> {
        SHTDN_W::new(self, 1)
    }
    ///Bit 2 - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<WCRrs> {
        LTDCEN_W::new(self, 2)
    }
    ///Bit 3 - DSI enable This bit enables the DSI Wrapper.
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W<WCRrs> {
        DSIEN_W::new(self, 3)
    }
}
/**DSI Wrapper control register

You can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:WCR)*/
pub struct WCRrs;
impl crate::RegisterSpec for WCRrs {
    type Ux = u32;
}
///`read()` method returns [`wcr::R`](R) reader structure
impl crate::Readable for WCRrs {}
///`write(|w| ..)` method takes [`wcr::W`](W) writer structure
impl crate::Writable for WCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WCR to value 0
impl crate::Resettable for WCRrs {}
