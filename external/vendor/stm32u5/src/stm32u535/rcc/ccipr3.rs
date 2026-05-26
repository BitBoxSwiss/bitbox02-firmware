///Register `CCIPR3` reader
pub type R = crate::R<CCIPR3rs>;
///Register `CCIPR3` writer
pub type W = crate::W<CCIPR3rs>;
/**LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL {
    ///0: PCLK3 selected
    Pclk3 = 0,
    ///1: SYSCLK selected
    Sysclk = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
    ///3: LSE selected
    Lse = 3,
    ///4: MSIK selected
    Msik = 4,
}
impl From<LPUART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPUART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPUART1SEL {}
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL>;
impl LPUART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1SEL> {
        match self.bits {
            0 => Some(LPUART1SEL::Pclk3),
            1 => Some(LPUART1SEL::Sysclk),
            2 => Some(LPUART1SEL::Hsi16),
            3 => Some(LPUART1SEL::Lse),
            4 => Some(LPUART1SEL::Msik),
            _ => None,
        }
    }
    ///PCLK3 selected
    #[inline(always)]
    pub fn is_pclk3(&self) -> bool {
        *self == LPUART1SEL::Pclk3
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == LPUART1SEL::Sysclk
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPUART1SEL::Hsi16
    }
    ///LSE selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL::Lse
    }
    ///MSIK selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == LPUART1SEL::Msik
    }
}
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPUART1SEL>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK3 selected
    #[inline(always)]
    pub fn pclk3(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pclk3)
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Sysclk)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Hsi16)
    }
    ///LSE selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Lse)
    }
    ///MSIK selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Msik)
    }
}
/**SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI3SEL {
    ///0: PCLK3 selected
    Pclk3 = 0,
    ///1: SYSCLK selected
    Sysclk = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
    ///3: MSIK selected
    Msik = 3,
}
impl From<SPI3SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI3SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI3SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI3SEL {}
///Field `SPI3SEL` reader - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type SPI3SEL_R = crate::FieldReader<SPI3SEL>;
impl SPI3SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI3SEL {
        match self.bits {
            0 => SPI3SEL::Pclk3,
            1 => SPI3SEL::Sysclk,
            2 => SPI3SEL::Hsi16,
            3 => SPI3SEL::Msik,
            _ => unreachable!(),
        }
    }
    ///PCLK3 selected
    #[inline(always)]
    pub fn is_pclk3(&self) -> bool {
        *self == SPI3SEL::Pclk3
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SPI3SEL::Sysclk
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SPI3SEL::Hsi16
    }
    ///MSIK selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == SPI3SEL::Msik
    }
}
///Field `SPI3SEL` writer - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type SPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPI3SEL, crate::Safe>;
impl<'a, REG> SPI3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK3 selected
    #[inline(always)]
    pub fn pclk3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Pclk3)
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Sysclk)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Hsi16)
    }
    ///MSIK selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Msik)
    }
}
///Field `I2C3SEL` reader - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub use SPI3SEL_R as I2C3SEL_R;
///Field `I2C3SEL` writer - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub use SPI3SEL_W as I2C3SEL_W;
/**LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM34SEL {
    ///0: MSIK clock selected
    Msik = 0,
    ///1: LSI selected
    Lsi = 1,
    ///2: HSI selected
    Hsi = 2,
    ///3: LSE selected
    Lse = 3,
}
impl From<LPTIM34SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM34SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM34SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM34SEL {}
///Field `LPTIM34SEL` reader - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
pub type LPTIM34SEL_R = crate::FieldReader<LPTIM34SEL>;
impl LPTIM34SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM34SEL {
        match self.bits {
            0 => LPTIM34SEL::Msik,
            1 => LPTIM34SEL::Lsi,
            2 => LPTIM34SEL::Hsi,
            3 => LPTIM34SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == LPTIM34SEL::Msik
    }
    ///LSI selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM34SEL::Lsi
    }
    ///HSI selected
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM34SEL::Hsi
    }
    ///LSE selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM34SEL::Lse
    }
}
///Field `LPTIM34SEL` writer - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
pub type LPTIM34SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPTIM34SEL, crate::Safe>;
impl<'a, REG> LPTIM34SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM34SEL::Msik)
    }
    ///LSI selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM34SEL::Lsi)
    }
    ///HSI selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM34SEL::Hsi)
    }
    ///LSE selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM34SEL::Lse)
    }
}
/**LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: MSIK clock selected
    Msik = 0,
    ///1: LSI selected
    Lsi = 1,
    ///2: HSI16 selected
    Hsi16 = 2,
    ///3: LSE selected
    Lse = 3,
}
impl From<LPTIM1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM1SEL {}
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SEL {
        match self.bits {
            0 => LPTIM1SEL::Msik,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi16,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == LPTIM1SEL::Msik
    }
    ///LSI selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL::Hsi16
    }
    ///LSE selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPTIM1SEL, crate::Safe>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Msik)
    }
    ///LSI selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///HSI16 selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi16)
    }
    ///LSE selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
/**ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCDACSEL {
    ///0: HCLK clock selected
    Hclk = 0,
    ///1: SYSCLK selected
    Sysclk = 1,
    ///2: PLL2 "R" (pll2_r_ck) selected
    Pll2r = 2,
    ///3: HSE clock selected
    Hse = 3,
    ///4: HSI16 clock selected
    Hsi16 = 4,
    ///5: MSIK clock selected
    Msik = 5,
}
impl From<ADCDACSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADCDACSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCDACSEL {
    type Ux = u8;
}
impl crate::IsEnum for ADCDACSEL {}
///Field `ADCDACSEL` reader - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
pub type ADCDACSEL_R = crate::FieldReader<ADCDACSEL>;
impl ADCDACSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCDACSEL> {
        match self.bits {
            0 => Some(ADCDACSEL::Hclk),
            1 => Some(ADCDACSEL::Sysclk),
            2 => Some(ADCDACSEL::Pll2r),
            3 => Some(ADCDACSEL::Hse),
            4 => Some(ADCDACSEL::Hsi16),
            5 => Some(ADCDACSEL::Msik),
            _ => None,
        }
    }
    ///HCLK clock selected
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == ADCDACSEL::Hclk
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCDACSEL::Sysclk
    }
    ///PLL2 "R" (pll2_r_ck) selected
    #[inline(always)]
    pub fn is_pll2r(&self) -> bool {
        *self == ADCDACSEL::Pll2r
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == ADCDACSEL::Hse
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == ADCDACSEL::Hsi16
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == ADCDACSEL::Msik
    }
}
///Field `ADCDACSEL` writer - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
pub type ADCDACSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADCDACSEL>;
impl<'a, REG> ADCDACSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK clock selected
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hclk)
    }
    ///SYSCLK selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Sysclk)
    }
    ///PLL2 "R" (pll2_r_ck) selected
    #[inline(always)]
    pub fn pll2r(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Pll2r)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hse)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hsi16)
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Msik)
    }
}
/**DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SEL {
    ///0: LSE selected
    Lse = 0,
    ///1: LSI selected
    Lsi = 1,
}
impl From<DAC1SEL> for bool {
    #[inline(always)]
    fn from(variant: DAC1SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DAC1SEL` reader - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
pub type DAC1SEL_R = crate::BitReader<DAC1SEL>;
impl DAC1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAC1SEL {
        match self.bits {
            false => DAC1SEL::Lse,
            true => DAC1SEL::Lsi,
        }
    }
    ///LSE selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == DAC1SEL::Lse
    }
    ///LSI selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == DAC1SEL::Lsi
    }
}
///Field `DAC1SEL` writer - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
pub type DAC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, DAC1SEL>;
impl<'a, REG> DAC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SEL::Lse)
    }
    ///LSI selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SEL::Lsi)
    }
}
/**ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADF1SEL {
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
impl From<ADF1SEL> for u8 {
    #[inline(always)]
    fn from(variant: ADF1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADF1SEL {
    type Ux = u8;
}
impl crate::IsEnum for ADF1SEL {}
///Field `ADF1SEL` reader - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
pub type ADF1SEL_R = crate::FieldReader<ADF1SEL>;
impl ADF1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADF1SEL> {
        match self.bits {
            0 => Some(ADF1SEL::Hclk),
            1 => Some(ADF1SEL::Pll1p),
            2 => Some(ADF1SEL::Pll3q),
            3 => Some(ADF1SEL::Audioclk),
            4 => Some(ADF1SEL::Msik),
            _ => None,
        }
    }
    ///HCLK selected
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == ADF1SEL::Hclk
    }
    ///PLL1 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn is_pll1p(&self) -> bool {
        *self == ADF1SEL::Pll1p
    }
    ///PLL3 "Q" (pll3_q_ck) selected
    #[inline(always)]
    pub fn is_pll3q(&self) -> bool {
        *self == ADF1SEL::Pll3q
    }
    ///input pin AUDIOCLK selected
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == ADF1SEL::Audioclk
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == ADF1SEL::Msik
    }
}
///Field `ADF1SEL` writer - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
pub type ADF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADF1SEL>;
impl<'a, REG> ADF1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK selected
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Hclk)
    }
    ///PLL1 "P" (pll1_p_ck) selected
    #[inline(always)]
    pub fn pll1p(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Pll1p)
    }
    ///PLL3 "Q" (pll3_q_ck) selected
    #[inline(always)]
    pub fn pll3q(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Pll3q)
    }
    ///input pin AUDIOCLK selected
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Audioclk)
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(ADF1SEL::Msik)
    }
}
impl R {
    ///Bits 0:2 - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
    #[inline(always)]
    pub fn lptim34sel(&self) -> LPTIM34SEL_R {
        LPTIM34SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
    #[inline(always)]
    pub fn dac1sel(&self) -> DAC1SEL_R {
        DAC1SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
    #[inline(always)]
    pub fn adf1sel(&self) -> ADF1SEL_R {
        ADF1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR3")
            .field("lpuart1sel", &self.lpuart1sel())
            .field("spi3sel", &self.spi3sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("lptim34sel", &self.lptim34sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("adcdacsel", &self.adcdacsel())
            .field("dac1sel", &self.dac1sel())
            .field("adf1sel", &self.adf1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPR3rs> {
        LPUART1SEL_W::new(self, 0)
    }
    ///Bits 3:4 - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<CCIPR3rs> {
        SPI3SEL_W::new(self, 3)
    }
    ///Bits 6:7 - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<CCIPR3rs> {
        I2C3SEL_W::new(self, 6)
    }
    ///Bits 8:9 - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
    #[inline(always)]
    pub fn lptim34sel(&mut self) -> LPTIM34SEL_W<CCIPR3rs> {
        LPTIM34SEL_W::new(self, 8)
    }
    ///Bits 10:11 - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPR3rs> {
        LPTIM1SEL_W::new(self, 10)
    }
    ///Bits 12:14 - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
    #[inline(always)]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<CCIPR3rs> {
        ADCDACSEL_W::new(self, 12)
    }
    ///Bit 15 - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
    #[inline(always)]
    pub fn dac1sel(&mut self) -> DAC1SEL_W<CCIPR3rs> {
        DAC1SEL_W::new(self, 15)
    }
    ///Bits 16:18 - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
    #[inline(always)]
    pub fn adf1sel(&mut self) -> ADF1SEL_W<CCIPR3rs> {
        ADF1SEL_W::new(self, 16)
    }
}
/**RCC peripherals independent clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:CCIPR3)*/
pub struct CCIPR3rs;
impl crate::RegisterSpec for CCIPR3rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr3::R`](R) reader structure
impl crate::Readable for CCIPR3rs {}
///`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure
impl crate::Writable for CCIPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR3 to value 0
impl crate::Resettable for CCIPR3rs {}
