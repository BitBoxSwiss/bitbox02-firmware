///Register `BCR1` reader
pub type R = crate::R<BCR1rs>;
///Register `BCR1` writer
pub type W = crate::W<BCR1rs>;
/**Memory bank enable bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBKEN {
    ///0: Corresponding memory bank is disabled
    Disabled = 0,
    ///1: Corresponding memory bank is enabled
    Enabled = 1,
}
impl From<MBKEN> for bool {
    #[inline(always)]
    fn from(variant: MBKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MBKEN` reader - Memory bank enable bit
pub type MBKEN_R = crate::BitReader<MBKEN>;
impl MBKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MBKEN {
        match self.bits {
            false => MBKEN::Disabled,
            true => MBKEN::Enabled,
        }
    }
    ///Corresponding memory bank is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MBKEN::Disabled
    }
    ///Corresponding memory bank is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MBKEN::Enabled
    }
}
///Field `MBKEN` writer - Memory bank enable bit
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG, MBKEN>;
impl<'a, REG> MBKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Corresponding memory bank is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MBKEN::Disabled)
    }
    ///Corresponding memory bank is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MBKEN::Enabled)
    }
}
/**Address/data multiplexing enable bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUXEN {
    ///0: Address/Data non-multiplexed
    Disabled = 0,
    ///1: Address/Data multiplexed on databus
    Enabled = 1,
}
impl From<MUXEN> for bool {
    #[inline(always)]
    fn from(variant: MUXEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MUXEN` reader - Address/data multiplexing enable bit
pub type MUXEN_R = crate::BitReader<MUXEN>;
impl MUXEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MUXEN {
        match self.bits {
            false => MUXEN::Disabled,
            true => MUXEN::Enabled,
        }
    }
    ///Address/Data non-multiplexed
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUXEN::Disabled
    }
    ///Address/Data multiplexed on databus
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUXEN::Enabled
    }
}
///Field `MUXEN` writer - Address/data multiplexing enable bit
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG, MUXEN>;
impl<'a, REG> MUXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address/Data non-multiplexed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUXEN::Disabled)
    }
    ///Address/Data multiplexed on databus
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUXEN::Enabled)
    }
}
/**Memory type

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTYP {
    ///0: SRAM memory type
    Sram = 0,
    ///1: PSRAM (CRAM) memory type
    Psram = 1,
    ///2: NOR Flash/OneNAND Flash
    Flash = 2,
}
impl From<MTYP> for u8 {
    #[inline(always)]
    fn from(variant: MTYP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MTYP {
    type Ux = u8;
}
impl crate::IsEnum for MTYP {}
///Field `MTYP` reader - Memory type
pub type MTYP_R = crate::FieldReader<MTYP>;
impl MTYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MTYP> {
        match self.bits {
            0 => Some(MTYP::Sram),
            1 => Some(MTYP::Psram),
            2 => Some(MTYP::Flash),
            _ => None,
        }
    }
    ///SRAM memory type
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MTYP::Sram
    }
    ///PSRAM (CRAM) memory type
    #[inline(always)]
    pub fn is_psram(&self) -> bool {
        *self == MTYP::Psram
    }
    ///NOR Flash/OneNAND Flash
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == MTYP::Flash
    }
}
///Field `MTYP` writer - Memory type
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MTYP>;
impl<'a, REG> MTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SRAM memory type
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::Sram)
    }
    ///PSRAM (CRAM) memory type
    #[inline(always)]
    pub fn psram(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::Psram)
    }
    ///NOR Flash/OneNAND Flash
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::Flash)
    }
}
/**Memory data bus width

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MWID {
    ///0: Memory data bus width 8 bits
    Bits8 = 0,
    ///1: Memory data bus width 16 bits
    Bits16 = 1,
    ///2: Memory data bus width 32 bits
    Bits32 = 2,
}
impl From<MWID> for u8 {
    #[inline(always)]
    fn from(variant: MWID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MWID {
    type Ux = u8;
}
impl crate::IsEnum for MWID {}
///Field `MWID` reader - Memory data bus width
pub type MWID_R = crate::FieldReader<MWID>;
impl MWID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MWID> {
        match self.bits {
            0 => Some(MWID::Bits8),
            1 => Some(MWID::Bits16),
            2 => Some(MWID::Bits32),
            _ => None,
        }
    }
    ///Memory data bus width 8 bits
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID::Bits8
    }
    ///Memory data bus width 16 bits
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID::Bits16
    }
    ///Memory data bus width 32 bits
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID::Bits32
    }
}
///Field `MWID` writer - Memory data bus width
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MWID>;
impl<'a, REG> MWID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Memory data bus width 8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits8)
    }
    ///Memory data bus width 16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits16)
    }
    ///Memory data bus width 32 bits
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits32)
    }
}
/**Flash access enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FACCEN {
    ///0: Corresponding NOR Flash memory access is disabled
    Disabled = 0,
    ///1: Corresponding NOR Flash memory access is enabled
    Enabled = 1,
}
impl From<FACCEN> for bool {
    #[inline(always)]
    fn from(variant: FACCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FACCEN` reader - Flash access enable
pub type FACCEN_R = crate::BitReader<FACCEN>;
impl FACCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FACCEN {
        match self.bits {
            false => FACCEN::Disabled,
            true => FACCEN::Enabled,
        }
    }
    ///Corresponding NOR Flash memory access is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FACCEN::Disabled
    }
    ///Corresponding NOR Flash memory access is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FACCEN::Enabled
    }
}
///Field `FACCEN` writer - Flash access enable
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG, FACCEN>;
impl<'a, REG> FACCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Corresponding NOR Flash memory access is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FACCEN::Disabled)
    }
    ///Corresponding NOR Flash memory access is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FACCEN::Enabled)
    }
}
/**Burst enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BURSTEN {
    ///0: Burst mode disabled
    Disabled = 0,
    ///1: Burst mode enabled
    Enabled = 1,
}
impl From<BURSTEN> for bool {
    #[inline(always)]
    fn from(variant: BURSTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BURSTEN` reader - Burst enable bit
pub type BURSTEN_R = crate::BitReader<BURSTEN>;
impl BURSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BURSTEN {
        match self.bits {
            false => BURSTEN::Disabled,
            true => BURSTEN::Enabled,
        }
    }
    ///Burst mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BURSTEN::Disabled
    }
    ///Burst mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BURSTEN::Enabled
    }
}
///Field `BURSTEN` writer - Burst enable bit
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BURSTEN>;
impl<'a, REG> BURSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Burst mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTEN::Disabled)
    }
    ///Burst mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTEN::Enabled)
    }
}
/**Wait signal polarity bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITPOL {
    ///0: NWAIT active low
    ActiveLow = 0,
    ///1: NWAIT active high
    ActiveHigh = 1,
}
impl From<WAITPOL> for bool {
    #[inline(always)]
    fn from(variant: WAITPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `WAITPOL` reader - Wait signal polarity bit
pub type WAITPOL_R = crate::BitReader<WAITPOL>;
impl WAITPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAITPOL {
        match self.bits {
            false => WAITPOL::ActiveLow,
            true => WAITPOL::ActiveHigh,
        }
    }
    ///NWAIT active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == WAITPOL::ActiveLow
    }
    ///NWAIT active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == WAITPOL::ActiveHigh
    }
}
///Field `WAITPOL` writer - Wait signal polarity bit
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG, WAITPOL>;
impl<'a, REG> WAITPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NWAIT active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPOL::ActiveLow)
    }
    ///NWAIT active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPOL::ActiveHigh)
    }
}
/**Wait timing configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITCFG {
    ///0: NWAIT signal is active one data cycle before wait state
    BeforeWaitState = 0,
    ///1: NWAIT signal is active during wait state
    DuringWaitState = 1,
}
impl From<WAITCFG> for bool {
    #[inline(always)]
    fn from(variant: WAITCFG) -> Self {
        variant as u8 != 0
    }
}
///Field `WAITCFG` reader - Wait timing configuration
pub type WAITCFG_R = crate::BitReader<WAITCFG>;
impl WAITCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAITCFG {
        match self.bits {
            false => WAITCFG::BeforeWaitState,
            true => WAITCFG::DuringWaitState,
        }
    }
    ///NWAIT signal is active one data cycle before wait state
    #[inline(always)]
    pub fn is_before_wait_state(&self) -> bool {
        *self == WAITCFG::BeforeWaitState
    }
    ///NWAIT signal is active during wait state
    #[inline(always)]
    pub fn is_during_wait_state(&self) -> bool {
        *self == WAITCFG::DuringWaitState
    }
}
///Field `WAITCFG` writer - Wait timing configuration
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG, WAITCFG>;
impl<'a, REG> WAITCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NWAIT signal is active one data cycle before wait state
    #[inline(always)]
    pub fn before_wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(WAITCFG::BeforeWaitState)
    }
    ///NWAIT signal is active during wait state
    #[inline(always)]
    pub fn during_wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(WAITCFG::DuringWaitState)
    }
}
/**Write enable bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WREN {
    ///0: Write operations disabled for the bank by the FMC
    Disabled = 0,
    ///1: Write operations enabled for the bank by the FMC
    Enabled = 1,
}
impl From<WREN> for bool {
    #[inline(always)]
    fn from(variant: WREN) -> Self {
        variant as u8 != 0
    }
}
///Field `WREN` reader - Write enable bit
pub type WREN_R = crate::BitReader<WREN>;
impl WREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WREN {
        match self.bits {
            false => WREN::Disabled,
            true => WREN::Enabled,
        }
    }
    ///Write operations disabled for the bank by the FMC
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WREN::Disabled
    }
    ///Write operations enabled for the bank by the FMC
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WREN::Enabled
    }
}
///Field `WREN` writer - Write enable bit
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG, WREN>;
impl<'a, REG> WREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write operations disabled for the bank by the FMC
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WREN::Disabled)
    }
    ///Write operations enabled for the bank by the FMC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WREN::Enabled)
    }
}
/**Wait enable bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITEN {
    ///0: Values inside the FMC_BWTR are taken into account
    Disabled = 0,
    ///1: NWAIT signal enabled
    Enabled = 1,
}
impl From<WAITEN> for bool {
    #[inline(always)]
    fn from(variant: WAITEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WAITEN` reader - Wait enable bit
pub type WAITEN_R = crate::BitReader<WAITEN>;
impl WAITEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAITEN {
        match self.bits {
            false => WAITEN::Disabled,
            true => WAITEN::Enabled,
        }
    }
    ///Values inside the FMC_BWTR are taken into account
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITEN::Disabled
    }
    ///NWAIT signal enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITEN::Enabled
    }
}
///Field `WAITEN` writer - Wait enable bit
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG, WAITEN>;
impl<'a, REG> WAITEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Values inside the FMC_BWTR are taken into account
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITEN::Disabled)
    }
    ///NWAIT signal enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITEN::Enabled)
    }
}
/**Extended mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMOD {
    ///0: Values inside the FMC_BWTR are not taken into account
    Disabled = 0,
    ///1: Values inside the FMC_BWTR are taken into account
    Enabled = 1,
}
impl From<EXTMOD> for bool {
    #[inline(always)]
    fn from(variant: EXTMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTMOD` reader - Extended mode enable
pub type EXTMOD_R = crate::BitReader<EXTMOD>;
impl EXTMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTMOD {
        match self.bits {
            false => EXTMOD::Disabled,
            true => EXTMOD::Enabled,
        }
    }
    ///Values inside the FMC_BWTR are not taken into account
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTMOD::Disabled
    }
    ///Values inside the FMC_BWTR are taken into account
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTMOD::Enabled
    }
}
///Field `EXTMOD` writer - Extended mode enable
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG, EXTMOD>;
impl<'a, REG> EXTMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Values inside the FMC_BWTR are not taken into account
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMOD::Disabled)
    }
    ///Values inside the FMC_BWTR are taken into account
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMOD::Enabled)
    }
}
/**Wait signal during asynchronous transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASYNCWAIT {
    ///0: Wait signal not used in asynchronous mode
    Disabled = 0,
    ///1: Wait signal used even in asynchronous mode
    Enabled = 1,
}
impl From<ASYNCWAIT> for bool {
    #[inline(always)]
    fn from(variant: ASYNCWAIT) -> Self {
        variant as u8 != 0
    }
}
///Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers
pub type ASYNCWAIT_R = crate::BitReader<ASYNCWAIT>;
impl ASYNCWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASYNCWAIT {
        match self.bits {
            false => ASYNCWAIT::Disabled,
            true => ASYNCWAIT::Enabled,
        }
    }
    ///Wait signal not used in asynchronous mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASYNCWAIT::Disabled
    }
    ///Wait signal used even in asynchronous mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASYNCWAIT::Enabled
    }
}
///Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG, ASYNCWAIT>;
impl<'a, REG> ASYNCWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wait signal not used in asynchronous mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCWAIT::Disabled)
    }
    ///Wait signal used even in asynchronous mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCWAIT::Enabled)
    }
}
/**CRAM Page Size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPSIZE {
    ///0: No burst split when crossing page boundary
    NoBurstSplit = 0,
    ///1: 128 bytes CRAM page size
    Bytes128 = 1,
    ///2: 256 bytes CRAM page size
    Bytes256 = 2,
    ///3: 512 bytes CRAM page size
    Bytes512 = 3,
    ///4: 1024 bytes CRAM page size
    Bytes1024 = 4,
}
impl From<CPSIZE> for u8 {
    #[inline(always)]
    fn from(variant: CPSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CPSIZE {
    type Ux = u8;
}
impl crate::IsEnum for CPSIZE {}
///Field `CPSIZE` reader - CRAM Page Size
pub type CPSIZE_R = crate::FieldReader<CPSIZE>;
impl CPSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CPSIZE> {
        match self.bits {
            0 => Some(CPSIZE::NoBurstSplit),
            1 => Some(CPSIZE::Bytes128),
            2 => Some(CPSIZE::Bytes256),
            3 => Some(CPSIZE::Bytes512),
            4 => Some(CPSIZE::Bytes1024),
            _ => None,
        }
    }
    ///No burst split when crossing page boundary
    #[inline(always)]
    pub fn is_no_burst_split(&self) -> bool {
        *self == CPSIZE::NoBurstSplit
    }
    ///128 bytes CRAM page size
    #[inline(always)]
    pub fn is_bytes128(&self) -> bool {
        *self == CPSIZE::Bytes128
    }
    ///256 bytes CRAM page size
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == CPSIZE::Bytes256
    }
    ///512 bytes CRAM page size
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == CPSIZE::Bytes512
    }
    ///1024 bytes CRAM page size
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == CPSIZE::Bytes1024
    }
}
///Field `CPSIZE` writer - CRAM Page Size
pub type CPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CPSIZE>;
impl<'a, REG> CPSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No burst split when crossing page boundary
    #[inline(always)]
    pub fn no_burst_split(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::NoBurstSplit)
    }
    ///128 bytes CRAM page size
    #[inline(always)]
    pub fn bytes128(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes128)
    }
    ///256 bytes CRAM page size
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes256)
    }
    ///512 bytes CRAM page size
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes512)
    }
    ///1024 bytes CRAM page size
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes1024)
    }
}
/**Write burst enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBURSTRW {
    ///0: Write operations are always performed in asynchronous mode
    Disabled = 0,
    ///1: Write operations are performed in synchronous mode
    Enabled = 1,
}
impl From<CBURSTRW> for bool {
    #[inline(always)]
    fn from(variant: CBURSTRW) -> Self {
        variant as u8 != 0
    }
}
///Field `CBURSTRW` reader - Write burst enable
pub type CBURSTRW_R = crate::BitReader<CBURSTRW>;
impl CBURSTRW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CBURSTRW {
        match self.bits {
            false => CBURSTRW::Disabled,
            true => CBURSTRW::Enabled,
        }
    }
    ///Write operations are always performed in asynchronous mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBURSTRW::Disabled
    }
    ///Write operations are performed in synchronous mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBURSTRW::Enabled
    }
}
///Field `CBURSTRW` writer - Write burst enable
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG, CBURSTRW>;
impl<'a, REG> CBURSTRW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write operations are always performed in asynchronous mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CBURSTRW::Disabled)
    }
    ///Write operations are performed in synchronous mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CBURSTRW::Enabled)
    }
}
///Field `CCLKEN` reader - Continuous clock enable
pub type CCLKEN_R = crate::BitReader;
///Field `CCLKEN` writer - Continuous clock enable
pub type CCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFDIS` reader - Write FIFO disable
pub type WFDIS_R = crate::BitReader;
///Field `WFDIS` writer - Write FIFO disable
pub type WFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBLSET` reader - Byte lane (NBL) setup
pub type NBLSET_R = crate::FieldReader;
///Field `NBLSET` writer - Byte lane (NBL) setup
pub type NBLSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FMCEN` reader - FMC controller enable
pub type FMCEN_R = crate::BitReader;
///Field `FMCEN` writer - FMC controller enable
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Memory bank enable bit
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Flash access enable
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM Page Size
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Continuous clock enable
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Write FIFO disable
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 31 - FMC controller enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR1")
            .field("mbken", &self.mbken())
            .field("muxen", &self.muxen())
            .field("mtyp", &self.mtyp())
            .field("mwid", &self.mwid())
            .field("faccen", &self.faccen())
            .field("bursten", &self.bursten())
            .field("waitpol", &self.waitpol())
            .field("waitcfg", &self.waitcfg())
            .field("wren", &self.wren())
            .field("waiten", &self.waiten())
            .field("extmod", &self.extmod())
            .field("asyncwait", &self.asyncwait())
            .field("cpsize", &self.cpsize())
            .field("cburstrw", &self.cburstrw())
            .field("cclken", &self.cclken())
            .field("wfdis", &self.wfdis())
            .field("nblset", &self.nblset())
            .field("fmcen", &self.fmcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Memory bank enable bit
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<BCR1rs> {
        MBKEN_W::new(self, 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<BCR1rs> {
        MUXEN_W::new(self, 1)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<BCR1rs> {
        MTYP_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<BCR1rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Flash access enable
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<BCR1rs> {
        FACCEN_W::new(self, 6)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR1rs> {
        BURSTEN_W::new(self, 8)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR1rs> {
        WAITPOL_W::new(self, 9)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR1rs> {
        WAITCFG_W::new(self, 11)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<BCR1rs> {
        WREN_W::new(self, 12)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<BCR1rs> {
        WAITEN_W::new(self, 13)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR1rs> {
        EXTMOD_W::new(self, 14)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR1rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    ///Bits 16:18 - CRAM Page Size
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<BCR1rs> {
        CPSIZE_W::new(self, 16)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR1rs> {
        CBURSTRW_W::new(self, 19)
    }
    ///Bit 20 - Continuous clock enable
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<BCR1rs> {
        CCLKEN_W::new(self, 20)
    }
    ///Bit 21 - Write FIFO disable
    #[inline(always)]
    pub fn wfdis(&mut self) -> WFDIS_W<BCR1rs> {
        WFDIS_W::new(self, 21)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&mut self) -> NBLSET_W<BCR1rs> {
        NBLSET_W::new(self, 22)
    }
    ///Bit 31 - FMC controller enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<BCR1rs> {
        FMCEN_W::new(self, 31)
    }
}
/**SRAM/NOR-Flash chip-select control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FMC:BCR1)*/
pub struct BCR1rs;
impl crate::RegisterSpec for BCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bcr1::R`](R) reader structure
impl crate::Readable for BCR1rs {}
///`write(|w| ..)` method takes [`bcr1::W`](W) writer structure
impl crate::Writable for BCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR1 to value 0x30db
impl crate::Resettable for BCR1rs {
    const RESET_VALUE: u32 = 0x30db;
}
