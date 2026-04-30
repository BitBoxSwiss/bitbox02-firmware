///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Transfer error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEF {
    ///0: This bit is cleared by writing 1 to CTEF
    Cleared = 0,
    ///1: This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode
    InvalidAddressAccessed = 1,
}
impl From<TEF> for bool {
    #[inline(always)]
    fn from(variant: TEF) -> Self {
        variant as u8 != 0
    }
}
///Field `TEF` reader - Transfer error flag
pub type TEF_R = crate::BitReader<TEF>;
impl TEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEF {
        match self.bits {
            false => TEF::Cleared,
            true => TEF::InvalidAddressAccessed,
        }
    }
    ///This bit is cleared by writing 1 to CTEF
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TEF::Cleared
    }
    ///This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode
    #[inline(always)]
    pub fn is_invalid_address_accessed(&self) -> bool {
        *self == TEF::InvalidAddressAccessed
    }
}
/**transfer complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF {
    ///0: This bit is cleared by writing 1 to CTCF
    Cleared = 0,
    ///1: This bit is set when the programmed number of data has been transferred
    TransferCompleted = 1,
}
impl From<TCF> for bool {
    #[inline(always)]
    fn from(variant: TCF) -> Self {
        variant as u8 != 0
    }
}
///Field `TCF` reader - transfer complete flag
pub type TCF_R = crate::BitReader<TCF>;
impl TCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCF {
        match self.bits {
            false => TCF::Cleared,
            true => TCF::TransferCompleted,
        }
    }
    ///This bit is cleared by writing 1 to CTCF
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TCF::Cleared
    }
    ///This bit is set when the programmed number of data has been transferred
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        *self == TCF::TransferCompleted
    }
}
/**FIFO threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF {
    ///0: It is cleared automatically as soon as the threshold condition is no longer true
    Cleared = 0,
    ///1: This bit is set when the FIFO threshold has been reached
    ThresholdReached = 1,
}
impl From<FTF> for bool {
    #[inline(always)]
    fn from(variant: FTF) -> Self {
        variant as u8 != 0
    }
}
///Field `FTF` reader - FIFO threshold flag
pub type FTF_R = crate::BitReader<FTF>;
impl FTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTF {
        match self.bits {
            false => FTF::Cleared,
            true => FTF::ThresholdReached,
        }
    }
    ///It is cleared automatically as soon as the threshold condition is no longer true
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == FTF::Cleared
    }
    ///This bit is set when the FIFO threshold has been reached
    #[inline(always)]
    pub fn is_threshold_reached(&self) -> bool {
        *self == FTF::ThresholdReached
    }
}
/**status match flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMF {
    ///0: It is cleared by writing 1 to CSMF
    Cleared = 0,
    ///1: This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)
    Matched = 1,
}
impl From<SMF> for bool {
    #[inline(always)]
    fn from(variant: SMF) -> Self {
        variant as u8 != 0
    }
}
///Field `SMF` reader - status match flag
pub type SMF_R = crate::BitReader<SMF>;
impl SMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMF {
        match self.bits {
            false => SMF::Cleared,
            true => SMF::Matched,
        }
    }
    ///It is cleared by writing 1 to CSMF
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SMF::Cleared
    }
    ///This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)
    #[inline(always)]
    pub fn is_matched(&self) -> bool {
        *self == SMF::Matched
    }
}
/**timeout flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF {
    ///0: This bit is cleared by writing 1 to CTOF
    Cleared = 0,
    ///1: This bit is set when timeout occurs
    Timeout = 1,
}
impl From<TOF> for bool {
    #[inline(always)]
    fn from(variant: TOF) -> Self {
        variant as u8 != 0
    }
}
///Field `TOF` reader - timeout flag
pub type TOF_R = crate::BitReader<TOF>;
impl TOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOF {
        match self.bits {
            false => TOF::Cleared,
            true => TOF::Timeout,
        }
    }
    ///This bit is cleared by writing 1 to CTOF
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TOF::Cleared
    }
    ///This bit is set when timeout occurs
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TOF::Timeout
    }
}
/**BUSY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty
    Cleared = 0,
    ///1: This bit is set when an operation is ongoing
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::Cleared,
            true => BUSY::Busy,
        }
    }
    ///This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == BUSY::Cleared
    }
    ///This bit is set when an operation is ongoing
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
///Field `FLEVEL` reader - FIFO level
pub type FLEVEL_R = crate::FieldReader;
impl R {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - status match flag
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - timeout flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tef", &self.tef())
            .field("tcf", &self.tcf())
            .field("ftf", &self.ftf())
            .field("smf", &self.smf())
            .field("tof", &self.tof())
            .field("busy", &self.busy())
            .field("flevel", &self.flevel())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OCTOSPI1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
