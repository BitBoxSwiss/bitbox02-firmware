///Register `WUCR2` reader
pub type R = crate::R<WUCR2rs>;
///Register `WUCR2` writer
pub type W = crate::W<WUCR2rs>;
/**Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP1 {
    ///0: Detection on high level (rising edge)
    RisingEdge = 0,
    ///1: Detection on low level (falling edge)
    FallingEdge = 1,
}
impl From<WUPP1> for bool {
    #[inline(always)]
    fn from(variant: WUPP1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPP1` reader - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
pub type WUPP1_R = crate::BitReader<WUPP1>;
impl WUPP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUPP1 {
        match self.bits {
            false => WUPP1::RisingEdge,
            true => WUPP1::FallingEdge,
        }
    }
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WUPP1::RisingEdge
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WUPP1::FallingEdge
    }
}
///Field `WUPP1` writer - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG, WUPP1>;
impl<'a, REG> WUPP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1::RisingEdge)
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1::FallingEdge)
    }
}
///Field `WUPP2` reader - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
pub use WUPP1_R as WUPP2_R;
///Field `WUPP3` reader - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
pub use WUPP1_R as WUPP3_R;
///Field `WUPP4` reader - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
pub use WUPP1_R as WUPP4_R;
///Field `WUPP5` reader - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
pub use WUPP1_R as WUPP5_R;
///Field `WUPP6` reader - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
pub use WUPP1_R as WUPP6_R;
///Field `WUPP7` reader - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
pub use WUPP1_R as WUPP7_R;
///Field `WUPP8` reader - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
pub use WUPP1_R as WUPP8_R;
///Field `WUPP2` writer - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
pub use WUPP1_W as WUPP2_W;
///Field `WUPP3` writer - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
pub use WUPP1_W as WUPP3_W;
///Field `WUPP4` writer - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
pub use WUPP1_W as WUPP4_W;
///Field `WUPP5` writer - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
pub use WUPP1_W as WUPP5_W;
///Field `WUPP6` writer - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
pub use WUPP1_W as WUPP6_W;
///Field `WUPP7` writer - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
pub use WUPP1_W as WUPP7_W;
///Field `WUPP8` writer - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
pub use WUPP1_W as WUPP8_W;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
    #[inline(always)]
    pub fn wupp7(&self) -> WUPP7_R {
        WUPP7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
    #[inline(always)]
    pub fn wupp8(&self) -> WUPP8_R {
        WUPP8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUCR2")
            .field("wupp1", &self.wupp1())
            .field("wupp2", &self.wupp2())
            .field("wupp3", &self.wupp3())
            .field("wupp4", &self.wupp4())
            .field("wupp5", &self.wupp5())
            .field("wupp6", &self.wupp6())
            .field("wupp7", &self.wupp7())
            .field("wupp8", &self.wupp8())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W<WUCR2rs> {
        WUPP1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W<WUCR2rs> {
        WUPP2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W<WUCR2rs> {
        WUPP3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W<WUCR2rs> {
        WUPP4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W<WUCR2rs> {
        WUPP5_W::new(self, 4)
    }
    ///Bit 5 - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
    #[inline(always)]
    pub fn wupp6(&mut self) -> WUPP6_W<WUCR2rs> {
        WUPP6_W::new(self, 5)
    }
    ///Bit 6 - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
    #[inline(always)]
    pub fn wupp7(&mut self) -> WUPP7_W<WUCR2rs> {
        WUPP7_W::new(self, 6)
    }
    ///Bit 7 - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
    #[inline(always)]
    pub fn wupp8(&mut self) -> WUPP8_W<WUCR2rs> {
        WUPP8_W::new(self, 7)
    }
}
/**PWR wakeup control register 2

You can [`read`](crate::Reg::read) this register and get [`wucr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:WUCR2)*/
pub struct WUCR2rs;
impl crate::RegisterSpec for WUCR2rs {
    type Ux = u32;
}
///`read()` method returns [`wucr2::R`](R) reader structure
impl crate::Readable for WUCR2rs {}
///`write(|w| ..)` method takes [`wucr2::W`](W) writer structure
impl crate::Writable for WUCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUCR2 to value 0
impl crate::Resettable for WUCR2rs {}
