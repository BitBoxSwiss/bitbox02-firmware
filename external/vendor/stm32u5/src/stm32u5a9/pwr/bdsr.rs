///Register `BDSR` reader
pub type R = crate::R<BDSRrs>;
/**Backup domain voltage level monitoring versus high threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATH {
    ///0: VBAT < high threshold
    BelowHigh = 0,
    ///1: VBAT ≥ high threshold
    AboveHigh = 1,
}
impl From<VBATH> for bool {
    #[inline(always)]
    fn from(variant: VBATH) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATH` reader - Backup domain voltage level monitoring versus high threshold
pub type VBATH_R = crate::BitReader<VBATH>;
impl VBATH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATH {
        match self.bits {
            false => VBATH::BelowHigh,
            true => VBATH::AboveHigh,
        }
    }
    ///VBAT < high threshold
    #[inline(always)]
    pub fn is_below_high(&self) -> bool {
        *self == VBATH::BelowHigh
    }
    ///VBAT ≥ high threshold
    #[inline(always)]
    pub fn is_above_high(&self) -> bool {
        *self == VBATH::AboveHigh
    }
}
/**Temperature level monitoring versus low threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPL {
    ///0: Temperature > low threshold
    AboveLow = 0,
    ///1: Temperature ≤ low threshold
    BelowLow = 1,
}
impl From<TEMPL> for bool {
    #[inline(always)]
    fn from(variant: TEMPL) -> Self {
        variant as u8 != 0
    }
}
///Field `TEMPL` reader - Temperature level monitoring versus low threshold
pub type TEMPL_R = crate::BitReader<TEMPL>;
impl TEMPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEMPL {
        match self.bits {
            false => TEMPL::AboveLow,
            true => TEMPL::BelowLow,
        }
    }
    ///Temperature > low threshold
    #[inline(always)]
    pub fn is_above_low(&self) -> bool {
        *self == TEMPL::AboveLow
    }
    ///Temperature ≤ low threshold
    #[inline(always)]
    pub fn is_below_low(&self) -> bool {
        *self == TEMPL::BelowLow
    }
}
/**Temperature level monitoring versus high threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPH {
    ///0: Temperature < high threshold
    BelowHigh = 0,
    ///1: Temperature ≥ high threshold
    AboveHigh = 1,
}
impl From<TEMPH> for bool {
    #[inline(always)]
    fn from(variant: TEMPH) -> Self {
        variant as u8 != 0
    }
}
///Field `TEMPH` reader - Temperature level monitoring versus high threshold
pub type TEMPH_R = crate::BitReader<TEMPH>;
impl TEMPH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEMPH {
        match self.bits {
            false => TEMPH::BelowHigh,
            true => TEMPH::AboveHigh,
        }
    }
    ///Temperature < high threshold
    #[inline(always)]
    pub fn is_below_high(&self) -> bool {
        *self == TEMPH::BelowHigh
    }
    ///Temperature ≥ high threshold
    #[inline(always)]
    pub fn is_above_high(&self) -> bool {
        *self == TEMPH::AboveHigh
    }
}
impl R {
    ///Bit 1 - Backup domain voltage level monitoring versus high threshold
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Temperature level monitoring versus low threshold
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Temperature level monitoring versus high threshold
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDSR")
            .field("vbath", &self.vbath())
            .field("templ", &self.templ())
            .field("temph", &self.temph())
            .finish()
    }
}
/**PWR Backup domain status register

You can [`read`](crate::Reg::read) this register and get [`bdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:BDSR)*/
pub struct BDSRrs;
impl crate::RegisterSpec for BDSRrs {
    type Ux = u32;
}
///`read()` method returns [`bdsr::R`](R) reader structure
impl crate::Readable for BDSRrs {}
///`reset()` method sets BDSR to value 0
impl crate::Resettable for BDSRrs {}
