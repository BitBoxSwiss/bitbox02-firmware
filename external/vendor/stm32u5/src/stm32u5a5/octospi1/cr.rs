///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: OCTOSPI disabled
    Disabled = 0,
    ///1: OCTOSPI enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///OCTOSPI disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///OCTOSPI enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OCTOSPI disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///OCTOSPI enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Abort request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT {
    ///0: No abort requested
    NotRequested = 0,
    ///1: Abort requested
    Requested = 1,
}
impl From<ABORT> for bool {
    #[inline(always)]
    fn from(variant: ABORT) -> Self {
        variant as u8 != 0
    }
}
///Field `ABORT` reader - Abort request
pub type ABORT_R = crate::BitReader<ABORT>;
impl ABORT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABORT {
        match self.bits {
            false => ABORT::NotRequested,
            true => ABORT::Requested,
        }
    }
    ///No abort requested
    #[inline(always)]
    pub fn is_not_requested(&self) -> bool {
        *self == ABORT::NotRequested
    }
    ///Abort requested
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == ABORT::Requested
    }
}
///Field `ABORT` writer - Abort request
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG, ABORT>;
impl<'a, REG> ABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No abort requested
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::NotRequested)
    }
    ///Abort requested
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::Requested)
    }
}
/**DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: DMA disabled for Indirect mode
    Disabled = 0,
    ///1: DMA enabled for Indirect mode
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA enable
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///DMA disabled for Indirect mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///DMA enabled for Indirect mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA disabled for Indirect mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///DMA enabled for Indirect mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
/**Timeout counter enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCEN {
    ///0: Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode
    Disabled = 0,
    ///1: Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\[15:0\] cycles of external device inactivity
    Enabled = 1,
}
impl From<TCEN> for bool {
    #[inline(always)]
    fn from(variant: TCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCEN` reader - Timeout counter enable
pub type TCEN_R = crate::BitReader<TCEN>;
impl TCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCEN {
        match self.bits {
            false => TCEN::Disabled,
            true => TCEN::Enabled,
        }
    }
    ///Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCEN::Disabled
    }
    ///Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\[15:0\] cycles of external device inactivity
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCEN::Enabled
    }
}
///Field `TCEN` writer - Timeout counter enable
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG, TCEN>;
impl<'a, REG> TCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCEN::Disabled)
    }
    ///Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\[15:0\] cycles of external device inactivity
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCEN::Enabled)
    }
}
/**Dual-memory configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMM {
    ///0: Dual-memory configuration disabled
    Disabled = 0,
    ///1: Dual-memory configuration enabled
    Enabled = 1,
}
impl From<DMM> for bool {
    #[inline(always)]
    fn from(variant: DMM) -> Self {
        variant as u8 != 0
    }
}
///Field `DMM` reader - Dual-memory configuration
pub type DMM_R = crate::BitReader<DMM>;
impl DMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMM {
        match self.bits {
            false => DMM::Disabled,
            true => DMM::Enabled,
        }
    }
    ///Dual-memory configuration disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMM::Disabled
    }
    ///Dual-memory configuration enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMM::Enabled
    }
}
///Field `DMM` writer - Dual-memory configuration
pub type DMM_W<'a, REG> = crate::BitWriter<'a, REG, DMM>;
impl<'a, REG> DMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Dual-memory configuration disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMM::Disabled)
    }
    ///Dual-memory configuration enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMM::Enabled)
    }
}
/**External memory select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSEL {
    ///0: External memory 1 selected (data exchanged over IO\[3:0\])
    Ext1 = 0,
    ///1: External memory 2 selected (data exchanged over IO\[7:4\])
    Ext2 = 1,
}
impl From<MSEL> for bool {
    #[inline(always)]
    fn from(variant: MSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `MSEL` reader - External memory select
pub type MSEL_R = crate::BitReader<MSEL>;
impl MSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSEL {
        match self.bits {
            false => MSEL::Ext1,
            true => MSEL::Ext2,
        }
    }
    ///External memory 1 selected (data exchanged over IO\[3:0\])
    #[inline(always)]
    pub fn is_ext1(&self) -> bool {
        *self == MSEL::Ext1
    }
    ///External memory 2 selected (data exchanged over IO\[7:4\])
    #[inline(always)]
    pub fn is_ext2(&self) -> bool {
        *self == MSEL::Ext2
    }
}
///Field `MSEL` writer - External memory select
pub type MSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSEL>;
impl<'a, REG> MSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External memory 1 selected (data exchanged over IO\[3:0\])
    #[inline(always)]
    pub fn ext1(self) -> &'a mut crate::W<REG> {
        self.variant(MSEL::Ext1)
    }
    ///External memory 2 selected (data exchanged over IO\[7:4\])
    #[inline(always)]
    pub fn ext2(self) -> &'a mut crate::W<REG> {
        self.variant(MSEL::Ext2)
    }
}
///Field `FTHRES` reader - FIFO threshold level
pub type FTHRES_R = crate::FieldReader;
///Field `FTHRES` writer - FIFO threshold level
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Transfer error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::Disabled,
            true => TEIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable
pub use TEIE_R as TCIE_R;
///Field `FTIE` reader - FIFO threshold interrupt enable
pub use TEIE_R as FTIE_R;
///Field `SMIE` reader - Status match interrupt enable
pub use TEIE_R as SMIE_R;
///Field `TOIE` reader - TimeOut interrupt enable
pub use TEIE_R as TOIE_R;
///Field `TCIE` writer - Transfer complete interrupt enable
pub use TEIE_W as TCIE_W;
///Field `FTIE` writer - FIFO threshold interrupt enable
pub use TEIE_W as FTIE_W;
///Field `SMIE` writer - Status match interrupt enable
pub use TEIE_W as SMIE_W;
///Field `TOIE` writer - TimeOut interrupt enable
pub use TEIE_W as TOIE_W;
/**Automatic poll mode stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APMS {
    ///0: Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI
    Running = 0,
    ///1: Automatic status-polling mode stops as soon as there is a match
    StopMatch = 1,
}
impl From<APMS> for bool {
    #[inline(always)]
    fn from(variant: APMS) -> Self {
        variant as u8 != 0
    }
}
///Field `APMS` reader - Automatic poll mode stop
pub type APMS_R = crate::BitReader<APMS>;
impl APMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APMS {
        match self.bits {
            false => APMS::Running,
            true => APMS::StopMatch,
        }
    }
    ///Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == APMS::Running
    }
    ///Automatic status-polling mode stops as soon as there is a match
    #[inline(always)]
    pub fn is_stop_match(&self) -> bool {
        *self == APMS::StopMatch
    }
}
///Field `APMS` writer - Automatic poll mode stop
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG, APMS>;
impl<'a, REG> APMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(APMS::Running)
    }
    ///Automatic status-polling mode stops as soon as there is a match
    #[inline(always)]
    pub fn stop_match(self) -> &'a mut crate::W<REG> {
        self.variant(APMS::StopMatch)
    }
}
/**Polling match mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMM {
    ///0: AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register
    AndmatchMode = 0,
    ///1: OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register
    Ormatchmode = 1,
}
impl From<PMM> for bool {
    #[inline(always)]
    fn from(variant: PMM) -> Self {
        variant as u8 != 0
    }
}
///Field `PMM` reader - Polling match mode
pub type PMM_R = crate::BitReader<PMM>;
impl PMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMM {
        match self.bits {
            false => PMM::AndmatchMode,
            true => PMM::Ormatchmode,
        }
    }
    ///AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register
    #[inline(always)]
    pub fn is_andmatch_mode(&self) -> bool {
        *self == PMM::AndmatchMode
    }
    ///OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register
    #[inline(always)]
    pub fn is_ormatchmode(&self) -> bool {
        *self == PMM::Ormatchmode
    }
}
///Field `PMM` writer - Polling match mode
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG, PMM>;
impl<'a, REG> PMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register
    #[inline(always)]
    pub fn andmatch_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PMM::AndmatchMode)
    }
    ///OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register
    #[inline(always)]
    pub fn ormatchmode(self) -> &'a mut crate::W<REG> {
        self.variant(PMM::Ormatchmode)
    }
}
/**Functional mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMODE {
    ///0: Indirect-write mode
    IndirectWrite = 0,
    ///1: Indirect-read mode
    IndirectRead = 1,
    ///2: Automatic status-polling mode
    AutomaticPolling = 2,
    ///3: Memory-mapped mode
    MemoryMapped = 3,
}
impl From<FMODE> for u8 {
    #[inline(always)]
    fn from(variant: FMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMODE {
    type Ux = u8;
}
impl crate::IsEnum for FMODE {}
///Field `FMODE` reader - Functional mode
pub type FMODE_R = crate::FieldReader<FMODE>;
impl FMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMODE {
        match self.bits {
            0 => FMODE::IndirectWrite,
            1 => FMODE::IndirectRead,
            2 => FMODE::AutomaticPolling,
            3 => FMODE::MemoryMapped,
            _ => unreachable!(),
        }
    }
    ///Indirect-write mode
    #[inline(always)]
    pub fn is_indirect_write(&self) -> bool {
        *self == FMODE::IndirectWrite
    }
    ///Indirect-read mode
    #[inline(always)]
    pub fn is_indirect_read(&self) -> bool {
        *self == FMODE::IndirectRead
    }
    ///Automatic status-polling mode
    #[inline(always)]
    pub fn is_automatic_polling(&self) -> bool {
        *self == FMODE::AutomaticPolling
    }
    ///Memory-mapped mode
    #[inline(always)]
    pub fn is_memory_mapped(&self) -> bool {
        *self == FMODE::MemoryMapped
    }
}
///Field `FMODE` writer - Functional mode
pub type FMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMODE, crate::Safe>;
impl<'a, REG> FMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Indirect-write mode
    #[inline(always)]
    pub fn indirect_write(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::IndirectWrite)
    }
    ///Indirect-read mode
    #[inline(always)]
    pub fn indirect_read(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::IndirectRead)
    }
    ///Automatic status-polling mode
    #[inline(always)]
    pub fn automatic_polling(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::AutomaticPolling)
    }
    ///Memory-mapped mode
    #[inline(always)]
    pub fn memory_mapped(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::MemoryMapped)
    }
}
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Dual-memory configuration
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - External memory select
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - FIFO threshold level
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 28:29 - Functional mode
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("fmode", &self.fmode())
            .field("pmm", &self.pmm())
            .field("apms", &self.apms())
            .field("teie", &self.teie())
            .field("toie", &self.toie())
            .field("smie", &self.smie())
            .field("ftie", &self.ftie())
            .field("tcie", &self.tcie())
            .field("fthres", &self.fthres())
            .field("msel", &self.msel())
            .field("dmm", &self.dmm())
            .field("tcen", &self.tcen())
            .field("dmaen", &self.dmaen())
            .field("abort", &self.abort())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 1)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 2)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<CRrs> {
        TCEN_W::new(self, 3)
    }
    ///Bit 6 - Dual-memory configuration
    #[inline(always)]
    pub fn dmm(&mut self) -> DMM_W<CRrs> {
        DMM_W::new(self, 6)
    }
    ///Bit 7 - External memory select
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<CRrs> {
        MSEL_W::new(self, 7)
    }
    ///Bits 8:12 - FIFO threshold level
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W<CRrs> {
        FTHRES_W::new(self, 8)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 16)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 17)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W<CRrs> {
        FTIE_W::new(self, 18)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W<CRrs> {
        SMIE_W::new(self, 19)
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<CRrs> {
        TOIE_W::new(self, 20)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W<CRrs> {
        APMS_W::new(self, 22)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W<CRrs> {
        PMM_W::new(self, 23)
    }
    ///Bits 28:29 - Functional mode
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W<CRrs> {
        FMODE_W::new(self, 28)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OCTOSPI1:CR)*/
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
