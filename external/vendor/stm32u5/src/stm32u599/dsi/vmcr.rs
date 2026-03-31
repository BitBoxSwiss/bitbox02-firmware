///Register `VMCR` reader
pub type R = crate::R<VMCRrs>;
///Register `VMCR` writer
pub type W = crate::W<VMCRrs>;
///Field `VMT` reader - Video mode type This field configures the video mode transmission type : 1x: Burst mode
pub type VMT_R = crate::FieldReader;
///Field `VMT` writer - Video mode type This field configures the video mode transmission type : 1x: Burst mode
pub type VMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPVSAE` reader - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
pub type LPVSAE_R = crate::BitReader;
///Field `LPVSAE` writer - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
pub type LPVSAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVBPE` reader - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
pub type LPVBPE_R = crate::BitReader;
///Field `LPVBPE` writer - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
pub type LPVBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVFPE` reader - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
pub type LPVFPE_R = crate::BitReader;
///Field `LPVFPE` writer - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
pub type LPVFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVAE` reader - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
pub type LPVAE_R = crate::BitReader;
///Field `LPVAE` writer - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
pub type LPVAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPHBPE` reader - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
pub type LPHBPE_R = crate::BitReader;
///Field `LPHBPE` writer - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
pub type LPHBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPHFPE` reader - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
pub type LPHFPE_R = crate::BitReader;
///Field `LPHFPE` writer - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
pub type LPHFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBTAAE` reader - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
pub type FBTAAE_R = crate::BitReader;
///Field `FBTAAE` writer - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
pub type FBTAAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPCE` reader - Low-power command enable This bit enables the command transmission only in low-power mode.
pub type LPCE_R = crate::BitReader;
///Field `LPCE` writer - Low-power command enable This bit enables the command transmission only in low-power mode.
pub type LPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGE` reader - Pattern generator enable This bit enables the video mode pattern generator.
pub type PGE_R = crate::BitReader;
///Field `PGE` writer - Pattern generator enable This bit enables the video mode pattern generator.
pub type PGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGM` reader - Pattern generator mode This bit configures the pattern generator mode.
pub type PGM_R = crate::BitReader;
///Field `PGM` writer - Pattern generator mode This bit configures the pattern generator mode.
pub type PGM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGO` reader - Pattern generator orientation This bit configures the color bar orientation.
pub type PGO_R = crate::BitReader;
///Field `PGO` writer - Pattern generator orientation This bit configures the color bar orientation.
pub type PGO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Video mode type This field configures the video mode transmission type : 1x: Burst mode
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Low-power command enable This bit enables the command transmission only in low-power mode.
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pattern generator enable This bit enables the video mode pattern generator.
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Pattern generator mode This bit configures the pattern generator mode.
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Pattern generator orientation This bit configures the color bar orientation.
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCR")
            .field("vmt", &self.vmt())
            .field("lpvsae", &self.lpvsae())
            .field("lpvbpe", &self.lpvbpe())
            .field("lpvfpe", &self.lpvfpe())
            .field("lpvae", &self.lpvae())
            .field("lphbpe", &self.lphbpe())
            .field("lphfpe", &self.lphfpe())
            .field("fbtaae", &self.fbtaae())
            .field("lpce", &self.lpce())
            .field("pge", &self.pge())
            .field("pgm", &self.pgm())
            .field("pgo", &self.pgo())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Video mode type This field configures the video mode transmission type : 1x: Burst mode
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W<VMCRrs> {
        VMT_W::new(self, 0)
    }
    ///Bit 8 - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W<VMCRrs> {
        LPVSAE_W::new(self, 8)
    }
    ///Bit 9 - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<VMCRrs> {
        LPVBPE_W::new(self, 9)
    }
    ///Bit 10 - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<VMCRrs> {
        LPVFPE_W::new(self, 10)
    }
    ///Bit 11 - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
    #[inline(always)]
    pub fn lpvae(&mut self) -> LPVAE_W<VMCRrs> {
        LPVAE_W::new(self, 11)
    }
    ///Bit 12 - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W<VMCRrs> {
        LPHBPE_W::new(self, 12)
    }
    ///Bit 13 - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
    #[inline(always)]
    pub fn lphfpe(&mut self) -> LPHFPE_W<VMCRrs> {
        LPHFPE_W::new(self, 13)
    }
    ///Bit 14 - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W<VMCRrs> {
        FBTAAE_W::new(self, 14)
    }
    ///Bit 15 - Low-power command enable This bit enables the command transmission only in low-power mode.
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W<VMCRrs> {
        LPCE_W::new(self, 15)
    }
    ///Bit 16 - Pattern generator enable This bit enables the video mode pattern generator.
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W<VMCRrs> {
        PGE_W::new(self, 16)
    }
    ///Bit 20 - Pattern generator mode This bit configures the pattern generator mode.
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W<VMCRrs> {
        PGM_W::new(self, 20)
    }
    ///Bit 24 - Pattern generator orientation This bit configures the color bar orientation.
    #[inline(always)]
    pub fn pgo(&mut self) -> PGO_W<VMCRrs> {
        PGO_W::new(self, 24)
    }
}
/**DSI Host video mode configuration register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VMCR)*/
pub struct VMCRrs;
impl crate::RegisterSpec for VMCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmcr::R`](R) reader structure
impl crate::Readable for VMCRrs {}
///`write(|w| ..)` method takes [`vmcr::W`](W) writer structure
impl crate::Writable for VMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VMCR to value 0
impl crate::Resettable for VMCRrs {}
