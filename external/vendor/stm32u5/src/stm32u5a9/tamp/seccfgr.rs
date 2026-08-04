///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `BKPRWSEC` reader - BKPRWSEC
pub type BKPRWSEC_R = crate::FieldReader;
///Field `BKPRWSEC` writer - BKPRWSEC
pub type BKPRWSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CNT1SEC` reader - CNT1SEC
pub type CNT1SEC_R = crate::BitReader;
///Field `CNT1SEC` writer - CNT1SEC
pub type CNT1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPWSEC` reader - BKPWSEC
pub type BKPWSEC_R = crate::FieldReader;
///Field `BKPWSEC` writer - BKPWSEC
pub type BKPWSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BHKLOCK` reader - BHKLOCK
pub type BHKLOCK_R = crate::BitReader;
///Field `BHKLOCK` writer - BHKLOCK
pub type BHKLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPSEC` reader - TAMPSEC
pub type TAMPSEC_R = crate::BitReader;
///Field `TAMPSEC` writer - TAMPSEC
pub type TAMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - BKPRWSEC
    #[inline(always)]
    pub fn bkprwsec(&self) -> BKPRWSEC_R {
        BKPRWSEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 15 - CNT1SEC
    #[inline(always)]
    pub fn cnt1sec(&self) -> CNT1SEC_R {
        CNT1SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - BKPWSEC
    #[inline(always)]
    pub fn bkpwsec(&self) -> BKPWSEC_R {
        BKPWSEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 30 - BHKLOCK
    #[inline(always)]
    pub fn bhklock(&self) -> BHKLOCK_R {
        BHKLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - TAMPSEC
    #[inline(always)]
    pub fn tampsec(&self) -> TAMPSEC_R {
        TAMPSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("bkprwsec", &self.bkprwsec())
            .field("cnt1sec", &self.cnt1sec())
            .field("bkpwsec", &self.bkpwsec())
            .field("bhklock", &self.bhklock())
            .field("tampsec", &self.tampsec())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - BKPRWSEC
    #[inline(always)]
    pub fn bkprwsec(&mut self) -> BKPRWSEC_W<SECCFGRrs> {
        BKPRWSEC_W::new(self, 0)
    }
    ///Bit 15 - CNT1SEC
    #[inline(always)]
    pub fn cnt1sec(&mut self) -> CNT1SEC_W<SECCFGRrs> {
        CNT1SEC_W::new(self, 15)
    }
    ///Bits 16:23 - BKPWSEC
    #[inline(always)]
    pub fn bkpwsec(&mut self) -> BKPWSEC_W<SECCFGRrs> {
        BKPWSEC_W::new(self, 16)
    }
    ///Bit 30 - BHKLOCK
    #[inline(always)]
    pub fn bhklock(&mut self) -> BHKLOCK_W<SECCFGRrs> {
        BHKLOCK_W::new(self, 30)
    }
    ///Bit 31 - TAMPSEC
    #[inline(always)]
    pub fn tampsec(&mut self) -> TAMPSEC_W<SECCFGRrs> {
        TAMPSEC_W::new(self, 31)
    }
}
/**TAMP secure mode register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TAMP:SECCFGR)*/
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
