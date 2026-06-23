///Register `RIS` reader
pub type R = crate::R<RISrs>;
/**Capture complete raw interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_RIS {
    ///0: No new capture
    NoNewCapture = 0,
    ///1: A frame has been captured
    FrameCaptured = 1,
}
impl From<FRAME_RIS> for bool {
    #[inline(always)]
    fn from(variant: FRAME_RIS) -> Self {
        variant as u8 != 0
    }
}
///Field `FRAME_RIS` reader - Capture complete raw interrupt status
pub type FRAME_RIS_R = crate::BitReader<FRAME_RIS>;
impl FRAME_RIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRAME_RIS {
        match self.bits {
            false => FRAME_RIS::NoNewCapture,
            true => FRAME_RIS::FrameCaptured,
        }
    }
    ///No new capture
    #[inline(always)]
    pub fn is_no_new_capture(&self) -> bool {
        *self == FRAME_RIS::NoNewCapture
    }
    ///A frame has been captured
    #[inline(always)]
    pub fn is_frame_captured(&self) -> bool {
        *self == FRAME_RIS::FrameCaptured
    }
}
/**Overrun raw interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_RIS {
    ///0: No data buffer overrun occurred
    NoOverrun = 0,
    ///1: A data buffer overrun occurred and the data FIFO is corrupted. The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register
    OverrunOccured = 1,
}
impl From<OVR_RIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_RIS) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_RIS` reader - Overrun raw interrupt status
pub type OVR_RIS_R = crate::BitReader<OVR_RIS>;
impl OVR_RIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR_RIS {
        match self.bits {
            false => OVR_RIS::NoOverrun,
            true => OVR_RIS::OverrunOccured,
        }
    }
    ///No data buffer overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_RIS::NoOverrun
    }
    ///A data buffer overrun occurred and the data FIFO is corrupted. The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register
    #[inline(always)]
    pub fn is_overrun_occured(&self) -> bool {
        *self == OVR_RIS::OverrunOccured
    }
}
/**Synchronization error raw interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_RIS {
    ///0: No synchronization error detected
    NoError = 0,
    ///1: Embedded synchronization characters are not received in the correct order
    SynchronizationError = 1,
}
impl From<ERR_RIS> for bool {
    #[inline(always)]
    fn from(variant: ERR_RIS) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR_RIS` reader - Synchronization error raw interrupt status
pub type ERR_RIS_R = crate::BitReader<ERR_RIS>;
impl ERR_RIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERR_RIS {
        match self.bits {
            false => ERR_RIS::NoError,
            true => ERR_RIS::SynchronizationError,
        }
    }
    ///No synchronization error detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERR_RIS::NoError
    }
    ///Embedded synchronization characters are not received in the correct order
    #[inline(always)]
    pub fn is_synchronization_error(&self) -> bool {
        *self == ERR_RIS::SynchronizationError
    }
}
/**DCMI_VSYNC raw interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_RIS {
    ///0: Interrupt cleared
    Cleared = 0,
    ///1: Interrupt set
    Set = 1,
}
impl From<VSYNC_RIS> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_RIS) -> Self {
        variant as u8 != 0
    }
}
///Field `VSYNC_RIS` reader - DCMI_VSYNC raw interrupt status
pub type VSYNC_RIS_R = crate::BitReader<VSYNC_RIS>;
impl VSYNC_RIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC_RIS {
        match self.bits {
            false => VSYNC_RIS::Cleared,
            true => VSYNC_RIS::Set,
        }
    }
    ///Interrupt cleared
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == VSYNC_RIS::Cleared
    }
    ///Interrupt set
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == VSYNC_RIS::Set
    }
}
/**Line raw interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_RIS {
    ///0: Interrupt cleared
    Cleared = 0,
    ///1: Interrupt set
    Set = 1,
}
impl From<LINE_RIS> for bool {
    #[inline(always)]
    fn from(variant: LINE_RIS) -> Self {
        variant as u8 != 0
    }
}
///Field `LINE_RIS` reader - Line raw interrupt status
pub type LINE_RIS_R = crate::BitReader<LINE_RIS>;
impl LINE_RIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LINE_RIS {
        match self.bits {
            false => LINE_RIS::Cleared,
            true => LINE_RIS::Set,
        }
    }
    ///Interrupt cleared
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == LINE_RIS::Cleared
    }
    ///Interrupt set
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LINE_RIS::Set
    }
}
impl R {
    ///Bit 0 - Capture complete raw interrupt status
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overrun raw interrupt status
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization error raw interrupt status
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DCMI_VSYNC raw interrupt status
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Line raw interrupt status
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("line_ris", &self.line_ris())
            .field("vsync_ris", &self.vsync_ris())
            .field("err_ris", &self.err_ris())
            .field("ovr_ris", &self.ovr_ris())
            .field("frame_ris", &self.frame_ris())
            .finish()
    }
}
/**raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCMI:RIS)*/
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RISrs {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RISrs {}
