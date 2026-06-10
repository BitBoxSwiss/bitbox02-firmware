///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
/**WUP1 secure protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUP1SEC {
    ///0: Bits related to WKUPx pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and PWR_WUSCR can be read and written with secure or nonsecure access
    NonSecure = 0,
    ///1: Bits related to WKUPx pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and PWR_WUSCR can be read and written only with secure access
    Secure = 1,
}
impl From<WUP1SEC> for bool {
    #[inline(always)]
    fn from(variant: WUP1SEC) -> Self {
        variant as u8 != 0
    }
}
///Field `WUP1SEC` reader - WUP1 secure protection
pub type WUP1SEC_R = crate::BitReader<WUP1SEC>;
impl WUP1SEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUP1SEC {
        match self.bits {
            false => WUP1SEC::NonSecure,
            true => WUP1SEC::Secure,
        }
    }
    ///Bits related to WKUPx pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and PWR_WUSCR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == WUP1SEC::NonSecure
    }
    ///Bits related to WKUPx pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and PWR_WUSCR can be read and written only with secure access
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == WUP1SEC::Secure
    }
}
///Field `WUP1SEC` writer - WUP1 secure protection
pub type WUP1SEC_W<'a, REG> = crate::BitWriter<'a, REG, WUP1SEC>;
impl<'a, REG> WUP1SEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bits related to WKUPx pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and PWR_WUSCR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(WUP1SEC::NonSecure)
    }
    ///Bits related to WKUPx pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and PWR_WUSCR can be read and written only with secure access
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(WUP1SEC::Secure)
    }
}
///Field `WUP2SEC` reader - WUP2 secure protection
pub use WUP1SEC_R as WUP2SEC_R;
///Field `WUP3SEC` reader - WUP3 secure protection
pub use WUP1SEC_R as WUP3SEC_R;
///Field `WUP4SEC` reader - WUP4 secure protection
pub use WUP1SEC_R as WUP4SEC_R;
///Field `WUP5SEC` reader - WUP5 secure protection
pub use WUP1SEC_R as WUP5SEC_R;
///Field `WUP6SEC` reader - WUP6 secure protection
pub use WUP1SEC_R as WUP6SEC_R;
///Field `WUP7SEC` reader - WUP7 secure protection
pub use WUP1SEC_R as WUP7SEC_R;
///Field `WUP8SEC` reader - WUP8 secure protection
pub use WUP1SEC_R as WUP8SEC_R;
///Field `WUP2SEC` writer - WUP2 secure protection
pub use WUP1SEC_W as WUP2SEC_W;
///Field `WUP3SEC` writer - WUP3 secure protection
pub use WUP1SEC_W as WUP3SEC_W;
///Field `WUP4SEC` writer - WUP4 secure protection
pub use WUP1SEC_W as WUP4SEC_W;
///Field `WUP5SEC` writer - WUP5 secure protection
pub use WUP1SEC_W as WUP5SEC_W;
///Field `WUP6SEC` writer - WUP6 secure protection
pub use WUP1SEC_W as WUP6SEC_W;
///Field `WUP7SEC` writer - WUP7 secure protection
pub use WUP1SEC_W as WUP7SEC_W;
///Field `WUP8SEC` writer - WUP8 secure protection
pub use WUP1SEC_W as WUP8SEC_W;
/**Low-power modes secure protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMSEC {
    ///0: PWR_CR1, PWR_CR2 and CSSF in the PWR_SR can be read and written with secure or nonsecure access
    NonSecure = 0,
    ///1: PWR_CR1, PWR_CR2, and CSSF in the PWR_SR can be read and written only with secure access
    Secure = 1,
}
impl From<LPMSEC> for bool {
    #[inline(always)]
    fn from(variant: LPMSEC) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMSEC` reader - Low-power modes secure protection
pub type LPMSEC_R = crate::BitReader<LPMSEC>;
impl LPMSEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMSEC {
        match self.bits {
            false => LPMSEC::NonSecure,
            true => LPMSEC::Secure,
        }
    }
    ///PWR_CR1, PWR_CR2 and CSSF in the PWR_SR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == LPMSEC::NonSecure
    }
    ///PWR_CR1, PWR_CR2, and CSSF in the PWR_SR can be read and written only with secure access
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == LPMSEC::Secure
    }
}
///Field `LPMSEC` writer - Low-power modes secure protection
pub type LPMSEC_W<'a, REG> = crate::BitWriter<'a, REG, LPMSEC>;
impl<'a, REG> LPMSEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PWR_CR1, PWR_CR2 and CSSF in the PWR_SR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(LPMSEC::NonSecure)
    }
    ///PWR_CR1, PWR_CR2, and CSSF in the PWR_SR can be read and written only with secure access
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(LPMSEC::Secure)
    }
}
/**Voltage detection and monitoring secure protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDMSEC {
    ///0: PWR_SVMCR and PWR_CR3 can be read and written with secure or nonsecure access
    NonSecure = 0,
    ///1: PWR_SVMCR and PWR_CR3 can be read and written only with secure access
    Secure = 1,
}
impl From<VDMSEC> for bool {
    #[inline(always)]
    fn from(variant: VDMSEC) -> Self {
        variant as u8 != 0
    }
}
///Field `VDMSEC` reader - Voltage detection and monitoring secure protection
pub type VDMSEC_R = crate::BitReader<VDMSEC>;
impl VDMSEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDMSEC {
        match self.bits {
            false => VDMSEC::NonSecure,
            true => VDMSEC::Secure,
        }
    }
    ///PWR_SVMCR and PWR_CR3 can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == VDMSEC::NonSecure
    }
    ///PWR_SVMCR and PWR_CR3 can be read and written only with secure access
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == VDMSEC::Secure
    }
}
///Field `VDMSEC` writer - Voltage detection and monitoring secure protection
pub type VDMSEC_W<'a, REG> = crate::BitWriter<'a, REG, VDMSEC>;
impl<'a, REG> VDMSEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PWR_SVMCR and PWR_CR3 can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(VDMSEC::NonSecure)
    }
    ///PWR_SVMCR and PWR_CR3 can be read and written only with secure access
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(VDMSEC::Secure)
    }
}
/**Backup domain secure protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBSEC {
    ///0: PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written with secure or nonsecure access
    NonSecure = 0,
    ///1: PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written only with secure access
    Secure = 1,
}
impl From<VBSEC> for bool {
    #[inline(always)]
    fn from(variant: VBSEC) -> Self {
        variant as u8 != 0
    }
}
///Field `VBSEC` reader - Backup domain secure protection
pub type VBSEC_R = crate::BitReader<VBSEC>;
impl VBSEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBSEC {
        match self.bits {
            false => VBSEC::NonSecure,
            true => VBSEC::Secure,
        }
    }
    ///PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == VBSEC::NonSecure
    }
    ///PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written only with secure access
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == VBSEC::Secure
    }
}
///Field `VBSEC` writer - Backup domain secure protection
pub type VBSEC_W<'a, REG> = crate::BitWriter<'a, REG, VBSEC>;
impl<'a, REG> VBSEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(VBSEC::NonSecure)
    }
    ///PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written only with secure access
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(VBSEC::Secure)
    }
}
/**Pull-up/pull-down secure protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APCSEC {
    ///0: PWR_APCR can be read and written with secure or nonsecure access
    NonSecure = 0,
    ///1: PWR_APCR can be read and written only with secure access
    Secure = 1,
}
impl From<APCSEC> for bool {
    #[inline(always)]
    fn from(variant: APCSEC) -> Self {
        variant as u8 != 0
    }
}
///Field `APCSEC` reader - Pull-up/pull-down secure protection
pub type APCSEC_R = crate::BitReader<APCSEC>;
impl APCSEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APCSEC {
        match self.bits {
            false => APCSEC::NonSecure,
            true => APCSEC::Secure,
        }
    }
    ///PWR_APCR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == APCSEC::NonSecure
    }
    ///PWR_APCR can be read and written only with secure access
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == APCSEC::Secure
    }
}
///Field `APCSEC` writer - Pull-up/pull-down secure protection
pub type APCSEC_W<'a, REG> = crate::BitWriter<'a, REG, APCSEC>;
impl<'a, REG> APCSEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PWR_APCR can be read and written with secure or nonsecure access
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(APCSEC::NonSecure)
    }
    ///PWR_APCR can be read and written only with secure access
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(APCSEC::Secure)
    }
}
impl R {
    ///Bit 0 - WUP1 secure protection
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUP2 secure protection
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUP3 secure protection
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUP4 secure protection
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUP5 secure protection
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUP6 secure protection
    #[inline(always)]
    pub fn wup6sec(&self) -> WUP6SEC_R {
        WUP6SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUP7 secure protection
    #[inline(always)]
    pub fn wup7sec(&self) -> WUP7SEC_R {
        WUP7SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - WUP8 secure protection
    #[inline(always)]
    pub fn wup8sec(&self) -> WUP8SEC_R {
        WUP8SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - Low-power modes secure protection
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Voltage detection and monitoring secure protection
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Backup domain secure protection
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pull-up/pull-down secure protection
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("wup1sec", &self.wup1sec())
            .field("wup2sec", &self.wup2sec())
            .field("wup3sec", &self.wup3sec())
            .field("wup4sec", &self.wup4sec())
            .field("wup5sec", &self.wup5sec())
            .field("wup6sec", &self.wup6sec())
            .field("wup7sec", &self.wup7sec())
            .field("wup8sec", &self.wup8sec())
            .field("lpmsec", &self.lpmsec())
            .field("vdmsec", &self.vdmsec())
            .field("vbsec", &self.vbsec())
            .field("apcsec", &self.apcsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUP1 secure protection
    #[inline(always)]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<SECCFGRrs> {
        WUP1SEC_W::new(self, 0)
    }
    ///Bit 1 - WUP2 secure protection
    #[inline(always)]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<SECCFGRrs> {
        WUP2SEC_W::new(self, 1)
    }
    ///Bit 2 - WUP3 secure protection
    #[inline(always)]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<SECCFGRrs> {
        WUP3SEC_W::new(self, 2)
    }
    ///Bit 3 - WUP4 secure protection
    #[inline(always)]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<SECCFGRrs> {
        WUP4SEC_W::new(self, 3)
    }
    ///Bit 4 - WUP5 secure protection
    #[inline(always)]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<SECCFGRrs> {
        WUP5SEC_W::new(self, 4)
    }
    ///Bit 5 - WUP6 secure protection
    #[inline(always)]
    pub fn wup6sec(&mut self) -> WUP6SEC_W<SECCFGRrs> {
        WUP6SEC_W::new(self, 5)
    }
    ///Bit 6 - WUP7 secure protection
    #[inline(always)]
    pub fn wup7sec(&mut self) -> WUP7SEC_W<SECCFGRrs> {
        WUP7SEC_W::new(self, 6)
    }
    ///Bit 7 - WUP8 secure protection
    #[inline(always)]
    pub fn wup8sec(&mut self) -> WUP8SEC_W<SECCFGRrs> {
        WUP8SEC_W::new(self, 7)
    }
    ///Bit 12 - Low-power modes secure protection
    #[inline(always)]
    pub fn lpmsec(&mut self) -> LPMSEC_W<SECCFGRrs> {
        LPMSEC_W::new(self, 12)
    }
    ///Bit 13 - Voltage detection and monitoring secure protection
    #[inline(always)]
    pub fn vdmsec(&mut self) -> VDMSEC_W<SECCFGRrs> {
        VDMSEC_W::new(self, 13)
    }
    ///Bit 14 - Backup domain secure protection
    #[inline(always)]
    pub fn vbsec(&mut self) -> VBSEC_W<SECCFGRrs> {
        VBSEC_W::new(self, 14)
    }
    ///Bit 15 - Pull-up/pull-down secure protection
    #[inline(always)]
    pub fn apcsec(&mut self) -> APCSEC_W<SECCFGRrs> {
        APCSEC_W::new(self, 15)
    }
}
/**PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:SECCFGR)*/
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
