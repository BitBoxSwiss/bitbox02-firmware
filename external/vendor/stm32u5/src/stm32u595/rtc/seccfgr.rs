///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `ALRASEC` reader - ALRASEC
pub type ALRASEC_R = crate::BitReader;
///Field `ALRASEC` writer - ALRASEC
pub type ALRASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBSEC` reader - ALRBSEC
pub type ALRBSEC_R = crate::BitReader;
///Field `ALRBSEC` writer - ALRBSEC
pub type ALRBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTSEC` reader - WUTSEC
pub type WUTSEC_R = crate::BitReader;
///Field `WUTSEC` writer - WUTSEC
pub type WUTSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSEC` reader - TSSEC
pub type TSSEC_R = crate::BitReader;
///Field `TSSEC` writer - TSSEC
pub type TSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEC` reader - CALSEC
pub type CALSEC_R = crate::BitReader;
///Field `CALSEC` writer - CALSEC
pub type CALSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITSEC` reader - INITSEC
pub type INITSEC_R = crate::BitReader;
///Field `INITSEC` writer - INITSEC
pub type INITSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC` reader - SEC
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - SEC
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ALRASEC
    #[inline(always)]
    pub fn alrasec(&self) -> ALRASEC_R {
        ALRASEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ALRBSEC
    #[inline(always)]
    pub fn alrbsec(&self) -> ALRBSEC_R {
        ALRBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUTSEC
    #[inline(always)]
    pub fn wutsec(&self) -> WUTSEC_R {
        WUTSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSSEC
    #[inline(always)]
    pub fn tssec(&self) -> TSSEC_R {
        TSSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - CALSEC
    #[inline(always)]
    pub fn calsec(&self) -> CALSEC_R {
        CALSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - INITSEC
    #[inline(always)]
    pub fn initsec(&self) -> INITSEC_R {
        INITSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SEC
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sec", &self.sec())
            .field("initsec", &self.initsec())
            .field("calsec", &self.calsec())
            .field("tssec", &self.tssec())
            .field("wutsec", &self.wutsec())
            .field("alrbsec", &self.alrbsec())
            .field("alrasec", &self.alrasec())
            .finish()
    }
}
impl W {
    ///Bit 0 - ALRASEC
    #[inline(always)]
    pub fn alrasec(&mut self) -> ALRASEC_W<SECCFGRrs> {
        ALRASEC_W::new(self, 0)
    }
    ///Bit 1 - ALRBSEC
    #[inline(always)]
    pub fn alrbsec(&mut self) -> ALRBSEC_W<SECCFGRrs> {
        ALRBSEC_W::new(self, 1)
    }
    ///Bit 2 - WUTSEC
    #[inline(always)]
    pub fn wutsec(&mut self) -> WUTSEC_W<SECCFGRrs> {
        WUTSEC_W::new(self, 2)
    }
    ///Bit 3 - TSSEC
    #[inline(always)]
    pub fn tssec(&mut self) -> TSSEC_W<SECCFGRrs> {
        TSSEC_W::new(self, 3)
    }
    ///Bit 13 - CALSEC
    #[inline(always)]
    pub fn calsec(&mut self) -> CALSEC_W<SECCFGRrs> {
        CALSEC_W::new(self, 13)
    }
    ///Bit 14 - INITSEC
    #[inline(always)]
    pub fn initsec(&mut self) -> INITSEC_W<SECCFGRrs> {
        INITSEC_W::new(self, 14)
    }
    ///Bit 15 - SEC
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 15)
    }
}
/**RTC secure mode control register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RTC:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
