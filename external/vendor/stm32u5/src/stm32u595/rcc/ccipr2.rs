///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
/**MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDF1SEL {
    ///0: HCLK selected
    Hclk = 0,
    ///1: PLL1 "P" (pll1_p_ck) selected
    Pll1p = 1,
    ///2: PLL3 "Q" (pll3_q_ck) selected
    Pll3q = 2,
    ///3: input pin AUDIOCLK selected
    Audioclk = 3,
    ///4: MSIK clock selected
    Msik = 4,
}
impl From<MDF1SEL> for u8 {
    #[inline(always)]
    fn from(variant: MDF1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDF1SEL {
    type Ux = u8;
}
impl crate::IsEnum for MDF1SEL {}
///Field `MDF1SEL` reader - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
pub type MDF1SEL_R = crate::FieldReader<MDF1SEL>;
impl MDF1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MDF1SEL> {
        match self.bits {
            0 => Some(MDF1SEL::Hclk),
            1 => Some(MDF1SEL::Pll1p),
            2 => Some(MDF1SEL::Pll3q),
            3 => Some(MDF1SEL::Audioclk),
            4 => Some(MDF1SEL::Msik),
            _ => None,
        }
    }
    ///HCLK selected
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MDF1SEL::Hclk
    }
    ///PLL1 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn is_pll1p(&self) -> bool {
        *self == MDF1SEL::Pll1p
    }
    ///PLL3 "Q" (pll3_q_ck) selected
    #[inline(always)]
    pub fn is_pll3q(&self) -> bool {
        *self == MDF1SEL::Pll3q
    }
    ///input pin AUDIOCLK selected
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == MDF1SEL::Audioclk
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == MDF1SEL::Msik
    }
}
///Field `MDF1SEL` writer - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
pub type MDF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MDF1SEL>;
impl<'a, REG> MDF1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK selected
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MDF1SEL::Hclk)
    }
    ///PLL1 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn pll1p(self) -> &'a mut crate::W<REG> {
        self.variant(MDF1SEL::Pll1p)
    }
    ///PLL3 "Q" (pll3_q_ck) selected
    #[inline(always)]
    pub fn pll3q(self) -> &'a mut crate::W<REG> {
        self.variant(MDF1SEL::Pll3q)
    }
    ///input pin AUDIOCLK selected
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(MDF1SEL::Audioclk)
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(MDF1SEL::Msik)
    }
}
/**SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    ///0: PLL2 "P" (pll2_p_ck) selected
    Pll2p = 0,
    ///1: PLL3 "P" (pll3_p_ck) selected
    Pll3p = 1,
    ///2: PLL1 "P" (pll1_p_ck) selected
    Pll1p = 2,
    ///3: input pin AUDIOCLK selected
    Audioclk = 3,
    ///4: HSI16 clock selected
    Hsi16 = 4,
}
impl From<SAI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for SAI1SEL {}
///Field `SAI1SEL` reader - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI1SEL> {
        match self.bits {
            0 => Some(SAI1SEL::Pll2p),
            1 => Some(SAI1SEL::Pll3p),
            2 => Some(SAI1SEL::Pll1p),
            3 => Some(SAI1SEL::Audioclk),
            4 => Some(SAI1SEL::Hsi16),
            _ => None,
        }
    }
    ///PLL2 "P" (pll2_p_ck) selected
    #[inline(always)]
    pub fn is_pll2p(&self) -> bool {
        *self == SAI1SEL::Pll2p
    }
    ///PLL3 "P" (pll3_p_ck) selected
    #[inline(always)]
    pub fn is_pll3p(&self) -> bool {
        *self == SAI1SEL::Pll3p
    }
    ///PLL1 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn is_pll1p(&self) -> bool {
        *self == SAI1SEL::Pll1p
    }
    ///input pin AUDIOCLK selected
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == SAI1SEL::Audioclk
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SAI1SEL::Hsi16
    }
}
///Field `SAI1SEL` writer - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI1SEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL2 "P" (pll2_p_ck) selected
    #[inline(always)]
    pub fn pll2p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll2p)
    }
    ///PLL3 "P" (pll3_p_ck) selected
    #[inline(always)]
    pub fn pll3p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll3p)
    }
    ///PLL1 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn pll1p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll1p)
    }
    ///input pin AUDIOCLK selected
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Audioclk)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Hsi16)
    }
}
///Field `SAI2SEL` reader - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use SAI1SEL_R as SAI2SEL_R;
///Field `SAI2SEL` writer - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use SAI1SEL_W as SAI2SEL_W;
/**SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAESSEL {
    ///0: SHSI selected
    Shsi = 0,
    ///1: SHSI / 2 selected, can be used in range 4
    ShsiDiv2 = 1,
}
impl From<SAESSEL> for bool {
    #[inline(always)]
    fn from(variant: SAESSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SAESSEL` reader - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SAESSEL_R = crate::BitReader<SAESSEL>;
impl SAESSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAESSEL {
        match self.bits {
            false => SAESSEL::Shsi,
            true => SAESSEL::ShsiDiv2,
        }
    }
    ///SHSI selected
    #[inline(always)]
    pub fn is_shsi(&self) -> bool {
        *self == SAESSEL::Shsi
    }
    ///SHSI / 2 selected, can be used in range 4
    #[inline(always)]
    pub fn is_shsi_div2(&self) -> bool {
        *self == SAESSEL::ShsiDiv2
    }
}
///Field `SAESSEL` writer - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SAESSEL_W<'a, REG> = crate::BitWriter<'a, REG, SAESSEL>;
impl<'a, REG> SAESSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SHSI selected
    #[inline(always)]
    pub fn shsi(self) -> &'a mut crate::W<REG> {
        self.variant(SAESSEL::Shsi)
    }
    ///SHSI / 2 selected, can be used in range 4
    #[inline(always)]
    pub fn shsi_div2(self) -> &'a mut crate::W<REG> {
        self.variant(SAESSEL::ShsiDiv2)
    }
}
/**RNG kernel clock source selection These bits are used to select the RNG kernel clock source.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    ///0: HSI48 clock selected
    Hsi48 = 0,
    ///1: HSI48 / 2 selected, can be used in range 4
    Hsi48Div2 = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
}
impl From<RNGSEL> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNGSEL {
    type Ux = u8;
}
impl crate::IsEnum for RNGSEL {}
///Field `RNGSEL` reader - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNGSEL> {
        match self.bits {
            0 => Some(RNGSEL::Hsi48),
            1 => Some(RNGSEL::Hsi48Div2),
            2 => Some(RNGSEL::Hsi16),
            _ => None,
        }
    }
    ///HSI48 clock selected
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == RNGSEL::Hsi48
    }
    ///HSI48 / 2 selected, can be used in range 4
    #[inline(always)]
    pub fn is_hsi48_div2(&self) -> bool {
        *self == RNGSEL::Hsi48Div2
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == RNGSEL::Hsi16
    }
}
///Field `RNGSEL` writer - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RNGSEL>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI48 clock selected
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi48)
    }
    ///HSI48 / 2 selected, can be used in range 4
    #[inline(always)]
    pub fn hsi48_div2(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi48Div2)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi16)
    }
}
/**SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL {
    ///0: ICLK clock selected
    Iclk = 0,
    ///1: PLL1 "P" (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)
    Pll1p = 1,
}
impl From<SDMMCSEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMCSEL` reader - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL>;
impl SDMMCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMCSEL {
        match self.bits {
            false => SDMMCSEL::Iclk,
            true => SDMMCSEL::Pll1p,
        }
    }
    ///ICLK clock selected
    #[inline(always)]
    pub fn is_iclk(&self) -> bool {
        *self == SDMMCSEL::Iclk
    }
    ///PLL1 "P" (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)
    #[inline(always)]
    pub fn is_pll1p(&self) -> bool {
        *self == SDMMCSEL::Pll1p
    }
}
///Field `SDMMCSEL` writer - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMCSEL>;
impl<'a, REG> SDMMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ICLK clock selected
    #[inline(always)]
    pub fn iclk(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Iclk)
    }
    ///PLL1 "P" (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)
    #[inline(always)]
    pub fn pll1p(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll1p)
    }
}
/**DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISEL {
    ///0: PLL3 "P" (pll3_p_ck) selected
    Pll3p = 0,
    ///1: DSI PHY PLL output selected
    DsiPhyPll = 1,
}
impl From<DSISEL> for bool {
    #[inline(always)]
    fn from(variant: DSISEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DSISEL` reader - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
pub type DSISEL_R = crate::BitReader<DSISEL>;
impl DSISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSISEL {
        match self.bits {
            false => DSISEL::Pll3p,
            true => DSISEL::DsiPhyPll,
        }
    }
    ///PLL3 "P" (pll3_p_ck) selected
    #[inline(always)]
    pub fn is_pll3p(&self) -> bool {
        *self == DSISEL::Pll3p
    }
    ///DSI PHY PLL output selected
    #[inline(always)]
    pub fn is_dsi_phy_pll(&self) -> bool {
        *self == DSISEL::DsiPhyPll
    }
}
///Field `DSISEL` writer - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
pub type DSISEL_W<'a, REG> = crate::BitWriter<'a, REG, DSISEL>;
impl<'a, REG> DSISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL3 "P" (pll3_p_ck) selected
    #[inline(always)]
    pub fn pll3p(self) -> &'a mut crate::W<REG> {
        self.variant(DSISEL::Pll3p)
    }
    ///DSI PHY PLL output selected
    #[inline(always)]
    pub fn dsi_phy_pll(self) -> &'a mut crate::W<REG> {
        self.variant(DSISEL::DsiPhyPll)
    }
}
/**USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART6SEL {
    ///0: PCLK1 selected
    Pclk1 = 0,
    ///1: SYSCLK selected
    Sysclk = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
    ///3: MSIK selected
    Msik = 3,
}
impl From<USART6SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART6SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART6SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART6SEL {}
///Field `USART6SEL` reader - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type USART6SEL_R = crate::FieldReader<USART6SEL>;
impl USART6SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART6SEL {
        match self.bits {
            0 => USART6SEL::Pclk1,
            1 => USART6SEL::Sysclk,
            2 => USART6SEL::Hsi16,
            3 => USART6SEL::Msik,
            _ => unreachable!(),
        }
    }
    ///PCLK1 selected
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == USART6SEL::Pclk1
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART6SEL::Sysclk
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART6SEL::Hsi16
    }
    ///MSIK selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == USART6SEL::Msik
    }
}
///Field `USART6SEL` writer - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type USART6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART6SEL, crate::Safe>;
impl<'a, REG> USART6SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK1 selected
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART6SEL::Pclk1)
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART6SEL::Sysclk)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(USART6SEL::Hsi16)
    }
    ///MSIK selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(USART6SEL::Msik)
    }
}
/**LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCSEL {
    ///0: PLL3 "R" (pll3_r_ck) selected
    Pll3r = 0,
    ///1: PLL2 "R" (pll2_r_ck) selected
    Pll2r = 1,
}
impl From<LTDCSEL> for bool {
    #[inline(always)]
    fn from(variant: LTDCSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCSEL` reader - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type LTDCSEL_R = crate::BitReader<LTDCSEL>;
impl LTDCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCSEL {
        match self.bits {
            false => LTDCSEL::Pll3r,
            true => LTDCSEL::Pll2r,
        }
    }
    ///PLL3 "R" (pll3_r_ck) selected
    #[inline(always)]
    pub fn is_pll3r(&self) -> bool {
        *self == LTDCSEL::Pll3r
    }
    ///PLL2 "R" (pll2_r_ck) selected
    #[inline(always)]
    pub fn is_pll2r(&self) -> bool {
        *self == LTDCSEL::Pll2r
    }
}
///Field `LTDCSEL` writer - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type LTDCSEL_W<'a, REG> = crate::BitWriter<'a, REG, LTDCSEL>;
impl<'a, REG> LTDCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL3 "R" (pll3_r_ck) selected
    #[inline(always)]
    pub fn pll3r(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCSEL::Pll3r)
    }
    ///PLL2 "R" (pll2_r_ck) selected
    #[inline(always)]
    pub fn pll2r(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCSEL::Pll2r)
    }
}
/**OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCTOSPISEL {
    ///0: SYSCLK selected
    Sysclk = 0,
    ///1: MSIK selected
    Msik = 1,
    ///2: PLL1 "Q" (pll1_q_ck) selected, can be up to 200 MHz
    Pll1q = 2,
    ///3: PLL2 "Q" (pll2_q_ck) selected, can be up to 200 MHz
    Pll2q = 3,
}
impl From<OCTOSPISEL> for u8 {
    #[inline(always)]
    fn from(variant: OCTOSPISEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCTOSPISEL {
    type Ux = u8;
}
impl crate::IsEnum for OCTOSPISEL {}
///Field `OCTOSPISEL` reader - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
pub type OCTOSPISEL_R = crate::FieldReader<OCTOSPISEL>;
impl OCTOSPISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCTOSPISEL {
        match self.bits {
            0 => OCTOSPISEL::Sysclk,
            1 => OCTOSPISEL::Msik,
            2 => OCTOSPISEL::Pll1q,
            3 => OCTOSPISEL::Pll2q,
            _ => unreachable!(),
        }
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == OCTOSPISEL::Sysclk
    }
    ///MSIK selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == OCTOSPISEL::Msik
    }
    ///PLL1 "Q" (pll1_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn is_pll1q(&self) -> bool {
        *self == OCTOSPISEL::Pll1q
    }
    ///PLL2 "Q" (pll2_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn is_pll2q(&self) -> bool {
        *self == OCTOSPISEL::Pll2q
    }
}
///Field `OCTOSPISEL` writer - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
pub type OCTOSPISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OCTOSPISEL, crate::Safe>;
impl<'a, REG> OCTOSPISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPISEL::Sysclk)
    }
    ///MSIK selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPISEL::Msik)
    }
    ///PLL1 "Q" (pll1_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn pll1q(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPISEL::Pll1q)
    }
    ///PLL2 "Q" (pll2_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn pll2q(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPISEL::Pll2q)
    }
}
/**HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSPI1SEL {
    ///0: SYSCLK selected
    Sysclk = 0,
    ///1: PLL1 "Q" (pll1_q_ck) selected, can be up to 200 MHz
    Pll1q = 1,
    ///2: PLL2 "Q" (pll2_q_ck) selected, can be up to 200 MHz
    Pll2q = 2,
    ///3: PLL3 "R" (pll3_r_ck) selected, can be up to 200 MHz
    Pll3r = 3,
}
impl From<HSPI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: HSPI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSPI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for HSPI1SEL {}
///Field `HSPI1SEL` reader - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type HSPI1SEL_R = crate::FieldReader<HSPI1SEL>;
impl HSPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSPI1SEL {
        match self.bits {
            0 => HSPI1SEL::Sysclk,
            1 => HSPI1SEL::Pll1q,
            2 => HSPI1SEL::Pll2q,
            3 => HSPI1SEL::Pll3r,
            _ => unreachable!(),
        }
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == HSPI1SEL::Sysclk
    }
    ///PLL1 "Q" (pll1_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn is_pll1q(&self) -> bool {
        *self == HSPI1SEL::Pll1q
    }
    ///PLL2 "Q" (pll2_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn is_pll2q(&self) -> bool {
        *self == HSPI1SEL::Pll2q
    }
    ///PLL3 "R" (pll3_r_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn is_pll3r(&self) -> bool {
        *self == HSPI1SEL::Pll3r
    }
}
///Field `HSPI1SEL` writer - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type HSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HSPI1SEL, crate::Safe>;
impl<'a, REG> HSPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(HSPI1SEL::Sysclk)
    }
    ///PLL1 "Q" (pll1_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn pll1q(self) -> &'a mut crate::W<REG> {
        self.variant(HSPI1SEL::Pll1q)
    }
    ///PLL2 "Q" (pll2_q_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn pll2q(self) -> &'a mut crate::W<REG> {
        self.variant(HSPI1SEL::Pll2q)
    }
    ///PLL3 "R" (pll3_r_ck) selected, can be up to 200 MHz
    #[inline(always)]
    pub fn pll3r(self) -> &'a mut crate::W<REG> {
        self.variant(HSPI1SEL::Pll3r)
    }
}
///Field `I2C5SEL` reader - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use USART6SEL_R as I2C5SEL_R;
///Field `I2C6SEL` reader - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use USART6SEL_R as I2C6SEL_R;
///Field `I2C5SEL` writer - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use USART6SEL_W as I2C5SEL_W;
///Field `I2C6SEL` writer - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use USART6SEL_W as I2C6SEL_W;
/**OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OTGHSSEL {
    ///0: HSE selected
    Hse = 0,
    ///1: PLL1 "Q" (pll1_q_ck) selected
    Pll1p = 1,
    ///2: HSE/2 selected
    Hse2 = 2,
    ///3: PLL1 "P" divided by 2 (pll1_p_ck/2) selected
    Pll1pDiv2 = 3,
}
impl From<OTGHSSEL> for u8 {
    #[inline(always)]
    fn from(variant: OTGHSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OTGHSSEL {
    type Ux = u8;
}
impl crate::IsEnum for OTGHSSEL {}
///Field `OTGHSSEL` reader - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type OTGHSSEL_R = crate::FieldReader<OTGHSSEL>;
impl OTGHSSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OTGHSSEL {
        match self.bits {
            0 => OTGHSSEL::Hse,
            1 => OTGHSSEL::Pll1p,
            2 => OTGHSSEL::Hse2,
            3 => OTGHSSEL::Pll1pDiv2,
            _ => unreachable!(),
        }
    }
    ///HSE selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == OTGHSSEL::Hse
    }
    ///PLL1 "Q" (pll1_q_ck) selected
    #[inline(always)]
    pub fn is_pll1p(&self) -> bool {
        *self == OTGHSSEL::Pll1p
    }
    ///HSE/2 selected
    #[inline(always)]
    pub fn is_hse2(&self) -> bool {
        *self == OTGHSSEL::Hse2
    }
    ///PLL1 "P" divided by 2 (pll1_p_ck/2) selected
    #[inline(always)]
    pub fn is_pll1p_div2(&self) -> bool {
        *self == OTGHSSEL::Pll1pDiv2
    }
}
///Field `OTGHSSEL` writer - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type OTGHSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OTGHSSEL, crate::Safe>;
impl<'a, REG> OTGHSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSE selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(OTGHSSEL::Hse)
    }
    ///PLL1 "Q" (pll1_q_ck) selected
    #[inline(always)]
    pub fn pll1p(self) -> &'a mut crate::W<REG> {
        self.variant(OTGHSSEL::Pll1p)
    }
    ///HSE/2 selected
    #[inline(always)]
    pub fn hse2(self) -> &'a mut crate::W<REG> {
        self.variant(OTGHSSEL::Hse2)
    }
    ///PLL1 "P" divided by 2 (pll1_p_ck/2) selected
    #[inline(always)]
    pub fn pll1p_div2(self) -> &'a mut crate::W<REG> {
        self.variant(OTGHSSEL::Pll1pDiv2)
    }
}
impl R {
    ///Bits 0:2 - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
    #[inline(always)]
    pub fn mdf1sel(&self) -> MDF1SEL_R {
        MDF1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 5:7 - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saessel(&self) -> SAESSEL_R {
        SAESSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcsel(&self) -> LTDCSEL_R {
        LTDCSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:21 - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
    #[inline(always)]
    pub fn octospisel(&self) -> OCTOSPISEL_R {
        OCTOSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1sel(&self) -> HSPI1SEL_R {
        HSPI1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5sel(&self) -> I2C5SEL_R {
        I2C5SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6sel(&self) -> I2C6SEL_R {
        I2C6SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 30:31 - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghssel(&self) -> OTGHSSEL_R {
        OTGHSSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("mdf1sel", &self.mdf1sel())
            .field("sai1sel", &self.sai1sel())
            .field("sai2sel", &self.sai2sel())
            .field("saessel", &self.saessel())
            .field("rngsel", &self.rngsel())
            .field("sdmmcsel", &self.sdmmcsel())
            .field("dsisel", &self.dsisel())
            .field("usart6sel", &self.usart6sel())
            .field("ltdcsel", &self.ltdcsel())
            .field("octospisel", &self.octospisel())
            .field("hspi1sel", &self.hspi1sel())
            .field("i2c5sel", &self.i2c5sel())
            .field("i2c6sel", &self.i2c6sel())
            .field("otghssel", &self.otghssel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
    #[inline(always)]
    pub fn mdf1sel(&mut self) -> MDF1SEL_W<CCIPR2rs> {
        MDF1SEL_W::new(self, 0)
    }
    ///Bits 5:7 - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<CCIPR2rs> {
        SAI1SEL_W::new(self, 5)
    }
    ///Bits 8:10 - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<CCIPR2rs> {
        SAI2SEL_W::new(self, 8)
    }
    ///Bit 11 - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saessel(&mut self) -> SAESSEL_W<CCIPR2rs> {
        SAESSEL_W::new(self, 11)
    }
    ///Bits 12:13 - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W<CCIPR2rs> {
        RNGSEL_W::new(self, 12)
    }
    ///Bit 14 - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<CCIPR2rs> {
        SDMMCSEL_W::new(self, 14)
    }
    ///Bit 15 - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W<CCIPR2rs> {
        DSISEL_W::new(self, 15)
    }
    ///Bits 16:17 - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart6sel(&mut self) -> USART6SEL_W<CCIPR2rs> {
        USART6SEL_W::new(self, 16)
    }
    ///Bit 18 - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcsel(&mut self) -> LTDCSEL_W<CCIPR2rs> {
        LTDCSEL_W::new(self, 18)
    }
    ///Bits 20:21 - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
    #[inline(always)]
    pub fn octospisel(&mut self) -> OCTOSPISEL_W<CCIPR2rs> {
        OCTOSPISEL_W::new(self, 20)
    }
    ///Bits 22:23 - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1sel(&mut self) -> HSPI1SEL_W<CCIPR2rs> {
        HSPI1SEL_W::new(self, 22)
    }
    ///Bits 24:25 - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5sel(&mut self) -> I2C5SEL_W<CCIPR2rs> {
        I2C5SEL_W::new(self, 24)
    }
    ///Bits 26:27 - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6sel(&mut self) -> I2C6SEL_W<CCIPR2rs> {
        I2C6SEL_W::new(self, 26)
    }
    ///Bits 30:31 - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghssel(&mut self) -> OTGHSSEL_W<CCIPR2rs> {
        OTGHSSEL_W::new(self, 30)
    }
}
/**RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
