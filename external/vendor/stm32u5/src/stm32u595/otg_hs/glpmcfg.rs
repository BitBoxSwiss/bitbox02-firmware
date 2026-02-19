///Register `GLPMCFG` reader
pub type R = crate::R<GLPMCFGrs>;
///Register `GLPMCFG` writer
pub type W = crate::W<GLPMCFGrs>;
///Field `LPMEN` reader - LPMEN
pub type LPMEN_R = crate::BitReader;
///Field `LPMEN` writer - LPMEN
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMACK` reader - LPMACK
pub type LPMACK_R = crate::BitReader;
///Field `LPMACK` writer - LPMACK
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BESL` reader - BESL
pub type BESL_R = crate::FieldReader;
///Field `BESL` writer - BESL
pub type BESL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REMWAKE` reader - REMWAKE
pub type REMWAKE_R = crate::BitReader;
///Field `REMWAKE` writer - REMWAKE
pub type REMWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1SSEN` reader - L1SSEN
pub type L1SSEN_R = crate::BitReader;
///Field `L1SSEN` writer - L1SSEN
pub type L1SSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BESLTHRS` reader - BESLTHRS
pub type BESLTHRS_R = crate::FieldReader;
///Field `BESLTHRS` writer - BESLTHRS
pub type BESLTHRS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `L1DSEN` reader - L1DSEN
pub type L1DSEN_R = crate::BitReader;
///Field `L1DSEN` writer - L1DSEN
pub type L1DSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMRSP` reader - LPMRSP
pub type LPMRSP_R = crate::FieldReader;
///Field `SLPSTS` reader - SLPSTS
pub type SLPSTS_R = crate::BitReader;
///Field `L1RSMOK` reader - L1RSMOK
pub type L1RSMOK_R = crate::BitReader;
///Field `LPMCHIDX` reader - LPMCHIDX
pub type LPMCHIDX_R = crate::FieldReader;
///Field `LPMCHIDX` writer - LPMCHIDX
pub type LPMCHIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPMRCNT` reader - LPMRCNT
pub type LPMRCNT_R = crate::FieldReader;
///Field `LPMRCNT` writer - LPMRCNT
pub type LPMRCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SNDLPM` reader - SNDLPM
pub type SNDLPM_R = crate::BitReader;
///Field `SNDLPM` writer - SNDLPM
pub type SNDLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMRCNTSTS` reader - LPMRCNTSTS
pub type LPMRCNTSTS_R = crate::FieldReader;
///Field `ENBESL` reader - ENBESL
pub type ENBESL_R = crate::BitReader;
///Field `ENBESL` writer - ENBESL
pub type ENBESL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPMEN
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPMACK
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - BESL
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - REMWAKE
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - L1SSEN
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - BESLTHRS
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - L1DSEN
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - LPMRSP
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - SLPSTS
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - L1RSMOK
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20 - LPMCHIDX
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:23 - LPMRCNT
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - SNDLPM
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - LPMRCNTSTS
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - ENBESL
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLPMCFG")
            .field("lpmen", &self.lpmen())
            .field("lpmack", &self.lpmack())
            .field("besl", &self.besl())
            .field("remwake", &self.remwake())
            .field("l1ssen", &self.l1ssen())
            .field("beslthrs", &self.beslthrs())
            .field("l1dsen", &self.l1dsen())
            .field("lpmrsp", &self.lpmrsp())
            .field("slpsts", &self.slpsts())
            .field("l1rsmok", &self.l1rsmok())
            .field("lpmchidx", &self.lpmchidx())
            .field("lpmrcnt", &self.lpmrcnt())
            .field("sndlpm", &self.sndlpm())
            .field("lpmrcntsts", &self.lpmrcntsts())
            .field("enbesl", &self.enbesl())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPMEN
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W<GLPMCFGrs> {
        LPMEN_W::new(self, 0)
    }
    ///Bit 1 - LPMACK
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W<GLPMCFGrs> {
        LPMACK_W::new(self, 1)
    }
    ///Bits 2:5 - BESL
    #[inline(always)]
    pub fn besl(&mut self) -> BESL_W<GLPMCFGrs> {
        BESL_W::new(self, 2)
    }
    ///Bit 6 - REMWAKE
    #[inline(always)]
    pub fn remwake(&mut self) -> REMWAKE_W<GLPMCFGrs> {
        REMWAKE_W::new(self, 6)
    }
    ///Bit 7 - L1SSEN
    #[inline(always)]
    pub fn l1ssen(&mut self) -> L1SSEN_W<GLPMCFGrs> {
        L1SSEN_W::new(self, 7)
    }
    ///Bits 8:11 - BESLTHRS
    #[inline(always)]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<GLPMCFGrs> {
        BESLTHRS_W::new(self, 8)
    }
    ///Bit 12 - L1DSEN
    #[inline(always)]
    pub fn l1dsen(&mut self) -> L1DSEN_W<GLPMCFGrs> {
        L1DSEN_W::new(self, 12)
    }
    ///Bits 17:20 - LPMCHIDX
    #[inline(always)]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<GLPMCFGrs> {
        LPMCHIDX_W::new(self, 17)
    }
    ///Bits 21:23 - LPMRCNT
    #[inline(always)]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W<GLPMCFGrs> {
        LPMRCNT_W::new(self, 21)
    }
    ///Bit 24 - SNDLPM
    #[inline(always)]
    pub fn sndlpm(&mut self) -> SNDLPM_W<GLPMCFGrs> {
        SNDLPM_W::new(self, 24)
    }
    ///Bit 28 - ENBESL
    #[inline(always)]
    pub fn enbesl(&mut self) -> ENBESL_W<GLPMCFGrs> {
        ENBESL_W::new(self, 28)
    }
}
/**OTG core LPM configuration register

You can [`read`](crate::Reg::read) this register and get [`glpmcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glpmcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:GLPMCFG)*/
pub struct GLPMCFGrs;
impl crate::RegisterSpec for GLPMCFGrs {
    type Ux = u32;
}
///`read()` method returns [`glpmcfg::R`](R) reader structure
impl crate::Readable for GLPMCFGrs {}
///`write(|w| ..)` method takes [`glpmcfg::W`](W) writer structure
impl crate::Writable for GLPMCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GLPMCFG to value 0
impl crate::Resettable for GLPMCFGrs {}
