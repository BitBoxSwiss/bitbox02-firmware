///Register `PLL2CFGR` reader
pub type R = crate::R<PLL2CFGRrs>;
///Register `PLL2CFGR` writer
pub type W = crate::W<PLL2CFGRrs>;
/**PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2SRC {
    ///0: No clock sent to PLLx
    NoClock = 0,
    ///1: MSIS clock selected as PLLx clock entry
    Msis = 1,
    ///2: HSI16 clock selected as PLLx clock entry
    Hsi16 = 2,
    ///3: HSE clock selected as PLLx clock entry
    Hse = 3,
}
impl From<PLL2SRC> for u8 {
    #[inline(always)]
    fn from(variant: PLL2SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2SRC {
    type Ux = u8;
}
impl crate::IsEnum for PLL2SRC {}
///Field `PLL2SRC` reader - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
pub type PLL2SRC_R = crate::FieldReader<PLL2SRC>;
impl PLL2SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2SRC {
        match self.bits {
            0 => PLL2SRC::NoClock,
            1 => PLL2SRC::Msis,
            2 => PLL2SRC::Hsi16,
            3 => PLL2SRC::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock sent to PLLx
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLL2SRC::NoClock
    }
    ///MSIS clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == PLL2SRC::Msis
    }
    ///HSI16 clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLL2SRC::Hsi16
    }
    ///HSE clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL2SRC::Hse
    }
}
///Field `PLL2SRC` writer - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
pub type PLL2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL2SRC, crate::Safe>;
impl<'a, REG> PLL2SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to PLLx
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::NoClock)
    }
    ///MSIS clock selected as PLLx clock entry
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::Msis)
    }
    ///HSI16 clock selected as PLLx clock entry
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::Hsi16)
    }
    ///HSE clock selected as PLLx clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::Hse)
    }
}
/**PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2RGE {
    ///3: PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    Range2 = 3,
    ///0: PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    Range1 = 0,
}
impl From<PLL2RGE> for u8 {
    #[inline(always)]
    fn from(variant: PLL2RGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2RGE {
    type Ux = u8;
}
impl crate::IsEnum for PLL2RGE {}
///Field `PLL2RGE` reader - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
pub type PLL2RGE_R = crate::FieldReader<PLL2RGE>;
impl PLL2RGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RGE {
        match self.bits {
            3 => PLL2RGE::Range2,
            _ => PLL2RGE::Range1,
        }
    }
    ///PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL2RGE::Range2
    }
    ///PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        matches!(self.variant(), PLL2RGE::Range1)
    }
}
///Field `PLL2RGE` writer - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
pub type PLL2RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL2RGE, crate::Safe>;
impl<'a, REG> PLL2RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE::Range2)
    }
    ///PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE::Range1)
    }
}
/**PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2FRACEN {
    ///0: No effect
    NoEffect = 0,
    ///1: Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    Latch = 1,
}
impl From<PLL2FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL2FRACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2FRACEN` reader - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
pub type PLL2FRACEN_R = crate::BitReader<PLL2FRACEN>;
impl PLL2FRACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2FRACEN {
        match self.bits {
            false => PLL2FRACEN::NoEffect,
            true => PLL2FRACEN::Latch,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PLL2FRACEN::NoEffect
    }
    ///Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == PLL2FRACEN::Latch
    }
}
///Field `PLL2FRACEN` writer - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
pub type PLL2FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2FRACEN>;
impl<'a, REG> PLL2FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2FRACEN::NoEffect)
    }
    ///Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2FRACEN::Latch)
    }
}
/**Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2M {
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
impl From<PLL2M> for u8 {
    #[inline(always)]
    fn from(variant: PLL2M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2M {
    type Ux = u8;
}
impl crate::IsEnum for PLL2M {}
///Field `PLL2M` reader - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2M_R = crate::FieldReader<PLL2M>;
impl PLL2M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2M {
        match self.bits {
            0 => PLL2M::Div1,
            1 => PLL2M::Div2,
            2 => PLL2M::Div3,
            3 => PLL2M::Div4,
            4 => PLL2M::Div5,
            5 => PLL2M::Div6,
            6 => PLL2M::Div7,
            7 => PLL2M::Div8,
            8 => PLL2M::Div9,
            9 => PLL2M::Div10,
            10 => PLL2M::Div11,
            11 => PLL2M::Div12,
            12 => PLL2M::Div13,
            13 => PLL2M::Div14,
            14 => PLL2M::Div15,
            15 => PLL2M::Div16,
            _ => unreachable!(),
        }
    }
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL2M::Div1
    }
    ///division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL2M::Div2
    }
    ///division by 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLL2M::Div3
    }
    ///division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL2M::Div4
    }
    ///division by 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLL2M::Div5
    }
    ///division by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL2M::Div6
    }
    ///division by 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLL2M::Div7
    }
    ///division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL2M::Div8
    }
    ///division by 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLL2M::Div9
    }
    ///division by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL2M::Div10
    }
    ///division by 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLL2M::Div11
    }
    ///division by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL2M::Div12
    }
    ///division by 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLL2M::Div13
    }
    ///division by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL2M::Div14
    }
    ///division by 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLL2M::Div15
    }
    ///division by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL2M::Div16
    }
}
///Field `PLL2M` writer - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2M_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLL2M, crate::Safe>;
impl<'a, REG> PLL2M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div1)
    }
    ///division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div2)
    }
    ///division by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div3)
    }
    ///division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div4)
    }
    ///division by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div5)
    }
    ///division by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div6)
    }
    ///division by 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div7)
    }
    ///division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div8)
    }
    ///division by 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div9)
    }
    ///division by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div10)
    }
    ///division by 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div11)
    }
    ///division by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div12)
    }
    ///division by 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div13)
    }
    ///division by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div14)
    }
    ///division by 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div15)
    }
    ///division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M::Div16)
    }
}
/**PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2PEN {
    ///0: pllx_p_ck output disabled
    Disabled = 0,
    ///1: pllx_p_ck output enabled
    Enabled = 1,
}
impl From<PLL2PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL2PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2PEN` reader - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
pub type PLL2PEN_R = crate::BitReader<PLL2PEN>;
impl PLL2PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2PEN {
        match self.bits {
            false => PLL2PEN::Disabled,
            true => PLL2PEN::Enabled,
        }
    }
    ///pllx_p_ck output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL2PEN::Disabled
    }
    ///pllx_p_ck output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL2PEN::Enabled
    }
}
///Field `PLL2PEN` writer - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
pub type PLL2PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2PEN>;
impl<'a, REG> PLL2PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_p_ck output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2PEN::Disabled)
    }
    ///pllx_p_ck output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2PEN::Enabled)
    }
}
/**PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2QEN {
    ///0: pllx_q_ck output disabled
    Disabled = 0,
    ///1: pllx_q_ck output enabled
    Enabled = 1,
}
impl From<PLL2QEN> for bool {
    #[inline(always)]
    fn from(variant: PLL2QEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2QEN` reader - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
pub type PLL2QEN_R = crate::BitReader<PLL2QEN>;
impl PLL2QEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2QEN {
        match self.bits {
            false => PLL2QEN::Disabled,
            true => PLL2QEN::Enabled,
        }
    }
    ///pllx_q_ck output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL2QEN::Disabled
    }
    ///pllx_q_ck output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL2QEN::Enabled
    }
}
///Field `PLL2QEN` writer - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
pub type PLL2QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2QEN>;
impl<'a, REG> PLL2QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_q_ck output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2QEN::Disabled)
    }
    ///pllx_q_ck output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2QEN::Enabled)
    }
}
/**PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2REN {
    ///0: pllx_r_ck ready interrupt disabled
    Disabled = 0,
    ///1: pllx_r_ck ready interrupt enabled
    Enabled = 1,
}
impl From<PLL2REN> for bool {
    #[inline(always)]
    fn from(variant: PLL2REN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2REN` reader - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
pub type PLL2REN_R = crate::BitReader<PLL2REN>;
impl PLL2REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2REN {
        match self.bits {
            false => PLL2REN::Disabled,
            true => PLL2REN::Enabled,
        }
    }
    ///pllx_r_ck ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL2REN::Disabled
    }
    ///pllx_r_ck ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL2REN::Enabled
    }
}
///Field `PLL2REN` writer - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
pub type PLL2REN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2REN>;
impl<'a, REG> PLL2REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_r_ck ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2REN::Disabled)
    }
    ///pllx_r_ck ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2REN::Enabled)
    }
}
impl R {
    ///Bits 0:1 - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
    #[inline(always)]
    pub fn pll2src(&self) -> PLL2SRC_R {
        PLL2SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2m(&self) -> PLL2M_R {
        PLL2M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
    #[inline(always)]
    pub fn pll2pen(&self) -> PLL2PEN_R {
        PLL2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
    #[inline(always)]
    pub fn pll2qen(&self) -> PLL2QEN_R {
        PLL2QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
    #[inline(always)]
    pub fn pll2ren(&self) -> PLL2REN_R {
        PLL2REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CFGR")
            .field("pll2src", &self.pll2src())
            .field("pll2rge", &self.pll2rge())
            .field("pll2fracen", &self.pll2fracen())
            .field("pll2m", &self.pll2m())
            .field("pll2pen", &self.pll2pen())
            .field("pll2qen", &self.pll2qen())
            .field("pll2ren", &self.pll2ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
    #[inline(always)]
    pub fn pll2src(&mut self) -> PLL2SRC_W<PLL2CFGRrs> {
        PLL2SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<PLL2CFGRrs> {
        PLL2RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<PLL2CFGRrs> {
        PLL2FRACEN_W::new(self, 4)
    }
    ///Bits 8:11 - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2m(&mut self) -> PLL2M_W<PLL2CFGRrs> {
        PLL2M_W::new(self, 8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
    #[inline(always)]
    pub fn pll2pen(&mut self) -> PLL2PEN_W<PLL2CFGRrs> {
        PLL2PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
    #[inline(always)]
    pub fn pll2qen(&mut self) -> PLL2QEN_W<PLL2CFGRrs> {
        PLL2QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
    #[inline(always)]
    pub fn pll2ren(&mut self) -> PLL2REN_W<PLL2CFGRrs> {
        PLL2REN_W::new(self, 18)
    }
}
/**RCC PLL2 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:PLL2CFGR)*/
pub struct PLL2CFGRrs;
impl crate::RegisterSpec for PLL2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2cfgr::R`](R) reader structure
impl crate::Readable for PLL2CFGRrs {}
///`write(|w| ..)` method takes [`pll2cfgr::W`](W) writer structure
impl crate::Writable for PLL2CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CFGR to value 0
impl crate::Resettable for PLL2CFGRrs {}
