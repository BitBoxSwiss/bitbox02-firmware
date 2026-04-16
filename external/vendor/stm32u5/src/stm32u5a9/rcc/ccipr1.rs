///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
/**USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: PCLKx selected
    Pclk = 0,
    ///1: SYSCLK selected
    Sysclk = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
    ///3: MSIK selected
    Msik = 3,
}
impl From<USART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART1SEL {}
///Field `USART1SEL` reader - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1SEL {
        match self.bits {
            0 => USART1SEL::Pclk,
            1 => USART1SEL::Sysclk,
            2 => USART1SEL::Hsi16,
            3 => USART1SEL::Msik,
            _ => unreachable!(),
        }
    }
    ///PCLKx selected
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SEL::Pclk
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL::Sysclk
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL::Hsi16
    }
    ///MSIK selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == USART1SEL::Msik
    }
}
///Field `USART1SEL` writer - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1SEL, crate::Safe>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKx selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pclk)
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Sysclk)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi16)
    }
    ///MSIK selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Msik)
    }
}
///Field `USART2SEL` reader - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use USART1SEL_R as USART2SEL_R;
///Field `USART3SEL` reader - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub use USART1SEL_R as USART3SEL_R;
///Field `UART4SEL` reader - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub use USART1SEL_R as UART4SEL_R;
///Field `UART5SEL` reader - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub use USART1SEL_R as UART5SEL_R;
///Field `I2C1SEL` reader - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub use USART1SEL_R as I2C1SEL_R;
///Field `I2C2SEL` reader - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub use USART1SEL_R as I2C2SEL_R;
///Field `I2C4SEL` reader - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
pub use USART1SEL_R as I2C4SEL_R;
///Field `SPI2SEL` reader - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub use USART1SEL_R as SPI2SEL_R;
///Field `USART2SEL` writer - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub use USART1SEL_W as USART2SEL_W;
///Field `USART3SEL` writer - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub use USART1SEL_W as USART3SEL_W;
///Field `UART4SEL` writer - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub use USART1SEL_W as UART4SEL_W;
///Field `UART5SEL` writer - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub use USART1SEL_W as UART5SEL_W;
///Field `I2C1SEL` writer - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub use USART1SEL_W as I2C1SEL_W;
///Field `I2C2SEL` writer - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub use USART1SEL_W as I2C2SEL_W;
///Field `I2C4SEL` writer - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
pub use USART1SEL_W as I2C4SEL_W;
///Field `SPI2SEL` writer - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub use USART1SEL_W as SPI2SEL_W;
/**Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM2SEL {
    ///0: PCLK1 selected
    Pclk1 = 0,
    ///1: LSI selected
    Lsi = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
    ///3: MSIK selected
    Msik = 3,
}
impl From<LPTIM2SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM2SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM2SEL {}
///Field `LPTIM2SEL` reader - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_R = crate::FieldReader<LPTIM2SEL>;
impl LPTIM2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2SEL {
        match self.bits {
            0 => LPTIM2SEL::Pclk1,
            1 => LPTIM2SEL::Lsi,
            2 => LPTIM2SEL::Hsi16,
            3 => LPTIM2SEL::Msik,
            _ => unreachable!(),
        }
    }
    ///PCLK1 selected
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == LPTIM2SEL::Pclk1
    }
    ///LSI selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM2SEL::Lsi
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM2SEL::Hsi16
    }
    ///MSIK selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == LPTIM2SEL::Msik
    }
}
///Field `LPTIM2SEL` writer - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPTIM2SEL, crate::Safe>;
impl<'a, REG> LPTIM2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK1 selected
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Pclk1)
    }
    ///LSI selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Lsi)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Hsi16)
    }
    ///MSIK selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Msik)
    }
}
///Field `SPI1SEL` reader - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub use USART1SEL_R as SPI1SEL_R;
///Field `SPI1SEL` writer - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub use USART1SEL_W as SPI1SEL_W;
/**SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSTICKSEL {
    ///0: HCLK/8 selected
    HclkDiv8 = 0,
    ///1: LSI selected
    Lsi = 1,
    ///2: LSE selected
    Lse = 2,
}
impl From<SYSTICKSEL> for u8 {
    #[inline(always)]
    fn from(variant: SYSTICKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSTICKSEL {
    type Ux = u8;
}
impl crate::IsEnum for SYSTICKSEL {}
///Field `SYSTICKSEL` reader - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
pub type SYSTICKSEL_R = crate::FieldReader<SYSTICKSEL>;
impl SYSTICKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSTICKSEL> {
        match self.bits {
            0 => Some(SYSTICKSEL::HclkDiv8),
            1 => Some(SYSTICKSEL::Lsi),
            2 => Some(SYSTICKSEL::Lse),
            _ => None,
        }
    }
    ///HCLK/8 selected
    #[inline(always)]
    pub fn is_hclk_div8(&self) -> bool {
        *self == SYSTICKSEL::HclkDiv8
    }
    ///LSI selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == SYSTICKSEL::Lsi
    }
    ///LSE selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SYSTICKSEL::Lse
    }
}
///Field `SYSTICKSEL` writer - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYSTICKSEL>;
impl<'a, REG> SYSTICKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK/8 selected
    #[inline(always)]
    pub fn hclk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::HclkDiv8)
    }
    ///LSI selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::Lsi)
    }
    ///LSE selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL::Lse)
    }
}
/**FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCAN1SEL {
    ///0: HSE clock selected
    Hse = 0,
    ///1: PLL1 "Q" (pll2_q_ck) selected
    Pll1q = 1,
    ///2: PLL2 "P" (pll1_p_ck) selected
    Pll2p = 2,
}
impl From<FDCAN1SEL> for u8 {
    #[inline(always)]
    fn from(variant: FDCAN1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FDCAN1SEL {
    type Ux = u8;
}
impl crate::IsEnum for FDCAN1SEL {}
///Field `FDCAN1SEL` reader - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
pub type FDCAN1SEL_R = crate::FieldReader<FDCAN1SEL>;
impl FDCAN1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDCAN1SEL> {
        match self.bits {
            0 => Some(FDCAN1SEL::Hse),
            1 => Some(FDCAN1SEL::Pll1q),
            2 => Some(FDCAN1SEL::Pll2p),
            _ => None,
        }
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCAN1SEL::Hse
    }
    ///PLL1 "Q" (pll2_q_ck) selected
    #[inline(always)]
    pub fn is_pll1q(&self) -> bool {
        *self == FDCAN1SEL::Pll1q
    }
    ///PLL2 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn is_pll2p(&self) -> bool {
        *self == FDCAN1SEL::Pll2p
    }
}
///Field `FDCAN1SEL` writer - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
pub type FDCAN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCAN1SEL>;
impl<'a, REG> FDCAN1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL::Hse)
    }
    ///PLL1 "Q" (pll2_q_ck) selected
    #[inline(always)]
    pub fn pll1q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL::Pll1q)
    }
    ///PLL2 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn pll2p(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL::Pll2p)
    }
}
/**Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLKSEL {
    ///0: HSI48 clock selected
    Hsi = 0,
    ///1: PLL2 "Q" (pll2_q_ck) selected
    Pll2q = 1,
    ///2: PLL1 "Q" (pll1_q_ck) selected
    Pll1q = 2,
    ///3: MSIK clock selected
    Msik = 3,
}
impl From<ICLKSEL> for u8 {
    #[inline(always)]
    fn from(variant: ICLKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICLKSEL {
    type Ux = u8;
}
impl crate::IsEnum for ICLKSEL {}
///Field `ICLKSEL` reader - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
pub type ICLKSEL_R = crate::FieldReader<ICLKSEL>;
impl ICLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICLKSEL {
        match self.bits {
            0 => ICLKSEL::Hsi,
            1 => ICLKSEL::Pll2q,
            2 => ICLKSEL::Pll1q,
            3 => ICLKSEL::Msik,
            _ => unreachable!(),
        }
    }
    ///HSI48 clock selected
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ICLKSEL::Hsi
    }
    ///PLL2 "Q" (pll2_q_ck) selected
    #[inline(always)]
    pub fn is_pll2q(&self) -> bool {
        *self == ICLKSEL::Pll2q
    }
    ///PLL1 "Q" (pll1_q_ck) selected
    #[inline(always)]
    pub fn is_pll1q(&self) -> bool {
        *self == ICLKSEL::Pll1q
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == ICLKSEL::Msik
    }
}
///Field `ICLKSEL` writer - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
pub type ICLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ICLKSEL, crate::Safe>;
impl<'a, REG> ICLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI48 clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(ICLKSEL::Hsi)
    }
    ///PLL2 "Q" (pll2_q_ck) selected
    #[inline(always)]
    pub fn pll2q(self) -> &'a mut crate::W<REG> {
        self.variant(ICLKSEL::Pll2q)
    }
    ///PLL1 "Q" (pll1_q_ck) selected
    #[inline(always)]
    pub fn pll1q(self) -> &'a mut crate::W<REG> {
        self.variant(ICLKSEL::Pll1q)
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(ICLKSEL::Msik)
    }
}
/**Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\] value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMICSEL {
    ///4: HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    HsiMsisMsis = 4,
    ///5: HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    HsiMsisMsik = 5,
    ///6: HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    HsiMsikMsis = 6,
    ///7: HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    HsiMsikMsik = 7,
    ///0: HSI, MSIK and MSIS dividers disabled
    Disabled = 0,
}
impl From<TIMICSEL> for u8 {
    #[inline(always)]
    fn from(variant: TIMICSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMICSEL {
    type Ux = u8;
}
impl crate::IsEnum for TIMICSEL {}
///Field `TIMICSEL` reader - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\] value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
pub type TIMICSEL_R = crate::FieldReader<TIMICSEL>;
impl TIMICSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMICSEL {
        match self.bits {
            4 => TIMICSEL::HsiMsisMsis,
            5 => TIMICSEL::HsiMsisMsik,
            6 => TIMICSEL::HsiMsikMsis,
            7 => TIMICSEL::HsiMsikMsik,
            _ => TIMICSEL::Disabled,
        }
    }
    ///HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn is_hsi_msis_msis(&self) -> bool {
        *self == TIMICSEL::HsiMsisMsis
    }
    ///HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn is_hsi_msis_msik(&self) -> bool {
        *self == TIMICSEL::HsiMsisMsik
    }
    ///HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn is_hsi_msik_msis(&self) -> bool {
        *self == TIMICSEL::HsiMsikMsis
    }
    ///HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn is_hsi_msik_msik(&self) -> bool {
        *self == TIMICSEL::HsiMsikMsik
    }
    ///HSI, MSIK and MSIS dividers disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        matches!(self.variant(), TIMICSEL::Disabled)
    }
}
///Field `TIMICSEL` writer - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\] value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
pub type TIMICSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TIMICSEL, crate::Safe>;
impl<'a, REG> TIMICSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn hsi_msis_msis(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::HsiMsisMsis)
    }
    ///HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn hsi_msis_msik(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::HsiMsisMsik)
    }
    ///HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn hsi_msik_msis(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::HsiMsikMsis)
    }
    ///HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture
    #[inline(always)]
    pub fn hsi_msik_msik(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::HsiMsikMsik)
    }
    ///HSI, MSIK and MSIS dividers disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::Disabled)
    }
}
impl R {
    ///Bits 0:1 - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
    #[inline(always)]
    pub fn fdcan1sel(&self) -> FDCAN1SEL_R {
        FDCAN1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
    #[inline(always)]
    pub fn iclksel(&self) -> ICLKSEL_R {
        ICLKSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 29:31 - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\] value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR1")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("uart4sel", &self.uart4sel())
            .field("uart5sel", &self.uart5sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c4sel", &self.i2c4sel())
            .field("spi2sel", &self.spi2sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("spi1sel", &self.spi1sel())
            .field("systicksel", &self.systicksel())
            .field("fdcan1sel", &self.fdcan1sel())
            .field("iclksel", &self.iclksel())
            .field("timicsel", &self.timicsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPR1rs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 4:5 - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W<CCIPR1rs> {
        USART3SEL_W::new(self, 4)
    }
    ///Bits 6:7 - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W<CCIPR1rs> {
        UART4SEL_W::new(self, 6)
    }
    ///Bits 8:9 - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W<CCIPR1rs> {
        UART5SEL_W::new(self, 8)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPR1rs> {
        I2C1SEL_W::new(self, 10)
    }
    ///Bits 12:13 - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<CCIPR1rs> {
        I2C2SEL_W::new(self, 12)
    }
    ///Bits 14:15 - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<CCIPR1rs> {
        I2C4SEL_W::new(self, 14)
    }
    ///Bits 16:17 - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<CCIPR1rs> {
        SPI2SEL_W::new(self, 16)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPR1rs> {
        LPTIM2SEL_W::new(self, 18)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<CCIPR1rs> {
        SPI1SEL_W::new(self, 20)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
    #[inline(always)]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<CCIPR1rs> {
        SYSTICKSEL_W::new(self, 22)
    }
    ///Bits 24:25 - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
    #[inline(always)]
    pub fn fdcan1sel(&mut self) -> FDCAN1SEL_W<CCIPR1rs> {
        FDCAN1SEL_W::new(self, 24)
    }
    ///Bits 26:27 - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
    #[inline(always)]
    pub fn iclksel(&mut self) -> ICLKSEL_W<CCIPR1rs> {
        ICLKSEL_W::new(self, 26)
    }
    ///Bits 29:31 - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\] value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
    #[inline(always)]
    pub fn timicsel(&mut self) -> TIMICSEL_W<CCIPR1rs> {
        TIMICSEL_W::new(self, 29)
    }
}
/**RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:CCIPR1)*/
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr1::R`](R) reader structure
impl crate::Readable for CCIPR1rs {}
///`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR1 to value 0
impl crate::Resettable for CCIPR1rs {}
