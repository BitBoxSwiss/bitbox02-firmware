///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**line interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIF {
    ///0: Programmed line not reached
    NotReached = 0,
    ///1: Line interrupt generated when a programmed line is reached
    Reached = 1,
}
impl From<LIF> for bool {
    #[inline(always)]
    fn from(variant: LIF) -> Self {
        variant as u8 != 0
    }
}
///Field `LIF` reader - line interrupt flag
pub type LIF_R = crate::BitReader<LIF>;
impl LIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LIF {
        match self.bits {
            false => LIF::NotReached,
            true => LIF::Reached,
        }
    }
    ///Programmed line not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == LIF::NotReached
    }
    ///Line interrupt generated when a programmed line is reached
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == LIF::Reached
    }
}
/**FIFO underrun interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUIF {
    ///0: No FIFO underrun
    NoUnderrun = 0,
    ///1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO
    Underrun = 1,
}
impl From<FUIF> for bool {
    #[inline(always)]
    fn from(variant: FUIF) -> Self {
        variant as u8 != 0
    }
}
///Field `FUIF` reader - FIFO underrun interrupt flag
pub type FUIF_R = crate::BitReader<FUIF>;
impl FUIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FUIF {
        match self.bits {
            false => FUIF::NoUnderrun,
            true => FUIF::Underrun,
        }
    }
    ///No FIFO underrun
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == FUIF::NoUnderrun
    }
    ///FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == FUIF::Underrun
    }
}
/**transfer error interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIF {
    ///0: No transfer error
    NoError = 0,
    ///1: Transfer error interrupt generated when a bus error occurs
    Error = 1,
}
impl From<TERRIF> for bool {
    #[inline(always)]
    fn from(variant: TERRIF) -> Self {
        variant as u8 != 0
    }
}
///Field `TERRIF` reader - transfer error interrupt flag
pub type TERRIF_R = crate::BitReader<TERRIF>;
impl TERRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TERRIF {
        match self.bits {
            false => TERRIF::NoError,
            true => TERRIF::Error,
        }
    }
    ///No transfer error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TERRIF::NoError
    }
    ///Transfer error interrupt generated when a bus error occurs
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TERRIF::Error
    }
}
/**register reload interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIF {
    ///0: No register reload
    NoReload = 0,
    ///1: Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)
    Reload = 1,
}
impl From<RRIF> for bool {
    #[inline(always)]
    fn from(variant: RRIF) -> Self {
        variant as u8 != 0
    }
}
///Field `RRIF` reader - register reload interrupt flag
pub type RRIF_R = crate::BitReader<RRIF>;
impl RRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRIF {
        match self.bits {
            false => RRIF::NoReload,
            true => RRIF::Reload,
        }
    }
    ///No register reload
    #[inline(always)]
    pub fn is_no_reload(&self) -> bool {
        *self == RRIF::NoReload
    }
    ///Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == RRIF::Reload
    }
}
impl R {
    ///Bit 0 - line interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO underrun interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - transfer error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - register reload interrupt flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("lif", &self.lif())
            .field("fuif", &self.fuif())
            .field("terrif", &self.terrif())
            .field("rrif", &self.rrif())
            .finish()
    }
}
/**LTDC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
