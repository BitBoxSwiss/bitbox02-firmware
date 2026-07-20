///Register `GINTSTS` reader
pub type R = crate::R<GINTSTSrs>;
///Register `GINTSTS` writer
pub type W = crate::W<GINTSTSrs>;
///Field `CMOD` reader - CMOD
pub type CMOD_R = crate::BitReader;
///Field `MMIS` reader - MMIS
pub type MMIS_R = crate::BitReader;
///Field `MMIS` writer - MMIS
pub type MMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINT` reader - OTGINT
pub type OTGINT_R = crate::BitReader;
///Field `SOF` reader - SOF
pub type SOF_R = crate::BitReader;
///Field `SOF` writer - SOF
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVL` reader - RXFLVL
pub type RXFLVL_R = crate::BitReader;
///Field `NPTXFE` reader - NPTXFE
pub type NPTXFE_R = crate::BitReader;
///Field `GINAKEFF` reader - GINAKEFF
pub type GINAKEFF_R = crate::BitReader;
///Field `GONAKEFF` reader - GONAKEFF
pub type GONAKEFF_R = crate::BitReader;
///Field `ESUSP` reader - ESUSP
pub type ESUSP_R = crate::BitReader;
///Field `ESUSP` writer - ESUSP
pub type ESUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSUSP` reader - USBSUSP
pub type USBSUSP_R = crate::BitReader;
///Field `USBSUSP` writer - USBSUSP
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRST` reader - USBRST
pub type USBRST_R = crate::BitReader;
///Field `USBRST` writer - USBRST
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENUMDNE` reader - ENUMDNE
pub type ENUMDNE_R = crate::BitReader;
///Field `ENUMDNE` writer - ENUMDNE
pub type ENUMDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOODRP` reader - ISOODRP
pub type ISOODRP_R = crate::BitReader;
///Field `ISOODRP` writer - ISOODRP
pub type ISOODRP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPF` reader - EOPF
pub type EOPF_R = crate::BitReader;
///Field `EOPF` writer - EOPF
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEPINT` reader - IEPINT
pub type IEPINT_R = crate::BitReader;
///Field `OEPINT` reader - OEPINT
pub type OEPINT_R = crate::BitReader;
///Field `IISOIXFR` reader - IISOIXFR
pub type IISOIXFR_R = crate::BitReader;
///Field `IISOIXFR` writer - IISOIXFR
pub type IISOIXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPXFR` reader - IPXFR
pub type IPXFR_R = crate::BitReader;
///Field `IPXFR` writer - IPXFR
pub type IPXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAFSUSP` reader - DATAFSUSP
pub type DATAFSUSP_R = crate::BitReader;
///Field `DATAFSUSP` writer - DATAFSUSP
pub type DATAFSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTDET` reader - RSTDET
pub type RSTDET_R = crate::BitReader;
///Field `RSTDET` writer - RSTDET
pub type RSTDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPRTINT` reader - HPRTINT
pub type HPRTINT_R = crate::BitReader;
///Field `HCINT` reader - HCINT
pub type HCINT_R = crate::BitReader;
///Field `PTXFE` reader - PTXFE
pub type PTXFE_R = crate::BitReader;
///Field `LPMINT` reader - LPMINT
pub type LPMINT_R = crate::BitReader;
///Field `LPMINT` writer - LPMINT
pub type LPMINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSCHG` reader - CIDSCHG
pub type CIDSCHG_R = crate::BitReader;
///Field `CIDSCHG` writer - CIDSCHG
pub type CIDSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCINT` reader - DISCINT
pub type DISCINT_R = crate::BitReader;
///Field `DISCINT` writer - DISCINT
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRQINT` reader - SRQINT
pub type SRQINT_R = crate::BitReader;
///Field `SRQINT` writer - SRQINT
pub type SRQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPINT` reader - WKUPINT
pub type WKUPINT_R = crate::BitReader;
///Field `WKUPINT` writer - WKUPINT
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CMOD
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MMIS
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTGINT
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SOF
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RXFLVL
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NPTXFE
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GINAKEFF
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GONAKEFF
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - ESUSP
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USBSUSP
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USBRST
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ENUMDNE
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ISOODRP
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EOPF
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - IEPINT
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OEPINT
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IISOIXFR
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - IPXFR
    #[inline(always)]
    pub fn ipxfr(&self) -> IPXFR_R {
        IPXFR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DATAFSUSP
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - RSTDET
    #[inline(always)]
    pub fn rstdet(&self) -> RSTDET_R {
        RSTDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - HPRTINT
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - HCINT
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PTXFE
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LPMINT
    #[inline(always)]
    pub fn lpmint(&self) -> LPMINT_R {
        LPMINT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CIDSCHG
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DISCINT
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRQINT
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - WKUPINT
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("cmod", &self.cmod())
            .field("mmis", &self.mmis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvl", &self.rxflvl())
            .field("nptxfe", &self.nptxfe())
            .field("ginakeff", &self.ginakeff())
            .field("gonakeff", &self.gonakeff())
            .field("esusp", &self.esusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdne", &self.enumdne())
            .field("isoodrp", &self.isoodrp())
            .field("eopf", &self.eopf())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("iisoixfr", &self.iisoixfr())
            .field("ipxfr", &self.ipxfr())
            .field("datafsusp", &self.datafsusp())
            .field("rstdet", &self.rstdet())
            .field("hprtint", &self.hprtint())
            .field("hcint", &self.hcint())
            .field("ptxfe", &self.ptxfe())
            .field("lpmint", &self.lpmint())
            .field("cidschg", &self.cidschg())
            .field("discint", &self.discint())
            .field("srqint", &self.srqint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
impl W {
    ///Bit 1 - MMIS
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W<GINTSTSrs> {
        MMIS_W::new(self, 1)
    }
    ///Bit 3 - SOF
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<GINTSTSrs> {
        SOF_W::new(self, 3)
    }
    ///Bit 10 - ESUSP
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W<GINTSTSrs> {
        ESUSP_W::new(self, 10)
    }
    ///Bit 11 - USBSUSP
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<GINTSTSrs> {
        USBSUSP_W::new(self, 11)
    }
    ///Bit 12 - USBRST
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<GINTSTSrs> {
        USBRST_W::new(self, 12)
    }
    ///Bit 13 - ENUMDNE
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W<GINTSTSrs> {
        ENUMDNE_W::new(self, 13)
    }
    ///Bit 14 - ISOODRP
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W<GINTSTSrs> {
        ISOODRP_W::new(self, 14)
    }
    ///Bit 15 - EOPF
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<GINTSTSrs> {
        EOPF_W::new(self, 15)
    }
    ///Bit 20 - IISOIXFR
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<GINTSTSrs> {
        IISOIXFR_W::new(self, 20)
    }
    ///Bit 21 - IPXFR
    #[inline(always)]
    pub fn ipxfr(&mut self) -> IPXFR_W<GINTSTSrs> {
        IPXFR_W::new(self, 21)
    }
    ///Bit 22 - DATAFSUSP
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<GINTSTSrs> {
        DATAFSUSP_W::new(self, 22)
    }
    ///Bit 23 - RSTDET
    #[inline(always)]
    pub fn rstdet(&mut self) -> RSTDET_W<GINTSTSrs> {
        RSTDET_W::new(self, 23)
    }
    ///Bit 27 - LPMINT
    #[inline(always)]
    pub fn lpmint(&mut self) -> LPMINT_W<GINTSTSrs> {
        LPMINT_W::new(self, 27)
    }
    ///Bit 28 - CIDSCHG
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W<GINTSTSrs> {
        CIDSCHG_W::new(self, 28)
    }
    ///Bit 29 - DISCINT
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W<GINTSTSrs> {
        DISCINT_W::new(self, 29)
    }
    ///Bit 30 - SRQINT
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W<GINTSTSrs> {
        SRQINT_W::new(self, 30)
    }
    ///Bit 31 - WKUPINT
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W<GINTSTSrs> {
        WKUPINT_W::new(self, 31)
    }
}
/**This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.

You can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:GINTSTS)*/
pub struct GINTSTSrs;
impl crate::RegisterSpec for GINTSTSrs {
    type Ux = u32;
}
///`read()` method returns [`gintsts::R`](R) reader structure
impl crate::Readable for GINTSTSrs {}
///`write(|w| ..)` method takes [`gintsts::W`](W) writer structure
impl crate::Writable for GINTSTSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GINTSTS to value 0x0400_0020
impl crate::Resettable for GINTSTSrs {
    const RESET_VALUE: u32 = 0x0400_0020;
}
