///Register `PRIVCR` reader
pub type R = crate::R<PRIVCRrs>;
///Register `PRIVCR` writer
pub type W = crate::W<PRIVCRrs>;
///Field `ALRAPRIV` reader - ALRAPRIV
pub type ALRAPRIV_R = crate::BitReader;
///Field `ALRAPRIV` writer - ALRAPRIV
pub type ALRAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBPRIV` reader - ALRBPRIV
pub type ALRBPRIV_R = crate::BitReader;
///Field `ALRBPRIV` writer - ALRBPRIV
pub type ALRBPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTPRIV` reader - WUTPRIV
pub type WUTPRIV_R = crate::BitReader;
///Field `WUTPRIV` writer - WUTPRIV
pub type WUTPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSPRIV` reader - TSPRIV
pub type TSPRIV_R = crate::BitReader;
///Field `TSPRIV` writer - TSPRIV
pub type TSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALPRIV` reader - CALPRIV
pub type CALPRIV_R = crate::BitReader;
///Field `CALPRIV` writer - CALPRIV
pub type CALPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITPRIV` reader - INITPRIV
pub type INITPRIV_R = crate::BitReader;
///Field `INITPRIV` writer - INITPRIV
pub type INITPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - PRIV
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - PRIV
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ALRAPRIV
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ALRBPRIV
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUTPRIV
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSPRIV
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - CALPRIV
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - INITPRIV
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PRIV
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCR")
            .field("priv_", &self.priv_())
            .field("initpriv", &self.initpriv())
            .field("calpriv", &self.calpriv())
            .field("tspriv", &self.tspriv())
            .field("wutpriv", &self.wutpriv())
            .field("alrbpriv", &self.alrbpriv())
            .field("alrapriv", &self.alrapriv())
            .finish()
    }
}
impl W {
    ///Bit 0 - ALRAPRIV
    #[inline(always)]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W<PRIVCRrs> {
        ALRAPRIV_W::new(self, 0)
    }
    ///Bit 1 - ALRBPRIV
    #[inline(always)]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W<PRIVCRrs> {
        ALRBPRIV_W::new(self, 1)
    }
    ///Bit 2 - WUTPRIV
    #[inline(always)]
    pub fn wutpriv(&mut self) -> WUTPRIV_W<PRIVCRrs> {
        WUTPRIV_W::new(self, 2)
    }
    ///Bit 3 - TSPRIV
    #[inline(always)]
    pub fn tspriv(&mut self) -> TSPRIV_W<PRIVCRrs> {
        TSPRIV_W::new(self, 3)
    }
    ///Bit 13 - CALPRIV
    #[inline(always)]
    pub fn calpriv(&mut self) -> CALPRIV_W<PRIVCRrs> {
        CALPRIV_W::new(self, 13)
    }
    ///Bit 14 - INITPRIV
    #[inline(always)]
    pub fn initpriv(&mut self) -> INITPRIV_W<PRIVCRrs> {
        INITPRIV_W::new(self, 14)
    }
    ///Bit 15 - PRIV
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<PRIVCRrs> {
        PRIV_W::new(self, 15)
    }
}
/**RTC privilege mode control register

You can [`read`](crate::Reg::read) this register and get [`privcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RTC:PRIVCR)*/
pub struct PRIVCRrs;
impl crate::RegisterSpec for PRIVCRrs {
    type Ux = u32;
}
///`read()` method returns [`privcr::R`](R) reader structure
impl crate::Readable for PRIVCRrs {}
///`write(|w| ..)` method takes [`privcr::W`](W) writer structure
impl crate::Writable for PRIVCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCR to value 0
impl crate::Resettable for PRIVCRrs {}
