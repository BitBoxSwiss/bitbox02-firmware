///Register `PLL1CFGR` reader
pub type R = crate::R<PLL1CFGRrs>;
///Register `PLL1CFGR` writer
pub type W = crate::W<PLL1CFGRrs>;
/**PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1SRC {
    ///0: No clock sent to PLLx
    NoClock = 0,
    ///1: MSIS clock selected as PLLx clock entry
    Msis = 1,
    ///2: HSI16 clock selected as PLLx clock entry
    Hsi16 = 2,
    ///3: HSE clock selected as PLLx clock entry
    Hse = 3,
}
impl From<PLL1SRC> for u8 {
    #[inline(always)]
    fn from(variant: PLL1SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1SRC {
    type Ux = u8;
}
impl crate::IsEnum for PLL1SRC {}
///Field `PLL1SRC` reader - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
pub type PLL1SRC_R = crate::FieldReader<PLL1SRC>;
impl PLL1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1SRC {
        match self.bits {
            0 => PLL1SRC::NoClock,
            1 => PLL1SRC::Msis,
            2 => PLL1SRC::Hsi16,
            3 => PLL1SRC::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock sent to PLLx
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLL1SRC::NoClock
    }
    ///MSIS clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == PLL1SRC::Msis
    }
    ///HSI16 clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLL1SRC::Hsi16
    }
    ///HSE clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL1SRC::Hse
    }
}
///Field `PLL1SRC` writer - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
pub type PLL1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL1SRC, crate::Safe>;
impl<'a, REG> PLL1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to PLLx
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::NoClock)
    }
    ///MSIS clock selected as PLLx clock entry
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Msis)
    }
    ///HSI16 clock selected as PLLx clock entry
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Hsi16)
    }
    ///HSE clock selected as PLLx clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Hse)
    }
}
/**PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1RGE {
    ///3: PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    Range2 = 3,
    ///0: PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    Range1 = 0,
}
impl From<PLL1RGE> for u8 {
    #[inline(always)]
    fn from(variant: PLL1RGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1RGE {
    type Ux = u8;
}
impl crate::IsEnum for PLL1RGE {}
///Field `PLL1RGE` reader - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_R = crate::FieldReader<PLL1RGE>;
impl PLL1RGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RGE {
        match self.bits {
            3 => PLL1RGE::Range2,
            _ => PLL1RGE::Range1,
        }
    }
    ///PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL1RGE::Range2
    }
    ///PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        matches!(self.variant(), PLL1RGE::Range1)
    }
}
///Field `PLL1RGE` writer - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL1RGE, crate::Safe>;
impl<'a, REG> PLL1RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range2)
    }
    ///PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range1)
    }
}
/**PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1FRACEN {
    ///0: No effect
    NoEffect = 0,
    ///1: Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    Latch = 1,
}
impl From<PLL1FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1FRACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1FRACEN` reader - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
pub type PLL1FRACEN_R = crate::BitReader<PLL1FRACEN>;
impl PLL1FRACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1FRACEN {
        match self.bits {
            false => PLL1FRACEN::NoEffect,
            true => PLL1FRACEN::Latch,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PLL1FRACEN::NoEffect
    }
    ///Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == PLL1FRACEN::Latch
    }
}
///Field `PLL1FRACEN` writer - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1FRACEN>;
impl<'a, REG> PLL1FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::NoEffect)
    }
    ///Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Latch)
    }
}
/**Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1M {
    ///0: division by 1 (bypass)
    Div1 = 0,
    ///1: division by 2
    Div2 = 1,
    ///2: division by 3
    Div3 = 2,
    ///3: division by 4
    Div4 = 3,
    ///4: division by 5
    Div5 = 4,
    ///5: division by 6
    Div6 = 5,
    ///6: division by 7
    Div7 = 6,
    ///7: division by 8
    Div8 = 7,
    ///8: division by 9
    Div9 = 8,
    ///9: division by 10
    Div10 = 9,
    ///10: division by 11
    Div11 = 10,
    ///11: division by 12
    Div12 = 11,
    ///12: division by 13
    Div13 = 12,
    ///13: division by 14
    Div14 = 13,
    ///14: division by 15
    Div15 = 14,
    ///15: division by 16
    Div16 = 15,
}
impl From<PLL1M> for u8 {
    #[inline(always)]
    fn from(variant: PLL1M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1M {
    type Ux = u8;
}
impl crate::IsEnum for PLL1M {}
///Field `PLL1M` reader - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_R = crate::FieldReader<PLL1M>;
impl PLL1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1M {
        match self.bits {
            0 => PLL1M::Div1,
            1 => PLL1M::Div2,
            2 => PLL1M::Div3,
            3 => PLL1M::Div4,
            4 => PLL1M::Div5,
            5 => PLL1M::Div6,
            6 => PLL1M::Div7,
            7 => PLL1M::Div8,
            8 => PLL1M::Div9,
            9 => PLL1M::Div10,
            10 => PLL1M::Div11,
            11 => PLL1M::Div12,
            12 => PLL1M::Div13,
            13 => PLL1M::Div14,
            14 => PLL1M::Div15,
            15 => PLL1M::Div16,
            _ => unreachable!(),
        }
    }
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL1M::Div1
    }
    ///division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL1M::Div2
    }
    ///division by 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLL1M::Div3
    }
    ///division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL1M::Div4
    }
    ///division by 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLL1M::Div5
    }
    ///division by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL1M::Div6
    }
    ///division by 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLL1M::Div7
    }
    ///division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL1M::Div8
    }
    ///division by 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLL1M::Div9
    }
    ///division by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL1M::Div10
    }
    ///division by 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLL1M::Div11
    }
    ///division by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL1M::Div12
    }
    ///division by 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLL1M::Div13
    }
    ///division by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL1M::Div14
    }
    ///division by 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLL1M::Div15
    }
    ///division by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL1M::Div16
    }
}
///Field `PLL1M` writer - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLL1M, crate::Safe>;
impl<'a, REG> PLL1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div1)
    }
    ///division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div2)
    }
    ///division by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div3)
    }
    ///division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div4)
    }
    ///division by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div5)
    }
    ///division by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div6)
    }
    ///division by 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div7)
    }
    ///division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div8)
    }
    ///division by 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div9)
    }
    ///division by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div10)
    }
    ///division by 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div11)
    }
    ///division by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div12)
    }
    ///division by 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div13)
    }
    ///division by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div14)
    }
    ///division by 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div15)
    }
    ///division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M::Div16)
    }
}
/**Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1MBOOST {
    ///0: division by 1 (bypass)
    Div1 = 0,
    ///1: division by 2
    Div2 = 1,
    ///2: division by 4
    Div4 = 2,
    ///3: division by 6
    Div6 = 3,
    ///4: division by 8
    Div8 = 4,
    ///5: division by 10
    Div10 = 5,
    ///6: division by 12
    Div12 = 6,
    ///7: division by 14
    Div14 = 7,
    ///8: division by 16
    Div16 = 8,
}
impl From<PLL1MBOOST> for u8 {
    #[inline(always)]
    fn from(variant: PLL1MBOOST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1MBOOST {
    type Ux = u8;
}
impl crate::IsEnum for PLL1MBOOST {}
///Field `PLL1MBOOST` reader - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
pub type PLL1MBOOST_R = crate::FieldReader<PLL1MBOOST>;
impl PLL1MBOOST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1MBOOST> {
        match self.bits {
            0 => Some(PLL1MBOOST::Div1),
            1 => Some(PLL1MBOOST::Div2),
            2 => Some(PLL1MBOOST::Div4),
            3 => Some(PLL1MBOOST::Div6),
            4 => Some(PLL1MBOOST::Div8),
            5 => Some(PLL1MBOOST::Div10),
            6 => Some(PLL1MBOOST::Div12),
            7 => Some(PLL1MBOOST::Div14),
            8 => Some(PLL1MBOOST::Div16),
            _ => None,
        }
    }
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL1MBOOST::Div1
    }
    ///division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL1MBOOST::Div2
    }
    ///division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL1MBOOST::Div4
    }
    ///division by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL1MBOOST::Div6
    }
    ///division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL1MBOOST::Div8
    }
    ///division by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL1MBOOST::Div10
    }
    ///division by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL1MBOOST::Div12
    }
    ///division by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL1MBOOST::Div14
    }
    ///division by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL1MBOOST::Div16
    }
}
///Field `PLL1MBOOST` writer - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
pub type PLL1MBOOST_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLL1MBOOST>;
impl<'a, REG> PLL1MBOOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div1)
    }
    ///division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div2)
    }
    ///division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div4)
    }
    ///division by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div6)
    }
    ///division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div8)
    }
    ///division by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div10)
    }
    ///division by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div12)
    }
    ///division by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div14)
    }
    ///division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1MBOOST::Div16)
    }
}
/**PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1PEN {
    ///0: pllx_p_ck output disabled
    Disabled = 0,
    ///1: pllx_p_ck output enabled
    Enabled = 1,
}
impl From<PLL1PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1PEN` reader - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
pub type PLL1PEN_R = crate::BitReader<PLL1PEN>;
impl PLL1PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1PEN {
        match self.bits {
            false => PLL1PEN::Disabled,
            true => PLL1PEN::Enabled,
        }
    }
    ///pllx_p_ck output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL1PEN::Disabled
    }
    ///pllx_p_ck output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL1PEN::Enabled
    }
}
///Field `PLL1PEN` writer - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
pub type PLL1PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1PEN>;
impl<'a, REG> PLL1PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_p_ck output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN::Disabled)
    }
    ///pllx_p_ck output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN::Enabled)
    }
}
/**PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1QEN {
    ///0: pllx_q_ck output disabled
    Disabled = 0,
    ///1: pllx_q_ck output enabled
    Enabled = 1,
}
impl From<PLL1QEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1QEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1QEN` reader - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
pub type PLL1QEN_R = crate::BitReader<PLL1QEN>;
impl PLL1QEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1QEN {
        match self.bits {
            false => PLL1QEN::Disabled,
            true => PLL1QEN::Enabled,
        }
    }
    ///pllx_q_ck output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL1QEN::Disabled
    }
    ///pllx_q_ck output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL1QEN::Enabled
    }
}
///Field `PLL1QEN` writer - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
pub type PLL1QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1QEN>;
impl<'a, REG> PLL1QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_q_ck output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1QEN::Disabled)
    }
    ///pllx_q_ck output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1QEN::Enabled)
    }
}
/**PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1REN {
    ///0: pllx_r_ck ready interrupt disabled
    Disabled = 0,
    ///1: pllx_r_ck ready interrupt enabled
    Enabled = 1,
}
impl From<PLL1REN> for bool {
    #[inline(always)]
    fn from(variant: PLL1REN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1REN` reader - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
pub type PLL1REN_R = crate::BitReader<PLL1REN>;
impl PLL1REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1REN {
        match self.bits {
            false => PLL1REN::Disabled,
            true => PLL1REN::Enabled,
        }
    }
    ///pllx_r_ck ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL1REN::Disabled
    }
    ///pllx_r_ck ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL1REN::Enabled
    }
}
///Field `PLL1REN` writer - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
pub type PLL1REN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1REN>;
impl<'a, REG> PLL1REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_r_ck ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1REN::Disabled)
    }
    ///pllx_r_ck ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1REN::Enabled)
    }
}
impl R {
    ///Bits 0:1 - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1m(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
    #[inline(always)]
    pub fn pll1mboost(&self) -> PLL1MBOOST_R {
        PLL1MBOOST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR")
            .field("pll1src", &self.pll1src())
            .field("pll1rge", &self.pll1rge())
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1m", &self.pll1m())
            .field("pll1mboost", &self.pll1mboost())
            .field("pll1pen", &self.pll1pen())
            .field("pll1qen", &self.pll1qen())
            .field("pll1ren", &self.pll1ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
    #[inline(always)]
    pub fn pll1src(&mut self) -> PLL1SRC_W<PLL1CFGRrs> {
        PLL1SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<PLL1CFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<PLL1CFGRrs> {
        PLL1FRACEN_W::new(self, 4)
    }
    ///Bits 8:11 - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1m(&mut self) -> PLL1M_W<PLL1CFGRrs> {
        PLL1M_W::new(self, 8)
    }
    ///Bits 12:15 - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
    #[inline(always)]
    pub fn pll1mboost(&mut self) -> PLL1MBOOST_W<PLL1CFGRrs> {
        PLL1MBOOST_W::new(self, 12)
    }
    ///Bit 16 - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
    #[inline(always)]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<PLL1CFGRrs> {
        PLL1PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
    #[inline(always)]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<PLL1CFGRrs> {
        PLL1QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
    #[inline(always)]
    pub fn pll1ren(&mut self) -> PLL1REN_W<PLL1CFGRrs> {
        PLL1REN_W::new(self, 18)
    }
}
/**RCC PLL1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL1CFGR)*/
pub struct PLL1CFGRrs;
impl crate::RegisterSpec for PLL1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr::R`](R) reader structure
impl crate::Readable for PLL1CFGRrs {}
///`write(|w| ..)` method takes [`pll1cfgr::W`](W) writer structure
impl crate::Writable for PLL1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR to value 0
impl crate::Resettable for PLL1CFGRrs {}
