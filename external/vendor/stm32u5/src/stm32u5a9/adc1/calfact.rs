///Register `CALFACT` reader
pub type R = crate::R<CALFACTrs>;
///Register `CALFACT` writer
pub type W = crate::W<CALFACTrs>;
///Field `I_APB_ADDR` reader - I_APB_ADDR
pub type I_APB_ADDR_R = crate::FieldReader;
///Field `I_APB_DATA` reader - I_APB_DATA
pub type I_APB_DATA_R = crate::FieldReader;
/**VALIDITY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALIDITYR {
    ///0: Operation still in progress
    InProgress = 0,
    ///1: Operation complete
    Complete = 1,
}
impl From<VALIDITYR> for bool {
    #[inline(always)]
    fn from(variant: VALIDITYR) -> Self {
        variant as u8 != 0
    }
}
///Field `VALIDITY` reader - VALIDITY
pub type VALIDITY_R = crate::BitReader<VALIDITYR>;
impl VALIDITY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VALIDITYR {
        match self.bits {
            false => VALIDITYR::InProgress,
            true => VALIDITYR::Complete,
        }
    }
    ///Operation still in progress
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == VALIDITYR::InProgress
    }
    ///Operation complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == VALIDITYR::Complete
    }
}
/**LATCH_COEF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LATCH_COEF {
    ///0: No effect
    NoEffect = 0,
    ///1: Calibration factor latched in the analog block on LATCH_COEF bit transition from 0 to 1. Prior to latching the calibration factor, CALFACT\[31:0\] bits must be programmed with the content of CALINDEX\[3:0\] bits.
    Latch = 1,
}
impl From<LATCH_COEF> for bool {
    #[inline(always)]
    fn from(variant: LATCH_COEF) -> Self {
        variant as u8 != 0
    }
}
///Field `LATCH_COEF` reader - LATCH_COEF
pub type LATCH_COEF_R = crate::BitReader<LATCH_COEF>;
impl LATCH_COEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LATCH_COEF {
        match self.bits {
            false => LATCH_COEF::NoEffect,
            true => LATCH_COEF::Latch,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LATCH_COEF::NoEffect
    }
    ///Calibration factor latched in the analog block on LATCH_COEF bit transition from 0 to 1. Prior to latching the calibration factor, CALFACT\[31:0\] bits must be programmed with the content of CALINDEX\[3:0\] bits.
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == LATCH_COEF::Latch
    }
}
///Field `LATCH_COEF` writer - LATCH_COEF
pub type LATCH_COEF_W<'a, REG> = crate::BitWriter<'a, REG, LATCH_COEF>;
impl<'a, REG> LATCH_COEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LATCH_COEF::NoEffect)
    }
    ///Calibration factor latched in the analog block on LATCH_COEF bit transition from 0 to 1. Prior to latching the calibration factor, CALFACT\[31:0\] bits must be programmed with the content of CALINDEX\[3:0\] bits.
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(LATCH_COEF::Latch)
    }
}
/**CAPTURE_COEF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTURE_COEF {
    ///0: Calibration factor not captured
    Disabled = 0,
    ///1: Calibration factor available in CALFACT\[31:0\] bits, the calibration factor index being defined by CALINDEX\[3:0\] bits
    Enabled = 1,
}
impl From<CAPTURE_COEF> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_COEF) -> Self {
        variant as u8 != 0
    }
}
///Field `CAPTURE_COEF` reader - CAPTURE_COEF
pub type CAPTURE_COEF_R = crate::BitReader<CAPTURE_COEF>;
impl CAPTURE_COEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CAPTURE_COEF {
        match self.bits {
            false => CAPTURE_COEF::Disabled,
            true => CAPTURE_COEF::Enabled,
        }
    }
    ///Calibration factor not captured
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPTURE_COEF::Disabled
    }
    ///Calibration factor available in CALFACT\[31:0\] bits, the calibration factor index being defined by CALINDEX\[3:0\] bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPTURE_COEF::Enabled
    }
}
///Field `CAPTURE_COEF` writer - CAPTURE_COEF
pub type CAPTURE_COEF_W<'a, REG> = crate::BitWriter<'a, REG, CAPTURE_COEF>;
impl<'a, REG> CAPTURE_COEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration factor not captured
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURE_COEF::Disabled)
    }
    ///Calibration factor available in CALFACT\[31:0\] bits, the calibration factor index being defined by CALINDEX\[3:0\] bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURE_COEF::Enabled)
    }
}
impl R {
    ///Bits 0:7 - I_APB_ADDR
    #[inline(always)]
    pub fn i_apb_addr(&self) -> I_APB_ADDR_R {
        I_APB_ADDR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - I_APB_DATA
    #[inline(always)]
    pub fn i_apb_data(&self) -> I_APB_DATA_R {
        I_APB_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - VALIDITY
    #[inline(always)]
    pub fn validity(&self) -> VALIDITY_R {
        VALIDITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - LATCH_COEF
    #[inline(always)]
    pub fn latch_coef(&self) -> LATCH_COEF_R {
        LATCH_COEF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CAPTURE_COEF
    #[inline(always)]
    pub fn capture_coef(&self) -> CAPTURE_COEF_R {
        CAPTURE_COEF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT")
            .field("capture_coef", &self.capture_coef())
            .field("latch_coef", &self.latch_coef())
            .field("validity", &self.validity())
            .field("i_apb_data", &self.i_apb_data())
            .field("i_apb_addr", &self.i_apb_addr())
            .finish()
    }
}
impl W {
    ///Bit 24 - LATCH_COEF
    #[inline(always)]
    pub fn latch_coef(&mut self) -> LATCH_COEF_W<CALFACTrs> {
        LATCH_COEF_W::new(self, 24)
    }
    ///Bit 25 - CAPTURE_COEF
    #[inline(always)]
    pub fn capture_coef(&mut self) -> CAPTURE_COEF_W<CALFACTrs> {
        CAPTURE_COEF_W::new(self, 25)
    }
}
/**ADC user control register

You can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC1:CALFACT)*/
pub struct CALFACTrs;
impl crate::RegisterSpec for CALFACTrs {
    type Ux = u32;
}
///`read()` method returns [`calfact::R`](R) reader structure
impl crate::Readable for CALFACTrs {}
///`write(|w| ..)` method takes [`calfact::W`](W) writer structure
impl crate::Writable for CALFACTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALFACT to value 0
impl crate::Resettable for CALFACTrs {}
