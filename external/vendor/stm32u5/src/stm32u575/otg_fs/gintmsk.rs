///Register `GINTMSK` reader
pub type R = crate::R<GINTMSKrs>;
///Register `GINTMSK` writer
pub type W = crate::W<GINTMSKrs>;
///Field `MMISM` reader - MMISM
pub type MMISM_R = crate::BitReader;
///Field `MMISM` writer - MMISM
pub type MMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINT` reader - OTGINT
pub type OTGINT_R = crate::BitReader;
///Field `OTGINT` writer - OTGINT
pub type OTGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFM` reader - SOFM
pub type SOFM_R = crate::BitReader;
///Field `SOFM` writer - SOFM
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVLM` reader - RXFLVLM
pub type RXFLVLM_R = crate::BitReader;
///Field `RXFLVLM` writer - RXFLVLM
pub type RXFLVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPTXFEM` reader - NPTXFEM
pub type NPTXFEM_R = crate::BitReader;
///Field `NPTXFEM` writer - NPTXFEM
pub type NPTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GINAKEFFM` reader - GINAKEFFM
pub type GINAKEFFM_R = crate::BitReader;
///Field `GINAKEFFM` writer - GINAKEFFM
pub type GINAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GONAKEFFM` reader - GONAKEFFM
pub type GONAKEFFM_R = crate::BitReader;
///Field `GONAKEFFM` writer - GONAKEFFM
pub type GONAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESUSPM` reader - ESUSPM
pub type ESUSPM_R = crate::BitReader;
///Field `ESUSPM` writer - ESUSPM
pub type ESUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSUSPM` reader - USBSUSPM
pub type USBSUSPM_R = crate::BitReader;
///Field `USBSUSPM` writer - USBSUSPM
pub type USBSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRST` reader - USBRST
pub type USBRST_R = crate::BitReader;
///Field `USBRST` writer - USBRST
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENUMDNEM` reader - ENUMDNEM
pub type ENUMDNEM_R = crate::BitReader;
///Field `ENUMDNEM` writer - ENUMDNEM
pub type ENUMDNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOODRPM` reader - ISOODRPM
pub type ISOODRPM_R = crate::BitReader;
///Field `ISOODRPM` writer - ISOODRPM
pub type ISOODRPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPFM` reader - EOPFM
pub type EOPFM_R = crate::BitReader;
///Field `EOPFM` writer - EOPFM
pub type EOPFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEPINT` reader - IEPINT
pub type IEPINT_R = crate::BitReader;
///Field `IEPINT` writer - IEPINT
pub type IEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OEPINT` reader - OEPINT
pub type OEPINT_R = crate::BitReader;
///Field `OEPINT` writer - OEPINT
pub type OEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IISOIXFRM` reader - IISOIXFRM
pub type IISOIXFRM_R = crate::BitReader;
///Field `IISOIXFRM` writer - IISOIXFRM
pub type IISOIXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPXFRM` reader - IPXFRM
pub type IPXFRM_R = crate::BitReader;
///Field `IPXFRM` writer - IPXFRM
pub type IPXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTDETM` reader - RSTDETM
pub type RSTDETM_R = crate::BitReader;
///Field `RSTDETM` writer - RSTDETM
pub type RSTDETM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRTIM` reader - PRTIM
pub type PRTIM_R = crate::BitReader;
///Field `PRTIM` writer - PRTIM
pub type PRTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCIM` reader - HCIM
pub type HCIM_R = crate::BitReader;
///Field `HCIM` writer - HCIM
pub type HCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTXFEM` reader - PTXFEM
pub type PTXFEM_R = crate::BitReader;
///Field `PTXFEM` writer - PTXFEM
pub type PTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMINTM` reader - LPMINTM
pub type LPMINTM_R = crate::BitReader;
///Field `LPMINTM` writer - LPMINTM
pub type LPMINTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSCHGM` reader - CIDSCHGM
pub type CIDSCHGM_R = crate::BitReader;
///Field `CIDSCHGM` writer - CIDSCHGM
pub type CIDSCHGM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCINT` reader - DISCINT
pub type DISCINT_R = crate::BitReader;
///Field `DISCINT` writer - DISCINT
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRQIM` reader - SRQIM
pub type SRQIM_R = crate::BitReader;
///Field `SRQIM` writer - SRQIM
pub type SRQIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUIM` reader - WUIM
pub type WUIM_R = crate::BitReader;
///Field `WUIM` writer - WUIM
pub type WUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - MMISM
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTGINT
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SOFM
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RXFLVLM
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NPTXFEM
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GINAKEFFM
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GONAKEFFM
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - ESUSPM
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USBSUSPM
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USBRST
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ENUMDNEM
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ISOODRPM
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EOPFM
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
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
    ///Bit 20 - IISOIXFRM
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - IPXFRM
    #[inline(always)]
    pub fn ipxfrm(&self) -> IPXFRM_R {
        IPXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - RSTDETM
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - PRTIM
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - HCIM
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PTXFEM
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LPMINTM
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CIDSCHGM
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DISCINT
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRQIM
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - WUIM
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("mmism", &self.mmism())
            .field("otgint", &self.otgint())
            .field("sofm", &self.sofm())
            .field("rxflvlm", &self.rxflvlm())
            .field("nptxfem", &self.nptxfem())
            .field("ginakeffm", &self.ginakeffm())
            .field("gonakeffm", &self.gonakeffm())
            .field("esuspm", &self.esuspm())
            .field("usbsuspm", &self.usbsuspm())
            .field("usbrst", &self.usbrst())
            .field("enumdnem", &self.enumdnem())
            .field("isoodrpm", &self.isoodrpm())
            .field("eopfm", &self.eopfm())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("iisoixfrm", &self.iisoixfrm())
            .field("ipxfrm", &self.ipxfrm())
            .field("rstdetm", &self.rstdetm())
            .field("prtim", &self.prtim())
            .field("hcim", &self.hcim())
            .field("ptxfem", &self.ptxfem())
            .field("lpmintm", &self.lpmintm())
            .field("cidschgm", &self.cidschgm())
            .field("discint", &self.discint())
            .field("srqim", &self.srqim())
            .field("wuim", &self.wuim())
            .finish()
    }
}
impl W {
    ///Bit 1 - MMISM
    #[inline(always)]
    pub fn mmism(&mut self) -> MMISM_W<GINTMSKrs> {
        MMISM_W::new(self, 1)
    }
    ///Bit 2 - OTGINT
    #[inline(always)]
    pub fn otgint(&mut self) -> OTGINT_W<GINTMSKrs> {
        OTGINT_W::new(self, 2)
    }
    ///Bit 3 - SOFM
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<GINTMSKrs> {
        SOFM_W::new(self, 3)
    }
    ///Bit 4 - RXFLVLM
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<GINTMSKrs> {
        RXFLVLM_W::new(self, 4)
    }
    ///Bit 5 - NPTXFEM
    #[inline(always)]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<GINTMSKrs> {
        NPTXFEM_W::new(self, 5)
    }
    ///Bit 6 - GINAKEFFM
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<GINTMSKrs> {
        GINAKEFFM_W::new(self, 6)
    }
    ///Bit 7 - GONAKEFFM
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<GINTMSKrs> {
        GONAKEFFM_W::new(self, 7)
    }
    ///Bit 10 - ESUSPM
    #[inline(always)]
    pub fn esuspm(&mut self) -> ESUSPM_W<GINTMSKrs> {
        ESUSPM_W::new(self, 10)
    }
    ///Bit 11 - USBSUSPM
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<GINTMSKrs> {
        USBSUSPM_W::new(self, 11)
    }
    ///Bit 12 - USBRST
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<GINTMSKrs> {
        USBRST_W::new(self, 12)
    }
    ///Bit 13 - ENUMDNEM
    #[inline(always)]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<GINTMSKrs> {
        ENUMDNEM_W::new(self, 13)
    }
    ///Bit 14 - ISOODRPM
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<GINTMSKrs> {
        ISOODRPM_W::new(self, 14)
    }
    ///Bit 15 - EOPFM
    #[inline(always)]
    pub fn eopfm(&mut self) -> EOPFM_W<GINTMSKrs> {
        EOPFM_W::new(self, 15)
    }
    ///Bit 18 - IEPINT
    #[inline(always)]
    pub fn iepint(&mut self) -> IEPINT_W<GINTMSKrs> {
        IEPINT_W::new(self, 18)
    }
    ///Bit 19 - OEPINT
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W<GINTMSKrs> {
        OEPINT_W::new(self, 19)
    }
    ///Bit 20 - IISOIXFRM
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<GINTMSKrs> {
        IISOIXFRM_W::new(self, 20)
    }
    ///Bit 21 - IPXFRM
    #[inline(always)]
    pub fn ipxfrm(&mut self) -> IPXFRM_W<GINTMSKrs> {
        IPXFRM_W::new(self, 21)
    }
    ///Bit 23 - RSTDETM
    #[inline(always)]
    pub fn rstdetm(&mut self) -> RSTDETM_W<GINTMSKrs> {
        RSTDETM_W::new(self, 23)
    }
    ///Bit 24 - PRTIM
    #[inline(always)]
    pub fn prtim(&mut self) -> PRTIM_W<GINTMSKrs> {
        PRTIM_W::new(self, 24)
    }
    ///Bit 25 - HCIM
    #[inline(always)]
    pub fn hcim(&mut self) -> HCIM_W<GINTMSKrs> {
        HCIM_W::new(self, 25)
    }
    ///Bit 26 - PTXFEM
    #[inline(always)]
    pub fn ptxfem(&mut self) -> PTXFEM_W<GINTMSKrs> {
        PTXFEM_W::new(self, 26)
    }
    ///Bit 27 - LPMINTM
    #[inline(always)]
    pub fn lpmintm(&mut self) -> LPMINTM_W<GINTMSKrs> {
        LPMINTM_W::new(self, 27)
    }
    ///Bit 28 - CIDSCHGM
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<GINTMSKrs> {
        CIDSCHGM_W::new(self, 28)
    }
    ///Bit 29 - DISCINT
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W<GINTMSKrs> {
        DISCINT_W::new(self, 29)
    }
    ///Bit 30 - SRQIM
    #[inline(always)]
    pub fn srqim(&mut self) -> SRQIM_W<GINTMSKrs> {
        SRQIM_W::new(self, 30)
    }
    ///Bit 31 - WUIM
    #[inline(always)]
    pub fn wuim(&mut self) -> WUIM_W<GINTMSKrs> {
        WUIM_W::new(self, 31)
    }
}
/**This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.

You can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OTG_FS:GINTMSK)*/
pub struct GINTMSKrs;
impl crate::RegisterSpec for GINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`gintmsk::R`](R) reader structure
impl crate::Readable for GINTMSKrs {}
///`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure
impl crate::Writable for GINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GINTMSK to value 0
impl crate::Resettable for GINTMSKrs {}
