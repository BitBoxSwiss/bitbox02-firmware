///Register `WCFGR` reader
pub type R = crate::R<WCFGRrs>;
///Register `WCFGR` writer
pub type W = crate::W<WCFGRrs>;
///Field `DSIM` reader - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type DSIM_R = crate::BitReader;
///Field `DSIM` writer - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type DSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLMUX` reader - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
pub type COLMUX_R = crate::FieldReader;
///Field `COLMUX` writer - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
pub type COLMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TESRC` reader - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TESRC_R = crate::BitReader;
///Field `TESRC` writer - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TESRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEPOL` reader - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TEPOL_R = crate::BitReader;
///Field `TEPOL` writer - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AR` reader - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type AR_R = crate::BitReader;
///Field `AR` writer - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type AR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
pub type VSPOL_R = crate::BitReader;
///Field `VSPOL` writer - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCFGR")
            .field("dsim", &self.dsim())
            .field("colmux", &self.colmux())
            .field("tesrc", &self.tesrc())
            .field("tepol", &self.tepol())
            .field("ar", &self.ar())
            .field("vspol", &self.vspol())
            .finish()
    }
}
impl W {
    ///Bit 0 - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn dsim(&mut self) -> DSIM_W<WCFGRrs> {
        DSIM_W::new(self, 0)
    }
    ///Bits 1:3 - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
    #[inline(always)]
    pub fn colmux(&mut self) -> COLMUX_W<WCFGRrs> {
        COLMUX_W::new(self, 1)
    }
    ///Bit 4 - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tesrc(&mut self) -> TESRC_W<WCFGRrs> {
        TESRC_W::new(self, 4)
    }
    ///Bit 5 - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tepol(&mut self) -> TEPOL_W<WCFGRrs> {
        TEPOL_W::new(self, 5)
    }
    ///Bit 6 - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<WCFGRrs> {
        AR_W::new(self, 6)
    }
    ///Bit 7 - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0).
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<WCFGRrs> {
        VSPOL_W::new(self, 7)
    }
}
/**DSI Wrapper configuration register

You can [`read`](crate::Reg::read) this register and get [`wcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:WCFGR)*/
pub struct WCFGRrs;
impl crate::RegisterSpec for WCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`wcfgr::R`](R) reader structure
impl crate::Readable for WCFGRrs {}
///`write(|w| ..)` method takes [`wcfgr::W`](W) writer structure
impl crate::Writable for WCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WCFGR to value 0
impl crate::Resettable for WCFGRrs {}
