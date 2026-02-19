///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Computation complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCF {
    ///0: Computation not completed
    NotCompleted = 0,
    ///1: Computation completed
    Completed = 1,
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
            false => CCF::NotCompleted,
            true => CCF::Completed,
        }
    }
    ///Computation not completed
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == CCF::NotCompleted
    }
    ///Computation completed
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CCF::Completed
    }
}
/**Read or write error interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWEIF {
    ///0: No error detected
    NoError = 0,
    ///1: The error detected
    Error = 1,
}
impl From<RWEIF> for bool {
    #[inline(always)]
    fn from(variant: RWEIF) -> Self {
        variant as u8 != 0
    }
}
///Field `RWEIF` reader - Read or write error interrupt flag
pub type RWEIF_R = crate::BitReader<RWEIF>;
impl RWEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWEIF {
        match self.bits {
            false => RWEIF::NoError,
            true => RWEIF::Error,
        }
    }
    ///No error detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RWEIF::NoError
    }
    ///The error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RWEIF::Error
    }
}
///Field `KEIF` reader - Key error interrupt flag
pub use RWEIF_R as KEIF_R;
impl R {
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt flag
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt flag
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rweif", &self.rweif())
            .field("keif", &self.keif())
            .field("ccf", &self.ccf())
            .finish()
    }
}
/**interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#AES:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
