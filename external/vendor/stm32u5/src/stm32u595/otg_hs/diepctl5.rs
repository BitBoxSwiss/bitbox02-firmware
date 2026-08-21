///Register `DIEPCTL5` reader
pub type R = crate::R<DIEPCTL5rs>;
///Register `DIEPCTL5` writer
pub type W = crate::W<DIEPCTL5rs>;
///Field `MPSIZ` reader - MPSIZ
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - MPSIZ
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USBAEP
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USBAEP
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EONUM_DPIP` reader - EONUM_DPIP
pub type EONUM_DPIP_R = crate::BitReader;
///Field `NAKSTS` reader - NAKSTS
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - EPTYP
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - EPTYP
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STALL` reader - STALL
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - TXFNUM
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - TXFNUM
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CNAK` writer - CNAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - SNAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD0PID_SEVNFRM` writer - SD0PID_SEVNFRM
pub type SD0PID_SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SODDFRM` writer - SODDFRM
pub type SODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - EPDIS
pub type EPDIS_R = crate::BitReader;
///Field `EPDIS` writer - EPDIS
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPENA` reader - EPENA
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - EPENA
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - EONUM_DPIP
    #[inline(always)]
    pub fn eonum_dpip(&self) -> EONUM_DPIP_R {
        EONUM_DPIP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAKSTS
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - STALL
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:25 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL5")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("eonum_dpip", &self.eonum_dpip())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<DIEPCTL5rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<DIEPCTL5rs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<DIEPCTL5rs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 21 - STALL
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<DIEPCTL5rs> {
        STALL_W::new(self, 21)
    }
    ///Bits 22:25 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<DIEPCTL5rs> {
        TXFNUM_W::new(self, 22)
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<DIEPCTL5rs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<DIEPCTL5rs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - SD0PID_SEVNFRM
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<DIEPCTL5rs> {
        SD0PID_SEVNFRM_W::new(self, 28)
    }
    ///Bit 29 - SODDFRM
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W<DIEPCTL5rs> {
        SODDFRM_W::new(self, 29)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<DIEPCTL5rs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<DIEPCTL5rs> {
        EPENA_W::new(self, 31)
    }
}
/**The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:DIEPCTL5)*/
pub struct DIEPCTL5rs;
impl crate::RegisterSpec for DIEPCTL5rs {
    type Ux = u32;
}
///`read()` method returns [`diepctl5::R`](R) reader structure
impl crate::Readable for DIEPCTL5rs {}
///`write(|w| ..)` method takes [`diepctl5::W`](W) writer structure
impl crate::Writable for DIEPCTL5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPCTL5 to value 0
impl crate::Resettable for DIEPCTL5rs {}
