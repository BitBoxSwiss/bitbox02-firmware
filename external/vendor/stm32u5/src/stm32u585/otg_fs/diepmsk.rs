///Register `DIEPMSK` reader
pub type R = crate::R<DIEPMSKrs>;
///Register `DIEPMSK` writer
pub type W = crate::W<DIEPMSKrs>;
///Field `XFRCM` reader - XFRCM
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - XFRCM
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDM` reader - EPDM
pub type EPDM_R = crate::BitReader;
///Field `EPDM` writer - EPDM
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOM` reader - TOM
pub type TOM_R = crate::BitReader;
///Field `TOM` writer - TOM
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFEMSK` reader - ITTXFEMSK
pub type ITTXFEMSK_R = crate::BitReader;
///Field `ITTXFEMSK` writer - ITTXFEMSK
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNMM` reader - INEPNMM
pub type INEPNMM_R = crate::BitReader;
///Field `INEPNMM` writer - INEPNMM
pub type INEPNMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNEM` reader - INEPNEM
pub type INEPNEM_R = crate::BitReader;
///Field `INEPNEM` writer - INEPNEM
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKM` reader - NAKM
pub type NAKM_R = crate::BitReader;
///Field `NAKM` writer - NAKM
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDM
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - TOM
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ITTXFEMSK
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - INEPNMM
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - INEPNEM
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - NAKM
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("tom", &self.tom())
            .field("ittxfemsk", &self.ittxfemsk())
            .field("inepnmm", &self.inepnmm())
            .field("inepnem", &self.inepnem())
            .field("nakm", &self.nakm())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<DIEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - EPDM
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<DIEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    ///Bit 3 - TOM
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W<DIEPMSKrs> {
        TOM_W::new(self, 3)
    }
    ///Bit 4 - ITTXFEMSK
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<DIEPMSKrs> {
        ITTXFEMSK_W::new(self, 4)
    }
    ///Bit 5 - INEPNMM
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W<DIEPMSKrs> {
        INEPNMM_W::new(self, 5)
    }
    ///Bit 6 - INEPNEM
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W<DIEPMSKrs> {
        INEPNEM_W::new(self, 6)
    }
    ///Bit 13 - NAKM
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<DIEPMSKrs> {
        NAKM_W::new(self, 13)
    }
}
/**This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.

You can [`read`](crate::Reg::read) this register and get [`diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPMSK)*/
pub struct DIEPMSKrs;
impl crate::RegisterSpec for DIEPMSKrs {
    type Ux = u32;
}
///`read()` method returns [`diepmsk::R`](R) reader structure
impl crate::Readable for DIEPMSKrs {}
///`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure
impl crate::Writable for DIEPMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPMSK to value 0
impl crate::Resettable for DIEPMSKrs {}
