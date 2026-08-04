///Register `PLL3CFGR` reader
pub type R = crate::R<PLL3CFGRrs>;
///Register `PLL3CFGR` writer
pub type W = crate::W<PLL3CFGRrs>;
/**PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3SRC {
    ///0: No clock sent to PLLx
    NoClock = 0,
    ///1: MSIS clock selected as PLLx clock entry
    Msis = 1,
    ///2: HSI16 clock selected as PLLx clock entry
    Hsi16 = 2,
    ///3: HSE clock selected as PLLx clock entry
    Hse = 3,
}
impl From<PLL3SRC> for u8 {
    #[inline(always)]
    fn from(variant: PLL3SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL3SRC {
    type Ux = u8;
}
impl crate::IsEnum for PLL3SRC {}
///Field `PLL3SRC` reader - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
pub type PLL3SRC_R = crate::FieldReader<PLL3SRC>;
impl PLL3SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3SRC {
        match self.bits {
            0 => PLL3SRC::NoClock,
            1 => PLL3SRC::Msis,
            2 => PLL3SRC::Hsi16,
            3 => PLL3SRC::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock sent to PLLx
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLL3SRC::NoClock
    }
    ///MSIS clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == PLL3SRC::Msis
    }
    ///HSI16 clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLL3SRC::Hsi16
    }
    ///HSE clock selected as PLLx clock entry
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL3SRC::Hse
    }
}
///Field `PLL3SRC` writer - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
pub type PLL3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL3SRC, crate::Safe>;
impl<'a, REG> PLL3SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to PLLx
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::NoClock)
    }
    ///MSIS clock selected as PLLx clock entry
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Msis)
    }
    ///HSI16 clock selected as PLLx clock entry
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Hsi16)
    }
    ///HSE clock selected as PLLx clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Hse)
    }
}
/**PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3RGE {
    ///3: PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    Range2 = 3,
    ///0: PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    Range1 = 0,
}
impl From<PLL3RGE> for u8 {
    #[inline(always)]
    fn from(variant: PLL3RGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL3RGE {
    type Ux = u8;
}
impl crate::IsEnum for PLL3RGE {}
///Field `PLL3RGE` reader - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
pub type PLL3RGE_R = crate::FieldReader<PLL3RGE>;
impl PLL3RGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3RGE {
        match self.bits {
            3 => PLL3RGE::Range2,
            _ => PLL3RGE::Range1,
        }
    }
    ///PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL3RGE::Range2
    }
    ///PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        matches!(self.variant(), PLL3RGE::Range1)
    }
}
///Field `PLL3RGE` writer - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
pub type PLL3RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL3RGE, crate::Safe>;
impl<'a, REG> PLL3RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLx input (refx_ck) clock range frequency between 8 and 16 MHz
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range2)
    }
    ///PLLx input (refx_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range1)
    }
}
/**PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3FRACEN {
    ///0: No effect
    NoEffect = 0,
    ///1: Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    Latch = 1,
}
impl From<PLL3FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3FRACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
pub type PLL3FRACEN_R = crate::BitReader<PLL3FRACEN>;
impl PLL3FRACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3FRACEN {
        match self.bits {
            false => PLL3FRACEN::NoEffect,
            true => PLL3FRACEN::Latch,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PLL3FRACEN::NoEffect
    }
    ///Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == PLL3FRACEN::Latch
    }
}
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
pub type PLL3FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3FRACEN>;
impl<'a, REG> PLL3FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3FRACEN::NoEffect)
    }
    ///Content of PLLxFRACN latched in the Σ∆ modulator on PLLxFRACEN transition from 0 to 1
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3FRACEN::Latch)
    }
}
/**Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3M {
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
impl From<PLL3M> for u8 {
    #[inline(always)]
    fn from(variant: PLL3M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL3M {
    type Ux = u8;
}
impl crate::IsEnum for PLL3M {}
///Field `PLL3M` reader - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3M_R = crate::FieldReader<PLL3M>;
impl PLL3M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3M {
        match self.bits {
            0 => PLL3M::Div1,
            1 => PLL3M::Div2,
            2 => PLL3M::Div3,
            3 => PLL3M::Div4,
            4 => PLL3M::Div5,
            5 => PLL3M::Div6,
            6 => PLL3M::Div7,
            7 => PLL3M::Div8,
            8 => PLL3M::Div9,
            9 => PLL3M::Div10,
            10 => PLL3M::Div11,
            11 => PLL3M::Div12,
            12 => PLL3M::Div13,
            13 => PLL3M::Div14,
            14 => PLL3M::Div15,
            15 => PLL3M::Div16,
            _ => unreachable!(),
        }
    }
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL3M::Div1
    }
    ///division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL3M::Div2
    }
    ///division by 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLL3M::Div3
    }
    ///division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL3M::Div4
    }
    ///division by 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLL3M::Div5
    }
    ///division by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL3M::Div6
    }
    ///division by 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLL3M::Div7
    }
    ///division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL3M::Div8
    }
    ///division by 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLL3M::Div9
    }
    ///division by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL3M::Div10
    }
    ///division by 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLL3M::Div11
    }
    ///division by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL3M::Div12
    }
    ///division by 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLL3M::Div13
    }
    ///division by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL3M::Div14
    }
    ///division by 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLL3M::Div15
    }
    ///division by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL3M::Div16
    }
}
///Field `PLL3M` writer - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3M_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLL3M, crate::Safe>;
impl<'a, REG> PLL3M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///division by 1 (bypass)
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div1)
    }
    ///division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div2)
    }
    ///division by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div3)
    }
    ///division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div4)
    }
    ///division by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div5)
    }
    ///division by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div6)
    }
    ///division by 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div7)
    }
    ///division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div8)
    }
    ///division by 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div9)
    }
    ///division by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div10)
    }
    ///division by 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div11)
    }
    ///division by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div12)
    }
    ///division by 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div13)
    }
    ///division by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div14)
    }
    ///division by 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div15)
    }
    ///division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3M::Div16)
    }
}
/**PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3PEN {
    ///0: pllx_p_ck output disabled
    Disabled = 0,
    ///1: pllx_p_ck output enabled
    Enabled = 1,
}
impl From<PLL3PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3PEN` reader - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
pub type PLL3PEN_R = crate::BitReader<PLL3PEN>;
impl PLL3PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3PEN {
        match self.bits {
            false => PLL3PEN::Disabled,
            true => PLL3PEN::Enabled,
        }
    }
    ///pllx_p_ck output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL3PEN::Disabled
    }
    ///pllx_p_ck output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL3PEN::Enabled
    }
}
///Field `PLL3PEN` writer - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
pub type PLL3PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3PEN>;
impl<'a, REG> PLL3PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_p_ck output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3PEN::Disabled)
    }
    ///pllx_p_ck output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3PEN::Enabled)
    }
}
/**PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3QEN {
    ///0: pllx_q_ck output disabled
    Disabled = 0,
    ///1: pllx_q_ck output enabled
    Enabled = 1,
}
impl From<PLL3QEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3QEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3QEN` reader - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
pub type PLL3QEN_R = crate::BitReader<PLL3QEN>;
impl PLL3QEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3QEN {
        match self.bits {
            false => PLL3QEN::Disabled,
            true => PLL3QEN::Enabled,
        }
    }
    ///pllx_q_ck output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL3QEN::Disabled
    }
    ///pllx_q_ck output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL3QEN::Enabled
    }
}
///Field `PLL3QEN` writer - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
pub type PLL3QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3QEN>;
impl<'a, REG> PLL3QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_q_ck output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3QEN::Disabled)
    }
    ///pllx_q_ck output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3QEN::Enabled)
    }
}
/**PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3REN {
    ///0: pllx_r_ck ready interrupt disabled
    Disabled = 0,
    ///1: pllx_r_ck ready interrupt enabled
    Enabled = 1,
}
impl From<PLL3REN> for bool {
    #[inline(always)]
    fn from(variant: PLL3REN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3REN` reader - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
pub type PLL3REN_R = crate::BitReader<PLL3REN>;
impl PLL3REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3REN {
        match self.bits {
            false => PLL3REN::Disabled,
            true => PLL3REN::Enabled,
        }
    }
    ///pllx_r_ck ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL3REN::Disabled
    }
    ///pllx_r_ck ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL3REN::Enabled
    }
}
///Field `PLL3REN` writer - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
pub type PLL3REN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3REN>;
impl<'a, REG> PLL3REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pllx_r_ck ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3REN::Disabled)
    }
    ///pllx_r_ck ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3REN::Enabled)
    }
}
impl R {
    ///Bits 0:1 - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3m(&self) -> PLL3M_R {
        PLL3M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CFGR")
            .field("pll3src", &self.pll3src())
            .field("pll3rge", &self.pll3rge())
            .field("pll3fracen", &self.pll3fracen())
            .field("pll3m", &self.pll3m())
            .field("pll3pen", &self.pll3pen())
            .field("pll3qen", &self.pll3qen())
            .field("pll3ren", &self.pll3ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
    #[inline(always)]
    pub fn pll3src(&mut self) -> PLL3SRC_W<PLL3CFGRrs> {
        PLL3SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<PLL3CFGRrs> {
        PLL3RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<PLL3CFGRrs> {
        PLL3FRACEN_W::new(self, 4)
    }
    ///Bits 8:11 - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3m(&mut self) -> PLL3M_W<PLL3CFGRrs> {
        PLL3M_W::new(self, 8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
    #[inline(always)]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<PLL3CFGRrs> {
        PLL3PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
    #[inline(always)]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<PLL3CFGRrs> {
        PLL3QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
    #[inline(always)]
    pub fn pll3ren(&mut self) -> PLL3REN_W<PLL3CFGRrs> {
        PLL3REN_W::new(self, 18)
    }
}
/**RCC PLL3 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL3CFGR)*/
pub struct PLL3CFGRrs;
impl crate::RegisterSpec for PLL3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll3cfgr::R`](R) reader structure
impl crate::Readable for PLL3CFGRrs {}
///`write(|w| ..)` method takes [`pll3cfgr::W`](W) writer structure
impl crate::Writable for PLL3CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CFGR to value 0
impl crate::Resettable for PLL3CFGRrs {}
