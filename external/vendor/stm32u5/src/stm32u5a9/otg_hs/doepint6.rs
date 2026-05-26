///Register `DOEPINT6` reader
pub type R = crate::R<DOEPINT6rs>;
///Register `DOEPINT6` writer
pub type W = crate::W<DOEPINT6rs>;
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERR` reader - AHBERR
pub type AHBERR_R = crate::BitReader;
///Field `AHBERR` writer - AHBERR
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUP` reader - STUP
pub type STUP_R = crate::BitReader;
///Field `STUP` writer - STUP
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDIS` reader - OTEPDIS
pub type OTEPDIS_R = crate::BitReader;
///Field `OTEPDIS` writer - OTEPDIS
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STSPHSRX` reader - STSPHSRX
pub type STSPHSRX_R = crate::BitReader;
///Field `STSPHSRX` writer - STSPHSRX
pub type STSPHSRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2BSTUP` reader - B2BSTUP
pub type B2BSTUP_R = crate::BitReader;
///Field `B2BSTUP` writer - B2BSTUP
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTPKTERR` reader - OUTPKTERR
pub type OUTPKTERR_R = crate::BitReader;
///Field `OUTPKTERR` writer - OUTPKTERR
pub type OUTPKTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNA` reader - BNA
pub type BNA_R = crate::BitReader;
///Field `BNA` writer - BNA
pub type BNA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERR` reader - BERR
pub type BERR_R = crate::BitReader;
///Field `BERR` writer - BERR
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - NYET
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - NYET
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STPKTRX` reader - STPKTRX
pub type STPKTRX_R = crate::BitReader;
///Field `STPKTRX` writer - STPKTRX
pub type STPKTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - AHBERR
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STSPHSRX
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - OUTPKTERR
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNA
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - BERR
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NYET
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - STPKTRX
    #[inline(always)]
    pub fn stpktrx(&self) -> STPKTRX_R {
        STPKTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT6")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("ahberr", &self.ahberr())
            .field("stup", &self.stup())
            .field("otepdis", &self.otepdis())
            .field("stsphsrx", &self.stsphsrx())
            .field("b2bstup", &self.b2bstup())
            .field("outpkterr", &self.outpkterr())
            .field("bna", &self.bna())
            .field("berr", &self.berr())
            .field("nak", &self.nak())
            .field("nyet", &self.nyet())
            .field("stpktrx", &self.stpktrx())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<DOEPINT6rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<DOEPINT6rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 2 - AHBERR
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<DOEPINT6rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<DOEPINT6rs> {
        STUP_W::new(self, 3)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<DOEPINT6rs> {
        OTEPDIS_W::new(self, 4)
    }
    ///Bit 5 - STSPHSRX
    #[inline(always)]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<DOEPINT6rs> {
        STSPHSRX_W::new(self, 5)
    }
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<DOEPINT6rs> {
        B2BSTUP_W::new(self, 6)
    }
    ///Bit 8 - OUTPKTERR
    #[inline(always)]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<DOEPINT6rs> {
        OUTPKTERR_W::new(self, 8)
    }
    ///Bit 9 - BNA
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W<DOEPINT6rs> {
        BNA_W::new(self, 9)
    }
    ///Bit 12 - BERR
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<DOEPINT6rs> {
        BERR_W::new(self, 12)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<DOEPINT6rs> {
        NAK_W::new(self, 13)
    }
    ///Bit 14 - NYET
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<DOEPINT6rs> {
        NYET_W::new(self, 14)
    }
    ///Bit 15 - STPKTRX
    #[inline(always)]
    pub fn stpktrx(&mut self) -> STPKTRX_W<DOEPINT6rs> {
        STPKTRX_W::new(self, 15)
    }
}
/**This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DOEPINT6)*/
pub struct DOEPINT6rs;
impl crate::RegisterSpec for DOEPINT6rs {
    type Ux = u32;
}
///`read()` method returns [`doepint6::R`](R) reader structure
impl crate::Readable for DOEPINT6rs {}
///`write(|w| ..)` method takes [`doepint6::W`](W) writer structure
impl crate::Writable for DOEPINT6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPINT6 to value 0x80
impl crate::Resettable for DOEPINT6rs {
    const RESET_VALUE: u32 = 0x80;
}
