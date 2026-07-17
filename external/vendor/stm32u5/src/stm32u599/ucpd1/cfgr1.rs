///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `HBITCLKDIV` reader - HBITCLKDIV
pub type HBITCLKDIV_R = crate::FieldReader;
///Field `HBITCLKDIV` writer - HBITCLKDIV
pub type HBITCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
///Field `IFRGAP` reader - IFRGAP
pub type IFRGAP_R = crate::FieldReader;
///Field `IFRGAP` writer - IFRGAP
pub type IFRGAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRANSWIN` reader - TRANSWIN
pub type TRANSWIN_R = crate::FieldReader;
///Field `TRANSWIN` writer - TRANSWIN
pub type TRANSWIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**PSC_USBPDCLK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC_USBPDCLK {
    ///0: Divide by 1
    Div1 = 0,
    ///1: Divide by 2
    Div2 = 1,
    ///2: Divide by 4
    Div4 = 2,
    ///3: Divide by 8
    Div8 = 3,
    ///4: Divide by 16
    Div16 = 4,
}
impl From<PSC_USBPDCLK> for u8 {
    #[inline(always)]
    fn from(variant: PSC_USBPDCLK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSC_USBPDCLK {
    type Ux = u8;
}
impl crate::IsEnum for PSC_USBPDCLK {}
///Field `PSC_USBPDCLK` reader - PSC_USBPDCLK
pub type PSC_USBPDCLK_R = crate::FieldReader<PSC_USBPDCLK>;
impl PSC_USBPDCLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSC_USBPDCLK> {
        match self.bits {
            0 => Some(PSC_USBPDCLK::Div1),
            1 => Some(PSC_USBPDCLK::Div2),
            2 => Some(PSC_USBPDCLK::Div4),
            3 => Some(PSC_USBPDCLK::Div8),
            4 => Some(PSC_USBPDCLK::Div16),
            _ => None,
        }
    }
    ///Divide by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PSC_USBPDCLK::Div1
    }
    ///Divide by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_USBPDCLK::Div2
    }
    ///Divide by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_USBPDCLK::Div4
    }
    ///Divide by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_USBPDCLK::Div8
    }
    ///Divide by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSC_USBPDCLK::Div16
    }
}
///Field `PSC_USBPDCLK` writer - PSC_USBPDCLK
pub type PSC_USBPDCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PSC_USBPDCLK>;
impl<'a, REG> PSC_USBPDCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_USBPDCLK::Div1)
    }
    ///Divide by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_USBPDCLK::Div2)
    }
    ///Divide by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_USBPDCLK::Div4)
    }
    ///Divide by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_USBPDCLK::Div8)
    }
    ///Divide by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_USBPDCLK::Div16)
    }
}
/**SOP detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORDSETEN0 {
    ///0: Flag disabled
    Disabled = 0,
    ///1: Flag enabled
    Enabled = 1,
}
impl From<RXORDSETEN0> for bool {
    #[inline(always)]
    fn from(variant: RXORDSETEN0) -> Self {
        variant as u8 != 0
    }
}
///Field `RXORDSETEN0` reader - SOP detection
pub type RXORDSETEN0_R = crate::BitReader<RXORDSETEN0>;
impl RXORDSETEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXORDSETEN0 {
        match self.bits {
            false => RXORDSETEN0::Disabled,
            true => RXORDSETEN0::Enabled,
        }
    }
    ///Flag disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXORDSETEN0::Disabled
    }
    ///Flag enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXORDSETEN0::Enabled
    }
}
///Field `RXORDSETEN0` writer - SOP detection
pub type RXORDSETEN0_W<'a, REG> = crate::BitWriter<'a, REG, RXORDSETEN0>;
impl<'a, REG> RXORDSETEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flag disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXORDSETEN0::Disabled)
    }
    ///Flag enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXORDSETEN0::Enabled)
    }
}
///Field `RXORDSETEN1` reader - SOP' detection
pub use RXORDSETEN0_R as RXORDSETEN1_R;
///Field `RXORDSETEN2` reader - SOP'' detection
pub use RXORDSETEN0_R as RXORDSETEN2_R;
///Field `RXORDSETEN3` reader - Hard Reset detection
pub use RXORDSETEN0_R as RXORDSETEN3_R;
///Field `RXORDSETEN4` reader - Cable Detect reset
pub use RXORDSETEN0_R as RXORDSETEN4_R;
///Field `RXORDSETEN5` reader - SOP'_Debug
pub use RXORDSETEN0_R as RXORDSETEN5_R;
///Field `RXORDSETEN6` reader - SOP'' Debug
pub use RXORDSETEN0_R as RXORDSETEN6_R;
///Field `RXORDSETEN7` reader - SOP extension #1
pub use RXORDSETEN0_R as RXORDSETEN7_R;
///Field `RXORDSETEN8` reader - SOP extension #2
pub use RXORDSETEN0_R as RXORDSETEN8_R;
///Field `RXORDSETEN1` writer - SOP' detection
pub use RXORDSETEN0_W as RXORDSETEN1_W;
///Field `RXORDSETEN2` writer - SOP'' detection
pub use RXORDSETEN0_W as RXORDSETEN2_W;
///Field `RXORDSETEN3` writer - Hard Reset detection
pub use RXORDSETEN0_W as RXORDSETEN3_W;
///Field `RXORDSETEN4` writer - Cable Detect reset
pub use RXORDSETEN0_W as RXORDSETEN4_W;
///Field `RXORDSETEN5` writer - SOP'_Debug
pub use RXORDSETEN0_W as RXORDSETEN5_W;
///Field `RXORDSETEN6` writer - SOP'' Debug
pub use RXORDSETEN0_W as RXORDSETEN6_W;
///Field `RXORDSETEN7` writer - SOP extension #1
pub use RXORDSETEN0_W as RXORDSETEN7_W;
///Field `RXORDSETEN8` writer - SOP extension #2
pub use RXORDSETEN0_W as RXORDSETEN8_W;
/**TXDMAEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    ///0: DMA mode for transmission disabled
    Disabled = 0,
    ///1: DMA mode for transmission enabled
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - TXDMAEN
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    ///DMA mode for transmission disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    ///DMA mode for transmission enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
///Field `TXDMAEN` writer - TXDMAEN
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode for transmission disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    ///DMA mode for transmission enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
/**RXDMAEN:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    ///0: DMA mode for reception disabled
    Disabled = 0,
    ///1: DMA mode for reception enabled
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - RXDMAEN:
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    ///DMA mode for reception disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    ///DMA mode for reception enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
///Field `RXDMAEN` writer - RXDMAEN:
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode for reception disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    ///DMA mode for reception enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
/**UCPDEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPDEN {
    ///0: UCPD peripheral disabled
    Disabled = 0,
    ///1: UCPD peripheral enabled
    Enabled = 1,
}
impl From<UCPDEN> for bool {
    #[inline(always)]
    fn from(variant: UCPDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `UCPDEN` reader - UCPDEN
pub type UCPDEN_R = crate::BitReader<UCPDEN>;
impl UCPDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UCPDEN {
        match self.bits {
            false => UCPDEN::Disabled,
            true => UCPDEN::Enabled,
        }
    }
    ///UCPD peripheral disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UCPDEN::Disabled
    }
    ///UCPD peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UCPDEN::Enabled
    }
}
///Field `UCPDEN` writer - UCPDEN
pub type UCPDEN_W<'a, REG> = crate::BitWriter<'a, REG, UCPDEN>;
impl<'a, REG> UCPDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UCPD peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UCPDEN::Disabled)
    }
    ///UCPD peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UCPDEN::Enabled)
    }
}
impl R {
    ///Bits 0:5 - HBITCLKDIV
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:10 - IFRGAP
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - TRANSWIN
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 17:19 - PSC_USBPDCLK
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - SOP detection
    #[inline(always)]
    pub fn rxordseten0(&self) -> RXORDSETEN0_R {
        RXORDSETEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SOP' detection
    #[inline(always)]
    pub fn rxordseten1(&self) -> RXORDSETEN1_R {
        RXORDSETEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SOP'' detection
    #[inline(always)]
    pub fn rxordseten2(&self) -> RXORDSETEN2_R {
        RXORDSETEN2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Hard Reset detection
    #[inline(always)]
    pub fn rxordseten3(&self) -> RXORDSETEN3_R {
        RXORDSETEN3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Cable Detect reset
    #[inline(always)]
    pub fn rxordseten4(&self) -> RXORDSETEN4_R {
        RXORDSETEN4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SOP'_Debug
    #[inline(always)]
    pub fn rxordseten5(&self) -> RXORDSETEN5_R {
        RXORDSETEN5_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SOP'' Debug
    #[inline(always)]
    pub fn rxordseten6(&self) -> RXORDSETEN6_R {
        RXORDSETEN6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SOP extension #1
    #[inline(always)]
    pub fn rxordseten7(&self) -> RXORDSETEN7_R {
        RXORDSETEN7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SOP extension #2
    #[inline(always)]
    pub fn rxordseten8(&self) -> RXORDSETEN8_R {
        RXORDSETEN8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - RXDMAEN:
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UCPDEN
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("hbitclkdiv", &self.hbitclkdiv())
            .field("ifrgap", &self.ifrgap())
            .field("transwin", &self.transwin())
            .field("psc_usbpdclk", &self.psc_usbpdclk())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("ucpden", &self.ucpden())
            .field("rxordseten0", &self.rxordseten0())
            .field("rxordseten1", &self.rxordseten1())
            .field("rxordseten2", &self.rxordseten2())
            .field("rxordseten3", &self.rxordseten3())
            .field("rxordseten4", &self.rxordseten4())
            .field("rxordseten5", &self.rxordseten5())
            .field("rxordseten6", &self.rxordseten6())
            .field("rxordseten7", &self.rxordseten7())
            .field("rxordseten8", &self.rxordseten8())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - HBITCLKDIV
    #[inline(always)]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W<CFGR1rs> {
        HBITCLKDIV_W::new(self, 0)
    }
    ///Bits 6:10 - IFRGAP
    #[inline(always)]
    pub fn ifrgap(&mut self) -> IFRGAP_W<CFGR1rs> {
        IFRGAP_W::new(self, 6)
    }
    ///Bits 11:15 - TRANSWIN
    #[inline(always)]
    pub fn transwin(&mut self) -> TRANSWIN_W<CFGR1rs> {
        TRANSWIN_W::new(self, 11)
    }
    ///Bits 17:19 - PSC_USBPDCLK
    #[inline(always)]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W<CFGR1rs> {
        PSC_USBPDCLK_W::new(self, 17)
    }
    ///Bit 20 - SOP detection
    #[inline(always)]
    pub fn rxordseten0(&mut self) -> RXORDSETEN0_W<CFGR1rs> {
        RXORDSETEN0_W::new(self, 20)
    }
    ///Bit 21 - SOP' detection
    #[inline(always)]
    pub fn rxordseten1(&mut self) -> RXORDSETEN1_W<CFGR1rs> {
        RXORDSETEN1_W::new(self, 21)
    }
    ///Bit 22 - SOP'' detection
    #[inline(always)]
    pub fn rxordseten2(&mut self) -> RXORDSETEN2_W<CFGR1rs> {
        RXORDSETEN2_W::new(self, 22)
    }
    ///Bit 23 - Hard Reset detection
    #[inline(always)]
    pub fn rxordseten3(&mut self) -> RXORDSETEN3_W<CFGR1rs> {
        RXORDSETEN3_W::new(self, 23)
    }
    ///Bit 24 - Cable Detect reset
    #[inline(always)]
    pub fn rxordseten4(&mut self) -> RXORDSETEN4_W<CFGR1rs> {
        RXORDSETEN4_W::new(self, 24)
    }
    ///Bit 25 - SOP'_Debug
    #[inline(always)]
    pub fn rxordseten5(&mut self) -> RXORDSETEN5_W<CFGR1rs> {
        RXORDSETEN5_W::new(self, 25)
    }
    ///Bit 26 - SOP'' Debug
    #[inline(always)]
    pub fn rxordseten6(&mut self) -> RXORDSETEN6_W<CFGR1rs> {
        RXORDSETEN6_W::new(self, 26)
    }
    ///Bit 27 - SOP extension #1
    #[inline(always)]
    pub fn rxordseten7(&mut self) -> RXORDSETEN7_W<CFGR1rs> {
        RXORDSETEN7_W::new(self, 27)
    }
    ///Bit 28 - SOP extension #2
    #[inline(always)]
    pub fn rxordseten8(&mut self) -> RXORDSETEN8_W<CFGR1rs> {
        RXORDSETEN8_W::new(self, 28)
    }
    ///Bit 29 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFGR1rs> {
        TXDMAEN_W::new(self, 29)
    }
    ///Bit 30 - RXDMAEN:
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFGR1rs> {
        RXDMAEN_W::new(self, 30)
    }
    ///Bit 31 - UCPDEN
    #[inline(always)]
    pub fn ucpden(&mut self) -> UCPDEN_W<CFGR1rs> {
        UCPDEN_W::new(self, 31)
    }
}
/**UCPD configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:CFGR1)*/
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
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
