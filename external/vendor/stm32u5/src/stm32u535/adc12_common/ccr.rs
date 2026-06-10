///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
/**ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: Input ADC clock not divided
    Div1 = 0,
    ///1: Input ADC clock divided by 2
    Div2 = 1,
    ///2: Input ADC clock divided by 4
    Div4 = 2,
    ///3: Input ADC clock divided by 6
    Div6 = 3,
    ///4: Input ADC clock divided by 8
    Div8 = 4,
    ///5: Input ADC clock divided by 10
    Div10 = 5,
    ///6: Input ADC clock divided by 12
    Div12 = 6,
    ///7: Input ADC clock divided by 16
    Div16 = 7,
    ///8: Input ADC clock divided by 32
    Div32 = 8,
    ///9: Input ADC clock divided by 64
    Div64 = 9,
    ///10: Input ADC clock divided by 128
    Div128 = 10,
    ///11: Input ADC clock divided by 256
    Div256 = 11,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Div1),
            1 => Some(PRESC::Div2),
            2 => Some(PRESC::Div4),
            3 => Some(PRESC::Div6),
            4 => Some(PRESC::Div8),
            5 => Some(PRESC::Div10),
            6 => Some(PRESC::Div12),
            7 => Some(PRESC::Div16),
            8 => Some(PRESC::Div32),
            9 => Some(PRESC::Div64),
            10 => Some(PRESC::Div128),
            11 => Some(PRESC::Div256),
            _ => None,
        }
    }
    ///Input ADC clock not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    ///Input ADC clock divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    ///Input ADC clock divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    ///Input ADC clock divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC::Div6
    }
    ///Input ADC clock divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    ///Input ADC clock divided by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC::Div10
    }
    ///Input ADC clock divided by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC::Div12
    }
    ///Input ADC clock divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    ///Input ADC clock divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    ///Input ADC clock divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    ///Input ADC clock divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
    ///Input ADC clock divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC::Div256
    }
}
///Field `PRESC` writer - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input ADC clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    ///Input ADC clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    ///Input ADC clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    ///Input ADC clock divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div6)
    }
    ///Input ADC clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    ///Input ADC clock divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div10)
    }
    ///Input ADC clock divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div12)
    }
    ///Input ADC clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    ///Input ADC clock divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    ///Input ADC clock divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    ///Input ADC clock divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
    ///Input ADC clock divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div256)
    }
}
/**VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    ///0: VREFINT channel disabled
    Disabled = 0,
    ///1: VREFINT channel enabled
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    ///VREFINT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    ///VREFINT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
///Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VREFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    ///VREFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
/**Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSENSESEL {
    ///0: Temperature sensor channel disabled
    Disabled = 0,
    ///1: Temperature sensor channel enabled
    Enabled = 1,
}
impl From<VSENSESEL> for bool {
    #[inline(always)]
    fn from(variant: VSENSESEL) -> Self {
        variant as u8 != 0
    }
}
///Field `VSENSESEL` reader - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type VSENSESEL_R = crate::BitReader<VSENSESEL>;
impl VSENSESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSENSESEL {
        match self.bits {
            false => VSENSESEL::Disabled,
            true => VSENSESEL::Enabled,
        }
    }
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSESEL::Disabled
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSESEL::Enabled
    }
}
///Field `VSENSESEL` writer - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type VSENSESEL_W<'a, REG> = crate::BitWriter<'a, REG, VSENSESEL>;
impl<'a, REG> VSENSESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSESEL::Disabled)
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSESEL::Enabled)
    }
}
/**VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN {
    ///0: VBAT channel disabled
    Disabled = 0,
    ///1: VBAT channel enabled
    Enabled = 1,
}
impl From<VBATEN> for bool {
    #[inline(always)]
    fn from(variant: VBATEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type VBATEN_R = crate::BitReader<VBATEN>;
impl VBATEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN {
        match self.bits {
            false => VBATEN::Disabled,
            true => VBATEN::Enabled,
        }
    }
    ///VBAT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN::Disabled
    }
    ///VBAT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN::Enabled
    }
}
///Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Disabled)
    }
    ///VBAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Enabled)
    }
}
impl R {
    ///Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("presc", &self.presc())
            .field("vrefen", &self.vrefen())
            .field("vsensesel", &self.vsensesel())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn vsensesel(&mut self) -> VSENSESEL_W<CCRrs> {
        VSENSESEL_W::new(self, 23)
    }
    ///Bit 24 - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC_CCR system control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC12_Common:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
