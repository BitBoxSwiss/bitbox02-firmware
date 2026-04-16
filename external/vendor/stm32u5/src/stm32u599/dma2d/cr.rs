///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    ///1: Launch the DMA2D
    Start = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<START>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<START> {
        match self.bits {
            true => Some(START::Start),
            _ => None,
        }
    }
    ///Launch the DMA2D
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START::Start
    }
}
///Field `START` writer - Start
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Launch the DMA2D
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START::Start)
    }
}
/**Suspend

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP {
    ///0: Transfer not suspended
    NotSuspended = 0,
    ///1: Transfer suspended
    Suspended = 1,
}
impl From<SUSP> for bool {
    #[inline(always)]
    fn from(variant: SUSP) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` reader - Suspend
pub type SUSP_R = crate::BitReader<SUSP>;
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSP {
        match self.bits {
            false => SUSP::NotSuspended,
            true => SUSP::Suspended,
        }
    }
    ///Transfer not suspended
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP::NotSuspended
    }
    ///Transfer suspended
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP::Suspended
    }
}
///Field `SUSP` writer - Suspend
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG, SUSP>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer not suspended
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::NotSuspended)
    }
    ///Transfer suspended
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::Suspended)
    }
}
/**Abort

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT {
    ///1: Transfer abort requested
    AbortRequest = 1,
}
impl From<ABORT> for bool {
    #[inline(always)]
    fn from(variant: ABORT) -> Self {
        variant as u8 != 0
    }
}
///Field `ABORT` reader - Abort
pub type ABORT_R = crate::BitReader<ABORT>;
impl ABORT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ABORT> {
        match self.bits {
            true => Some(ABORT::AbortRequest),
            _ => None,
        }
    }
    ///Transfer abort requested
    #[inline(always)]
    pub fn is_abort_request(&self) -> bool {
        *self == ABORT::AbortRequest
    }
}
///Field `ABORT` writer - Abort
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG, ABORT>;
impl<'a, REG> ABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer abort requested
    #[inline(always)]
    pub fn abort_request(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::AbortRequest)
    }
}
///Field `LOM` reader - Line Offset Mode
pub type LOM_R = crate::BitReader;
///Field `LOM` writer - Line Offset Mode
pub type LOM_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Transfer error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: TE interrupt disabled
    Disabled = 0,
    ///1: TE interrupt enabled
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
    ///TE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    ///TE interrupt enabled
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
    ///TE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    ///TE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
/**Transfer complete interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: TC interrupt disabled
    Disabled = 0,
    ///1: TC interrupt enabled
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    ///TC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
/**Transfer watermark interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWIE {
    ///0: TW interrupt disabled
    Disabled = 0,
    ///1: TW interrupt enabled
    Enabled = 1,
}
impl From<TWIE> for bool {
    #[inline(always)]
    fn from(variant: TWIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TWIE` reader - Transfer watermark interrupt enable
pub type TWIE_R = crate::BitReader<TWIE>;
impl TWIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TWIE {
        match self.bits {
            false => TWIE::Disabled,
            true => TWIE::Enabled,
        }
    }
    ///TW interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TWIE::Disabled
    }
    ///TW interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TWIE::Enabled
    }
}
///Field `TWIE` writer - Transfer watermark interrupt enable
pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG, TWIE>;
impl<'a, REG> TWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TW interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TWIE::Disabled)
    }
    ///TW interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TWIE::Enabled)
    }
}
/**CLUT access error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEIE {
    ///0: CAE interrupt disabled
    Disabled = 0,
    ///1: CAE interrupt enabled
    Enabled = 1,
}
impl From<CAEIE> for bool {
    #[inline(always)]
    fn from(variant: CAEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CAEIE` reader - CLUT access error interrupt enable
pub type CAEIE_R = crate::BitReader<CAEIE>;
impl CAEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CAEIE {
        match self.bits {
            false => CAEIE::Disabled,
            true => CAEIE::Enabled,
        }
    }
    ///CAE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAEIE::Disabled
    }
    ///CAE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAEIE::Enabled
    }
}
///Field `CAEIE` writer - CLUT access error interrupt enable
pub type CAEIE_W<'a, REG> = crate::BitWriter<'a, REG, CAEIE>;
impl<'a, REG> CAEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAEIE::Disabled)
    }
    ///CAE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAEIE::Enabled)
    }
}
/**CLUT transfer complete interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIE {
    ///0: CTC interrupt disabled
    Disabled = 0,
    ///1: CTC interrupt enabled
    Enabled = 1,
}
impl From<CTCIE> for bool {
    #[inline(always)]
    fn from(variant: CTCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIE` reader - CLUT transfer complete interrupt enable
pub type CTCIE_R = crate::BitReader<CTCIE>;
impl CTCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTCIE {
        match self.bits {
            false => CTCIE::Disabled,
            true => CTCIE::Enabled,
        }
    }
    ///CTC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTCIE::Disabled
    }
    ///CTC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTCIE::Enabled
    }
}
///Field `CTCIE` writer - CLUT transfer complete interrupt enable
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG, CTCIE>;
impl<'a, REG> CTCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CTC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIE::Disabled)
    }
    ///CTC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIE::Enabled)
    }
}
/**Configuration Error Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIE {
    ///0: CE interrupt disabled
    Disabled = 0,
    ///1: CE interrupt enabled
    Enabled = 1,
}
impl From<CEIE> for bool {
    #[inline(always)]
    fn from(variant: CEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CEIE` reader - Configuration Error Interrupt Enable
pub type CEIE_R = crate::BitReader<CEIE>;
impl CEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEIE {
        match self.bits {
            false => CEIE::Disabled,
            true => CEIE::Enabled,
        }
    }
    ///CE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEIE::Disabled
    }
    ///CE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEIE::Enabled
    }
}
///Field `CEIE` writer - Configuration Error Interrupt Enable
pub type CEIE_W<'a, REG> = crate::BitWriter<'a, REG, CEIE>;
impl<'a, REG> CEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEIE::Disabled)
    }
    ///CE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEIE::Enabled)
    }
}
/**DMA2D mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Memory-to-memory (FG fetch only)
    MemoryToMemory = 0,
    ///1: Memory-to-memory with PFC (FG fetch only with FG PFC active)
    MemoryToMemoryPfc = 1,
    ///2: Memory-to-memory with blending (FG and BG fetch with PFC and blending)
    MemoryToMemoryPfcblending = 2,
    ///3: Register-to-memory
    RegisterToMemory = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE` reader - DMA2D mode
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::MemoryToMemory),
            1 => Some(MODE::MemoryToMemoryPfc),
            2 => Some(MODE::MemoryToMemoryPfcblending),
            3 => Some(MODE::RegisterToMemory),
            _ => None,
        }
    }
    ///Memory-to-memory (FG fetch only)
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == MODE::MemoryToMemory
    }
    ///Memory-to-memory with PFC (FG fetch only with FG PFC active)
    #[inline(always)]
    pub fn is_memory_to_memory_pfc(&self) -> bool {
        *self == MODE::MemoryToMemoryPfc
    }
    ///Memory-to-memory with blending (FG and BG fetch with PFC and blending)
    #[inline(always)]
    pub fn is_memory_to_memory_pfcblending(&self) -> bool {
        *self == MODE::MemoryToMemoryPfcblending
    }
    ///Register-to-memory
    #[inline(always)]
    pub fn is_register_to_memory(&self) -> bool {
        *self == MODE::RegisterToMemory
    }
}
///Field `MODE` writer - DMA2D mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Memory-to-memory (FG fetch only)
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MemoryToMemory)
    }
    ///Memory-to-memory with PFC (FG fetch only with FG PFC active)
    #[inline(always)]
    pub fn memory_to_memory_pfc(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MemoryToMemoryPfc)
    }
    ///Memory-to-memory with blending (FG and BG fetch with PFC and blending)
    #[inline(always)]
    pub fn memory_to_memory_pfcblending(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MemoryToMemoryPfcblending)
    }
    ///Register-to-memory
    #[inline(always)]
    pub fn register_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::RegisterToMemory)
    }
}
impl R {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Abort
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Line Offset Mode
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transfer watermark interrupt enable
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CLUT access error interrupt enable
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:18 - DMA2D mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("mode", &self.mode())
            .field("ceie", &self.ceie())
            .field("ctcie", &self.ctcie())
            .field("caeie", &self.caeie())
            .field("twie", &self.twie())
            .field("tcie", &self.tcie())
            .field("teie", &self.teie())
            .field("lom", &self.lom())
            .field("abort", &self.abort())
            .field("susp", &self.susp())
            .field("start", &self.start())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 0)
    }
    ///Bit 1 - Suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<CRrs> {
        SUSP_W::new(self, 1)
    }
    ///Bit 2 - Abort
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 2)
    }
    ///Bit 6 - Line Offset Mode
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W<CRrs> {
        LOM_W::new(self, 6)
    }
    ///Bit 8 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 8)
    }
    ///Bit 9 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 9)
    }
    ///Bit 10 - Transfer watermark interrupt enable
    #[inline(always)]
    pub fn twie(&mut self) -> TWIE_W<CRrs> {
        TWIE_W::new(self, 10)
    }
    ///Bit 11 - CLUT access error interrupt enable
    #[inline(always)]
    pub fn caeie(&mut self) -> CAEIE_W<CRrs> {
        CAEIE_W::new(self, 11)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W<CRrs> {
        CTCIE_W::new(self, 12)
    }
    ///Bit 13 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W<CRrs> {
        CEIE_W::new(self, 13)
    }
    ///Bits 16:18 - DMA2D mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 16)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DMA2D:CR)*/
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
