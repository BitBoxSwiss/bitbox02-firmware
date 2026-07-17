///Register `SVMSR` reader
pub type R = crate::R<SVMSRrs>;
/**Regulator selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGS {
    ///0: LDO selected
    Ldo = 0,
    ///1: SMPS selected
    Smps = 1,
}
impl From<REGS> for bool {
    #[inline(always)]
    fn from(variant: REGS) -> Self {
        variant as u8 != 0
    }
}
///Field `REGS` reader - Regulator selection
pub type REGS_R = crate::BitReader<REGS>;
impl REGS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGS {
        match self.bits {
            false => REGS::Ldo,
            true => REGS::Smps,
        }
    }
    ///LDO selected
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == REGS::Ldo
    }
    ///SMPS selected
    #[inline(always)]
    pub fn is_smps(&self) -> bool {
        *self == REGS::Smps
    }
}
/**VDD voltage detector output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO {
    ///0: VDD is equal or above the PVD threshold selected by PVDLS\[2:0\]
    EqualOrAboveThreshold = 0,
    ///1: VDD is below the PVD threshold selected by PVDLS\[2:0\]
    BelowThreshold = 1,
}
impl From<PVDO> for bool {
    #[inline(always)]
    fn from(variant: PVDO) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - VDD voltage detector output
pub type PVDO_R = crate::BitReader<PVDO>;
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDO {
        match self.bits {
            false => PVDO::EqualOrAboveThreshold,
            true => PVDO::BelowThreshold,
        }
    }
    ///VDD is equal or above the PVD threshold selected by PVDLS\[2:0\]
    #[inline(always)]
    pub fn is_equal_or_above_threshold(&self) -> bool {
        *self == PVDO::EqualOrAboveThreshold
    }
    ///VDD is below the PVD threshold selected by PVDLS\[2:0\]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == PVDO::BelowThreshold
    }
}
/**Voltage level ready for currently used VOS

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTVOSRDY {
    ///0: VCORE is above or below the current voltage scaling provided by ACTVOS\[1:0\]
    NotReady = 0,
    ///1: VCORE is equal to the current voltage scaling provided by ACTVOS\[1:0\]
    Ready = 1,
}
impl From<ACTVOSRDY> for bool {
    #[inline(always)]
    fn from(variant: ACTVOSRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `ACTVOSRDY` reader - Voltage level ready for currently used VOS
pub type ACTVOSRDY_R = crate::BitReader<ACTVOSRDY>;
impl ACTVOSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACTVOSRDY {
        match self.bits {
            false => ACTVOSRDY::NotReady,
            true => ACTVOSRDY::Ready,
        }
    }
    ///VCORE is above or below the current voltage scaling provided by ACTVOS\[1:0\]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ACTVOSRDY::NotReady
    }
    ///VCORE is equal to the current voltage scaling provided by ACTVOS\[1:0\]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ACTVOSRDY::Ready
    }
}
/**VOS currently applied to VCORE This field provides the last VOS value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTVOS {
    ///0: Range 4 (lowest power)
    Range4 = 0,
    ///1: Range 3
    Range3 = 1,
    ///2: Range 2
    Range2 = 2,
    ///3: Range 1 (highest frequency)
    Range1 = 3,
}
impl From<ACTVOS> for u8 {
    #[inline(always)]
    fn from(variant: ACTVOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTVOS {
    type Ux = u8;
}
impl crate::IsEnum for ACTVOS {}
///Field `ACTVOS` reader - VOS currently applied to VCORE This field provides the last VOS value.
pub type ACTVOS_R = crate::FieldReader<ACTVOS>;
impl ACTVOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACTVOS {
        match self.bits {
            0 => ACTVOS::Range4,
            1 => ACTVOS::Range3,
            2 => ACTVOS::Range2,
            3 => ACTVOS::Range1,
            _ => unreachable!(),
        }
    }
    ///Range 4 (lowest power)
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == ACTVOS::Range4
    }
    ///Range 3
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == ACTVOS::Range3
    }
    ///Range 2
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == ACTVOS::Range2
    }
    ///Range 1 (highest frequency)
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == ACTVOS::Range1
    }
}
/**VDDUSB ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDUSBRDY {
    ///0: VDDUSB is below the threshold of the VDDUSB voltage monitor
    BelowThreshold = 0,
    ///1: VDDUSB is equal or above the threshold of the VDDUSB voltage monitor
    EqualOrAboveThreshold = 1,
}
impl From<VDDUSBRDY> for bool {
    #[inline(always)]
    fn from(variant: VDDUSBRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDUSBRDY` reader - VDDUSB ready
pub type VDDUSBRDY_R = crate::BitReader<VDDUSBRDY>;
impl VDDUSBRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDUSBRDY {
        match self.bits {
            false => VDDUSBRDY::BelowThreshold,
            true => VDDUSBRDY::EqualOrAboveThreshold,
        }
    }
    ///VDDUSB is below the threshold of the VDDUSB voltage monitor
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VDDUSBRDY::BelowThreshold
    }
    ///VDDUSB is equal or above the threshold of the VDDUSB voltage monitor
    #[inline(always)]
    pub fn is_equal_or_above_threshold(&self) -> bool {
        *self == VDDUSBRDY::EqualOrAboveThreshold
    }
}
/**VDDIO2 ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDIO2RDY {
    ///0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor
    BelowThreshold = 0,
    ///1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor
    EqualOrAboveThreshold = 1,
}
impl From<VDDIO2RDY> for bool {
    #[inline(always)]
    fn from(variant: VDDIO2RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDIO2RDY` reader - VDDIO2 ready
pub type VDDIO2RDY_R = crate::BitReader<VDDIO2RDY>;
impl VDDIO2RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDIO2RDY {
        match self.bits {
            false => VDDIO2RDY::BelowThreshold,
            true => VDDIO2RDY::EqualOrAboveThreshold,
        }
    }
    ///VDDIO2 is below the threshold of the VDDIO2 voltage monitor
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VDDIO2RDY::BelowThreshold
    }
    ///VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor
    #[inline(always)]
    pub fn is_equal_or_above_threshold(&self) -> bool {
        *self == VDDIO2RDY::EqualOrAboveThreshold
    }
}
/**VDDA ready versus 1.6V voltage monitor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDA1RDY {
    ///0: VDDA is below the threshold of the VDDA voltage monitor 1 (around 1.6 V)
    BelowThreshold = 0,
    ///1: VDDA is equal or above the threshold of the VDDA voltage monitor 1 (around 1.6 V)
    EqualOrAboveThreshold = 1,
}
impl From<VDDA1RDY> for bool {
    #[inline(always)]
    fn from(variant: VDDA1RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDA1RDY` reader - VDDA ready versus 1.6V voltage monitor
pub type VDDA1RDY_R = crate::BitReader<VDDA1RDY>;
impl VDDA1RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDA1RDY {
        match self.bits {
            false => VDDA1RDY::BelowThreshold,
            true => VDDA1RDY::EqualOrAboveThreshold,
        }
    }
    ///VDDA is below the threshold of the VDDA voltage monitor 1 (around 1.6 V)
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VDDA1RDY::BelowThreshold
    }
    ///VDDA is equal or above the threshold of the VDDA voltage monitor 1 (around 1.6 V)
    #[inline(always)]
    pub fn is_equal_or_above_threshold(&self) -> bool {
        *self == VDDA1RDY::EqualOrAboveThreshold
    }
}
/**VDDA ready versus 1.8 V voltage monitor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDA2RDY {
    ///0: VDDA is below the threshold of the VDDA voltage monitor 2 (around 1.8 V)
    BelowThreshold = 0,
    ///1: VDDA is equal or above the threshold of the VDDA voltage monitor 2 (around 1.8 V)
    AboveThreshold = 1,
}
impl From<VDDA2RDY> for bool {
    #[inline(always)]
    fn from(variant: VDDA2RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDA2RDY` reader - VDDA ready versus 1.8 V voltage monitor
pub type VDDA2RDY_R = crate::BitReader<VDDA2RDY>;
impl VDDA2RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDA2RDY {
        match self.bits {
            false => VDDA2RDY::BelowThreshold,
            true => VDDA2RDY::AboveThreshold,
        }
    }
    ///VDDA is below the threshold of the VDDA voltage monitor 2 (around 1.8 V)
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VDDA2RDY::BelowThreshold
    }
    ///VDDA is equal or above the threshold of the VDDA voltage monitor 2 (around 1.8 V)
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VDDA2RDY::AboveThreshold
    }
}
impl R {
    ///Bit 1 - Regulator selection
    #[inline(always)]
    pub fn regs(&self) -> REGS_R {
        REGS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - VDD voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 15 - Voltage level ready for currently used VOS
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - VOS currently applied to VCORE This field provides the last VOS value.
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - VDDUSB ready
    #[inline(always)]
    pub fn vddusbrdy(&self) -> VDDUSBRDY_R {
        VDDUSBRDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VDDIO2 ready
    #[inline(always)]
    pub fn vddio2rdy(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - VDDA ready versus 1.6V voltage monitor
    #[inline(always)]
    pub fn vdda1rdy(&self) -> VDDA1RDY_R {
        VDDA1RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - VDDA ready versus 1.8 V voltage monitor
    #[inline(always)]
    pub fn vdda2rdy(&self) -> VDDA2RDY_R {
        VDDA2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMSR")
            .field("regs", &self.regs())
            .field("pvdo", &self.pvdo())
            .field("actvosrdy", &self.actvosrdy())
            .field("actvos", &self.actvos())
            .field("vddusbrdy", &self.vddusbrdy())
            .field("vddio2rdy", &self.vddio2rdy())
            .field("vdda1rdy", &self.vdda1rdy())
            .field("vdda2rdy", &self.vdda2rdy())
            .finish()
    }
}
/**PWR supply voltage monitoring status register

You can [`read`](crate::Reg::read) this register and get [`svmsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:SVMSR)*/
pub struct SVMSRrs;
impl crate::RegisterSpec for SVMSRrs {
    type Ux = u32;
}
///`read()` method returns [`svmsr::R`](R) reader structure
impl crate::Readable for SVMSRrs {}
///`reset()` method sets SVMSR to value 0x8000
impl crate::Resettable for SVMSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
