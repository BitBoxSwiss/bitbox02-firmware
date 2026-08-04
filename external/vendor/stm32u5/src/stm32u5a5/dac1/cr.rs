///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**DAC channel%s enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1 {
    ///0: DAC Channel X disabled
    Disabled = 0,
    ///1: DAC Channel X enabled
    Enabled = 1,
}
impl From<EN1> for bool {
    #[inline(always)]
    fn from(variant: EN1) -> Self {
        variant as u8 != 0
    }
}
///Field `EN(1-2)` reader - DAC channel%s enable
pub type EN_R = crate::BitReader<EN1>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN1 {
        match self.bits {
            false => EN1::Disabled,
            true => EN1::Enabled,
        }
    }
    ///DAC Channel X disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1::Disabled
    }
    ///DAC Channel X enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1::Enabled
    }
}
///Field `EN(1-2)` writer - DAC channel%s enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN1>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC Channel X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Disabled)
    }
    ///DAC Channel X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Enabled)
    }
}
/**DAC channel%s trigger enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1 {
    ///0: DAC Channel X trigger disabled
    Disabled = 0,
    ///1: DAC Channel X trigger enabled
    Enabled = 1,
}
impl From<TEN1> for bool {
    #[inline(always)]
    fn from(variant: TEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `TEN(1-2)` reader - DAC channel%s trigger enable
pub type TEN_R = crate::BitReader<TEN1>;
impl TEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEN1 {
        match self.bits {
            false => TEN1::Disabled,
            true => TEN1::Enabled,
        }
    }
    ///DAC Channel X trigger disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1::Disabled
    }
    ///DAC Channel X trigger enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1::Enabled
    }
}
///Field `TEN(1-2)` writer - DAC channel%s trigger enable
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG, TEN1>;
impl<'a, REG> TEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC Channel X trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Disabled)
    }
    ///DAC Channel X trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Enabled)
    }
}
/**DAC channel1 trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1 {
    ///0: Software trigger
    Swtrig = 0,
    ///1: Timer 1 TRGO event
    Tim1trgo = 1,
    ///2: Timer 2 TRGO event
    Tim2trgo = 2,
    ///3: Timer 4 TRGO event
    Tim4trgo = 3,
    ///4: Timer 5 TRGO event
    Tim5trgo = 4,
    ///5: Timer 6 TRGO event
    Tim6trgo = 5,
    ///6: Timer 7 TRGO event
    Tim7trgo = 6,
    ///7: Timer 8 TRGO event
    Tim8trgo = 7,
    ///8: Timer 15 TRGO event
    Tim15trgo = 8,
    ///11: LPTIM1 CH1 event
    Lptim1ch1 = 11,
    ///12: LPTIM3 CH1 event
    Lptim3ch1 = 12,
    ///13: EXTI line 9
    Exti9 = 13,
}
impl From<TSEL1> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL1 {
    type Ux = u8;
}
impl crate::IsEnum for TSEL1 {}
///Field `TSEL1` reader - DAC channel1 trigger selection
pub type TSEL1_R = crate::FieldReader<TSEL1>;
impl TSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL1> {
        match self.bits {
            0 => Some(TSEL1::Swtrig),
            1 => Some(TSEL1::Tim1trgo),
            2 => Some(TSEL1::Tim2trgo),
            3 => Some(TSEL1::Tim4trgo),
            4 => Some(TSEL1::Tim5trgo),
            5 => Some(TSEL1::Tim6trgo),
            6 => Some(TSEL1::Tim7trgo),
            7 => Some(TSEL1::Tim8trgo),
            8 => Some(TSEL1::Tim15trgo),
            11 => Some(TSEL1::Lptim1ch1),
            12 => Some(TSEL1::Lptim3ch1),
            13 => Some(TSEL1::Exti9),
            _ => None,
        }
    }
    ///Software trigger
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == TSEL1::Swtrig
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == TSEL1::Tim1trgo
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == TSEL1::Tim2trgo
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == TSEL1::Tim4trgo
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == TSEL1::Tim5trgo
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        *self == TSEL1::Tim6trgo
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn is_tim7trgo(&self) -> bool {
        *self == TSEL1::Tim7trgo
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == TSEL1::Tim8trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15trgo(&self) -> bool {
        *self == TSEL1::Tim15trgo
    }
    ///LPTIM1 CH1 event
    #[inline(always)]
    pub fn is_lptim1ch1(&self) -> bool {
        *self == TSEL1::Lptim1ch1
    }
    ///LPTIM3 CH1 event
    #[inline(always)]
    pub fn is_lptim3ch1(&self) -> bool {
        *self == TSEL1::Lptim3ch1
    }
    ///EXTI line 9
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1::Exti9
    }
}
///Field `TSEL1` writer - DAC channel1 trigger selection
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TSEL1>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Software trigger
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Swtrig)
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim1trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim2trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim4trgo)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim5trgo)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim6trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim7trgo)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim8trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim15trgo)
    }
    ///LPTIM1 CH1 event
    #[inline(always)]
    pub fn lptim1ch1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Lptim1ch1)
    }
    ///LPTIM3 CH1 event
    #[inline(always)]
    pub fn lptim3ch1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Lptim3ch1)
    }
    ///EXTI line 9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Exti9)
    }
}
/**DAC channel%s noise/triangle wave generation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1 {
    ///0: Wave generation disabled
    Disabled = 0,
    ///1: Noise wave generation enabled
    Noise = 1,
    ///2: Triangle wave generation enabled
    Triangle = 2,
}
impl From<WAVE1> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAVE1 {
    type Ux = u8;
}
impl crate::IsEnum for WAVE1 {}
///Field `WAVE(1-2)` reader - DAC channel%s noise/triangle wave generation enable
pub type WAVE_R = crate::FieldReader<WAVE1>;
impl WAVE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAVE1 {
        match self.bits {
            0 => WAVE1::Disabled,
            1 => WAVE1::Noise,
            _ => WAVE1::Triangle,
        }
    }
    ///Wave generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1::Disabled
    }
    ///Noise wave generation enabled
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1::Noise
    }
    ///Triangle wave generation enabled
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        matches!(self.variant(), WAVE1::Triangle)
    }
}
///Field `WAVE(1-2)` writer - DAC channel%s noise/triangle wave generation enable
pub type WAVE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAVE1, crate::Safe>;
impl<'a, REG> WAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Wave generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Disabled)
    }
    ///Noise wave generation enabled
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Noise)
    }
    ///Triangle wave generation enabled
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Triangle)
    }
}
/**DAC channel%s mask/amplitude selector

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAMP1 {
    ///0: Unmask bit0 of LFSR/ triangle amplitude equal to 1
    Amp1 = 0,
    ///1: Unmask bits\[1:0\] of LFSR/ triangle amplitude equal to 3
    Amp3 = 1,
    ///2: Unmask bits\[2:0\] of LFSR/ triangle amplitude equal to 7
    Amp7 = 2,
    ///3: Unmask bits\[3:0\] of LFSR/ triangle amplitude equal to 15
    Amp15 = 3,
    ///4: Unmask bits\[4:0\] of LFSR/ triangle amplitude equal to 31
    Amp31 = 4,
    ///5: Unmask bits\[5:0\] of LFSR/ triangle amplitude equal 63
    Amp63 = 5,
    ///6: Unmask bits\[6:0\] of LFSR/ triangle amplitude equal to 127
    Amp127 = 6,
    ///7: Unmask bits\[7:0\] of LFSR/ triangle amplitude equal to 255
    Amp255 = 7,
    ///8: Unmask bits\[8:0\] of LFSR/ triangle amplitude equal to 511
    Amp511 = 8,
    ///9: Unmask bits\[9:0\] of LFSR/ triangle amplitude equal to 1023
    Amp1023 = 9,
    ///10: Unmask bits\[10:0\] of LFSR/ triangle amplitude equal to 2047
    Amp2047 = 10,
    ///11: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    Amp4095 = 11,
}
impl From<MAMP1> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAMP1 {
    type Ux = u8;
}
impl crate::IsEnum for MAMP1 {}
///Field `MAMP(1-2)` reader - DAC channel%s mask/amplitude selector
pub type MAMP_R = crate::FieldReader<MAMP1>;
impl MAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MAMP1 {
        match self.bits {
            0 => MAMP1::Amp1,
            1 => MAMP1::Amp3,
            2 => MAMP1::Amp7,
            3 => MAMP1::Amp15,
            4 => MAMP1::Amp31,
            5 => MAMP1::Amp63,
            6 => MAMP1::Amp127,
            7 => MAMP1::Amp255,
            8 => MAMP1::Amp511,
            9 => MAMP1::Amp1023,
            10 => MAMP1::Amp2047,
            _ => MAMP1::Amp4095,
        }
    }
    ///Unmask bit0 of LFSR/ triangle amplitude equal to 1
    #[inline(always)]
    pub fn is_amp1(&self) -> bool {
        *self == MAMP1::Amp1
    }
    ///Unmask bits\[1:0\] of LFSR/ triangle amplitude equal to 3
    #[inline(always)]
    pub fn is_amp3(&self) -> bool {
        *self == MAMP1::Amp3
    }
    ///Unmask bits\[2:0\] of LFSR/ triangle amplitude equal to 7
    #[inline(always)]
    pub fn is_amp7(&self) -> bool {
        *self == MAMP1::Amp7
    }
    ///Unmask bits\[3:0\] of LFSR/ triangle amplitude equal to 15
    #[inline(always)]
    pub fn is_amp15(&self) -> bool {
        *self == MAMP1::Amp15
    }
    ///Unmask bits\[4:0\] of LFSR/ triangle amplitude equal to 31
    #[inline(always)]
    pub fn is_amp31(&self) -> bool {
        *self == MAMP1::Amp31
    }
    ///Unmask bits\[5:0\] of LFSR/ triangle amplitude equal 63
    #[inline(always)]
    pub fn is_amp63(&self) -> bool {
        *self == MAMP1::Amp63
    }
    ///Unmask bits\[6:0\] of LFSR/ triangle amplitude equal to 127
    #[inline(always)]
    pub fn is_amp127(&self) -> bool {
        *self == MAMP1::Amp127
    }
    ///Unmask bits\[7:0\] of LFSR/ triangle amplitude equal to 255
    #[inline(always)]
    pub fn is_amp255(&self) -> bool {
        *self == MAMP1::Amp255
    }
    ///Unmask bits\[8:0\] of LFSR/ triangle amplitude equal to 511
    #[inline(always)]
    pub fn is_amp511(&self) -> bool {
        *self == MAMP1::Amp511
    }
    ///Unmask bits\[9:0\] of LFSR/ triangle amplitude equal to 1023
    #[inline(always)]
    pub fn is_amp1023(&self) -> bool {
        *self == MAMP1::Amp1023
    }
    ///Unmask bits\[10:0\] of LFSR/ triangle amplitude equal to 2047
    #[inline(always)]
    pub fn is_amp2047(&self) -> bool {
        *self == MAMP1::Amp2047
    }
    ///Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn is_amp4095(&self) -> bool {
        matches!(self.variant(), MAMP1::Amp4095)
    }
}
///Field `MAMP(1-2)` writer - DAC channel%s mask/amplitude selector
pub type MAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAMP1, crate::Safe>;
impl<'a, REG> MAMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Unmask bit0 of LFSR/ triangle amplitude equal to 1
    #[inline(always)]
    pub fn amp1(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp1)
    }
    ///Unmask bits\[1:0\] of LFSR/ triangle amplitude equal to 3
    #[inline(always)]
    pub fn amp3(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp3)
    }
    ///Unmask bits\[2:0\] of LFSR/ triangle amplitude equal to 7
    #[inline(always)]
    pub fn amp7(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp7)
    }
    ///Unmask bits\[3:0\] of LFSR/ triangle amplitude equal to 15
    #[inline(always)]
    pub fn amp15(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp15)
    }
    ///Unmask bits\[4:0\] of LFSR/ triangle amplitude equal to 31
    #[inline(always)]
    pub fn amp31(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp31)
    }
    ///Unmask bits\[5:0\] of LFSR/ triangle amplitude equal 63
    #[inline(always)]
    pub fn amp63(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp63)
    }
    ///Unmask bits\[6:0\] of LFSR/ triangle amplitude equal to 127
    #[inline(always)]
    pub fn amp127(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp127)
    }
    ///Unmask bits\[7:0\] of LFSR/ triangle amplitude equal to 255
    #[inline(always)]
    pub fn amp255(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp255)
    }
    ///Unmask bits\[8:0\] of LFSR/ triangle amplitude equal to 511
    #[inline(always)]
    pub fn amp511(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp511)
    }
    ///Unmask bits\[9:0\] of LFSR/ triangle amplitude equal to 1023
    #[inline(always)]
    pub fn amp1023(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp1023)
    }
    ///Unmask bits\[10:0\] of LFSR/ triangle amplitude equal to 2047
    #[inline(always)]
    pub fn amp2047(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp2047)
    }
    ///Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn amp4095(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp4095)
    }
}
/**DAC channel%s DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1 {
    ///0: DAC Channel X DMA mode disabled
    Disabled = 0,
    ///1: DAC Channel X DMA mode enabled
    Enabled = 1,
}
impl From<DMAEN1> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN(1-2)` reader - DAC channel%s DMA enable
pub type DMAEN_R = crate::BitReader<DMAEN1>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN1 {
        match self.bits {
            false => DMAEN1::Disabled,
            true => DMAEN1::Enabled,
        }
    }
    ///DAC Channel X DMA mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1::Disabled
    }
    ///DAC Channel X DMA mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1::Enabled
    }
}
///Field `DMAEN(1-2)` writer - DAC channel%s DMA enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN1>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC Channel X DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Disabled)
    }
    ///DAC Channel X DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Enabled)
    }
}
/**DAC channel%s DMA Underrun Interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1 {
    ///0: DAC channel X DMA Underrun Interrupt disabled
    Disabled = 0,
    ///1: DAC channel X DMA Underrun Interrupt enabled
    Enabled = 1,
}
impl From<DMAUDRIE1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAUDRIE(1-2)` reader - DAC channel%s DMA Underrun Interrupt enable
pub type DMAUDRIE_R = crate::BitReader<DMAUDRIE1>;
impl DMAUDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDRIE1 {
        match self.bits {
            false => DMAUDRIE1::Disabled,
            true => DMAUDRIE1::Enabled,
        }
    }
    ///DAC channel X DMA Underrun Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1::Disabled
    }
    ///DAC channel X DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1::Enabled
    }
}
///Field `DMAUDRIE(1-2)` writer - DAC channel%s DMA Underrun Interrupt enable
pub type DMAUDRIE_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDRIE1>;
impl<'a, REG> DMAUDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X DMA Underrun Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Disabled)
    }
    ///DAC channel X DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Enabled)
    }
}
/**DAC channel%s calibration enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN1 {
    ///0: DAC Channel X Normal operating mode
    Normal = 0,
    ///1: DAC Channel X calibration mode
    Calibration = 1,
}
impl From<CEN1> for bool {
    #[inline(always)]
    fn from(variant: CEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `CEN(1-2)` reader - DAC channel%s calibration enable
pub type CEN_R = crate::BitReader<CEN1>;
impl CEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEN1 {
        match self.bits {
            false => CEN1::Normal,
            true => CEN1::Calibration,
        }
    }
    ///DAC Channel X Normal operating mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CEN1::Normal
    }
    ///DAC Channel X calibration mode
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == CEN1::Calibration
    }
}
///Field `CEN(1-2)` writer - DAC channel%s calibration enable
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG, CEN1>;
impl<'a, REG> CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC Channel X Normal operating mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1::Normal)
    }
    ///DAC Channel X calibration mode
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1::Calibration)
    }
}
///Field `TSEL2` reader - DAC channel2 trigger selection
pub use TSEL1_R as TSEL2_R;
///Field `TSEL2` writer - DAC channel2 trigger selection
pub use TSEL1_W as TSEL2_W;
impl R {
    ///DAC channel(1-2) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EN1` field.</div>
    #[inline(always)]
    pub fn en(&self, n: u8) -> EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EN_R::new(((self.bits >> (n * 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) enable
    #[inline(always)]
    pub fn en_iter(&self) -> impl Iterator<Item = EN_R> + '_ {
        (0..2).map(move |n| EN_R::new(((self.bits >> (n * 16)) & 1) != 0))
    }
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    pub fn en1(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - DAC channel2 enable
    #[inline(always)]
    pub fn en2(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///DAC channel(1-2) trigger enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TEN1` field.</div>
    #[inline(always)]
    pub fn ten(&self, n: u8) -> TEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TEN_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) trigger enable
    #[inline(always)]
    pub fn ten_iter(&self) -> impl Iterator<Item = TEN_R> + '_ {
        (0..2).map(move |n| TEN_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0))
    }
    ///Bit 1 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 17 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 2:5 - DAC channel1 trigger selection
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///DAC channel(1-2) noise/triangle wave generation enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `WAVE1` field.</div>
    #[inline(always)]
    pub fn wave(&self, n: u8) -> WAVE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        WAVE_R::new(((self.bits >> (n * 16 + 6)) & 3) as u8)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave_iter(&self) -> impl Iterator<Item = WAVE_R> + '_ {
        (0..2).map(move |n| WAVE_R::new(((self.bits >> (n * 16 + 6)) & 3) as u8))
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave1(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave2(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///DAC channel(1-2) mask/amplitude selector
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MAMP1` field.</div>
    #[inline(always)]
    pub fn mamp(&self, n: u8) -> MAMP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MAMP_R::new(((self.bits >> (n * 16 + 8)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) mask/amplitude selector
    #[inline(always)]
    pub fn mamp_iter(&self) -> impl Iterator<Item = MAMP_R> + '_ {
        (0..2).map(move |n| MAMP_R::new(((self.bits >> (n * 16 + 8)) & 0x0f) as u8))
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP_R {
        MAMP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP_R {
        MAMP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///DAC channel(1-2) DMA enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMAEN1` field.</div>
    #[inline(always)]
    pub fn dmaen(&self, n: u8) -> DMAEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMAEN_R::new(((self.bits >> (n * 16 + 12)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) DMA enable
    #[inline(always)]
    pub fn dmaen_iter(&self) -> impl Iterator<Item = DMAEN_R> + '_ {
        (0..2).map(move |n| DMAEN_R::new(((self.bits >> (n * 16 + 12)) & 1) != 0))
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 28 - DAC channel2 DMA enable
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///DAC channel(1-2) DMA Underrun Interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMAUDRIE1` field.</div>
    #[inline(always)]
    pub fn dmaudrie(&self, n: u8) -> DMAUDRIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMAUDRIE_R::new(((self.bits >> (n * 16 + 13)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie_iter(&self) -> impl Iterator<Item = DMAUDRIE_R> + '_ {
        (0..2).map(move |n| DMAUDRIE_R::new(((self.bits >> (n * 16 + 13)) & 1) != 0))
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE_R {
        DMAUDRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE_R {
        DMAUDRIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///DAC channel(1-2) calibration enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CEN1` field.</div>
    #[inline(always)]
    pub fn cen(&self, n: u8) -> CEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CEN_R::new(((self.bits >> (n * 16 + 14)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) calibration enable
    #[inline(always)]
    pub fn cen_iter(&self) -> impl Iterator<Item = CEN_R> + '_ {
        (0..2).map(move |n| CEN_R::new(((self.bits >> (n * 16 + 14)) & 1) != 0))
    }
    ///Bit 14 - DAC channel1 calibration enable
    #[inline(always)]
    pub fn cen1(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 30 - DAC channel2 calibration enable
    #[inline(always)]
    pub fn cen2(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bits 18:21 - DAC channel2 trigger selection
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en1", &self.en1())
            .field("en2", &self.en2())
            .field("ten1", &self.ten1())
            .field("ten2", &self.ten2())
            .field("tsel1", &self.tsel1())
            .field("wave1", &self.wave1())
            .field("wave2", &self.wave2())
            .field("mamp1", &self.mamp1())
            .field("mamp2", &self.mamp2())
            .field("dmaen1", &self.dmaen1())
            .field("dmaen2", &self.dmaen2())
            .field("dmaudrie1", &self.dmaudrie1())
            .field("dmaudrie2", &self.dmaudrie2())
            .field("cen1", &self.cen1())
            .field("cen2", &self.cen2())
            .field("tsel2", &self.tsel2())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EN1` field.</div>
    #[inline(always)]
    pub fn en(&mut self, n: u8) -> EN_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EN_W::new(self, n * 16)
    }
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    pub fn en1(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 16 - DAC channel2 enable
    #[inline(always)]
    pub fn en2(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 16)
    }
    ///DAC channel(1-2) trigger enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TEN1` field.</div>
    #[inline(always)]
    pub fn ten(&mut self, n: u8) -> TEN_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TEN_W::new(self, n * 16 + 1)
    }
    ///Bit 1 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN_W<CRrs> {
        TEN_W::new(self, 1)
    }
    ///Bit 17 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN_W<CRrs> {
        TEN_W::new(self, 17)
    }
    ///Bits 2:5 - DAC channel1 trigger selection
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W<CRrs> {
        TSEL1_W::new(self, 2)
    }
    ///DAC channel(1-2) noise/triangle wave generation enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `WAVE1` field.</div>
    #[inline(always)]
    pub fn wave(&mut self, n: u8) -> WAVE_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        WAVE_W::new(self, n * 16 + 6)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE_W<CRrs> {
        WAVE_W::new(self, 6)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE_W<CRrs> {
        WAVE_W::new(self, 22)
    }
    ///DAC channel(1-2) mask/amplitude selector
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MAMP1` field.</div>
    #[inline(always)]
    pub fn mamp(&mut self, n: u8) -> MAMP_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MAMP_W::new(self, n * 16 + 8)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP_W<CRrs> {
        MAMP_W::new(self, 8)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP_W<CRrs> {
        MAMP_W::new(self, 24)
    }
    ///DAC channel(1-2) DMA enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMAEN1` field.</div>
    #[inline(always)]
    pub fn dmaen(&mut self, n: u8) -> DMAEN_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMAEN_W::new(self, n * 16 + 12)
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 12)
    }
    ///Bit 28 - DAC channel2 DMA enable
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 28)
    }
    ///DAC channel(1-2) DMA Underrun Interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMAUDRIE1` field.</div>
    #[inline(always)]
    pub fn dmaudrie(&mut self, n: u8) -> DMAUDRIE_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMAUDRIE_W::new(self, n * 16 + 13)
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE_W<CRrs> {
        DMAUDRIE_W::new(self, 13)
    }
    ///Bit 29 - DAC channel2 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE_W<CRrs> {
        DMAUDRIE_W::new(self, 29)
    }
    ///DAC channel(1-2) calibration enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CEN1` field.</div>
    #[inline(always)]
    pub fn cen(&mut self, n: u8) -> CEN_W<CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CEN_W::new(self, n * 16 + 14)
    }
    ///Bit 14 - DAC channel1 calibration enable
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN_W<CRrs> {
        CEN_W::new(self, 14)
    }
    ///Bit 30 - DAC channel2 calibration enable
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN_W<CRrs> {
        CEN_W::new(self, 30)
    }
    ///Bits 18:21 - DAC channel2 trigger selection
    #[inline(always)]
    pub fn tsel2(&mut self) -> TSEL2_W<CRrs> {
        TSEL2_W::new(self, 18)
    }
}
/**DAC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DAC1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
