///Register `DIEPINT4` reader
pub type R = crate::R<DIEPINT4rs>;
///Register `DIEPINT4` writer
pub type W = crate::W<DIEPINT4rs>;
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOC` reader - TOC
pub type TOC_R = crate::BitReader;
///Field `TOC` writer - TOC
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFE` reader - ITTXFE
pub type ITTXFE_R = crate::BitReader;
///Field `ITTXFE` writer - ITTXFE
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNM` reader - INEPNM
pub type INEPNM_R = crate::BitReader;
///Field `INEPNM` writer - INEPNM
pub type INEPNM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNE` reader - INEPNE
pub type INEPNE_R = crate::BitReader;
///Field `TXFE` reader - TXFE
pub type TXFE_R = crate::BitReader;
///Field `PKTDRPSTS` reader - PKTDRPSTS
pub type PKTDRPSTS_R = crate::BitReader;
///Field `PKTDRPSTS` writer - PKTDRPSTS
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - TOC
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ITTXFE
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - INEPNM
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - INEPNE
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFE
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - PKTDRPSTS
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT4")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("toc", &self.toc())
            .field("ittxfe", &self.ittxfe())
            .field("inepnm", &self.inepnm())
            .field("inepne", &self.inepne())
            .field("txfe", &self.txfe())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("nak", &self.nak())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<DIEPINT4rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<DIEPINT4rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 3 - TOC
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<DIEPINT4rs> {
        TOC_W::new(self, 3)
    }
    ///Bit 4 - ITTXFE
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<DIEPINT4rs> {
        ITTXFE_W::new(self, 4)
    }
    ///Bit 5 - INEPNM
    #[inline(always)]
    pub fn inepnm(&mut self) -> INEPNM_W<DIEPINT4rs> {
        INEPNM_W::new(self, 5)
    }
    ///Bit 11 - PKTDRPSTS
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DIEPINT4rs> {
        PKTDRPSTS_W::new(self, 11)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<DIEPINT4rs> {
        NAK_W::new(self, 13)
    }
}
/**This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:DIEPINT4)*/
pub struct DIEPINT4rs;
impl crate::RegisterSpec for DIEPINT4rs {
    type Ux = u32;
}
///`read()` method returns [`diepint4::R`](R) reader structure
impl crate::Readable for DIEPINT4rs {}
///`write(|w| ..)` method takes [`diepint4::W`](W) writer structure
impl crate::Writable for DIEPINT4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPINT4 to value 0x80
impl crate::Resettable for DIEPINT4rs {
    const RESET_VALUE: u32 = 0x80;
}
