///Register `HS_DOEPEACHMSK1` reader
pub type R = crate::R<HS_DOEPEACHMSK1rs>;
///Register `HS_DOEPEACHMSK1` writer
pub type W = crate::W<HS_DOEPEACHMSK1rs>;
///Field `XFRCM` reader - XFRCM
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - XFRCM
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDM` reader - EPDM
pub type EPDM_R = crate::BitReader;
///Field `EPDM` writer - EPDM
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERRM` reader - AHBERRM
pub type AHBERRM_R = crate::BitReader;
///Field `AHBERRM` writer - AHBERRM
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUPM` reader - STUPM
pub type STUPM_R = crate::BitReader;
///Field `STUPM` writer - STUPM
pub type STUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDM` reader - OTEPDM
pub type OTEPDM_R = crate::BitReader;
///Field `OTEPDM` writer - OTEPDM
pub type OTEPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2BSTUPM` reader - B2BSTUPM
pub type B2BSTUPM_R = crate::BitReader;
///Field `B2BSTUPM` writer - B2BSTUPM
pub type B2BSTUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTPKTERRM` reader - OUTPKTERRM
pub type OUTPKTERRM_R = crate::BitReader;
///Field `OUTPKTERRM` writer - OUTPKTERRM
pub type OUTPKTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNAM` reader - BNAM
pub type BNAM_R = crate::BitReader;
///Field `BNAM` writer - BNAM
pub type BNAM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERRM` reader - BERRM
pub type BERRM_R = crate::BitReader;
///Field `BERRM` writer - BERRM
pub type BERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKMSK` reader - NAKMSK
pub type NAKMSK_R = crate::BitReader;
///Field `NAKMSK` writer - NAKMSK
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYETMSK` reader - NYETMSK
pub type NYETMSK_R = crate::BitReader;
///Field `NYETMSK` writer - NYETMSK
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - AHBERRM
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STUPM
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDM
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - B2BSTUPM
    #[inline(always)]
    pub fn b2bstupm(&self) -> B2BSTUPM_R {
        B2BSTUPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - OUTPKTERRM
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNAM
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - BERRM
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAKMSK
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NYETMSK
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HS_DOEPEACHMSK1")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("ahberrm", &self.ahberrm())
            .field("stupm", &self.stupm())
            .field("otepdm", &self.otepdm())
            .field("b2bstupm", &self.b2bstupm())
            .field("outpkterrm", &self.outpkterrm())
            .field("bnam", &self.bnam())
            .field("berrm", &self.berrm())
            .field("nakmsk", &self.nakmsk())
            .field("nyetmsk", &self.nyetmsk())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<HS_DOEPEACHMSK1rs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - EPDM
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<HS_DOEPEACHMSK1rs> {
        EPDM_W::new(self, 1)
    }
    ///Bit 2 - AHBERRM
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W<HS_DOEPEACHMSK1rs> {
        AHBERRM_W::new(self, 2)
    }
    ///Bit 3 - STUPM
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W<HS_DOEPEACHMSK1rs> {
        STUPM_W::new(self, 3)
    }
    ///Bit 4 - OTEPDM
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W<HS_DOEPEACHMSK1rs> {
        OTEPDM_W::new(self, 4)
    }
    ///Bit 6 - B2BSTUPM
    #[inline(always)]
    pub fn b2bstupm(&mut self) -> B2BSTUPM_W<HS_DOEPEACHMSK1rs> {
        B2BSTUPM_W::new(self, 6)
    }
    ///Bit 8 - OUTPKTERRM
    #[inline(always)]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W<HS_DOEPEACHMSK1rs> {
        OUTPKTERRM_W::new(self, 8)
    }
    ///Bit 9 - BNAM
    #[inline(always)]
    pub fn bnam(&mut self) -> BNAM_W<HS_DOEPEACHMSK1rs> {
        BNAM_W::new(self, 9)
    }
    ///Bit 12 - BERRM
    #[inline(always)]
    pub fn berrm(&mut self) -> BERRM_W<HS_DOEPEACHMSK1rs> {
        BERRM_W::new(self, 12)
    }
    ///Bit 13 - NAKMSK
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W<HS_DOEPEACHMSK1rs> {
        NAKMSK_W::new(self, 13)
    }
    ///Bit 14 - NYETMSK
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<HS_DOEPEACHMSK1rs> {
        NYETMSK_W::new(self, 14)
    }
}
/**OTG device each OUT endpoint-1 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`hs_doepeachmsk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hs_doepeachmsk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:HS_DOEPEACHMSK1)*/
pub struct HS_DOEPEACHMSK1rs;
impl crate::RegisterSpec for HS_DOEPEACHMSK1rs {
    type Ux = u32;
}
///`read()` method returns [`hs_doepeachmsk1::R`](R) reader structure
impl crate::Readable for HS_DOEPEACHMSK1rs {}
///`write(|w| ..)` method takes [`hs_doepeachmsk1::W`](W) writer structure
impl crate::Writable for HS_DOEPEACHMSK1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HS_DOEPEACHMSK1 to value 0
impl crate::Resettable for HS_DOEPEACHMSK1rs {}
