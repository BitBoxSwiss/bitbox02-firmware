///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**DMNGT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMNGT {
    ///0: Store output data in DR only
    Dr = 0,
    ///1: DMA One Shot Mode selected
    DmaOneShot = 1,
    ///2: DFSDM mode selected
    Dfsdm = 2,
    ///3: DMA Circular Mode selected
    DmaCircular = 3,
}
impl From<DMNGT> for u8 {
    #[inline(always)]
    fn from(variant: DMNGT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMNGT {
    type Ux = u8;
}
impl crate::IsEnum for DMNGT {}
///Field `DMNGT` reader - DMNGT
pub type DMNGT_R = crate::FieldReader<DMNGT>;
impl DMNGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMNGT {
        match self.bits {
            0 => DMNGT::Dr,
            1 => DMNGT::DmaOneShot,
            2 => DMNGT::Dfsdm,
            3 => DMNGT::DmaCircular,
            _ => unreachable!(),
        }
    }
    ///Store output data in DR only
    #[inline(always)]
    pub fn is_dr(&self) -> bool {
        *self == DMNGT::Dr
    }
    ///DMA One Shot Mode selected
    #[inline(always)]
    pub fn is_dma_one_shot(&self) -> bool {
        *self == DMNGT::DmaOneShot
    }
    ///DFSDM mode selected
    #[inline(always)]
    pub fn is_dfsdm(&self) -> bool {
        *self == DMNGT::Dfsdm
    }
    ///DMA Circular Mode selected
    #[inline(always)]
    pub fn is_dma_circular(&self) -> bool {
        *self == DMNGT::DmaCircular
    }
}
///Field `DMNGT` writer - DMNGT
pub type DMNGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMNGT, crate::Safe>;
impl<'a, REG> DMNGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Store output data in DR only
    #[inline(always)]
    pub fn dr(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::Dr)
    }
    ///DMA One Shot Mode selected
    #[inline(always)]
    pub fn dma_one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::DmaOneShot)
    }
    ///DFSDM mode selected
    #[inline(always)]
    pub fn dfsdm(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::Dfsdm)
    }
    ///DMA Circular Mode selected
    #[inline(always)]
    pub fn dma_circular(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::DmaCircular)
    }
}
/**RES

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 14 bits
    FourteenBit = 0,
    ///1: 12 bits
    TwelveBit = 1,
    ///2: 10 bits
    TenBit = 2,
    ///3: 8 bits
    EightBit = 3,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
impl crate::IsEnum for RES {}
///Field `RES` reader - RES
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RES {
        match self.bits {
            0 => RES::FourteenBit,
            1 => RES::TwelveBit,
            2 => RES::TenBit,
            3 => RES::EightBit,
            _ => unreachable!(),
        }
    }
    ///14 bits
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == RES::FourteenBit
    }
    ///12 bits
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES::TwelveBit
    }
    ///10 bits
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES::TenBit
    }
    ///8 bits
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES::EightBit
    }
}
///Field `RES` writer - RES
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///14 bits
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::FourteenBit)
    }
    ///12 bits
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBit)
    }
    ///10 bits
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TenBit)
    }
    ///8 bits
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::EightBit)
    }
}
/**EXTSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: tim1_oc1
    Tim1Oc1 = 0,
    ///1: tim1_oc2
    Tim1Oc2 = 1,
    ///2: tim1_oc3
    Tim1Oc3 = 2,
    ///3: tim2_oc2
    Tim2Oc2 = 3,
    ///4: tim3_trgo
    Tim3Trgo = 4,
    ///5: tim4_oc4
    Tim4Oc4 = 5,
    ///6: exti11
    Exti11 = 6,
    ///7: tim8_trgo
    Tim8Trgo = 7,
    ///8: tim8_trgo2
    Tim8Trgo2 = 8,
    ///9: tim1_trgo
    Tim1Trgo = 9,
    ///10: tim1_trgo2
    Tim1Trgo2 = 10,
    ///11: tim2_trgo
    Tim2Trgo = 11,
    ///12: tim4_trgo
    Tim4Trgo = 12,
    ///13: tim6_trgo
    Tim6Trgo = 13,
    ///14: tim15_trgo
    Tim15Trgo = 14,
    ///15: tim3_oc4
    Tim3Oc4 = 15,
    ///16: exti15
    Exti15 = 16,
    ///18: lptim1_ch1
    Lptim1Ch1 = 18,
    ///19: lptim2_ch1
    Lptim2Ch1 = 19,
    ///20: lptim3_ch1
    Lptim3Ch1 = 20,
    ///21: lptim4_out
    Lptim4Out = 21,
}
impl From<EXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL {}
///Field `EXTSEL` reader - EXTSEL
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTSEL> {
        match self.bits {
            0 => Some(EXTSEL::Tim1Oc1),
            1 => Some(EXTSEL::Tim1Oc2),
            2 => Some(EXTSEL::Tim1Oc3),
            3 => Some(EXTSEL::Tim2Oc2),
            4 => Some(EXTSEL::Tim3Trgo),
            5 => Some(EXTSEL::Tim4Oc4),
            6 => Some(EXTSEL::Exti11),
            7 => Some(EXTSEL::Tim8Trgo),
            8 => Some(EXTSEL::Tim8Trgo2),
            9 => Some(EXTSEL::Tim1Trgo),
            10 => Some(EXTSEL::Tim1Trgo2),
            11 => Some(EXTSEL::Tim2Trgo),
            12 => Some(EXTSEL::Tim4Trgo),
            13 => Some(EXTSEL::Tim6Trgo),
            14 => Some(EXTSEL::Tim15Trgo),
            15 => Some(EXTSEL::Tim3Oc4),
            16 => Some(EXTSEL::Exti15),
            18 => Some(EXTSEL::Lptim1Ch1),
            19 => Some(EXTSEL::Lptim2Ch1),
            20 => Some(EXTSEL::Lptim3Ch1),
            21 => Some(EXTSEL::Lptim4Out),
            _ => None,
        }
    }
    ///tim1_oc1
    #[inline(always)]
    pub fn is_tim1_oc1(&self) -> bool {
        *self == EXTSEL::Tim1Oc1
    }
    ///tim1_oc2
    #[inline(always)]
    pub fn is_tim1_oc2(&self) -> bool {
        *self == EXTSEL::Tim1Oc2
    }
    ///tim1_oc3
    #[inline(always)]
    pub fn is_tim1_oc3(&self) -> bool {
        *self == EXTSEL::Tim1Oc3
    }
    ///tim2_oc2
    #[inline(always)]
    pub fn is_tim2_oc2(&self) -> bool {
        *self == EXTSEL::Tim2Oc2
    }
    ///tim3_trgo
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL::Tim3Trgo
    }
    ///tim4_oc4
    #[inline(always)]
    pub fn is_tim4_oc4(&self) -> bool {
        *self == EXTSEL::Tim4Oc4
    }
    ///exti11
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
    ///tim8_trgo
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == EXTSEL::Tim8Trgo
    }
    ///tim8_trgo2
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == EXTSEL::Tim8Trgo2
    }
    ///tim1_trgo
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL::Tim1Trgo
    }
    ///tim1_trgo2
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL::Tim1Trgo2
    }
    ///tim2_trgo
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL::Tim2Trgo
    }
    ///tim4_trgo
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == EXTSEL::Tim4Trgo
    }
    ///tim6_trgo
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL::Tim6Trgo
    }
    ///tim15_trgo
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL::Tim15Trgo
    }
    ///tim3_oc4
    #[inline(always)]
    pub fn is_tim3_oc4(&self) -> bool {
        *self == EXTSEL::Tim3Oc4
    }
    ///exti15
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == EXTSEL::Exti15
    }
    ///lptim1_ch1
    #[inline(always)]
    pub fn is_lptim1_ch1(&self) -> bool {
        *self == EXTSEL::Lptim1Ch1
    }
    ///lptim2_ch1
    #[inline(always)]
    pub fn is_lptim2_ch1(&self) -> bool {
        *self == EXTSEL::Lptim2Ch1
    }
    ///lptim3_ch1
    #[inline(always)]
    pub fn is_lptim3_ch1(&self) -> bool {
        *self == EXTSEL::Lptim3Ch1
    }
    ///lptim4_out
    #[inline(always)]
    pub fn is_lptim4_out(&self) -> bool {
        *self == EXTSEL::Lptim4Out
    }
}
///Field `EXTSEL` writer - EXTSEL
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///tim1_oc1
    #[inline(always)]
    pub fn tim1_oc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Oc1)
    }
    ///tim1_oc2
    #[inline(always)]
    pub fn tim1_oc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Oc2)
    }
    ///tim1_oc3
    #[inline(always)]
    pub fn tim1_oc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Oc3)
    }
    ///tim2_oc2
    #[inline(always)]
    pub fn tim2_oc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Oc2)
    }
    ///tim3_trgo
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Trgo)
    }
    ///tim4_oc4
    #[inline(always)]
    pub fn tim4_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4Oc4)
    }
    ///exti11
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
    ///tim8_trgo
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8Trgo)
    }
    ///tim8_trgo2
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8Trgo2)
    }
    ///tim1_trgo
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo)
    }
    ///tim1_trgo2
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo2)
    }
    ///tim2_trgo
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Trgo)
    }
    ///tim4_trgo
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4Trgo)
    }
    ///tim6_trgo
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim6Trgo)
    }
    ///tim15_trgo
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim15Trgo)
    }
    ///tim3_oc4
    #[inline(always)]
    pub fn tim3_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Oc4)
    }
    ///exti15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti15)
    }
    ///lptim1_ch1
    #[inline(always)]
    pub fn lptim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim1Ch1)
    }
    ///lptim2_ch1
    #[inline(always)]
    pub fn lptim2_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim2Ch1)
    }
    ///lptim3_ch1
    #[inline(always)]
    pub fn lptim3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim3Ch1)
    }
    ///lptim4_out
    #[inline(always)]
    pub fn lptim4_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim4Out)
    }
}
/**EXTEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    ///0: Hardware trigger detection disabled (conversions can be launched by software)
    Disabled = 0,
    ///1: Hardware trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Hardware trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<EXTEN> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN {
    type Ux = u8;
}
impl crate::IsEnum for EXTEN {}
///Field `EXTEN` reader - EXTEN
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN {
        match self.bits {
            0 => EXTEN::Disabled,
            1 => EXTEN::RisingEdge,
            2 => EXTEN::FallingEdge,
            3 => EXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Hardware trigger detection disabled (conversions can be launched by software)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
///Field `EXTEN` writer - EXTEN
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Hardware trigger detection disabled (conversions can be launched by software)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
/**OVRMOD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    ///0: ADC_DR register is preserved with the old data when an overrun is detected
    Preserve = 0,
    ///1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
    Overwrite = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - OVRMOD
pub type OVRMOD_R = crate::BitReader<OVRMOD>;
impl OVRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD {
        match self.bits {
            false => OVRMOD::Preserve,
            true => OVRMOD::Overwrite,
        }
    }
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD::Preserve
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD::Overwrite
    }
}
///Field `OVRMOD` writer - OVRMOD
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Preserve)
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Overwrite)
    }
}
/**CONT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: Single conversion mode
    Single = 0,
    ///1: Continuous conversion mode
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - CONT
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::Single,
            true => CONT::Continuous,
        }
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT::Single
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
///Field `CONT` writer - CONT
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Single)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
/**AUTDLY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY {
    ///0: Auto-delayed conversion mode off
    Disabled = 0,
    ///1: Auto-delayed conversion mode on
    Enabled = 1,
}
impl From<AUTDLY> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTDLY` reader - AUTDLY
pub type AUTDLY_R = crate::BitReader<AUTDLY>;
impl AUTDLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTDLY {
        match self.bits {
            false => AUTDLY::Disabled,
            true => AUTDLY::Enabled,
        }
    }
    ///Auto-delayed conversion mode off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTDLY::Disabled
    }
    ///Auto-delayed conversion mode on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTDLY::Enabled
    }
}
///Field `AUTDLY` writer - AUTDLY
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG, AUTDLY>;
impl<'a, REG> AUTDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto-delayed conversion mode off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::Disabled)
    }
    ///Auto-delayed conversion mode on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::Enabled)
    }
}
/**DISCEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    ///0: Discontinuous mode for regular channels disabled
    Disabled = 0,
    ///1: Discontinuous mode for regular channels enabled
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - DISCEN
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::Disabled,
            true => DISCEN::Enabled,
        }
    }
    ///Discontinuous mode for regular channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    ///Discontinuous mode for regular channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
///Field `DISCEN` writer - DISCEN
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode for regular channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    ///Discontinuous mode for regular channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
/**DISCNUM

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISCNUM {
    ///0: 1 channel
    N1 = 0,
    ///1: 2 channels
    N2 = 1,
    ///2: 3 channels
    N3 = 2,
    ///3: 4 channels
    N4 = 3,
    ///4: 5 channels
    N5 = 4,
    ///5: 6 channels
    N6 = 5,
    ///6: 7 channels
    N7 = 6,
    ///7: 8 channels
    N8 = 7,
}
impl From<DISCNUM> for u8 {
    #[inline(always)]
    fn from(variant: DISCNUM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DISCNUM {
    type Ux = u8;
}
impl crate::IsEnum for DISCNUM {}
///Field `DISCNUM` reader - DISCNUM
pub type DISCNUM_R = crate::FieldReader<DISCNUM>;
impl DISCNUM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISCNUM {
        match self.bits {
            0 => DISCNUM::N1,
            1 => DISCNUM::N2,
            2 => DISCNUM::N3,
            3 => DISCNUM::N4,
            4 => DISCNUM::N5,
            5 => DISCNUM::N6,
            6 => DISCNUM::N7,
            7 => DISCNUM::N8,
            _ => unreachable!(),
        }
    }
    ///1 channel
    #[inline(always)]
    pub fn is_n1(&self) -> bool {
        *self == DISCNUM::N1
    }
    ///2 channels
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == DISCNUM::N2
    }
    ///3 channels
    #[inline(always)]
    pub fn is_n3(&self) -> bool {
        *self == DISCNUM::N3
    }
    ///4 channels
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == DISCNUM::N4
    }
    ///5 channels
    #[inline(always)]
    pub fn is_n5(&self) -> bool {
        *self == DISCNUM::N5
    }
    ///6 channels
    #[inline(always)]
    pub fn is_n6(&self) -> bool {
        *self == DISCNUM::N6
    }
    ///7 channels
    #[inline(always)]
    pub fn is_n7(&self) -> bool {
        *self == DISCNUM::N7
    }
    ///8 channels
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == DISCNUM::N8
    }
}
///Field `DISCNUM` writer - DISCNUM
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DISCNUM, crate::Safe>;
impl<'a, REG> DISCNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 channel
    #[inline(always)]
    pub fn n1(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N1)
    }
    ///2 channels
    #[inline(always)]
    pub fn n2(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N2)
    }
    ///3 channels
    #[inline(always)]
    pub fn n3(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N3)
    }
    ///4 channels
    #[inline(always)]
    pub fn n4(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N4)
    }
    ///5 channels
    #[inline(always)]
    pub fn n5(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N5)
    }
    ///6 channels
    #[inline(always)]
    pub fn n6(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N6)
    }
    ///7 channels
    #[inline(always)]
    pub fn n7(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N7)
    }
    ///8 channels
    #[inline(always)]
    pub fn n8(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM::N8)
    }
}
/**JDISCEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN {
    ///0: Discontinuous mode on injected channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on injected channels enabled
    Enabled = 1,
}
impl From<JDISCEN> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JDISCEN` reader - JDISCEN
pub type JDISCEN_R = crate::BitReader<JDISCEN>;
impl JDISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JDISCEN {
        match self.bits {
            false => JDISCEN::Disabled,
            true => JDISCEN::Enabled,
        }
    }
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN::Disabled
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN::Enabled
    }
}
///Field `JDISCEN` writer - JDISCEN
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG, JDISCEN>;
impl<'a, REG> JDISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Disabled)
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Enabled)
    }
}
/**AWD1SGL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL {
    ///0: Analog watchdog 1 enabled on all channels
    AllChannels = 0,
    ///1: Analog watchdog 1 enabled on a single channel
    SingleChannel = 1,
}
impl From<AWD1SGL> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1SGL` reader - AWD1SGL
pub type AWD1SGL_R = crate::BitReader<AWD1SGL>;
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL {
        match self.bits {
            false => AWD1SGL::AllChannels,
            true => AWD1SGL::SingleChannel,
        }
    }
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWD1SGL::AllChannels
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWD1SGL::SingleChannel
    }
}
///Field `AWD1SGL` writer - AWD1SGL
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::AllChannels)
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::SingleChannel)
    }
}
/**AWD1EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN {
    ///0: Analog watchdog 1 disabled on regular channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on regular channels
    Enabled = 1,
}
impl From<AWD1EN> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1EN` reader - AWD1EN
pub type AWD1EN_R = crate::BitReader<AWD1EN>;
impl AWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN {
        match self.bits {
            false => AWD1EN::Disabled,
            true => AWD1EN::Enabled,
        }
    }
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN::Enabled
    }
}
///Field `AWD1EN` writer - AWD1EN
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Enabled)
    }
}
/**JAWD1EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN {
    ///0: Analog watchdog 1 disabled on injected channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on injected channels
    Enabled = 1,
}
impl From<JAWD1EN> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `JAWD1EN` reader - JAWD1EN
pub type JAWD1EN_R = crate::BitReader<JAWD1EN>;
impl JAWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAWD1EN {
        match self.bits {
            false => JAWD1EN::Disabled,
            true => JAWD1EN::Enabled,
        }
    }
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN::Enabled
    }
}
///Field `JAWD1EN` writer - JAWD1EN
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, JAWD1EN>;
impl<'a, REG> JAWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Enabled)
    }
}
/**JAUTO

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO {
    ///0: Automatic injected group conversion disabled
    Disabled = 0,
    ///1: Automatic injected group conversion enabled
    Enabled = 1,
}
impl From<JAUTO> for bool {
    #[inline(always)]
    fn from(variant: JAUTO) -> Self {
        variant as u8 != 0
    }
}
///Field `JAUTO` reader - JAUTO
pub type JAUTO_R = crate::BitReader<JAUTO>;
impl JAUTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAUTO {
        match self.bits {
            false => JAUTO::Disabled,
            true => JAUTO::Enabled,
        }
    }
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO::Disabled
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO::Enabled
    }
}
///Field `JAUTO` writer - JAUTO
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG, JAUTO>;
impl<'a, REG> JAUTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Disabled)
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Enabled)
    }
}
///Field `AWD1CH` reader - AWD1CH
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - AWD1CH
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - DMNGT
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:9 - EXTSEL
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AUTDLY
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - DISCNUM
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - JDISCEN
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - JAWD1EN
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - JAUTO
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("awd1ch", &self.awd1ch())
            .field("jauto", &self.jauto())
            .field("jawd1en", &self.jawd1en())
            .field("awd1en", &self.awd1en())
            .field("awd1sgl", &self.awd1sgl())
            .field("jdiscen", &self.jdiscen())
            .field("discnum", &self.discnum())
            .field("discen", &self.discen())
            .field("autdly", &self.autdly())
            .field("cont", &self.cont())
            .field("ovrmod", &self.ovrmod())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("res", &self.res())
            .field("dmngt", &self.dmngt())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DMNGT
    #[inline(always)]
    pub fn dmngt(&mut self) -> DMNGT_W<CFGR1rs> {
        DMNGT_W::new(self, 0)
    }
    ///Bits 2:3 - RES
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<CFGR1rs> {
        RES_W::new(self, 2)
    }
    ///Bits 5:9 - EXTSEL
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<CFGR1rs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - AUTDLY
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<CFGR1rs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - DISCNUM
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<CFGR1rs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - JDISCEN
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<CFGR1rs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - JAWD1EN
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<CFGR1rs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - JAUTO
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<CFGR1rs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0x8000_0000
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
