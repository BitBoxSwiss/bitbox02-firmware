///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Horizontal synchronization

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNC {
    ///0: Active line
    ActiveLine = 0,
    ///1: Synchronization between lines
    BetweenLines = 1,
}
impl From<HSYNC> for bool {
    #[inline(always)]
    fn from(variant: HSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSYNC` reader - Horizontal synchronization
pub type HSYNC_R = crate::BitReader<HSYNC>;
impl HSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSYNC {
        match self.bits {
            false => HSYNC::ActiveLine,
            true => HSYNC::BetweenLines,
        }
    }
    ///Active line
    #[inline(always)]
    pub fn is_active_line(&self) -> bool {
        *self == HSYNC::ActiveLine
    }
    ///Synchronization between lines
    #[inline(always)]
    pub fn is_between_lines(&self) -> bool {
        *self == HSYNC::BetweenLines
    }
}
/**Vertical synchronization

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC {
    ///0: Active frame
    ActiveFrame = 0,
    ///1: Synchronization between frames
    BetweenFrames = 1,
}
impl From<VSYNC> for bool {
    #[inline(always)]
    fn from(variant: VSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `VSYNC` reader - Vertical synchronization
pub type VSYNC_R = crate::BitReader<VSYNC>;
impl VSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC {
        match self.bits {
            false => VSYNC::ActiveFrame,
            true => VSYNC::BetweenFrames,
        }
    }
    ///Active frame
    #[inline(always)]
    pub fn is_active_frame(&self) -> bool {
        *self == VSYNC::ActiveFrame
    }
    ///Synchronization between frames
    #[inline(always)]
    pub fn is_between_frames(&self) -> bool {
        *self == VSYNC::BetweenFrames
    }
}
/**FIFO not empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FNE {
    ///0: FIFO contains valid data
    NotEmpty = 0,
    ///1: FIFO empty
    Empty = 1,
}
impl From<FNE> for bool {
    #[inline(always)]
    fn from(variant: FNE) -> Self {
        variant as u8 != 0
    }
}
///Field `FNE` reader - FIFO not empty
pub type FNE_R = crate::BitReader<FNE>;
impl FNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FNE {
        match self.bits {
            false => FNE::NotEmpty,
            true => FNE::Empty,
        }
    }
    ///FIFO contains valid data
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == FNE::NotEmpty
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FNE::Empty
    }
}
impl R {
    ///Bit 0 - Horizontal synchronization
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Vertical synchronization
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO not empty
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("fne", &self.fne())
            .field("vsync", &self.vsync())
            .field("hsync", &self.hsync())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DCMI:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
