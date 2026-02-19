///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JL` reader - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JL_R = crate::FieldReader;
///Field `JL` writer - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
/**External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL {
    ///0: tim1_trgo
    Tim1Trgo = 0,
    ///1: tim1_oc4
    Tim1Oc4 = 1,
    ///2: tim2_trgo
    Tim2Trgo = 2,
    ///3: tim2_oc1
    Tim2Oc1 = 3,
    ///4: tim3_oc4
    Tim3Oc4 = 4,
    ///5: tim4_trgo
    Tim4Trgo = 5,
    ///6: exti15
    Exti15 = 6,
    ///7: tim8_oc4
    Tim8Oc4 = 7,
    ///8: tim1_trgo2
    Tim1Trgo2 = 8,
    ///9: tim8_trgo
    Tim8Trgo = 9,
    ///10: tim8_trgo2
    Tim8Trgo2 = 10,
    ///11: tim3_oc3
    Tim3Oc3 = 11,
    ///12: tim3_trgo
    Tim3Trgo = 12,
    ///13: tim3_oc1
    Tim3Oc1 = 13,
    ///14: tim6_trgo
    Tim6Trgo = 14,
    ///15: tim15_trgo
    Tim15Trgo = 15,
    ///16: lptim1_ch2
    Lptim1Ch2 = 16,
    ///17: lptim2_ch2
    Lptim2Ch2 = 17,
    ///18: lptim3_ch1
    Lptim3Ch1 = 18,
    ///19: lptim4_out1
    Lptim4Out1 = 19,
}
impl From<JEXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for JEXTSEL {}
///Field `JEXTSEL` reader - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL>;
impl JEXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEXTSEL> {
        match self.bits {
            0 => Some(JEXTSEL::Tim1Trgo),
            1 => Some(JEXTSEL::Tim1Oc4),
            2 => Some(JEXTSEL::Tim2Trgo),
            3 => Some(JEXTSEL::Tim2Oc1),
            4 => Some(JEXTSEL::Tim3Oc4),
            5 => Some(JEXTSEL::Tim4Trgo),
            6 => Some(JEXTSEL::Exti15),
            7 => Some(JEXTSEL::Tim8Oc4),
            8 => Some(JEXTSEL::Tim1Trgo2),
            9 => Some(JEXTSEL::Tim8Trgo),
            10 => Some(JEXTSEL::Tim8Trgo2),
            11 => Some(JEXTSEL::Tim3Oc3),
            12 => Some(JEXTSEL::Tim3Trgo),
            13 => Some(JEXTSEL::Tim3Oc1),
            14 => Some(JEXTSEL::Tim6Trgo),
            15 => Some(JEXTSEL::Tim15Trgo),
            16 => Some(JEXTSEL::Lptim1Ch2),
            17 => Some(JEXTSEL::Lptim2Ch2),
            18 => Some(JEXTSEL::Lptim3Ch1),
            19 => Some(JEXTSEL::Lptim4Out1),
            _ => None,
        }
    }
    ///tim1_trgo
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo
    }
    ///tim1_oc4
    #[inline(always)]
    pub fn is_tim1_oc4(&self) -> bool {
        *self == JEXTSEL::Tim1Oc4
    }
    ///tim2_trgo
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == JEXTSEL::Tim2Trgo
    }
    ///tim2_oc1
    #[inline(always)]
    pub fn is_tim2_oc1(&self) -> bool {
        *self == JEXTSEL::Tim2Oc1
    }
    ///tim3_oc4
    #[inline(always)]
    pub fn is_tim3_oc4(&self) -> bool {
        *self == JEXTSEL::Tim3Oc4
    }
    ///tim4_trgo
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == JEXTSEL::Tim4Trgo
    }
    ///exti15
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
    ///tim8_oc4
    #[inline(always)]
    pub fn is_tim8_oc4(&self) -> bool {
        *self == JEXTSEL::Tim8Oc4
    }
    ///tim1_trgo2
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo2
    }
    ///tim8_trgo
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == JEXTSEL::Tim8Trgo
    }
    ///tim8_trgo2
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == JEXTSEL::Tim8Trgo2
    }
    ///tim3_oc3
    #[inline(always)]
    pub fn is_tim3_oc3(&self) -> bool {
        *self == JEXTSEL::Tim3Oc3
    }
    ///tim3_trgo
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == JEXTSEL::Tim3Trgo
    }
    ///tim3_oc1
    #[inline(always)]
    pub fn is_tim3_oc1(&self) -> bool {
        *self == JEXTSEL::Tim3Oc1
    }
    ///tim6_trgo
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == JEXTSEL::Tim6Trgo
    }
    ///tim15_trgo
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == JEXTSEL::Tim15Trgo
    }
    ///lptim1_ch2
    #[inline(always)]
    pub fn is_lptim1_ch2(&self) -> bool {
        *self == JEXTSEL::Lptim1Ch2
    }
    ///lptim2_ch2
    #[inline(always)]
    pub fn is_lptim2_ch2(&self) -> bool {
        *self == JEXTSEL::Lptim2Ch2
    }
    ///lptim3_ch1
    #[inline(always)]
    pub fn is_lptim3_ch1(&self) -> bool {
        *self == JEXTSEL::Lptim3Ch1
    }
    ///lptim4_out1
    #[inline(always)]
    pub fn is_lptim4_out1(&self) -> bool {
        *self == JEXTSEL::Lptim4Out1
    }
}
///Field `JEXTSEL` writer - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, JEXTSEL>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///tim1_trgo
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo)
    }
    ///tim1_oc4
    #[inline(always)]
    pub fn tim1_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Oc4)
    }
    ///tim2_trgo
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2Trgo)
    }
    ///tim2_oc1
    #[inline(always)]
    pub fn tim2_oc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2Oc1)
    }
    ///tim3_oc4
    #[inline(always)]
    pub fn tim3_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Oc4)
    }
    ///tim4_trgo
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4Trgo)
    }
    ///exti15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
    }
    ///tim8_oc4
    #[inline(always)]
    pub fn tim8_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8Oc4)
    }
    ///tim1_trgo2
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo2)
    }
    ///tim8_trgo
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8Trgo)
    }
    ///tim8_trgo2
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8Trgo2)
    }
    ///tim3_oc3
    #[inline(always)]
    pub fn tim3_oc3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Oc3)
    }
    ///tim3_trgo
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Trgo)
    }
    ///tim3_oc1
    #[inline(always)]
    pub fn tim3_oc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Oc1)
    }
    ///tim6_trgo
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim6Trgo)
    }
    ///tim15_trgo
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim15Trgo)
    }
    ///lptim1_ch2
    #[inline(always)]
    pub fn lptim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim1Ch2)
    }
    ///lptim2_ch2
    #[inline(always)]
    pub fn lptim2_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim2Ch2)
    }
    ///lptim3_ch1
    #[inline(always)]
    pub fn lptim3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim3Ch1)
    }
    ///lptim4_out1
    #[inline(always)]
    pub fn lptim4_out1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim4Out1)
    }
}
/**External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN {
    ///0: Hardware trigger detection disabled (conversions can be launched by software)
    Disabled = 0,
    ///1: Hardware trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Hardware trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<JEXTEN> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTEN {
    type Ux = u8;
}
impl crate::IsEnum for JEXTEN {}
///Field `JEXTEN` reader - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTEN_R = crate::FieldReader<JEXTEN>;
impl JEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEXTEN {
        match self.bits {
            0 => JEXTEN::Disabled,
            1 => JEXTEN::RisingEdge,
            2 => JEXTEN::FallingEdge,
            3 => JEXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Hardware trigger detection disabled (conversions can be launched by software)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN::Disabled
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN::RisingEdge
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN::FallingEdge
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN::BothEdges
    }
}
///Field `JEXTEN` writer - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, JEXTEN, crate::Safe>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Hardware trigger detection disabled (conversions can be launched by software)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::Disabled)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::RisingEdge)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::FallingEdge)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::BothEdges)
    }
}
///Field `JSQ(1-4)` reader - %s conversion in injected sequence
pub type JSQ_R = crate::FieldReader;
///Field `JSQ(1-4)` writer - %s conversion in injected sequence
pub type JSQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&self, n: u8) -> JSQ_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_R::new(((self.bits >> (n * 6 + 9)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(1-4) conversion in injected sequence
    #[inline(always)]
    pub fn jsq_iter(&self) -> impl Iterator<Item = JSQ_R> + '_ {
        (0..4).map(move |n| JSQ_R::new(((self.bits >> (n * 6 + 9)) & 0x1f) as u8))
    }
    ///Bits 9:13 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 15:19 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 21:25 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 27:31 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JSQR")
            .field("jl", &self.jl())
            .field("jextsel", &self.jextsel())
            .field("jexten", &self.jexten())
            .field("jsq1", &self.jsq1())
            .field("jsq2", &self.jsq2())
            .field("jsq3", &self.jsq3())
            .field("jsq4", &self.jsq4())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<JSQRrs> {
        JL_W::new(self, 0)
    }
    ///Bits 2:6 - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    ///Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<JSQRrs> {
        JEXTEN_W::new(self, 7)
    }
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&mut self, n: u8) -> JSQ_W<JSQRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_W::new(self, n * 6 + 9)
    }
    ///Bits 9:13 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 9)
    }
    ///Bits 15:19 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 15)
    }
    ///Bits 21:25 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 21)
    }
    ///Bits 27:31 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 27)
    }
}
/**ADC injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC1:JSQR)*/
pub struct JSQRrs;
impl crate::RegisterSpec for JSQRrs {
    type Ux = u32;
}
///`read()` method returns [`jsqr::R`](R) reader structure
impl crate::Readable for JSQRrs {}
///`write(|w| ..)` method takes [`jsqr::W`](W) writer structure
impl crate::Writable for JSQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JSQR to value 0
impl crate::Resettable for JSQRrs {}
