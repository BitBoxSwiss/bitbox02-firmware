///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
/**APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE3 {
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
impl From<PPRE3> for u8 {
    #[inline(always)]
    fn from(variant: PPRE3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE3 {
    type Ux = u8;
}
impl crate::IsEnum for PPRE3 {}
///Field `PPRE3` reader - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
pub type PPRE3_R = crate::FieldReader<PPRE3>;
impl PPRE3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PPRE3 {
        match self.bits {
            4 => PPRE3::Div2,
            5 => PPRE3::Div4,
            6 => PPRE3::Div8,
            7 => PPRE3::Div16,
            _ => PPRE3::Div1,
        }
    }
    ///PCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE3::Div2
    }
    ///PCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE3::Div4
    }
    ///PCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE3::Div8
    }
    ///PCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE3::Div16
    }
    ///PCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), PPRE3::Div1)
    }
}
///Field `PPRE3` writer - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
pub type PPRE3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE3, crate::Safe>;
impl<'a, REG> PPRE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3::Div2)
    }
    ///PCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3::Div4)
    }
    ///PCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3::Div8)
    }
    ///PCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3::Div16)
    }
    ///PCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3::Div1)
    }
}
/**AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB3DIS {
    ///0: AHB3 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: AHB3 clock disabled
    Disabled = 1,
}
impl From<AHB3DIS> for bool {
    #[inline(always)]
    fn from(variant: AHB3DIS) -> Self {
        variant as u8 != 0
    }
}
///Field `AHB3DIS` reader - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
pub type AHB3DIS_R = crate::BitReader<AHB3DIS>;
impl AHB3DIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AHB3DIS {
        match self.bits {
            false => AHB3DIS::Enabled,
            true => AHB3DIS::Disabled,
        }
    }
    ///AHB3 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB3DIS::Enabled
    }
    ///AHB3 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB3DIS::Disabled
    }
}
///Field `AHB3DIS` writer - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
pub type AHB3DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB3DIS>;
impl<'a, REG> AHB3DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB3 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB3DIS::Enabled)
    }
    ///AHB3 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB3DIS::Disabled)
    }
}
/**APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APB3DIS {
    ///0: APB3 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    Enabled = 0,
    ///1: APB3 clock disabled
    Disabled = 1,
}
impl From<APB3DIS> for bool {
    #[inline(always)]
    fn from(variant: APB3DIS) -> Self {
        variant as u8 != 0
    }
}
///Field `APB3DIS` reader - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
pub type APB3DIS_R = crate::BitReader<APB3DIS>;
impl APB3DIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APB3DIS {
        match self.bits {
            false => APB3DIS::Enabled,
            true => APB3DIS::Disabled,
        }
    }
    ///APB3 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APB3DIS::Enabled
    }
    ///APB3 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APB3DIS::Disabled
    }
}
///Field `APB3DIS` writer - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
pub type APB3DIS_W<'a, REG> = crate::BitWriter<'a, REG, APB3DIS>;
impl<'a, REG> APB3DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///APB3 clock enabled, distributed to peripherals according to their dedicated clock enable control bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APB3DIS::Enabled)
    }
    ///APB3 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APB3DIS::Disabled)
    }
}
impl R {
    ///Bits 4:6 - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
    #[inline(always)]
    pub fn ppre3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
    #[inline(always)]
    pub fn ahb3dis(&self) -> AHB3DIS_R {
        AHB3DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
    #[inline(always)]
    pub fn apb3dis(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("ppre3", &self.ppre3())
            .field("ahb3dis", &self.ahb3dis())
            .field("apb3dis", &self.apb3dis())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
    #[inline(always)]
    pub fn ppre3(&mut self) -> PPRE3_W<CFGR3rs> {
        PPRE3_W::new(self, 4)
    }
    ///Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
    #[inline(always)]
    pub fn ahb3dis(&mut self) -> AHB3DIS_W<CFGR3rs> {
        AHB3DIS_W::new(self, 16)
    }
    ///Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
    #[inline(always)]
    pub fn apb3dis(&mut self) -> APB3DIS_W<CFGR3rs> {
        APB3DIS_W::new(self, 17)
    }
}
/**RCC clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
