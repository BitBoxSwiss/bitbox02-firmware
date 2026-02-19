///Register `WUCR1` reader
pub type R = crate::R<WUCR1rs>;
///Register `WUCR1` writer
pub type W = crate::W<WUCR1rs>;
/**Wakeup pin WKUP1 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN1 {
    ///0: Wakeup pin disabled
    Disabled = 0,
    ///1: Wakeup pin enabled
    Enabled = 1,
}
impl From<WUPEN1> for bool {
    #[inline(always)]
    fn from(variant: WUPEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPEN1` reader - Wakeup pin WKUP1 enable
pub type WUPEN1_R = crate::BitReader<WUPEN1>;
impl WUPEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN1 {
        match self.bits {
            false => WUPEN1::Disabled,
            true => WUPEN1::Enabled,
        }
    }
    ///Wakeup pin disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN1::Disabled
    }
    ///Wakeup pin enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN1::Enabled
    }
}
///Field `WUPEN1` writer - Wakeup pin WKUP1 enable
pub type WUPEN1_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN1>;
impl<'a, REG> WUPEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wakeup pin disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1::Disabled)
    }
    ///Wakeup pin enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1::Enabled)
    }
}
///Field `WUPEN2` reader - Wakeup pin WKUP2 enable
pub use WUPEN1_R as WUPEN2_R;
///Field `WUPEN3` reader - Wakeup pin WKUP3 enable
pub use WUPEN1_R as WUPEN3_R;
///Field `WUPEN4` reader - Wakeup pin WKUP4 enable
pub use WUPEN1_R as WUPEN4_R;
///Field `WUPEN5` reader - Wakeup pin WKUP5 enable
pub use WUPEN1_R as WUPEN5_R;
///Field `WUPEN6` reader - Wakeup pin WKUP6 enable
pub use WUPEN1_R as WUPEN6_R;
///Field `WUPEN7` reader - Wakeup pin WKUP7 enable
pub use WUPEN1_R as WUPEN7_R;
///Field `WUPEN8` reader - Wakeup pin WKUP8 enable
pub use WUPEN1_R as WUPEN8_R;
///Field `WUPEN2` writer - Wakeup pin WKUP2 enable
pub use WUPEN1_W as WUPEN2_W;
///Field `WUPEN3` writer - Wakeup pin WKUP3 enable
pub use WUPEN1_W as WUPEN3_W;
///Field `WUPEN4` writer - Wakeup pin WKUP4 enable
pub use WUPEN1_W as WUPEN4_W;
///Field `WUPEN5` writer - Wakeup pin WKUP5 enable
pub use WUPEN1_W as WUPEN5_W;
///Field `WUPEN6` writer - Wakeup pin WKUP6 enable
pub use WUPEN1_W as WUPEN6_W;
///Field `WUPEN7` writer - Wakeup pin WKUP7 enable
pub use WUPEN1_W as WUPEN7_W;
///Field `WUPEN8` writer - Wakeup pin WKUP8 enable
pub use WUPEN1_W as WUPEN8_W;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 enable
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 enable
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 enable
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 enable
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 enable
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup pin WKUP6 enable
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup pin WKUP7 enable
    #[inline(always)]
    pub fn wupen7(&self) -> WUPEN7_R {
        WUPEN7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup pin WKUP8 enable
    #[inline(always)]
    pub fn wupen8(&self) -> WUPEN8_R {
        WUPEN8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUCR1")
            .field("wupen1", &self.wupen1())
            .field("wupen2", &self.wupen2())
            .field("wupen3", &self.wupen3())
            .field("wupen4", &self.wupen4())
            .field("wupen5", &self.wupen5())
            .field("wupen6", &self.wupen6())
            .field("wupen7", &self.wupen7())
            .field("wupen8", &self.wupen8())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 enable
    #[inline(always)]
    pub fn wupen1(&mut self) -> WUPEN1_W<WUCR1rs> {
        WUPEN1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 enable
    #[inline(always)]
    pub fn wupen2(&mut self) -> WUPEN2_W<WUCR1rs> {
        WUPEN2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUP3 enable
    #[inline(always)]
    pub fn wupen3(&mut self) -> WUPEN3_W<WUCR1rs> {
        WUPEN3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup pin WKUP4 enable
    #[inline(always)]
    pub fn wupen4(&mut self) -> WUPEN4_W<WUCR1rs> {
        WUPEN4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup pin WKUP5 enable
    #[inline(always)]
    pub fn wupen5(&mut self) -> WUPEN5_W<WUCR1rs> {
        WUPEN5_W::new(self, 4)
    }
    ///Bit 5 - Wakeup pin WKUP6 enable
    #[inline(always)]
    pub fn wupen6(&mut self) -> WUPEN6_W<WUCR1rs> {
        WUPEN6_W::new(self, 5)
    }
    ///Bit 6 - Wakeup pin WKUP7 enable
    #[inline(always)]
    pub fn wupen7(&mut self) -> WUPEN7_W<WUCR1rs> {
        WUPEN7_W::new(self, 6)
    }
    ///Bit 7 - Wakeup pin WKUP8 enable
    #[inline(always)]
    pub fn wupen8(&mut self) -> WUPEN8_W<WUCR1rs> {
        WUPEN8_W::new(self, 7)
    }
}
/**PWR wakeup control register 1

You can [`read`](crate::Reg::read) this register and get [`wucr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:WUCR1)*/
pub struct WUCR1rs;
impl crate::RegisterSpec for WUCR1rs {
    type Ux = u32;
}
///`read()` method returns [`wucr1::R`](R) reader structure
impl crate::Readable for WUCR1rs {}
///`write(|w| ..)` method takes [`wucr1::W`](W) writer structure
impl crate::Writable for WUCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUCR1 to value 0
impl crate::Resettable for WUCR1rs {}
