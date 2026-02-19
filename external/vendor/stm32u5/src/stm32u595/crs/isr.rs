///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**SYNC event OK flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKF {
    ///0: Signal not set
    NotSignaled = 0,
    ///1: Signal set
    Signaled = 1,
}
impl From<SYNCOKF> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKF) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCOKF` reader - SYNC event OK flag
pub type SYNCOKF_R = crate::BitReader<SYNCOKF>;
impl SYNCOKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOKF {
        match self.bits {
            false => SYNCOKF::NotSignaled,
            true => SYNCOKF::Signaled,
        }
    }
    ///Signal not set
    #[inline(always)]
    pub fn is_not_signaled(&self) -> bool {
        *self == SYNCOKF::NotSignaled
    }
    ///Signal set
    #[inline(always)]
    pub fn is_signaled(&self) -> bool {
        *self == SYNCOKF::Signaled
    }
}
///Field `SYNCWARNF` reader - SYNC warning flag
pub use SYNCOKF_R as SYNCWARNF_R;
///Field `ERRF` reader - Error flag
pub use SYNCOKF_R as ERRF_R;
///Field `ESYNCF` reader - Expected SYNC flag
pub use SYNCOKF_R as ESYNCF_R;
///Field `SYNCERR` reader - SYNC error
pub use SYNCOKF_R as SYNCERR_R;
///Field `SYNCMISS` reader - SYNC missed
pub use SYNCOKF_R as SYNCMISS_R;
///Field `TRIMOVF` reader - Trimming overflow or underflow
pub use SYNCOKF_R as TRIMOVF_R;
/**Frequency error direction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEDIR {
    ///0: Error in up-counting direction
    UpCounting = 0,
    ///1: Error in down-counting direction
    DownCounting = 1,
}
impl From<FEDIR> for bool {
    #[inline(always)]
    fn from(variant: FEDIR) -> Self {
        variant as u8 != 0
    }
}
///Field `FEDIR` reader - Frequency error direction
pub type FEDIR_R = crate::BitReader<FEDIR>;
impl FEDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEDIR {
        match self.bits {
            false => FEDIR::UpCounting,
            true => FEDIR::DownCounting,
        }
    }
    ///Error in up-counting direction
    #[inline(always)]
    pub fn is_up_counting(&self) -> bool {
        *self == FEDIR::UpCounting
    }
    ///Error in down-counting direction
    #[inline(always)]
    pub fn is_down_counting(&self) -> bool {
        *self == FEDIR::DownCounting
    }
}
///Field `FECAP` reader - Frequency error capture
pub type FECAP_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - SYNC event OK flag
    #[inline(always)]
    pub fn syncokf(&self) -> SYNCOKF_R {
        SYNCOKF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC warning flag
    #[inline(always)]
    pub fn syncwarnf(&self) -> SYNCWARNF_R {
        SYNCWARNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error flag
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC flag
    #[inline(always)]
    pub fn esyncf(&self) -> ESYNCF_R {
        ESYNCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SYNC error
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SYNC missed
    #[inline(always)]
    pub fn syncmiss(&self) -> SYNCMISS_R {
        SYNCMISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Trimming overflow or underflow
    #[inline(always)]
    pub fn trimovf(&self) -> TRIMOVF_R {
        TRIMOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Frequency error direction
    #[inline(always)]
    pub fn fedir(&self) -> FEDIR_R {
        FEDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - Frequency error capture
    #[inline(always)]
    pub fn fecap(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("fecap", &self.fecap())
            .field("fedir", &self.fedir())
            .field("syncokf", &self.syncokf())
            .field("trimovf", &self.trimovf())
            .field("syncmiss", &self.syncmiss())
            .field("syncerr", &self.syncerr())
            .field("esyncf", &self.esyncf())
            .field("errf", &self.errf())
            .field("syncwarnf", &self.syncwarnf())
            .finish()
    }
}
/**interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#CRS:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
