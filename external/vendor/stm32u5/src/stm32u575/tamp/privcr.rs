///Register `PRIVCR` reader
pub type R = crate::R<PRIVCRrs>;
///Register `PRIVCR` writer
pub type W = crate::W<PRIVCRrs>;
///Field `CNT1PRIV` reader - Monotonic counter 1 privilege protection
pub type CNT1PRIV_R = crate::BitReader;
///Field `CNT1PRIV` writer - Monotonic counter 1 privilege protection
pub type CNT1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection
pub type BKPRWPRIV_R = crate::BitReader;
///Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection
pub type BKPRWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection
pub type BKPWPRIV_R = crate::BitReader;
///Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection
pub type BKPWPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPPRIV` reader - Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection.
pub type TAMPPRIV_R = crate::BitReader;
///Field `TAMPPRIV` writer - Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection.
pub type TAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 15 - Monotonic counter 1 privilege protection
    #[inline(always)]
    pub fn cnt1priv(&self) -> CNT1PRIV_R {
        CNT1PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 29 - Backup registers zone 1 privilege protection
    #[inline(always)]
    pub fn bkprwpriv(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Backup registers zone 2 privilege protection
    #[inline(always)]
    pub fn bkpwpriv(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection.
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCR")
            .field("cnt1priv", &self.cnt1priv())
            .field("bkprwpriv", &self.bkprwpriv())
            .field("bkpwpriv", &self.bkpwpriv())
            .field("tamppriv", &self.tamppriv())
            .finish()
    }
}
impl W {
    ///Bit 15 - Monotonic counter 1 privilege protection
    #[inline(always)]
    pub fn cnt1priv(&mut self) -> CNT1PRIV_W<PRIVCRrs> {
        CNT1PRIV_W::new(self, 15)
    }
    ///Bit 29 - Backup registers zone 1 privilege protection
    #[inline(always)]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W<PRIVCRrs> {
        BKPRWPRIV_W::new(self, 29)
    }
    ///Bit 30 - Backup registers zone 2 privilege protection
    #[inline(always)]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W<PRIVCRrs> {
        BKPWPRIV_W::new(self, 30)
    }
    ///Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection.
    #[inline(always)]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W<PRIVCRrs> {
        TAMPPRIV_W::new(self, 31)
    }
}
/**TAMP privilege mode control register

You can [`read`](crate::Reg::read) this register and get [`privcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:PRIVCR)*/
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
