///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**TXMODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXMODE {
    ///0: Transmission of Tx packet previously defined in other registers
    RegisterSet = 0,
    ///1: Cable Reset sequence
    CableReset = 1,
    ///2: BIST test sequence (BIST Carrier Mode 2)
    Bisttest = 2,
}
impl From<TXMODE> for u8 {
    #[inline(always)]
    fn from(variant: TXMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXMODE {
    type Ux = u8;
}
impl crate::IsEnum for TXMODE {}
///Field `TXMODE` reader - TXMODE
pub type TXMODE_R = crate::FieldReader<TXMODE>;
impl TXMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXMODE> {
        match self.bits {
            0 => Some(TXMODE::RegisterSet),
            1 => Some(TXMODE::CableReset),
            2 => Some(TXMODE::Bisttest),
            _ => None,
        }
    }
    ///Transmission of Tx packet previously defined in other registers
    #[inline(always)]
    pub fn is_register_set(&self) -> bool {
        *self == TXMODE::RegisterSet
    }
    ///Cable Reset sequence
    #[inline(always)]
    pub fn is_cable_reset(&self) -> bool {
        *self == TXMODE::CableReset
    }
    ///BIST test sequence (BIST Carrier Mode 2)
    #[inline(always)]
    pub fn is_bisttest(&self) -> bool {
        *self == TXMODE::Bisttest
    }
}
///Field `TXMODE` writer - TXMODE
pub type TXMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TXMODE>;
impl<'a, REG> TXMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Transmission of Tx packet previously defined in other registers
    #[inline(always)]
    pub fn register_set(self) -> &'a mut crate::W<REG> {
        self.variant(TXMODE::RegisterSet)
    }
    ///Cable Reset sequence
    #[inline(always)]
    pub fn cable_reset(self) -> &'a mut crate::W<REG> {
        self.variant(TXMODE::CableReset)
    }
    ///BIST test sequence (BIST Carrier Mode 2)
    #[inline(always)]
    pub fn bisttest(self) -> &'a mut crate::W<REG> {
        self.variant(TXMODE::Bisttest)
    }
}
/**TXSEND

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSEND {
    ///0: No effect
    NoEffect = 0,
    ///1: Start Tx packet transmission
    Start = 1,
}
impl From<TXSEND> for bool {
    #[inline(always)]
    fn from(variant: TXSEND) -> Self {
        variant as u8 != 0
    }
}
///Field `TXSEND` reader - TXSEND
pub type TXSEND_R = crate::BitReader<TXSEND>;
impl TXSEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXSEND {
        match self.bits {
            false => TXSEND::NoEffect,
            true => TXSEND::Start,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TXSEND::NoEffect
    }
    ///Start Tx packet transmission
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == TXSEND::Start
    }
}
///Field `TXSEND` writer - TXSEND
pub type TXSEND_W<'a, REG> = crate::BitWriter<'a, REG, TXSEND>;
impl<'a, REG> TXSEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TXSEND::NoEffect)
    }
    ///Start Tx packet transmission
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(TXSEND::Start)
    }
}
/**TXHRST

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXHRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Start Tx Hard Reset message
    Start = 1,
}
impl From<TXHRST> for bool {
    #[inline(always)]
    fn from(variant: TXHRST) -> Self {
        variant as u8 != 0
    }
}
///Field `TXHRST` reader - TXHRST
pub type TXHRST_R = crate::BitReader<TXHRST>;
impl TXHRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXHRST {
        match self.bits {
            false => TXHRST::NoEffect,
            true => TXHRST::Start,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TXHRST::NoEffect
    }
    ///Start Tx Hard Reset message
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == TXHRST::Start
    }
}
///Field `TXHRST` writer - TXHRST
pub type TXHRST_W<'a, REG> = crate::BitWriter<'a, REG, TXHRST>;
impl<'a, REG> TXHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TXHRST::NoEffect)
    }
    ///Start Tx Hard Reset message
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(TXHRST::Start)
    }
}
/**RXMODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXMODE {
    ///0: Normal receive mode
    Normal = 0,
    ///1: BIST receive mode (BIST test data mode)
    Bist = 1,
}
impl From<RXMODE> for bool {
    #[inline(always)]
    fn from(variant: RXMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXMODE` reader - RXMODE
pub type RXMODE_R = crate::BitReader<RXMODE>;
impl RXMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXMODE {
        match self.bits {
            false => RXMODE::Normal,
            true => RXMODE::Bist,
        }
    }
    ///Normal receive mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RXMODE::Normal
    }
    ///BIST receive mode (BIST test data mode)
    #[inline(always)]
    pub fn is_bist(&self) -> bool {
        *self == RXMODE::Bist
    }
}
///Field `RXMODE` writer - RXMODE
pub type RXMODE_W<'a, REG> = crate::BitWriter<'a, REG, RXMODE>;
impl<'a, REG> RXMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal receive mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RXMODE::Normal)
    }
    ///BIST receive mode (BIST test data mode)
    #[inline(always)]
    pub fn bist(self) -> &'a mut crate::W<REG> {
        self.variant(RXMODE::Bist)
    }
}
/**PHYRXEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYRXEN {
    ///0: USB Power Delivery receiver disabled
    Disabled = 0,
    ///1: USB Power Delivery receiver enabled
    Enabled = 1,
}
impl From<PHYRXEN> for bool {
    #[inline(always)]
    fn from(variant: PHYRXEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PHYRXEN` reader - PHYRXEN
pub type PHYRXEN_R = crate::BitReader<PHYRXEN>;
impl PHYRXEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PHYRXEN {
        match self.bits {
            false => PHYRXEN::Disabled,
            true => PHYRXEN::Enabled,
        }
    }
    ///USB Power Delivery receiver disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYRXEN::Disabled
    }
    ///USB Power Delivery receiver enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYRXEN::Enabled
    }
}
///Field `PHYRXEN` writer - PHYRXEN
pub type PHYRXEN_W<'a, REG> = crate::BitWriter<'a, REG, PHYRXEN>;
impl<'a, REG> PHYRXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB Power Delivery receiver disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PHYRXEN::Disabled)
    }
    ///USB Power Delivery receiver enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PHYRXEN::Enabled)
    }
}
/**PHYCCSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYCCSEL {
    ///0: Use CC1 IO for Power Delivery communication
    Cc1 = 0,
    ///1: Use CC2 IO for Power Delivery communication
    Cc2 = 1,
}
impl From<PHYCCSEL> for bool {
    #[inline(always)]
    fn from(variant: PHYCCSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `PHYCCSEL` reader - PHYCCSEL
pub type PHYCCSEL_R = crate::BitReader<PHYCCSEL>;
impl PHYCCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PHYCCSEL {
        match self.bits {
            false => PHYCCSEL::Cc1,
            true => PHYCCSEL::Cc2,
        }
    }
    ///Use CC1 IO for Power Delivery communication
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == PHYCCSEL::Cc1
    }
    ///Use CC2 IO for Power Delivery communication
    #[inline(always)]
    pub fn is_cc2(&self) -> bool {
        *self == PHYCCSEL::Cc2
    }
}
///Field `PHYCCSEL` writer - PHYCCSEL
pub type PHYCCSEL_W<'a, REG> = crate::BitWriter<'a, REG, PHYCCSEL>;
impl<'a, REG> PHYCCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use CC1 IO for Power Delivery communication
    #[inline(always)]
    pub fn cc1(self) -> &'a mut crate::W<REG> {
        self.variant(PHYCCSEL::Cc1)
    }
    ///Use CC2 IO for Power Delivery communication
    #[inline(always)]
    pub fn cc2(self) -> &'a mut crate::W<REG> {
        self.variant(PHYCCSEL::Cc2)
    }
}
/**ANASUBMODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANASUBMODE {
    ///0: Disabled
    Disabled = 0,
    ///1: Default USB Rp
    RpDefaultUsb = 1,
    ///2: 1.5A Rp
    Rp1_5a = 2,
    ///3: 3A Rp
    Rp3a = 3,
}
impl From<ANASUBMODE> for u8 {
    #[inline(always)]
    fn from(variant: ANASUBMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANASUBMODE {
    type Ux = u8;
}
impl crate::IsEnum for ANASUBMODE {}
///Field `ANASUBMODE` reader - ANASUBMODE
pub type ANASUBMODE_R = crate::FieldReader<ANASUBMODE>;
impl ANASUBMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANASUBMODE {
        match self.bits {
            0 => ANASUBMODE::Disabled,
            1 => ANASUBMODE::RpDefaultUsb,
            2 => ANASUBMODE::Rp1_5a,
            3 => ANASUBMODE::Rp3a,
            _ => unreachable!(),
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANASUBMODE::Disabled
    }
    ///Default USB Rp
    #[inline(always)]
    pub fn is_rp_default_usb(&self) -> bool {
        *self == ANASUBMODE::RpDefaultUsb
    }
    ///1.5A Rp
    #[inline(always)]
    pub fn is_rp_1_5a(&self) -> bool {
        *self == ANASUBMODE::Rp1_5a
    }
    ///3A Rp
    #[inline(always)]
    pub fn is_rp_3a(&self) -> bool {
        *self == ANASUBMODE::Rp3a
    }
}
///Field `ANASUBMODE` writer - ANASUBMODE
pub type ANASUBMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ANASUBMODE, crate::Safe>;
impl<'a, REG> ANASUBMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANASUBMODE::Disabled)
    }
    ///Default USB Rp
    #[inline(always)]
    pub fn rp_default_usb(self) -> &'a mut crate::W<REG> {
        self.variant(ANASUBMODE::RpDefaultUsb)
    }
    ///1.5A Rp
    #[inline(always)]
    pub fn rp_1_5a(self) -> &'a mut crate::W<REG> {
        self.variant(ANASUBMODE::Rp1_5a)
    }
    ///3A Rp
    #[inline(always)]
    pub fn rp_3a(self) -> &'a mut crate::W<REG> {
        self.variant(ANASUBMODE::Rp3a)
    }
}
/**ANAMODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANAMODE {
    ///0: Source
    Source = 0,
    ///1: Sink
    Sink = 1,
}
impl From<ANAMODE> for bool {
    #[inline(always)]
    fn from(variant: ANAMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `ANAMODE` reader - ANAMODE
pub type ANAMODE_R = crate::BitReader<ANAMODE>;
impl ANAMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANAMODE {
        match self.bits {
            false => ANAMODE::Source,
            true => ANAMODE::Sink,
        }
    }
    ///Source
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == ANAMODE::Source
    }
    ///Sink
    #[inline(always)]
    pub fn is_sink(&self) -> bool {
        *self == ANAMODE::Sink
    }
}
///Field `ANAMODE` writer - ANAMODE
pub type ANAMODE_W<'a, REG> = crate::BitWriter<'a, REG, ANAMODE>;
impl<'a, REG> ANAMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Source
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(ANAMODE::Source)
    }
    ///Sink
    #[inline(always)]
    pub fn sink(self) -> &'a mut crate::W<REG> {
        self.variant(ANAMODE::Sink)
    }
}
/**CCENABLE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCENABLE {
    ///0: Both PHYs disabled
    Disabled = 0,
    ///1: CC1 PHY enabled
    Cc1enabled = 1,
    ///2: CC2 PHY enabled
    Cc2enabled = 2,
    ///3: CC1 and CC2 PHYs enabled
    BothEnabled = 3,
}
impl From<CCENABLE> for u8 {
    #[inline(always)]
    fn from(variant: CCENABLE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCENABLE {
    type Ux = u8;
}
impl crate::IsEnum for CCENABLE {}
///Field `CCENABLE` reader - CCENABLE
pub type CCENABLE_R = crate::FieldReader<CCENABLE>;
impl CCENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCENABLE {
        match self.bits {
            0 => CCENABLE::Disabled,
            1 => CCENABLE::Cc1enabled,
            2 => CCENABLE::Cc2enabled,
            3 => CCENABLE::BothEnabled,
            _ => unreachable!(),
        }
    }
    ///Both PHYs disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCENABLE::Disabled
    }
    ///CC1 PHY enabled
    #[inline(always)]
    pub fn is_cc1enabled(&self) -> bool {
        *self == CCENABLE::Cc1enabled
    }
    ///CC2 PHY enabled
    #[inline(always)]
    pub fn is_cc2enabled(&self) -> bool {
        *self == CCENABLE::Cc2enabled
    }
    ///CC1 and CC2 PHYs enabled
    #[inline(always)]
    pub fn is_both_enabled(&self) -> bool {
        *self == CCENABLE::BothEnabled
    }
}
///Field `CCENABLE` writer - CCENABLE
pub type CCENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CCENABLE, crate::Safe>;
impl<'a, REG> CCENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Both PHYs disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCENABLE::Disabled)
    }
    ///CC1 PHY enabled
    #[inline(always)]
    pub fn cc1enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCENABLE::Cc1enabled)
    }
    ///CC2 PHY enabled
    #[inline(always)]
    pub fn cc2enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCENABLE::Cc2enabled)
    }
    ///CC1 and CC2 PHYs enabled
    #[inline(always)]
    pub fn both_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCENABLE::BothEnabled)
    }
}
/**FRSRXEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRSRXEN {
    ///0: FRS Rx event detection disabled
    Disabled = 0,
    ///1: FRS Rx event detection enabled
    Enabled = 1,
}
impl From<FRSRXEN> for bool {
    #[inline(always)]
    fn from(variant: FRSRXEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSRXEN` reader - FRSRXEN
pub type FRSRXEN_R = crate::BitReader<FRSRXEN>;
impl FRSRXEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRSRXEN {
        match self.bits {
            false => FRSRXEN::Disabled,
            true => FRSRXEN::Enabled,
        }
    }
    ///FRS Rx event detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRSRXEN::Disabled
    }
    ///FRS Rx event detection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRSRXEN::Enabled
    }
}
///Field `FRSRXEN` writer - FRSRXEN
pub type FRSRXEN_W<'a, REG> = crate::BitWriter<'a, REG, FRSRXEN>;
impl<'a, REG> FRSRXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FRS Rx event detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRSRXEN::Disabled)
    }
    ///FRS Rx event detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRSRXEN::Enabled)
    }
}
/**FRSTX

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRSTX {
    ///0: No effect
    NoEffect = 0,
    ///1: FRS Tx signaling enabled
    Enabled = 1,
}
impl From<FRSTX> for bool {
    #[inline(always)]
    fn from(variant: FRSTX) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSTX` reader - FRSTX
pub type FRSTX_R = crate::BitReader<FRSTX>;
impl FRSTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRSTX {
        match self.bits {
            false => FRSTX::NoEffect,
            true => FRSTX::Enabled,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FRSTX::NoEffect
    }
    ///FRS Tx signaling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRSTX::Enabled
    }
}
///Field `FRSTX` writer - FRSTX
pub type FRSTX_W<'a, REG> = crate::BitWriter<'a, REG, FRSTX>;
impl<'a, REG> FRSTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FRSTX::NoEffect)
    }
    ///FRS Tx signaling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRSTX::Enabled)
    }
}
/**RDCH

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDCH {
    ///0: No effect
    NoEffect = 0,
    ///1: Rdch condition drive
    ConditionDrive = 1,
}
impl From<RDCH> for bool {
    #[inline(always)]
    fn from(variant: RDCH) -> Self {
        variant as u8 != 0
    }
}
///Field `RDCH` reader - RDCH
pub type RDCH_R = crate::BitReader<RDCH>;
impl RDCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDCH {
        match self.bits {
            false => RDCH::NoEffect,
            true => RDCH::ConditionDrive,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RDCH::NoEffect
    }
    ///Rdch condition drive
    #[inline(always)]
    pub fn is_condition_drive(&self) -> bool {
        *self == RDCH::ConditionDrive
    }
}
///Field `RDCH` writer - RDCH
pub type RDCH_W<'a, REG> = crate::BitWriter<'a, REG, RDCH>;
impl<'a, REG> RDCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RDCH::NoEffect)
    }
    ///Rdch condition drive
    #[inline(always)]
    pub fn condition_drive(self) -> &'a mut crate::W<REG> {
        self.variant(RDCH::ConditionDrive)
    }
}
/**CC1TCDIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1TCDIS {
    ///0: Type-C detector on the CCx line enabled
    Enabled = 0,
    ///1: Type-C detector on the CCx line disabled
    Disabled = 1,
}
impl From<CC1TCDIS> for bool {
    #[inline(always)]
    fn from(variant: CC1TCDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1TCDIS` reader - CC1TCDIS
pub type CC1TCDIS_R = crate::BitReader<CC1TCDIS>;
impl CC1TCDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1TCDIS {
        match self.bits {
            false => CC1TCDIS::Enabled,
            true => CC1TCDIS::Disabled,
        }
    }
    ///Type-C detector on the CCx line enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1TCDIS::Enabled
    }
    ///Type-C detector on the CCx line disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1TCDIS::Disabled
    }
}
///Field `CC1TCDIS` writer - CC1TCDIS
pub type CC1TCDIS_W<'a, REG> = crate::BitWriter<'a, REG, CC1TCDIS>;
impl<'a, REG> CC1TCDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Type-C detector on the CCx line enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1TCDIS::Enabled)
    }
    ///Type-C detector on the CCx line disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1TCDIS::Disabled)
    }
}
///Field `CC2TCDIS` reader - CC2TCDIS
pub use CC1TCDIS_R as CC2TCDIS_R;
///Field `CC2TCDIS` writer - CC2TCDIS
pub use CC1TCDIS_W as CC2TCDIS_W;
impl R {
    ///Bits 0:1 - TXMODE
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - TXSEND
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXHRST
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RXMODE
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PHYRXEN
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PHYCCSEL
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - ANASUBMODE
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - ANAMODE
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - CCENABLE
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 16 - FRSRXEN
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FRSTX
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RDCH
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CC1TCDIS
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CC2TCDIS
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("txmode", &self.txmode())
            .field("txsend", &self.txsend())
            .field("txhrst", &self.txhrst())
            .field("rxmode", &self.rxmode())
            .field("phyrxen", &self.phyrxen())
            .field("phyccsel", &self.phyccsel())
            .field("anasubmode", &self.anasubmode())
            .field("anamode", &self.anamode())
            .field("ccenable", &self.ccenable())
            .field("frsrxen", &self.frsrxen())
            .field("frstx", &self.frstx())
            .field("rdch", &self.rdch())
            .field("cc1tcdis", &self.cc1tcdis())
            .field("cc2tcdis", &self.cc2tcdis())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TXMODE
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W<CRrs> {
        TXMODE_W::new(self, 0)
    }
    ///Bit 2 - TXSEND
    #[inline(always)]
    pub fn txsend(&mut self) -> TXSEND_W<CRrs> {
        TXSEND_W::new(self, 2)
    }
    ///Bit 3 - TXHRST
    #[inline(always)]
    pub fn txhrst(&mut self) -> TXHRST_W<CRrs> {
        TXHRST_W::new(self, 3)
    }
    ///Bit 4 - RXMODE
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W<CRrs> {
        RXMODE_W::new(self, 4)
    }
    ///Bit 5 - PHYRXEN
    #[inline(always)]
    pub fn phyrxen(&mut self) -> PHYRXEN_W<CRrs> {
        PHYRXEN_W::new(self, 5)
    }
    ///Bit 6 - PHYCCSEL
    #[inline(always)]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W<CRrs> {
        PHYCCSEL_W::new(self, 6)
    }
    ///Bits 7:8 - ANASUBMODE
    #[inline(always)]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W<CRrs> {
        ANASUBMODE_W::new(self, 7)
    }
    ///Bit 9 - ANAMODE
    #[inline(always)]
    pub fn anamode(&mut self) -> ANAMODE_W<CRrs> {
        ANAMODE_W::new(self, 9)
    }
    ///Bits 10:11 - CCENABLE
    #[inline(always)]
    pub fn ccenable(&mut self) -> CCENABLE_W<CRrs> {
        CCENABLE_W::new(self, 10)
    }
    ///Bit 16 - FRSRXEN
    #[inline(always)]
    pub fn frsrxen(&mut self) -> FRSRXEN_W<CRrs> {
        FRSRXEN_W::new(self, 16)
    }
    ///Bit 17 - FRSTX
    #[inline(always)]
    pub fn frstx(&mut self) -> FRSTX_W<CRrs> {
        FRSTX_W::new(self, 17)
    }
    ///Bit 18 - RDCH
    #[inline(always)]
    pub fn rdch(&mut self) -> RDCH_W<CRrs> {
        RDCH_W::new(self, 18)
    }
    ///Bit 20 - CC1TCDIS
    #[inline(always)]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W<CRrs> {
        CC1TCDIS_W::new(self, 20)
    }
    ///Bit 21 - CC2TCDIS
    #[inline(always)]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W<CRrs> {
        CC2TCDIS_W::new(self, 21)
    }
}
/**UCPD control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:CR)*/
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
