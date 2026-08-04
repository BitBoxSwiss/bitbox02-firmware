///Register `MPCWM1BCFGR` reader
pub type R = crate::R<MPCWM1BCFGRrs>;
///Register `MPCWM1BCFGR` writer
pub type W = crate::W<MPCWM1BCFGRrs>;
///Field `SREN` reader - Sub-region enable
pub type SREN_R = crate::BitReader;
///Field `SREN` writer - Sub-region enable
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRLOCK` reader - Sub-region lock
pub type SRLOCK_R = crate::BitReader;
///Field `SRLOCK` writer - Sub-region lock
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC` reader - Secure sub-region
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - Secure sub-region
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - Privileged sub-region
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - Privileged sub-region
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sub-region enable
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sub-region lock
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Secure sub-region
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privileged sub-region
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM1BCFGR")
            .field("sren", &self.sren())
            .field("srlock", &self.srlock())
            .field("sec", &self.sec())
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sub-region enable
    #[inline(always)]
    pub fn sren(&mut self) -> SREN_W<MPCWM1BCFGRrs> {
        SREN_W::new(self, 0)
    }
    ///Bit 1 - Sub-region lock
    #[inline(always)]
    pub fn srlock(&mut self) -> SRLOCK_W<MPCWM1BCFGRrs> {
        SRLOCK_W::new(self, 1)
    }
    ///Bit 8 - Secure sub-region
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<MPCWM1BCFGRrs> {
        SEC_W::new(self, 8)
    }
    ///Bit 9 - Privileged sub-region
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<MPCWM1BCFGRrs> {
        PRIV_W::new(self, 9)
    }
}
/**TZSC memory 1 sub-region B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1bcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1bcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_TZSC:MPCWM1BCFGR)*/
pub struct MPCWM1BCFGRrs;
impl crate::RegisterSpec for MPCWM1BCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm1bcfgr::R`](R) reader structure
impl crate::Readable for MPCWM1BCFGRrs {}
///`write(|w| ..)` method takes [`mpcwm1bcfgr::W`](W) writer structure
impl crate::Writable for MPCWM1BCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM1BCFGR to value 0
impl crate::Resettable for MPCWM1BCFGRrs {}
