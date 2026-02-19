///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Update interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFR {
    ///0: No update occurred
    NoUpdateOccurred = 0,
    ///1: Update interrupt pending
    UpdatePending = 1,
}
impl From<UIFR> for bool {
    #[inline(always)]
    fn from(variant: UIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` reader - Update interrupt flag
pub type UIF_R = crate::BitReader<UIFR>;
impl UIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIFR {
        match self.bits {
            false => UIFR::NoUpdateOccurred,
            true => UIFR::UpdatePending,
        }
    }
    ///No update occurred
    #[inline(always)]
    pub fn is_no_update_occurred(&self) -> bool {
        *self == UIFR::NoUpdateOccurred
    }
    ///Update interrupt pending
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFR::UpdatePending
    }
}
/**Update interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<UIFW> for bool {
    #[inline(always)]
    fn from(variant: UIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` writer - Update interrupt flag
pub type UIF_W<'a, REG> = crate::BitWriter0C<'a, REG, UIFW>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UIFW::Clear)
    }
}
/**Capture/compare %s interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFR {
    ///0: No campture/compare has been detected
    NoMatch = 0,
    ///1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
    Match = 1,
}
impl From<CC1IFR> for bool {
    #[inline(always)]
    fn from(variant: CC1IFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CCIF(1-4)` reader - Capture/compare %s interrupt flag
pub type CCIF_R = crate::BitReader<CC1IFR>;
impl CCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IFR {
        match self.bits {
            false => CC1IFR::NoMatch,
            true => CC1IFR::Match,
        }
    }
    ///No campture/compare has been detected
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CC1IFR::NoMatch
    }
    ///If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IFR::Match
    }
}
/**Capture/compare %s interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CC1IFW> for bool {
    #[inline(always)]
    fn from(variant: CC1IFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCIF(1-4)` writer - Capture/compare %s interrupt flag
pub type CCIF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1IFW>;
impl<'a, REG> CCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IFW::Clear)
    }
}
/**COM interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIFR {
    ///0: No COM event occurred
    NoCom = 0,
    ///1: COM interrupt pending
    Com = 1,
}
impl From<COMIFR> for bool {
    #[inline(always)]
    fn from(variant: COMIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` reader - COM interrupt flag
pub type COMIF_R = crate::BitReader<COMIFR>;
impl COMIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMIFR {
        match self.bits {
            false => COMIFR::NoCom,
            true => COMIFR::Com,
        }
    }
    ///No COM event occurred
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        *self == COMIFR::NoCom
    }
    ///COM interrupt pending
    #[inline(always)]
    pub fn is_com(&self) -> bool {
        *self == COMIFR::Com
    }
}
/**COM interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<COMIFW> for bool {
    #[inline(always)]
    fn from(variant: COMIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` writer - COM interrupt flag
pub type COMIF_W<'a, REG> = crate::BitWriter0C<'a, REG, COMIFW>;
impl<'a, REG> COMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COMIFW::Clear)
    }
}
/**Trigger interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFR {
    ///0: No trigger event occurred
    NoTrigger = 0,
    ///1: Trigger interrupt pending
    Trigger = 1,
}
impl From<TIFR> for bool {
    #[inline(always)]
    fn from(variant: TIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `TIF` reader - Trigger interrupt flag
pub type TIF_R = crate::BitReader<TIFR>;
impl TIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIFR {
        match self.bits {
            false => TIFR::NoTrigger,
            true => TIFR::Trigger,
        }
    }
    ///No trigger event occurred
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TIFR::NoTrigger
    }
    ///Trigger interrupt pending
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TIFR::Trigger
    }
}
/**Trigger interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<TIFW> for bool {
    #[inline(always)]
    fn from(variant: TIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `TIF` writer - Trigger interrupt flag
pub type TIF_W<'a, REG> = crate::BitWriter0C<'a, REG, TIFW>;
impl<'a, REG> TIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIFW::Clear)
    }
}
/**Break interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIFR {
    ///0: No break event occurred
    NoTrigger = 0,
    ///1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register
    Trigger = 1,
}
impl From<BIFR> for bool {
    #[inline(always)]
    fn from(variant: BIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` reader - Break interrupt flag
pub type BIF_R = crate::BitReader<BIFR>;
impl BIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIFR {
        match self.bits {
            false => BIFR::NoTrigger,
            true => BIFR::Trigger,
        }
    }
    ///No break event occurred
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == BIFR::NoTrigger
    }
    ///An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == BIFR::Trigger
    }
}
/**Break interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<BIFW> for bool {
    #[inline(always)]
    fn from(variant: BIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` writer - Break interrupt flag
pub type BIF_W<'a, REG> = crate::BitWriter0C<'a, REG, BIFW>;
impl<'a, REG> BIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BIFW::Clear)
    }
}
/**Break 2 interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2IFR {
    ///0: No break event occurred
    NoTrigger = 0,
    ///1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register
    Trigger = 1,
}
impl From<B2IFR> for bool {
    #[inline(always)]
    fn from(variant: B2IFR) -> Self {
        variant as u8 != 0
    }
}
///Field `B2IF` reader - Break 2 interrupt flag
pub type B2IF_R = crate::BitReader<B2IFR>;
impl B2IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> B2IFR {
        match self.bits {
            false => B2IFR::NoTrigger,
            true => B2IFR::Trigger,
        }
    }
    ///No break event occurred
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == B2IFR::NoTrigger
    }
    ///An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == B2IFR::Trigger
    }
}
/**Break 2 interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2IFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<B2IFW> for bool {
    #[inline(always)]
    fn from(variant: B2IFW) -> Self {
        variant as u8 != 0
    }
}
///Field `B2IF` writer - Break 2 interrupt flag
pub type B2IF_W<'a, REG> = crate::BitWriter0C<'a, REG, B2IFW>;
impl<'a, REG> B2IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(B2IFW::Clear)
    }
}
/**Capture/Compare %s overcapture flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFR {
    ///0: No overcapture has been detected
    NoOvercapture = 0,
    ///1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
    Overcapture = 1,
}
impl From<CC1OFR> for bool {
    #[inline(always)]
    fn from(variant: CC1OFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CCOF(1-4)` reader - Capture/Compare %s overcapture flag
pub type CCOF_R = crate::BitReader<CC1OFR>;
impl CCOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1OFR {
        match self.bits {
            false => CC1OFR::NoOvercapture,
            true => CC1OFR::Overcapture,
        }
    }
    ///No overcapture has been detected
    #[inline(always)]
    pub fn is_no_overcapture(&self) -> bool {
        *self == CC1OFR::NoOvercapture
    }
    ///The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OFR::Overcapture
    }
}
/**Capture/Compare %s overcapture flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CC1OFW> for bool {
    #[inline(always)]
    fn from(variant: CC1OFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCOF(1-4)` writer - Capture/Compare %s overcapture flag
pub type CCOF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1OFW>;
impl<'a, REG> CCOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OFW::Clear)
    }
}
/**System Break interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBIFR {
    ///0: No break event occurred
    NoTrigger = 0,
    ///1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register
    Trigger = 1,
}
impl From<SBIFR> for bool {
    #[inline(always)]
    fn from(variant: SBIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SBIF` reader - System Break interrupt flag
pub type SBIF_R = crate::BitReader<SBIFR>;
impl SBIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBIFR {
        match self.bits {
            false => SBIFR::NoTrigger,
            true => SBIFR::Trigger,
        }
    }
    ///No break event occurred
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SBIFR::NoTrigger
    }
    ///An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SBIFR::Trigger
    }
}
/**System Break interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBIFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SBIFW> for bool {
    #[inline(always)]
    fn from(variant: SBIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `SBIF` writer - System Break interrupt flag
pub type SBIF_W<'a, REG> = crate::BitWriter0C<'a, REG, SBIFW>;
impl<'a, REG> SBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SBIFW::Clear)
    }
}
///Field `CC5IF` reader - Compare 5 interrupt flag
pub use CCIF_R as CC5IF_R;
///Field `CC6IF` reader - Compare 6 interrupt flag
pub use CCIF_R as CC6IF_R;
///Field `CC5IF` writer - Compare 5 interrupt flag
pub use CCIF_W as CC5IF_W;
///Field `CC6IF` writer - Compare 6 interrupt flag
pub use CCIF_W as CC6IF_W;
///Field `IDXF` reader - Index interrupt flag
pub type IDXF_R = crate::BitReader;
///Field `IDXF` writer - Index interrupt flag
pub type IDXF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRF` reader - Direction change interrupt flag
pub type DIRF_R = crate::BitReader;
///Field `DIRF` writer - Direction change interrupt flag
pub type DIRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IERRF` reader - Index error interrupt flag
pub type IERRF_R = crate::BitReader;
///Field `IERRF` writer - Index error interrupt flag
pub type IERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERRF` reader - Transition error interrupt flag
pub type TERRF_R = crate::BitReader;
///Field `TERRF` writer - Transition error interrupt flag
pub type TERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Capture/compare (1-4) interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IF` field.</div>
    #[inline(always)]
    pub fn ccif(&self, n: u8) -> CCIF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIF_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/compare (1-4) interrupt flag
    #[inline(always)]
    pub fn ccif_iter(&self) -> impl Iterator<Item = CCIF_R> + '_ {
        (0..4).map(move |n| CCIF_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Break 2 interrupt flag
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Capture/Compare (1-4) overcapture flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1OF` field.</div>
    #[inline(always)]
    pub fn ccof(&self, n: u8) -> CCOF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCOF_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-4) overcapture flag
    #[inline(always)]
    pub fn ccof_iter(&self) -> impl Iterator<Item = CCOF_R> + '_ {
        (0..4).map(move |n| CCOF_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - System Break interrupt flag
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Compare 5 interrupt flag
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Compare 6 interrupt flag
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Index interrupt flag
    #[inline(always)]
    pub fn idxf(&self) -> IDXF_R {
        IDXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Direction change interrupt flag
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Index error interrupt flag
    #[inline(always)]
    pub fn ierrf(&self) -> IERRF_R {
        IERRF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Transition error interrupt flag
    #[inline(always)]
    pub fn terrf(&self) -> TERRF_R {
        TERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("terrf", &self.terrf())
            .field("ierrf", &self.ierrf())
            .field("dirf", &self.dirf())
            .field("idxf", &self.idxf())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("cc6if", &self.cc6if())
            .field("cc5if", &self.cc5if())
            .field("sbif", &self.sbif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .field("b2if", &self.b2if())
            .field("bif", &self.bif())
            .field("tif", &self.tif())
            .field("comif", &self.comif())
            .field("uif", &self.uif())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    ///Capture/compare (1-4) interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IF` field.</div>
    #[inline(always)]
    pub fn ccif(&mut self, n: u8) -> CCIF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIF_W::new(self, n + 1)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 4)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<SRrs> {
        COMIF_W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<SRrs> {
        BIF_W::new(self, 7)
    }
    ///Bit 8 - Break 2 interrupt flag
    #[inline(always)]
    pub fn b2if(&mut self) -> B2IF_W<SRrs> {
        B2IF_W::new(self, 8)
    }
    ///Capture/Compare (1-4) overcapture flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1OF` field.</div>
    #[inline(always)]
    pub fn ccof(&mut self, n: u8) -> CCOF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCOF_W::new(self, n + 9)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 12)
    }
    ///Bit 13 - System Break interrupt flag
    #[inline(always)]
    pub fn sbif(&mut self) -> SBIF_W<SRrs> {
        SBIF_W::new(self, 13)
    }
    ///Bit 16 - Compare 5 interrupt flag
    #[inline(always)]
    pub fn cc5if(&mut self) -> CC5IF_W<SRrs> {
        CC5IF_W::new(self, 16)
    }
    ///Bit 17 - Compare 6 interrupt flag
    #[inline(always)]
    pub fn cc6if(&mut self) -> CC6IF_W<SRrs> {
        CC6IF_W::new(self, 17)
    }
    ///Bit 20 - Index interrupt flag
    #[inline(always)]
    pub fn idxf(&mut self) -> IDXF_W<SRrs> {
        IDXF_W::new(self, 20)
    }
    ///Bit 21 - Direction change interrupt flag
    #[inline(always)]
    pub fn dirf(&mut self) -> DIRF_W<SRrs> {
        DIRF_W::new(self, 21)
    }
    ///Bit 22 - Index error interrupt flag
    #[inline(always)]
    pub fn ierrf(&mut self) -> IERRF_W<SRrs> {
        IERRF_W::new(self, 22)
    }
    ///Bit 23 - Transition error interrupt flag
    #[inline(always)]
    pub fn terrf(&mut self) -> TERRF_W<SRrs> {
        TERRF_W::new(self, 23)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TIM1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_3fff;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
