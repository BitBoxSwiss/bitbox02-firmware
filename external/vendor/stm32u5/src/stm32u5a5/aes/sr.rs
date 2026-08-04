///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Computation complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCF {
    ///0: Computation complete
    Complete = 0,
    ///1: Computation not complete
    NotComplete = 1,
}
impl From<CCF> for bool {
    #[inline(always)]
    fn from(variant: CCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CCF` reader - Computation complete flag
pub type CCF_R = crate::BitReader<CCF>;
impl CCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCF {
        match self.bits {
            false => CCF::Complete,
            true => CCF::NotComplete,
        }
    }
    ///Computation complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCF::Complete
    }
    ///Computation not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCF::NotComplete
    }
}
/**Read error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERR {
    ///0: Read error not detected
    NoError = 0,
    ///1: Read error detected
    Error = 1,
}
impl From<RDERR> for bool {
    #[inline(always)]
    fn from(variant: RDERR) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERR` reader - Read error flag
pub type RDERR_R = crate::BitReader<RDERR>;
impl RDERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDERR {
        match self.bits {
            false => RDERR::NoError,
            true => RDERR::Error,
        }
    }
    ///Read error not detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERR::NoError
    }
    ///Read error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERR::Error
    }
}
/**Write error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRERR {
    ///0: Write error not detected
    NoError = 0,
    ///1: Write error detected
    Error = 1,
}
impl From<WRERR> for bool {
    #[inline(always)]
    fn from(variant: WRERR) -> Self {
        variant as u8 != 0
    }
}
///Field `WRERR` reader - Write error flag
pub type WRERR_R = crate::BitReader<WRERR>;
impl WRERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRERR {
        match self.bits {
            false => WRERR::NoError,
            true => WRERR::Error,
        }
    }
    ///Write error not detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRERR::NoError
    }
    ///Write error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRERR::Error
    }
}
/**BUSY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: Idle
    Idle = 0,
    ///1: Busy
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
            false => BUSY::Idle,
            true => BUSY::Busy,
        }
    }
    ///Idle
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY::Idle
    }
    ///Busy
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
///Field `KEYVALID` reader - Key Valid flag
pub type KEYVALID_R = crate::BitReader;
impl R {
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read error flag
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write error flag
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Key Valid flag
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("keyvalid", &self.keyvalid())
            .field("busy", &self.busy())
            .field("wrerr", &self.wrerr())
            .field("rderr", &self.rderr())
            .field("ccf", &self.ccf())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#AES:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
