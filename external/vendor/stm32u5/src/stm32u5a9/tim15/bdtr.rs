///Register `BDTR` reader
pub type R = crate::R<BDTRrs>;
///Register `BDTR` writer
pub type W = crate::W<BDTRrs>;
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Lock configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    ///0: No bit is write protected
    Off = 0,
    ///1: Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written
    Level1 = 1,
    ///2: LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written
    Level2 = 2,
    ///3: LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written
    Level3 = 3,
}
impl From<LOCK> for u8 {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK {
    type Ux = u8;
}
impl crate::IsEnum for LOCK {}
///Field `LOCK` reader - Lock configuration
pub type LOCK_R = crate::FieldReader<LOCK>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            0 => LOCK::Off,
            1 => LOCK::Level1,
            2 => LOCK::Level2,
            3 => LOCK::Level3,
            _ => unreachable!(),
        }
    }
    ///No bit is write protected
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK::Off
    }
    ///Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK::Level1
    }
    ///LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK::Level2
    }
    ///LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK::Level3
    }
}
///Field `LOCK` writer - Lock configuration
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LOCK, crate::Safe>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No bit is write protected
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Off)
    }
    ///Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level1)
    }
    ///LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level2)
    }
    ///LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level3)
    }
}
/**Off-state selection for Idle mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    ///0: When inactive, OC/OCN outputs are disabled
    HiZ = 0,
    ///1: When inactive, OC/OCN outputs are forced to idle level
    IdleLevel = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSI` reader - Off-state selection for Idle mode
pub type OSSI_R = crate::BitReader<OSSI>;
impl OSSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSSI {
        match self.bits {
            false => OSSI::HiZ,
            true => OSSI::IdleLevel,
        }
    }
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn is_hi_z(&self) -> bool {
        *self == OSSI::HiZ
    }
    ///When inactive, OC/OCN outputs are forced to idle level
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSI::IdleLevel
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn hi_z(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::HiZ)
    }
    ///When inactive, OC/OCN outputs are forced to idle level
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::IdleLevel)
    }
}
/**Off-state selection for Run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    ///0: When inactive, OC/OCN outputs are disabled
    HiZ = 0,
    ///1: When inactive, OC/OCN outputs are enabled with their inactive level
    IdleLevel = 1,
}
impl From<OSSR> for bool {
    #[inline(always)]
    fn from(variant: OSSR) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSR` reader - Off-state selection for Run mode
pub type OSSR_R = crate::BitReader<OSSR>;
impl OSSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSSR {
        match self.bits {
            false => OSSR::HiZ,
            true => OSSR::IdleLevel,
        }
    }
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn is_hi_z(&self) -> bool {
        *self == OSSR::HiZ
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSR::IdleLevel
    }
}
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn hi_z(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::HiZ)
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::IdleLevel)
    }
}
/**Break enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    ///0: Break function x disabled
    Disabled = 0,
    ///1: Break function x enabled
    Enabled = 1,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
///Field `BKE` reader - Break enable
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKE {
        match self.bits {
            false => BKE::Disabled,
            true => BKE::Enabled,
        }
    }
    ///Break function x disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE::Disabled
    }
    ///Break function x enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE::Enabled
    }
}
///Field `BKE` writer - Break enable
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break function x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Disabled)
    }
    ///Break function x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Enabled)
    }
}
/**Break polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    ///0: Break input BRKx is active low
    ActiveLow = 0,
    ///1: Break input BRKx is active high
    ActiveHigh = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - Break polarity
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::ActiveLow,
            true => BKP::ActiveHigh,
        }
    }
    ///Break input BRKx is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP::ActiveLow
    }
    ///Break input BRKx is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP::ActiveHigh
    }
}
///Field `BKP` writer - Break polarity
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRKx is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveLow)
    }
    ///Break input BRKx is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveHigh)
    }
}
/**Automatic output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    ///0: MOE can be set only by software
    Manual = 0,
    ///1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    Automatic = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
///Field `AOE` reader - Automatic output enable
pub type AOE_R = crate::BitReader<AOE>;
impl AOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AOE {
        match self.bits {
            false => AOE::Manual,
            true => AOE::Automatic,
        }
    }
    ///MOE can be set only by software
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == AOE::Manual
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AOE::Automatic
    }
}
///Field `AOE` writer - Automatic output enable
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MOE can be set only by software
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Manual)
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Automatic)
    }
}
/**Main output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    ///0: OC/OCN are disabled or forced idle depending on OSSI
    DisabledIdle = 0,
    ///1: OC/OCN are enabled if CCxE/CCxNE are set
    Enabled = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MOE` reader - Main output enable
pub type MOE_R = crate::BitReader<MOE>;
impl MOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOE {
        match self.bits {
            false => MOE::DisabledIdle,
            true => MOE::Enabled,
        }
    }
    ///OC/OCN are disabled or forced idle depending on OSSI
    #[inline(always)]
    pub fn is_disabled_idle(&self) -> bool {
        *self == MOE::DisabledIdle
    }
    ///OC/OCN are enabled if CCxE/CCxNE are set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE::Enabled
    }
}
///Field `MOE` writer - Main output enable
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC/OCN are disabled or forced idle depending on OSSI
    #[inline(always)]
    pub fn disabled_idle(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::DisabledIdle)
    }
    ///OC/OCN are enabled if CCxE/CCxNE are set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Enabled)
    }
}
///Field `BKF` reader - Break filter
pub type BKF_R = crate::FieldReader;
///Field `BKF` writer - Break filter
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BKDSRM` reader - Break Disarm
pub type BKDSRM_R = crate::BitReader;
///Field `BKDSRM` writer - Break Disarm
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKBID` reader - Break Bidirectional
pub type BKBID_R = crate::BitReader;
///Field `BKBID` writer - Break Bidirectional
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 26 - Break Disarm
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Break Bidirectional
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTR")
            .field("bkbid", &self.bkbid())
            .field("bkdsrm", &self.bkdsrm())
            .field("bkf", &self.bkf())
            .field("moe", &self.moe())
            .field("aoe", &self.aoe())
            .field("bkp", &self.bkp())
            .field("bke", &self.bke())
            .field("ossr", &self.ossr())
            .field("ossi", &self.ossi())
            .field("lock", &self.lock())
            .field("dtg", &self.dtg())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W<BDTRrs> {
        BKF_W::new(self, 16)
    }
    ///Bit 26 - Break Disarm
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    ///Bit 28 - Break Bidirectional
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
}
/**break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TIM15:BDTR)*/
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtr::R`](R) reader structure
impl crate::Readable for BDTRrs {}
///`write(|w| ..)` method takes [`bdtr::W`](W) writer structure
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTRrs {}
