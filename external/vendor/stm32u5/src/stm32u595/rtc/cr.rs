///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**WUCKSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUCKSEL {
    ///0: RTC/16 clock is selected
    Div16 = 0,
    ///1: RTC/8 clock is selected
    Div8 = 1,
    ///2: RTC/4 clock is selected
    Div4 = 2,
    ///3: RTC/2 clock is selected
    Div2 = 3,
    ///4: ck_spre (usually 1 Hz) clock is selected
    ClockSpare = 4,
    ///6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    ClockSpareWithOffset = 6,
}
impl From<WUCKSEL> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUCKSEL {
    type Ux = u8;
}
impl crate::IsEnum for WUCKSEL {}
///Field `WUCKSEL` reader - WUCKSEL
pub type WUCKSEL_R = crate::FieldReader<WUCKSEL>;
impl WUCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUCKSEL> {
        match self.bits {
            0 => Some(WUCKSEL::Div16),
            1 => Some(WUCKSEL::Div8),
            2 => Some(WUCKSEL::Div4),
            3 => Some(WUCKSEL::Div2),
            4 => Some(WUCKSEL::ClockSpare),
            6 => Some(WUCKSEL::ClockSpareWithOffset),
            _ => None,
        }
    }
    ///RTC/16 clock is selected
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WUCKSEL::Div16
    }
    ///RTC/8 clock is selected
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WUCKSEL::Div8
    }
    ///RTC/4 clock is selected
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WUCKSEL::Div4
    }
    ///RTC/2 clock is selected
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WUCKSEL::Div2
    }
    ///ck_spre (usually 1 Hz) clock is selected
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        *self == WUCKSEL::ClockSpare
    }
    ///ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        *self == WUCKSEL::ClockSpareWithOffset
    }
}
///Field `WUCKSEL` writer - WUCKSEL
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WUCKSEL>;
impl<'a, REG> WUCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RTC/16 clock is selected
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div16)
    }
    ///RTC/8 clock is selected
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div8)
    }
    ///RTC/4 clock is selected
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div4)
    }
    ///RTC/2 clock is selected
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div2)
    }
    ///ck_spre (usually 1 Hz) clock is selected
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::ClockSpare)
    }
    ///ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::ClockSpareWithOffset)
    }
}
/**TSEDGE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE {
    ///0: RTC_TS input rising edge generates a time-stamp event
    RisingEdge = 0,
    ///1: RTC_TS input falling edge generates a time-stamp event
    FallingEdge = 1,
}
impl From<TSEDGE> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEDGE` reader - TSEDGE
pub type TSEDGE_R = crate::BitReader<TSEDGE>;
impl TSEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEDGE {
        match self.bits {
            false => TSEDGE::RisingEdge,
            true => TSEDGE::FallingEdge,
        }
    }
    ///RTC_TS input rising edge generates a time-stamp event
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TSEDGE::RisingEdge
    }
    ///RTC_TS input falling edge generates a time-stamp event
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TSEDGE::FallingEdge
    }
}
///Field `TSEDGE` writer - TSEDGE
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG, TSEDGE>;
impl<'a, REG> TSEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_TS input rising edge generates a time-stamp event
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::RisingEdge)
    }
    ///RTC_TS input falling edge generates a time-stamp event
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::FallingEdge)
    }
}
/**REFCKON

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON {
    ///0: RTC_REFIN detection disabled
    Disabled = 0,
    ///1: RTC_REFIN detection enabled
    Enabled = 1,
}
impl From<REFCKON> for bool {
    #[inline(always)]
    fn from(variant: REFCKON) -> Self {
        variant as u8 != 0
    }
}
///Field `REFCKON` reader - REFCKON
pub type REFCKON_R = crate::BitReader<REFCKON>;
impl REFCKON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REFCKON {
        match self.bits {
            false => REFCKON::Disabled,
            true => REFCKON::Enabled,
        }
    }
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REFCKON::Disabled
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REFCKON::Enabled
    }
}
///Field `REFCKON` writer - REFCKON
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG, REFCKON>;
impl<'a, REG> REFCKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::Disabled)
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::Enabled)
    }
}
/**BYPSHAD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSHAD {
    ///0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    ShadowReg = 0,
    ///1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    BypassShadowReg = 1,
}
impl From<BYPSHAD> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPSHAD` reader - BYPSHAD
pub type BYPSHAD_R = crate::BitReader<BYPSHAD>;
impl BYPSHAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BYPSHAD {
        match self.bits {
            false => BYPSHAD::ShadowReg,
            true => BYPSHAD::BypassShadowReg,
        }
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        *self == BYPSHAD::ShadowReg
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        *self == BYPSHAD::BypassShadowReg
    }
}
///Field `BYPSHAD` writer - BYPSHAD
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG, BYPSHAD>;
impl<'a, REG> BYPSHAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD::ShadowReg)
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD::BypassShadowReg)
    }
}
/**FMT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT {
    ///0: 24 hour/day format
    TwentyFourHour = 0,
    ///1: AM/PM hour format
    AmPm = 1,
}
impl From<FMT> for bool {
    #[inline(always)]
    fn from(variant: FMT) -> Self {
        variant as u8 != 0
    }
}
///Field `FMT` reader - FMT
pub type FMT_R = crate::BitReader<FMT>;
impl FMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMT {
        match self.bits {
            false => FMT::TwentyFourHour,
            true => FMT::AmPm,
        }
    }
    ///24 hour/day format
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        *self == FMT::TwentyFourHour
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        *self == FMT::AmPm
    }
}
///Field `FMT` writer - FMT
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG, FMT>;
impl<'a, REG> FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///24 hour/day format
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::TwentyFourHour)
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::AmPm)
    }
}
///Field `SSRUIE` reader - SSRUIE
pub type SSRUIE_R = crate::BitReader;
///Field `SSRUIE` writer - SSRUIE
pub type SSRUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Alarm %s enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE {
    ///0: Alarm disabled
    Disabled = 0,
    ///1: Alarm enabled
    Enabled = 1,
}
impl From<ALRAE> for bool {
    #[inline(always)]
    fn from(variant: ALRAE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRE(A,B)` reader - Alarm %s enable
pub type ALRE_R = crate::BitReader<ALRAE>;
impl ALRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAE {
        match self.bits {
            false => ALRAE::Disabled,
            true => ALRAE::Enabled,
        }
    }
    ///Alarm disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAE::Disabled
    }
    ///Alarm enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAE::Enabled
    }
}
///Field `ALRE(A,B)` writer - Alarm %s enable
pub type ALRE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAE>;
impl<'a, REG> ALRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::Disabled)
    }
    ///Alarm enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::Enabled)
    }
}
/**WUTE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTE {
    ///0: Wakeup timer disabled
    Disabled = 0,
    ///1: Wakeup timer enabled
    Enabled = 1,
}
impl From<WUTE> for bool {
    #[inline(always)]
    fn from(variant: WUTE) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTE` reader - WUTE
pub type WUTE_R = crate::BitReader<WUTE>;
impl WUTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUTE {
        match self.bits {
            false => WUTE::Disabled,
            true => WUTE::Enabled,
        }
    }
    ///Wakeup timer disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTE::Disabled
    }
    ///Wakeup timer enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTE::Enabled
    }
}
///Field `WUTE` writer - WUTE
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG, WUTE>;
impl<'a, REG> WUTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wakeup timer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTE::Disabled)
    }
    ///Wakeup timer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTE::Enabled)
    }
}
/**TSE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE {
    ///0: Timestamp disabled
    Disabled = 0,
    ///1: Timestamp enabled
    Enabled = 1,
}
impl From<TSE> for bool {
    #[inline(always)]
    fn from(variant: TSE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSE` reader - TSE
pub type TSE_R = crate::BitReader<TSE>;
impl TSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSE {
        match self.bits {
            false => TSE::Disabled,
            true => TSE::Enabled,
        }
    }
    ///Timestamp disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSE::Disabled
    }
    ///Timestamp enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSE::Enabled
    }
}
///Field `TSE` writer - TSE
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG, TSE>;
impl<'a, REG> TSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timestamp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::Disabled)
    }
    ///Timestamp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::Enabled)
    }
}
/**Alarm %s interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE {
    ///0: Alarm Interrupt disabled
    Disabled = 0,
    ///1: Alarm Interrupt enabled
    Enabled = 1,
}
impl From<ALRAIE> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRIE(A,B)` reader - Alarm %s interrupt enable
pub type ALRIE_R = crate::BitReader<ALRAIE>;
impl ALRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAIE {
        match self.bits {
            false => ALRAIE::Disabled,
            true => ALRAIE::Enabled,
        }
    }
    ///Alarm Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAIE::Disabled
    }
    ///Alarm Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAIE::Enabled
    }
}
///Field `ALRIE(A,B)` writer - Alarm %s interrupt enable
pub type ALRIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAIE>;
impl<'a, REG> ALRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::Disabled)
    }
    ///Alarm Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::Enabled)
    }
}
/**WUTIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTIE {
    ///0: Wakeup timer interrupt disabled
    Disabled = 0,
    ///1: Wakeup timer interrupt enabled
    Enabled = 1,
}
impl From<WUTIE> for bool {
    #[inline(always)]
    fn from(variant: WUTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTIE` reader - WUTIE
pub type WUTIE_R = crate::BitReader<WUTIE>;
impl WUTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUTIE {
        match self.bits {
            false => WUTIE::Disabled,
            true => WUTIE::Enabled,
        }
    }
    ///Wakeup timer interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTIE::Disabled
    }
    ///Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTIE::Enabled
    }
}
///Field `WUTIE` writer - WUTIE
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG, WUTIE>;
impl<'a, REG> WUTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wakeup timer interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTIE::Disabled)
    }
    ///Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTIE::Enabled)
    }
}
/**TSIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE {
    ///0: Time-stamp Interrupt disabled
    Disabled = 0,
    ///1: Time-stamp Interrupt enabled
    Enabled = 1,
}
impl From<TSIE> for bool {
    #[inline(always)]
    fn from(variant: TSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIE` reader - TSIE
pub type TSIE_R = crate::BitReader<TSIE>;
impl TSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSIE {
        match self.bits {
            false => TSIE::Disabled,
            true => TSIE::Enabled,
        }
    }
    ///Time-stamp Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIE::Disabled
    }
    ///Time-stamp Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIE::Enabled
    }
}
///Field `TSIE` writer - TSIE
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG, TSIE>;
impl<'a, REG> TSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Time-stamp Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::Disabled)
    }
    ///Time-stamp Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::Enabled)
    }
}
/**ADD1H

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1HW {
    ///1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    Add1 = 1,
}
impl From<ADD1HW> for bool {
    #[inline(always)]
    fn from(variant: ADD1HW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1H` writer - ADD1H
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG, ADD1HW>;
impl<'a, REG> ADD1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    #[inline(always)]
    pub fn add1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1HW::Add1)
    }
}
/**SUB1H

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1HW {
    ///1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    Sub1 = 1,
}
impl From<SUB1HW> for bool {
    #[inline(always)]
    fn from(variant: SUB1HW) -> Self {
        variant as u8 != 0
    }
}
///Field `SUB1H` writer - SUB1H
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG, SUB1HW>;
impl<'a, REG> SUB1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    #[inline(always)]
    pub fn sub1(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1HW::Sub1)
    }
}
/**BKP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    ///0: Daylight Saving Time change has not been performed
    DstnotChanged = 0,
    ///1: Daylight Saving Time change has been performed
    Dstchanged = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - BKP
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::DstnotChanged,
            true => BKP::Dstchanged,
        }
    }
    ///Daylight Saving Time change has not been performed
    #[inline(always)]
    pub fn is_dstnot_changed(&self) -> bool {
        *self == BKP::DstnotChanged
    }
    ///Daylight Saving Time change has been performed
    #[inline(always)]
    pub fn is_dstchanged(&self) -> bool {
        *self == BKP::Dstchanged
    }
}
///Field `BKP` writer - BKP
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Daylight Saving Time change has not been performed
    #[inline(always)]
    pub fn dstnot_changed(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::DstnotChanged)
    }
    ///Daylight Saving Time change has been performed
    #[inline(always)]
    pub fn dstchanged(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::Dstchanged)
    }
}
/**COSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSEL {
    ///0: Calibration output is 512 Hz (with default prescaler setting)
    CalFreq512hz = 0,
    ///1: Calibration output is 1 Hz (with default prescaler setting)
    CalFreq1hz = 1,
}
impl From<COSEL> for bool {
    #[inline(always)]
    fn from(variant: COSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `COSEL` reader - COSEL
pub type COSEL_R = crate::BitReader<COSEL>;
impl COSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COSEL {
        match self.bits {
            false => COSEL::CalFreq512hz,
            true => COSEL::CalFreq1hz,
        }
    }
    ///Calibration output is 512 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        *self == COSEL::CalFreq512hz
    }
    ///Calibration output is 1 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        *self == COSEL::CalFreq1hz
    }
}
///Field `COSEL` writer - COSEL
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG, COSEL>;
impl<'a, REG> COSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration output is 512 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL::CalFreq512hz)
    }
    ///Calibration output is 1 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL::CalFreq1hz)
    }
}
/**POL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL {
    ///0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    High = 0,
    ///1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    Low = 1,
}
impl From<POL> for bool {
    #[inline(always)]
    fn from(variant: POL) -> Self {
        variant as u8 != 0
    }
}
///Field `POL` reader - POL
pub type POL_R = crate::BitReader<POL>;
impl POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POL {
        match self.bits {
            false => POL::High,
            true => POL::Low,
        }
    }
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL::High
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL::Low
    }
}
///Field `POL` writer - POL
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG, POL>;
impl<'a, REG> POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(POL::High)
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(POL::Low)
    }
}
/**OSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL {
    ///0: Output disabled
    Disabled = 0,
    ///1: Alarm A output enabled
    AlarmA = 1,
    ///2: Alarm B output enabled
    AlarmB = 2,
    ///3: Wakeup output enabled
    Wakeup = 3,
}
impl From<OSEL> for u8 {
    #[inline(always)]
    fn from(variant: OSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSEL {
    type Ux = u8;
}
impl crate::IsEnum for OSEL {}
///Field `OSEL` reader - OSEL
pub type OSEL_R = crate::FieldReader<OSEL>;
impl OSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSEL {
        match self.bits {
            0 => OSEL::Disabled,
            1 => OSEL::AlarmA,
            2 => OSEL::AlarmB,
            3 => OSEL::Wakeup,
            _ => unreachable!(),
        }
    }
    ///Output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSEL::Disabled
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OSEL::AlarmA
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OSEL::AlarmB
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OSEL::Wakeup
    }
}
///Field `OSEL` writer - OSEL
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSEL, crate::Safe>;
impl<'a, REG> OSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::Disabled)
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::AlarmA)
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::AlarmB)
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::Wakeup)
    }
}
/**COE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE {
    ///0: Calibration output disabled
    Disabled = 0,
    ///1: Calibration output enabled
    Enabled = 1,
}
impl From<COE> for bool {
    #[inline(always)]
    fn from(variant: COE) -> Self {
        variant as u8 != 0
    }
}
///Field `COE` reader - COE
pub type COE_R = crate::BitReader<COE>;
impl COE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COE {
        match self.bits {
            false => COE::Disabled,
            true => COE::Enabled,
        }
    }
    ///Calibration output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COE::Disabled
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COE::Enabled
    }
}
///Field `COE` writer - COE
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG, COE>;
impl<'a, REG> COE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COE::Disabled)
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COE::Enabled)
    }
}
/**ITSE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSE {
    ///0: Internal event timestamp disabled
    Disabled = 0,
    ///1: Internal event timestamp enabled
    Enabled = 1,
}
impl From<ITSE> for bool {
    #[inline(always)]
    fn from(variant: ITSE) -> Self {
        variant as u8 != 0
    }
}
///Field `ITSE` reader - ITSE
pub type ITSE_R = crate::BitReader<ITSE>;
impl ITSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITSE {
        match self.bits {
            false => ITSE::Disabled,
            true => ITSE::Enabled,
        }
    }
    ///Internal event timestamp disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITSE::Disabled
    }
    ///Internal event timestamp enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITSE::Enabled
    }
}
///Field `ITSE` writer - ITSE
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG, ITSE>;
impl<'a, REG> ITSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal event timestamp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITSE::Disabled)
    }
    ///Internal event timestamp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITSE::Enabled)
    }
}
/**TAMPTS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPTS {
    ///0: Tamper detection event does not cause a RTC timestamp to be saved
    Disabled = 0,
    ///1: Save RTC timestamp on tamper detection event
    Enabled = 1,
}
impl From<TAMPTS> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPTS` reader - TAMPTS
pub type TAMPTS_R = crate::BitReader<TAMPTS>;
impl TAMPTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPTS {
        match self.bits {
            false => TAMPTS::Disabled,
            true => TAMPTS::Enabled,
        }
    }
    ///Tamper detection event does not cause a RTC timestamp to be saved
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPTS::Disabled
    }
    ///Save RTC timestamp on tamper detection event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPTS::Enabled
    }
}
///Field `TAMPTS` writer - TAMPTS
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPTS>;
impl<'a, REG> TAMPTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper detection event does not cause a RTC timestamp to be saved
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::Disabled)
    }
    ///Save RTC timestamp on tamper detection event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::Enabled)
    }
}
/**TAMPOE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPOE {
    ///0: The tamper flag is not routed on TAMPALRM
    Disabled = 0,
    ///1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL
    Enabled = 1,
}
impl From<TAMPOE> for bool {
    #[inline(always)]
    fn from(variant: TAMPOE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPOE` reader - TAMPOE
pub type TAMPOE_R = crate::BitReader<TAMPOE>;
impl TAMPOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPOE {
        match self.bits {
            false => TAMPOE::Disabled,
            true => TAMPOE::Enabled,
        }
    }
    ///The tamper flag is not routed on TAMPALRM
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPOE::Disabled
    }
    ///The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPOE::Enabled
    }
}
///Field `TAMPOE` writer - TAMPOE
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPOE>;
impl<'a, REG> TAMPOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The tamper flag is not routed on TAMPALRM
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPOE::Disabled)
    }
    ///The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPOE::Enabled)
    }
}
///Field `ALRAFCLR` reader - ALRAFCLR
pub type ALRAFCLR_R = crate::BitReader;
///Field `ALRAFCLR` writer - ALRAFCLR
pub type ALRAFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBFCLR` reader - ALRBFCLR
pub type ALRBFCLR_R = crate::BitReader;
///Field `ALRBFCLR` writer - ALRBFCLR
pub type ALRBFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
/**TAMPALRM_PU

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_PU {
    ///0: No pull-up is applied on TAMPALRM output
    NoPullUp = 0,
    ///1: A pull-up is applied on TAMPALRM output
    PullUp = 1,
}
impl From<TAMPALRM_PU> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_PU) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPALRM_PU` reader - TAMPALRM_PU
pub type TAMPALRM_PU_R = crate::BitReader<TAMPALRM_PU>;
impl TAMPALRM_PU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_PU {
        match self.bits {
            false => TAMPALRM_PU::NoPullUp,
            true => TAMPALRM_PU::PullUp,
        }
    }
    ///No pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn is_no_pull_up(&self) -> bool {
        *self == TAMPALRM_PU::NoPullUp
    }
    ///A pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == TAMPALRM_PU::PullUp
    }
}
///Field `TAMPALRM_PU` writer - TAMPALRM_PU
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_PU>;
impl<'a, REG> TAMPALRM_PU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn no_pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU::NoPullUp)
    }
    ///A pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU::PullUp)
    }
}
/**TAMPALRM_TYPE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_TYPE {
    ///0: TAMPALRM is push-pull output
    PushPull = 0,
    ///1: TAMPALRM is open-drain output
    OpenDrain = 1,
}
impl From<TAMPALRM_TYPE> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_TYPE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPALRM_TYPE` reader - TAMPALRM_TYPE
pub type TAMPALRM_TYPE_R = crate::BitReader<TAMPALRM_TYPE>;
impl TAMPALRM_TYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_TYPE {
        match self.bits {
            false => TAMPALRM_TYPE::PushPull,
            true => TAMPALRM_TYPE::OpenDrain,
        }
    }
    ///TAMPALRM is push-pull output
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == TAMPALRM_TYPE::PushPull
    }
    ///TAMPALRM is open-drain output
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == TAMPALRM_TYPE::OpenDrain
    }
}
///Field `TAMPALRM_TYPE` writer - TAMPALRM_TYPE
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_TYPE>;
impl<'a, REG> TAMPALRM_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TAMPALRM is push-pull output
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE::PushPull)
    }
    ///TAMPALRM is open-drain output
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE::OpenDrain)
    }
}
/**OUT2EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT2EN {
    ///0: RTC output 2 disable
    Disabled = 0,
    ///1: RTC output 2 enable
    Enabled = 1,
}
impl From<OUT2EN> for bool {
    #[inline(always)]
    fn from(variant: OUT2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OUT2EN` reader - OUT2EN
pub type OUT2EN_R = crate::BitReader<OUT2EN>;
impl OUT2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUT2EN {
        match self.bits {
            false => OUT2EN::Disabled,
            true => OUT2EN::Enabled,
        }
    }
    ///RTC output 2 disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUT2EN::Disabled
    }
    ///RTC output 2 enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OUT2EN::Enabled
    }
}
///Field `OUT2EN` writer - OUT2EN
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG, OUT2EN>;
impl<'a, REG> OUT2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC output 2 disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OUT2EN::Disabled)
    }
    ///RTC output 2 enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OUT2EN::Enabled)
    }
}
impl R {
    ///Bits 0:2 - WUCKSEL
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - TSEDGE
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - REFCKON
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BYPSHAD
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FMT
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SSRUIE
    #[inline(always)]
    pub fn ssruie(&self) -> SSRUIE_R {
        SSRUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Alarm (A,B) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAE` field.</div>
    #[inline(always)]
    pub fn alre(&self, n: u8) -> ALRE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRE_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) enable
    #[inline(always)]
    pub fn alre_iter(&self) -> impl Iterator<Item = ALRE_R> + '_ {
        (0..2).map(move |n| ALRE_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRE_R {
        ALRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRE_R {
        ALRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WUTE
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TSE
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Alarm (A,B) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAIE` field.</div>
    #[inline(always)]
    pub fn alrie(&self, n: u8) -> ALRIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRIE_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) interrupt enable
    #[inline(always)]
    pub fn alrie_iter(&self) -> impl Iterator<Item = ALRIE_R> + '_ {
        (0..2).map(move |n| ALRIE_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WUTIE
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TSIE
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - COSEL
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - POL
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - OSEL
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - COE
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ITSE
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TAMPTS
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TAMPOE
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ALRAFCLR
    #[inline(always)]
    pub fn alrafclr(&self) -> ALRAFCLR_R {
        ALRAFCLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ALRBFCLR
    #[inline(always)]
    pub fn alrbfclr(&self) -> ALRBFCLR_R {
        ALRBFCLR_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TAMPALRM_PU
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TAMPALRM_TYPE
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OUT2EN
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wucksel", &self.wucksel())
            .field("tsedge", &self.tsedge())
            .field("refckon", &self.refckon())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("ssruie", &self.ssruie())
            .field("alrae", &self.alrae())
            .field("alrbe", &self.alrbe())
            .field("wute", &self.wute())
            .field("tse", &self.tse())
            .field("alraie", &self.alraie())
            .field("alrbie", &self.alrbie())
            .field("wutie", &self.wutie())
            .field("tsie", &self.tsie())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .field("itse", &self.itse())
            .field("tampts", &self.tampts())
            .field("tampoe", &self.tampoe())
            .field("alrafclr", &self.alrafclr())
            .field("alrbfclr", &self.alrbfclr())
            .field("tampalrm_pu", &self.tampalrm_pu())
            .field("tampalrm_type", &self.tampalrm_type())
            .field("out2en", &self.out2en())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - WUCKSEL
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W<CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    ///Bit 3 - TSEDGE
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - REFCKON
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 5 - BYPSHAD
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - FMT
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 7 - SSRUIE
    #[inline(always)]
    pub fn ssruie(&mut self) -> SSRUIE_W<CRrs> {
        SSRUIE_W::new(self, 7)
    }
    ///Alarm (A,B) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAE` field.</div>
    #[inline(always)]
    pub fn alre(&mut self, n: u8) -> ALRE_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRE_W::new(self, n + 8)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRE_W<CRrs> {
        ALRE_W::new(self, 8)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRE_W<CRrs> {
        ALRE_W::new(self, 9)
    }
    ///Bit 10 - WUTE
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W<CRrs> {
        WUTE_W::new(self, 10)
    }
    ///Bit 11 - TSE
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<CRrs> {
        TSE_W::new(self, 11)
    }
    ///Alarm (A,B) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAIE` field.</div>
    #[inline(always)]
    pub fn alrie(&mut self, n: u8) -> ALRIE_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRIE_W::new(self, n + 12)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRIE_W<CRrs> {
        ALRIE_W::new(self, 12)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRIE_W<CRrs> {
        ALRIE_W::new(self, 13)
    }
    ///Bit 14 - WUTIE
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W<CRrs> {
        WUTIE_W::new(self, 14)
    }
    ///Bit 15 - TSIE
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - ADD1H
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - SUB1H
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - COSEL
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - POL
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - OSEL
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - COE
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<CRrs> {
        COE_W::new(self, 23)
    }
    ///Bit 24 - ITSE
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W<CRrs> {
        ITSE_W::new(self, 24)
    }
    ///Bit 25 - TAMPTS
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<CRrs> {
        TAMPTS_W::new(self, 25)
    }
    ///Bit 26 - TAMPOE
    #[inline(always)]
    pub fn tampoe(&mut self) -> TAMPOE_W<CRrs> {
        TAMPOE_W::new(self, 26)
    }
    ///Bit 27 - ALRAFCLR
    #[inline(always)]
    pub fn alrafclr(&mut self) -> ALRAFCLR_W<CRrs> {
        ALRAFCLR_W::new(self, 27)
    }
    ///Bit 28 - ALRBFCLR
    #[inline(always)]
    pub fn alrbfclr(&mut self) -> ALRBFCLR_W<CRrs> {
        ALRBFCLR_W::new(self, 28)
    }
    ///Bit 29 - TAMPALRM_PU
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    ///Bit 30 - TAMPALRM_TYPE
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    ///Bit 31 - OUT2EN
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W<CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
/**RTC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RTC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
