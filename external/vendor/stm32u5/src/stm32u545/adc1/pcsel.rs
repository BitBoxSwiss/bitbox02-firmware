///Register `PCSEL` reader
pub type R = crate::R<PCSELrs>;
///Register `PCSEL` writer
pub type W = crate::W<PCSELrs>;
/**PCSEL0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSEL0 {
    ///0: Input channel x is not preselected for conversion, the ADC conversion of this channel shows a wrong result.
    NotPreselected = 0,
    ///1: Input channel x is preselected for conversion
    Preselected = 1,
}
impl From<PCSEL0> for bool {
    #[inline(always)]
    fn from(variant: PCSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `PCSEL0` reader - PCSEL0
pub type PCSEL0_R = crate::BitReader<PCSEL0>;
impl PCSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCSEL0 {
        match self.bits {
            false => PCSEL0::NotPreselected,
            true => PCSEL0::Preselected,
        }
    }
    ///Input channel x is not preselected for conversion, the ADC conversion of this channel shows a wrong result.
    #[inline(always)]
    pub fn is_not_preselected(&self) -> bool {
        *self == PCSEL0::NotPreselected
    }
    ///Input channel x is preselected for conversion
    #[inline(always)]
    pub fn is_preselected(&self) -> bool {
        *self == PCSEL0::Preselected
    }
}
///Field `PCSEL0` writer - PCSEL0
pub type PCSEL0_W<'a, REG> = crate::BitWriter<'a, REG, PCSEL0>;
impl<'a, REG> PCSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel x is not preselected for conversion, the ADC conversion of this channel shows a wrong result.
    #[inline(always)]
    pub fn not_preselected(self) -> &'a mut crate::W<REG> {
        self.variant(PCSEL0::NotPreselected)
    }
    ///Input channel x is preselected for conversion
    #[inline(always)]
    pub fn preselected(self) -> &'a mut crate::W<REG> {
        self.variant(PCSEL0::Preselected)
    }
}
///Field `PCSEL1` reader - PCSEL1
pub use PCSEL0_R as PCSEL1_R;
///Field `PCSEL2` reader - PCSEL2
pub use PCSEL0_R as PCSEL2_R;
///Field `PCSEL3` reader - PCSEL3
pub use PCSEL0_R as PCSEL3_R;
///Field `PCSEL4` reader - PCSEL4
pub use PCSEL0_R as PCSEL4_R;
///Field `PCSEL5` reader - PCSEL5
pub use PCSEL0_R as PCSEL5_R;
///Field `PCSEL6` reader - PCSEL6
pub use PCSEL0_R as PCSEL6_R;
///Field `PCSEL7` reader - PCSEL7
pub use PCSEL0_R as PCSEL7_R;
///Field `PCSEL8` reader - PCSEL8
pub use PCSEL0_R as PCSEL8_R;
///Field `PCSEL9` reader - PCSEL9
pub use PCSEL0_R as PCSEL9_R;
///Field `PCSEL10` reader - PCSEL10
pub use PCSEL0_R as PCSEL10_R;
///Field `PCSEL11` reader - PCSEL11
pub use PCSEL0_R as PCSEL11_R;
///Field `PCSEL12` reader - PCSEL12
pub use PCSEL0_R as PCSEL12_R;
///Field `PCSEL13` reader - PCSEL13
pub use PCSEL0_R as PCSEL13_R;
///Field `PCSEL14` reader - PCSEL14
pub use PCSEL0_R as PCSEL14_R;
///Field `PCSEL15` reader - PCSEL15
pub use PCSEL0_R as PCSEL15_R;
///Field `PCSEL16` reader - PCSEL16
pub use PCSEL0_R as PCSEL16_R;
///Field `PCSEL17` reader - PCSEL17
pub use PCSEL0_R as PCSEL17_R;
///Field `PCSEL18` reader - PCSEL18
pub use PCSEL0_R as PCSEL18_R;
///Field `PCSEL19` reader - PCSEL19
pub use PCSEL0_R as PCSEL19_R;
///Field `PCSEL1` writer - PCSEL1
pub use PCSEL0_W as PCSEL1_W;
///Field `PCSEL2` writer - PCSEL2
pub use PCSEL0_W as PCSEL2_W;
///Field `PCSEL3` writer - PCSEL3
pub use PCSEL0_W as PCSEL3_W;
///Field `PCSEL4` writer - PCSEL4
pub use PCSEL0_W as PCSEL4_W;
///Field `PCSEL5` writer - PCSEL5
pub use PCSEL0_W as PCSEL5_W;
///Field `PCSEL6` writer - PCSEL6
pub use PCSEL0_W as PCSEL6_W;
///Field `PCSEL7` writer - PCSEL7
pub use PCSEL0_W as PCSEL7_W;
///Field `PCSEL8` writer - PCSEL8
pub use PCSEL0_W as PCSEL8_W;
///Field `PCSEL9` writer - PCSEL9
pub use PCSEL0_W as PCSEL9_W;
///Field `PCSEL10` writer - PCSEL10
pub use PCSEL0_W as PCSEL10_W;
///Field `PCSEL11` writer - PCSEL11
pub use PCSEL0_W as PCSEL11_W;
///Field `PCSEL12` writer - PCSEL12
pub use PCSEL0_W as PCSEL12_W;
///Field `PCSEL13` writer - PCSEL13
pub use PCSEL0_W as PCSEL13_W;
///Field `PCSEL14` writer - PCSEL14
pub use PCSEL0_W as PCSEL14_W;
///Field `PCSEL15` writer - PCSEL15
pub use PCSEL0_W as PCSEL15_W;
///Field `PCSEL16` writer - PCSEL16
pub use PCSEL0_W as PCSEL16_W;
///Field `PCSEL17` writer - PCSEL17
pub use PCSEL0_W as PCSEL17_W;
///Field `PCSEL18` writer - PCSEL18
pub use PCSEL0_W as PCSEL18_W;
///Field `PCSEL19` writer - PCSEL19
pub use PCSEL0_W as PCSEL19_W;
impl R {
    ///Bit 0 - PCSEL0
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PCSEL1
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PCSEL2
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PCSEL3
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PCSEL4
    #[inline(always)]
    pub fn pcsel4(&self) -> PCSEL4_R {
        PCSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PCSEL5
    #[inline(always)]
    pub fn pcsel5(&self) -> PCSEL5_R {
        PCSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PCSEL6
    #[inline(always)]
    pub fn pcsel6(&self) -> PCSEL6_R {
        PCSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PCSEL7
    #[inline(always)]
    pub fn pcsel7(&self) -> PCSEL7_R {
        PCSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PCSEL8
    #[inline(always)]
    pub fn pcsel8(&self) -> PCSEL8_R {
        PCSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PCSEL9
    #[inline(always)]
    pub fn pcsel9(&self) -> PCSEL9_R {
        PCSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PCSEL10
    #[inline(always)]
    pub fn pcsel10(&self) -> PCSEL10_R {
        PCSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PCSEL11
    #[inline(always)]
    pub fn pcsel11(&self) -> PCSEL11_R {
        PCSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PCSEL12
    #[inline(always)]
    pub fn pcsel12(&self) -> PCSEL12_R {
        PCSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PCSEL13
    #[inline(always)]
    pub fn pcsel13(&self) -> PCSEL13_R {
        PCSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PCSEL14
    #[inline(always)]
    pub fn pcsel14(&self) -> PCSEL14_R {
        PCSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PCSEL15
    #[inline(always)]
    pub fn pcsel15(&self) -> PCSEL15_R {
        PCSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - PCSEL16
    #[inline(always)]
    pub fn pcsel16(&self) -> PCSEL16_R {
        PCSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PCSEL17
    #[inline(always)]
    pub fn pcsel17(&self) -> PCSEL17_R {
        PCSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PCSEL18
    #[inline(always)]
    pub fn pcsel18(&self) -> PCSEL18_R {
        PCSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PCSEL19
    #[inline(always)]
    pub fn pcsel19(&self) -> PCSEL19_R {
        PCSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSEL")
            .field("pcsel0", &self.pcsel0())
            .field("pcsel19", &self.pcsel19())
            .field("pcsel18", &self.pcsel18())
            .field("pcsel17", &self.pcsel17())
            .field("pcsel16", &self.pcsel16())
            .field("pcsel15", &self.pcsel15())
            .field("pcsel14", &self.pcsel14())
            .field("pcsel13", &self.pcsel13())
            .field("pcsel12", &self.pcsel12())
            .field("pcsel11", &self.pcsel11())
            .field("pcsel10", &self.pcsel10())
            .field("pcsel9", &self.pcsel9())
            .field("pcsel8", &self.pcsel8())
            .field("pcsel7", &self.pcsel7())
            .field("pcsel6", &self.pcsel6())
            .field("pcsel5", &self.pcsel5())
            .field("pcsel4", &self.pcsel4())
            .field("pcsel3", &self.pcsel3())
            .field("pcsel2", &self.pcsel2())
            .field("pcsel1", &self.pcsel1())
            .finish()
    }
}
impl W {
    ///Bit 0 - PCSEL0
    #[inline(always)]
    pub fn pcsel0(&mut self) -> PCSEL0_W<PCSELrs> {
        PCSEL0_W::new(self, 0)
    }
    ///Bit 1 - PCSEL1
    #[inline(always)]
    pub fn pcsel1(&mut self) -> PCSEL1_W<PCSELrs> {
        PCSEL1_W::new(self, 1)
    }
    ///Bit 2 - PCSEL2
    #[inline(always)]
    pub fn pcsel2(&mut self) -> PCSEL2_W<PCSELrs> {
        PCSEL2_W::new(self, 2)
    }
    ///Bit 3 - PCSEL3
    #[inline(always)]
    pub fn pcsel3(&mut self) -> PCSEL3_W<PCSELrs> {
        PCSEL3_W::new(self, 3)
    }
    ///Bit 4 - PCSEL4
    #[inline(always)]
    pub fn pcsel4(&mut self) -> PCSEL4_W<PCSELrs> {
        PCSEL4_W::new(self, 4)
    }
    ///Bit 5 - PCSEL5
    #[inline(always)]
    pub fn pcsel5(&mut self) -> PCSEL5_W<PCSELrs> {
        PCSEL5_W::new(self, 5)
    }
    ///Bit 6 - PCSEL6
    #[inline(always)]
    pub fn pcsel6(&mut self) -> PCSEL6_W<PCSELrs> {
        PCSEL6_W::new(self, 6)
    }
    ///Bit 7 - PCSEL7
    #[inline(always)]
    pub fn pcsel7(&mut self) -> PCSEL7_W<PCSELrs> {
        PCSEL7_W::new(self, 7)
    }
    ///Bit 8 - PCSEL8
    #[inline(always)]
    pub fn pcsel8(&mut self) -> PCSEL8_W<PCSELrs> {
        PCSEL8_W::new(self, 8)
    }
    ///Bit 9 - PCSEL9
    #[inline(always)]
    pub fn pcsel9(&mut self) -> PCSEL9_W<PCSELrs> {
        PCSEL9_W::new(self, 9)
    }
    ///Bit 10 - PCSEL10
    #[inline(always)]
    pub fn pcsel10(&mut self) -> PCSEL10_W<PCSELrs> {
        PCSEL10_W::new(self, 10)
    }
    ///Bit 11 - PCSEL11
    #[inline(always)]
    pub fn pcsel11(&mut self) -> PCSEL11_W<PCSELrs> {
        PCSEL11_W::new(self, 11)
    }
    ///Bit 12 - PCSEL12
    #[inline(always)]
    pub fn pcsel12(&mut self) -> PCSEL12_W<PCSELrs> {
        PCSEL12_W::new(self, 12)
    }
    ///Bit 13 - PCSEL13
    #[inline(always)]
    pub fn pcsel13(&mut self) -> PCSEL13_W<PCSELrs> {
        PCSEL13_W::new(self, 13)
    }
    ///Bit 14 - PCSEL14
    #[inline(always)]
    pub fn pcsel14(&mut self) -> PCSEL14_W<PCSELrs> {
        PCSEL14_W::new(self, 14)
    }
    ///Bit 15 - PCSEL15
    #[inline(always)]
    pub fn pcsel15(&mut self) -> PCSEL15_W<PCSELrs> {
        PCSEL15_W::new(self, 15)
    }
    ///Bit 16 - PCSEL16
    #[inline(always)]
    pub fn pcsel16(&mut self) -> PCSEL16_W<PCSELrs> {
        PCSEL16_W::new(self, 16)
    }
    ///Bit 17 - PCSEL17
    #[inline(always)]
    pub fn pcsel17(&mut self) -> PCSEL17_W<PCSELrs> {
        PCSEL17_W::new(self, 17)
    }
    ///Bit 18 - PCSEL18
    #[inline(always)]
    pub fn pcsel18(&mut self) -> PCSEL18_W<PCSELrs> {
        PCSEL18_W::new(self, 18)
    }
    ///Bit 19 - PCSEL19
    #[inline(always)]
    pub fn pcsel19(&mut self) -> PCSEL19_W<PCSELrs> {
        PCSEL19_W::new(self, 19)
    }
}
/**ADC channel preselection register

You can [`read`](crate::Reg::read) this register and get [`pcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:PCSEL)*/
pub struct PCSELrs;
impl crate::RegisterSpec for PCSELrs {
    type Ux = u32;
}
///`read()` method returns [`pcsel::R`](R) reader structure
impl crate::Readable for PCSELrs {}
///`write(|w| ..)` method takes [`pcsel::W`](W) writer structure
impl crate::Writable for PCSELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCSEL to value 0
impl crate::Resettable for PCSELrs {}
