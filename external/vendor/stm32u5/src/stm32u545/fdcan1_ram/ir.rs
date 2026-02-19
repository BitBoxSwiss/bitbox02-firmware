///Register `IR` reader
pub type R = crate::R<IRrs>;
///Register `IR` writer
pub type W = crate::W<IRrs>;
///Field `RF0N` reader - RF0N
pub type RF0N_R = crate::BitReader;
///Field `RF0N` writer - RF0N
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0F` reader - RF0F
pub type RF0F_R = crate::BitReader;
///Field `RF0F` writer - RF0F
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0L` reader - RF0L
pub type RF0L_R = crate::BitReader;
///Field `RF0L` writer - RF0L
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1N` reader - RF1N
pub type RF1N_R = crate::BitReader;
///Field `RF1N` writer - RF1N
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1F` reader - RF1F
pub type RF1F_R = crate::BitReader;
///Field `RF1F` writer - RF1F
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1L` reader - RF1L
pub type RF1L_R = crate::BitReader;
///Field `RF1L` writer - RF1L
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPM` reader - HPM
pub type HPM_R = crate::BitReader;
///Field `HPM` writer - HPM
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC` reader - TC
pub type TC_R = crate::BitReader;
///Field `TC` writer - TC
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCF` reader - TCF
pub type TCF_R = crate::BitReader;
///Field `TCF` writer - TCF
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFE` reader - TFE
pub type TFE_R = crate::BitReader;
///Field `TFE` writer - TFE
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFN` reader - TEFN
pub type TEFN_R = crate::BitReader;
///Field `TEFN` writer - TEFN
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFF` reader - TEFF
pub type TEFF_R = crate::BitReader;
///Field `TEFF` writer - TEFF
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFL` reader - TEFL
pub type TEFL_R = crate::BitReader;
///Field `TEFL` writer - TEFL
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSW` reader - TSW
pub type TSW_R = crate::BitReader;
///Field `TSW` writer - TSW
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAF` reader - MRAF
pub type MRAF_R = crate::BitReader;
///Field `MRAF` writer - MRAF
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOO` reader - TOO
pub type TOO_R = crate::BitReader;
///Field `TOO` writer - TOO
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELO` reader - ELO
pub type ELO_R = crate::BitReader;
///Field `ELO` writer - ELO
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP` reader - EP
pub type EP_R = crate::BitReader;
///Field `EP` writer - EP
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EW` reader - EW
pub type EW_R = crate::BitReader;
///Field `EW` writer - EW
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BO` reader - BO
pub type BO_R = crate::BitReader;
///Field `BO` writer - BO
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDI` reader - WDI
pub type WDI_R = crate::BitReader;
///Field `WDI` writer - WDI
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEA` reader - PEA
pub type PEA_R = crate::BitReader;
///Field `PEA` writer - PEA
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PED` reader - PED
pub type PED_R = crate::BitReader;
///Field `PED` writer - PED
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARA` reader - ARA
pub type ARA_R = crate::BitReader;
///Field `ARA` writer - ARA
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RF0N
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RF0F
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RF0L
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RF1N
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RF1F
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RF1L
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HPM
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TCF
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TFE
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TEFN
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TEFF
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TEFL
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TSW
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MRAF
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TOO
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELO
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - EP
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - EW
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - BO
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WDI
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - PEA
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - PED
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ARA
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("rf0n", &self.rf0n())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
impl W {
    ///Bit 0 - RF0N
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W<IRrs> {
        RF0N_W::new(self, 0)
    }
    ///Bit 1 - RF0F
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W<IRrs> {
        RF0F_W::new(self, 1)
    }
    ///Bit 2 - RF0L
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W<IRrs> {
        RF0L_W::new(self, 2)
    }
    ///Bit 3 - RF1N
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W<IRrs> {
        RF1N_W::new(self, 3)
    }
    ///Bit 4 - RF1F
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W<IRrs> {
        RF1F_W::new(self, 4)
    }
    ///Bit 5 - RF1L
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<IRrs> {
        RF1L_W::new(self, 5)
    }
    ///Bit 6 - HPM
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W<IRrs> {
        HPM_W::new(self, 6)
    }
    ///Bit 7 - TC
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<IRrs> {
        TC_W::new(self, 7)
    }
    ///Bit 8 - TCF
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<IRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - TFE
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<IRrs> {
        TFE_W::new(self, 9)
    }
    ///Bit 10 - TEFN
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W<IRrs> {
        TEFN_W::new(self, 10)
    }
    ///Bit 11 - TEFF
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W<IRrs> {
        TEFF_W::new(self, 11)
    }
    ///Bit 12 - TEFL
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<IRrs> {
        TEFL_W::new(self, 12)
    }
    ///Bit 13 - TSW
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W<IRrs> {
        TSW_W::new(self, 13)
    }
    ///Bit 14 - MRAF
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W<IRrs> {
        MRAF_W::new(self, 14)
    }
    ///Bit 15 - TOO
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W<IRrs> {
        TOO_W::new(self, 15)
    }
    ///Bit 16 - ELO
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W<IRrs> {
        ELO_W::new(self, 16)
    }
    ///Bit 17 - EP
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<IRrs> {
        EP_W::new(self, 17)
    }
    ///Bit 18 - EW
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<IRrs> {
        EW_W::new(self, 18)
    }
    ///Bit 19 - BO
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<IRrs> {
        BO_W::new(self, 19)
    }
    ///Bit 20 - WDI
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W<IRrs> {
        WDI_W::new(self, 20)
    }
    ///Bit 21 - PEA
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W<IRrs> {
        PEA_W::new(self, 21)
    }
    ///Bit 22 - PED
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W<IRrs> {
        PED_W::new(self, 22)
    }
    ///Bit 23 - ARA
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W<IRrs> {
        ARA_W::new(self, 23)
    }
}
/**FDCAN Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FDCAN1_RAM:IR)*/
pub struct IRrs;
impl crate::RegisterSpec for IRrs {
    type Ux = u32;
}
///`read()` method returns [`ir::R`](R) reader structure
impl crate::Readable for IRrs {}
///`write(|w| ..)` method takes [`ir::W`](W) writer structure
impl crate::Writable for IRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IRrs {}
