///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: HCLK divided by 2
    Div2 = 8,
    ///9: HCLK divided by 4
    Div4 = 9,
    ///10: HCLK divided by 8
    Div8 = 10,
    ///11: HCLK divided by 16
    Div16 = 11,
    ///12: HCLK divided by 64
    Div64 = 12,
    ///13: HCLK divided by 128
    Div128 = 13,
    ///14: HCLK divided by 256
    Div256 = 14,
    ///15: HCLK divided by 512
    Div512 = 15,
    ///0: HCLK not divided
    Div1 = 0,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
impl crate::IsEnum for HPRE {}
///Field `HPRE` reader - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPRE {
        match self.bits {
            8 => HPRE::Div2,
            9 => HPRE::Div4,
            10 => HPRE::Div8,
            11 => HPRE::Div16,
            12 => HPRE::Div64,
            13 => HPRE::Div128,
            14 => HPRE::Div256,
            15 => HPRE::Div512,
            _ => HPRE::Div1,
        }
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    ///HCLK divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    ///HCLK divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    ///HCLK divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    ///HCLK divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), HPRE::Div1)
    }
}
///Field `HPRE` writer - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE, crate::Safe>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    ///HCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    ///HCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    ///HCLK divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    ///HCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
}
/**APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1 {
    ///4: PCLK divided by 2
    Div2 = 4,
    ///5: PCLK divided by 4
    Div4 = 5,
    ///6: PCLK divided by 8
    Div8 = 6,
    ///7: PCLK divided by 16
    Div16 = 7,
    ///0: PCLK not divided
    Div1 = 0,
}
impl From<PPRE1> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1 {
    type Ux = u8;
}
impl crate::IsEnum for PPRE1 {}
///Field `PPRE1` reader - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
pub type PPRE1_R = crate::FieldReader<PPRE1>;
impl PPRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PPRE1 {
        match self.bits {
            4 => PPRE1::Div2,
            5 => PPRE1::Div4,
            6 => PPRE1::Div8,
            7 => PPRE1::Div16,
            _ => PPRE1::Div1,
        }
    }
    ///PCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1::Div2
    }
    ///PCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1::Div4
    }
    ///PCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1::Div8
    }
    ///PCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1::Div16
    }
    ///PCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), PPRE1::Div1)
    }
}
///Field `PPRE1` writer - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE1, crate::Safe>;
impl<'a, REG> PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div2)
    }
    ///PCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div4)
    }
    ///PCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div8)
    }
    ///PCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div16)
    }
    ///PCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div1)
    }
}
///Field `PPRE2` reader - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
pub use PPRE1_R as PPRE2_R;
///Field `PPRE2` writer - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
pub use PPRE1_W as PPRE2_W;
/**DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.

Value on reset: 6*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DPRE {
    ///4: DCLK divided by 2
    Div2 = 4,
    ///5: DCLK divided by 4
    Div4 = 5,
    ///6: DCLK divided by 8
    Div8 = 6,
    ///7: DCLK divided by 16
    Div16 = 7,
    ///0: DCLK not divided
    Div1 = 0,
}
impl From<DPRE> for u8 {
    #[inline(always)]
    fn from(variant: DPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DPRE {
    type Ux = u8;
}
impl crate::IsEnum for DPRE {}
///Field `DPRE` reader - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type DPRE_R = crate::FieldReader<DPRE>;
impl DPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPRE {
        match self.bits {
            4 => DPRE::Div2,
            5 => DPRE::Div4,
            6 => DPRE::Div8,
            7 => DPRE::Div16,
            _ => DPRE::Div1,
        }
    }
    ///DCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DPRE::Div2
    }
    ///DCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DPRE::Div4
    }
    ///DCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DPRE::Div8
    }
    ///DCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DPRE::Div16
    }
    ///DCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), DPRE::Div1)
    }
}
///Field `DPRE` writer - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type DPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DPRE, crate::Safe>;
impl<'a, REG> DPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DPRE::Div2)
    }
    ///DCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DPRE::Div4)
    }
    ///DCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DPRE::Div8)
    }
    ///DCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DPRE::Div16)
    }
    ///DCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DPRE::Div1)
    }
}
/**AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB1DIS {
    ///0: AHB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: AHB1 clock disabled
    Disabled = 1,
}
impl From<AHB1DIS> for bool {
    #[inline(always)]
    fn from(variant: AHB1DIS) -> Self {
        variant as u8 != 0
    }
}
///Field `AHB1DIS` reader - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
pub type AHB1DIS_R = crate::BitReader<AHB1DIS>;
impl AHB1DIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AHB1DIS {
        match self.bits {
            false => AHB1DIS::Enabled,
            true => AHB1DIS::Disabled,
        }
    }
    ///AHB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB1DIS::Enabled
    }
    ///AHB1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB1DIS::Disabled
    }
}
///Field `AHB1DIS` writer - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
pub type AHB1DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB1DIS>;
impl<'a, REG> AHB1DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS::Enabled)
    }
    ///AHB1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS::Disabled)
    }
}
/**AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2DIS1 {
    ///0: AHB2_1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: AHB2_1 clock disabled
    Disabled = 1,
}
impl From<AHB2DIS1> for bool {
    #[inline(always)]
    fn from(variant: AHB2DIS1) -> Self {
        variant as u8 != 0
    }
}
///Field `AHB2DIS1` reader - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
pub type AHB2DIS1_R = crate::BitReader<AHB2DIS1>;
impl AHB2DIS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AHB2DIS1 {
        match self.bits {
            false => AHB2DIS1::Enabled,
            true => AHB2DIS1::Disabled,
        }
    }
    ///AHB2_1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB2DIS1::Enabled
    }
    ///AHB2_1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB2DIS1::Disabled
    }
}
///Field `AHB2DIS1` writer - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
pub type AHB2DIS1_W<'a, REG> = crate::BitWriter<'a, REG, AHB2DIS1>;
impl<'a, REG> AHB2DIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB2_1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2DIS1::Enabled)
    }
    ///AHB2_1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2DIS1::Disabled)
    }
}
/**AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2DIS2 {
    ///0: AHB2_2 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: AHB2_2 clock disabled
    Disabled = 1,
}
impl From<AHB2DIS2> for bool {
    #[inline(always)]
    fn from(variant: AHB2DIS2) -> Self {
        variant as u8 != 0
    }
}
///Field `AHB2DIS2` reader - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
pub type AHB2DIS2_R = crate::BitReader<AHB2DIS2>;
impl AHB2DIS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AHB2DIS2 {
        match self.bits {
            false => AHB2DIS2::Enabled,
            true => AHB2DIS2::Disabled,
        }
    }
    ///AHB2_2 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB2DIS2::Enabled
    }
    ///AHB2_2 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB2DIS2::Disabled
    }
}
///Field `AHB2DIS2` writer - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
pub type AHB2DIS2_W<'a, REG> = crate::BitWriter<'a, REG, AHB2DIS2>;
impl<'a, REG> AHB2DIS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB2_2 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2DIS2::Enabled)
    }
    ///AHB2_2 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2DIS2::Disabled)
    }
}
/**APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APB1DIS {
    ///0: APB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: APB1 clock disabled
    Disabled = 1,
}
impl From<APB1DIS> for bool {
    #[inline(always)]
    fn from(variant: APB1DIS) -> Self {
        variant as u8 != 0
    }
}
///Field `APB1DIS` reader - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
pub type APB1DIS_R = crate::BitReader<APB1DIS>;
impl APB1DIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APB1DIS {
        match self.bits {
            false => APB1DIS::Enabled,
            true => APB1DIS::Disabled,
        }
    }
    ///APB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APB1DIS::Enabled
    }
    ///APB1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APB1DIS::Disabled
    }
}
///Field `APB1DIS` writer - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
pub type APB1DIS_W<'a, REG> = crate::BitWriter<'a, REG, APB1DIS>;
impl<'a, REG> APB1DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///APB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIS::Enabled)
    }
    ///APB1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIS::Disabled)
    }
}
/**APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APB2DIS {
    ///0: APB2 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: APB2 clock disabled
    Disabled = 1,
}
impl From<APB2DIS> for bool {
    #[inline(always)]
    fn from(variant: APB2DIS) -> Self {
        variant as u8 != 0
    }
}
///Field `APB2DIS` reader - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
pub type APB2DIS_R = crate::BitReader<APB2DIS>;
impl APB2DIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APB2DIS {
        match self.bits {
            false => APB2DIS::Enabled,
            true => APB2DIS::Disabled,
        }
    }
    ///APB2 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APB2DIS::Enabled
    }
    ///APB2 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APB2DIS::Disabled
    }
}
///Field `APB2DIS` writer - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
pub type APB2DIS_W<'a, REG> = crate::BitWriter<'a, REG, APB2DIS>;
impl<'a, REG> APB2DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///APB2 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APB2DIS::Enabled)
    }
    ///APB2 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APB2DIS::Disabled)
    }
}
impl R {
    ///Bits 0:3 - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dpre(&self) -> DPRE_R {
        DPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
    #[inline(always)]
    pub fn ahb1dis(&self) -> AHB1DIS_R {
        AHB1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
    #[inline(always)]
    pub fn ahb2dis1(&self) -> AHB2DIS1_R {
        AHB2DIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
    #[inline(always)]
    pub fn ahb2dis2(&self) -> AHB2DIS2_R {
        AHB2DIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
    #[inline(always)]
    pub fn apb1dis(&self) -> APB1DIS_R {
        APB1DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
    #[inline(always)]
    pub fn apb2dis(&self) -> APB2DIS_R {
        APB2DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("dpre", &self.dpre())
            .field("ahb1dis", &self.ahb1dis())
            .field("ahb2dis1", &self.ahb2dis1())
            .field("ahb2dis2", &self.ahb2dis2())
            .field("apb1dis", &self.apb1dis())
            .field("apb2dis", &self.apb2dis())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<CFGR2rs> {
        HPRE_W::new(self, 0)
    }
    ///Bits 4:6 - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGR2rs> {
        PPRE1_W::new(self, 4)
    }
    ///Bits 8:10 - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGR2rs> {
        PPRE2_W::new(self, 8)
    }
    ///Bits 12:14 - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dpre(&mut self) -> DPRE_W<CFGR2rs> {
        DPRE_W::new(self, 12)
    }
    ///Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
    #[inline(always)]
    pub fn ahb1dis(&mut self) -> AHB1DIS_W<CFGR2rs> {
        AHB1DIS_W::new(self, 16)
    }
    ///Bit 17 - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
    #[inline(always)]
    pub fn ahb2dis1(&mut self) -> AHB2DIS1_W<CFGR2rs> {
        AHB2DIS1_W::new(self, 17)
    }
    ///Bit 18 - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
    #[inline(always)]
    pub fn ahb2dis2(&mut self) -> AHB2DIS2_W<CFGR2rs> {
        AHB2DIS2_W::new(self, 18)
    }
    ///Bit 19 - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
    #[inline(always)]
    pub fn apb1dis(&mut self) -> APB1DIS_W<CFGR2rs> {
        APB1DIS_W::new(self, 19)
    }
    ///Bit 20 - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
    #[inline(always)]
    pub fn apb2dis(&mut self) -> APB2DIS_W<CFGR2rs> {
        APB2DIS_W::new(self, 20)
    }
}
/**RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0x6000
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0x6000;
}
