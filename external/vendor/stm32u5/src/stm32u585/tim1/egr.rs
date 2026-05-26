///Register `EGR` writer
pub type W = crate::W<EGRrs>;
/**Update generation This bit can be set by software, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///1: Re-initializes the timer counter and generates an update of the registers.
    Update = 1,
}
impl From<UG> for bool {
    #[inline(always)]
    fn from(variant: UG) -> Self {
        variant as u8 != 0
    }
}
///Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware.
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Re-initializes the timer counter and generates an update of the registers.
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UG::Update)
    }
}
/**Capture/compare %s generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1GW {
    ///1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
    Trigger = 1,
}
impl From<CC1GW> for bool {
    #[inline(always)]
    fn from(variant: CC1GW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCG(1-4)` writer - Capture/compare %s generation
pub type CCG_W<'a, REG> = crate::BitWriter<'a, REG, CC1GW>;
impl<'a, REG> CCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(CC1GW::Trigger)
    }
}
/**Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMGW {
    ///1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
    Trigger = 1,
}
impl From<COMGW> for bool {
    #[inline(always)]
    fn from(variant: COMGW) -> Self {
        variant as u8 != 0
    }
}
///Field `COMG` writer - Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG, COMGW>;
impl<'a, REG> COMG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(COMGW::Trigger)
    }
}
/**Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGW {
    ///1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
    Trigger = 1,
}
impl From<TGW> for bool {
    #[inline(always)]
    fn from(variant: TGW) -> Self {
        variant as u8 != 0
    }
}
///Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG, TGW>;
impl<'a, REG> TG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TGW::Trigger)
    }
}
/**Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGW {
    ///1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
    Trigger = 1,
}
impl From<BGW> for bool {
    #[inline(always)]
    fn from(variant: BGW) -> Self {
        variant as u8 != 0
    }
}
///Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG, BGW>;
impl<'a, REG> BG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(BGW::Trigger)
    }
}
/**Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2GW {
    ///1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled
    Trigger = 1,
}
impl From<B2GW> for bool {
    #[inline(always)]
    fn from(variant: B2GW) -> Self {
        variant as u8 != 0
    }
}
///Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG, B2GW>;
impl<'a, REG> B2G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(B2GW::Trigger)
    }
}
impl core::fmt::Debug for crate::generic::Reg<EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    ///Capture/compare (1-4) generation
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1G` field.</div>
    #[inline(always)]
    pub fn ccg(&mut self, n: u8) -> CCG_W<EGRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCG_W::new(self, n + 1)
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 generation
    #[inline(always)]
    pub fn cc2g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 generation
    #[inline(always)]
    pub fn cc3g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 generation
    #[inline(always)]
    pub fn cc4g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 4)
    }
    ///Bit 5 - Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<EGRrs> {
        BG_W::new(self, 7)
    }
    ///Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn b2g(&mut self) -> B2G_W<EGRrs> {
        B2G_W::new(self, 8)
    }
}
/**TIM1 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TIM1:EGR)*/
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {}
