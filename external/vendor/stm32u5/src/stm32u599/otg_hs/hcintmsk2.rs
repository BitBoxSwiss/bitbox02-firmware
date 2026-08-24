///Register `HCINTMSK2` reader
pub type R = crate::R<HCINTMSK2rs>;
///Register `HCINTMSK2` writer
pub type W = crate::W<HCINTMSK2rs>;
///Field `XFRCM` reader - XFRCM
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - XFRCM
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHHM` reader - CHHM
pub type CHHM_R = crate::BitReader;
///Field `CHHM` writer - CHHM
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALLM` reader - STALLM
pub type STALLM_R = crate::BitReader;
///Field `STALLM` writer - STALLM
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKM` reader - NAKM
pub type NAKM_R = crate::BitReader;
///Field `NAKM` writer - NAKM
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKM` reader - ACKM
pub type ACKM_R = crate::BitReader;
///Field `ACKM` writer - ACKM
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERRM` reader - TXERRM
pub type TXERRM_R = crate::BitReader;
///Field `TXERRM` writer - TXERRM
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BBERRM` reader - BBERRM
pub type BBERRM_R = crate::BitReader;
///Field `BBERRM` writer - BBERRM
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRMORM` reader - FRMORM
pub type FRMORM_R = crate::BitReader;
///Field `FRMORM` writer - FRMORM
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTERRM` reader - DTERRM
pub type DTERRM_R = crate::BitReader;
///Field `DTERRM` writer - DTERRM
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CHHM
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - STALLM
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAKM
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACKM
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - TXERRM
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BBERRM
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FRMORM
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DTERRM
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK2")
            .field("xfrcm", &self.xfrcm())
            .field("chhm", &self.chhm())
            .field("stallm", &self.stallm())
            .field("nakm", &self.nakm())
            .field("ackm", &self.ackm())
            .field("txerrm", &self.txerrm())
            .field("bberrm", &self.bberrm())
            .field("frmorm", &self.frmorm())
            .field("dterrm", &self.dterrm())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<HCINTMSK2rs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - CHHM
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W<HCINTMSK2rs> {
        CHHM_W::new(self, 1)
    }
    ///Bit 3 - STALLM
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W<HCINTMSK2rs> {
        STALLM_W::new(self, 3)
    }
    ///Bit 4 - NAKM
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<HCINTMSK2rs> {
        NAKM_W::new(self, 4)
    }
    ///Bit 5 - ACKM
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W<HCINTMSK2rs> {
        ACKM_W::new(self, 5)
    }
    ///Bit 7 - TXERRM
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W<HCINTMSK2rs> {
        TXERRM_W::new(self, 7)
    }
    ///Bit 8 - BBERRM
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W<HCINTMSK2rs> {
        BBERRM_W::new(self, 8)
    }
    ///Bit 9 - FRMORM
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W<HCINTMSK2rs> {
        FRMORM_W::new(self, 9)
    }
    ///Bit 10 - DTERRM
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W<HCINTMSK2rs> {
        DTERRM_W::new(self, 10)
    }
}
/**This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:HCINTMSK2)*/
pub struct HCINTMSK2rs;
impl crate::RegisterSpec for HCINTMSK2rs {
    type Ux = u32;
}
///`read()` method returns [`hcintmsk2::R`](R) reader structure
impl crate::Readable for HCINTMSK2rs {}
///`write(|w| ..)` method takes [`hcintmsk2::W`](W) writer structure
impl crate::Writable for HCINTMSK2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCINTMSK2 to value 0
impl crate::Resettable for HCINTMSK2rs {}
