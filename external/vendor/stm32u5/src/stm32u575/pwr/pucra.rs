///Register `PUCRA` reader
pub type R = crate::R<PUCRArs>;
///Register `PUCRA` writer
pub type W = crate::W<PUCRArs>;
/**Port A pull-up bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0 {
    ///0: Pull-up disabled
    Disabled = 0,
    ///1: Pull-up enabled
    Enabled = 1,
}
impl From<PU0> for bool {
    #[inline(always)]
    fn from(variant: PU0) -> Self {
        variant as u8 != 0
    }
}
///Field `PU0` reader - Port A pull-up bit
pub type PU0_R = crate::BitReader<PU0>;
impl PU0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PU0 {
        match self.bits {
            false => PU0::Disabled,
            true => PU0::Enabled,
        }
    }
    ///Pull-up disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0::Disabled
    }
    ///Pull-up enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0::Enabled
    }
}
///Field `PU0` writer - Port A pull-up bit
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG, PU0>;
impl<'a, REG> PU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pull-up disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Disabled)
    }
    ///Pull-up enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Enabled)
    }
}
///Field `PU1` reader - Port A pull-up bit
pub use PU0_R as PU1_R;
///Field `PU2` reader - Port A pull-up bit
pub use PU0_R as PU2_R;
///Field `PU3` reader - Port A pull-up bit
pub use PU0_R as PU3_R;
///Field `PU4` reader - Port A pull-up bit
pub use PU0_R as PU4_R;
///Field `PU5` reader - Port A pull-up bit
pub use PU0_R as PU5_R;
///Field `PU6` reader - Port A pull-up bit
pub use PU0_R as PU6_R;
///Field `PU7` reader - Port A pull-up bit
pub use PU0_R as PU7_R;
///Field `PU8` reader - Port A pull-up bit
pub use PU0_R as PU8_R;
///Field `PU9` reader - Port A pull-up bit
pub use PU0_R as PU9_R;
///Field `PU10` reader - Port A pull-up bit
pub use PU0_R as PU10_R;
///Field `PU11` reader - Port A pull-up bit
pub use PU0_R as PU11_R;
///Field `PU12` reader - Port A pull-up bit
pub use PU0_R as PU12_R;
///Field `PU13` reader - Port A pull-up bit
pub use PU0_R as PU13_R;
///Field `PU15` reader - Port A pull-up bit 15 When set, this bit activates the pull-up on PA15 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD15 bit is also set.
pub use PU0_R as PU15_R;
///Field `PU1` writer - Port A pull-up bit
pub use PU0_W as PU1_W;
///Field `PU2` writer - Port A pull-up bit
pub use PU0_W as PU2_W;
///Field `PU3` writer - Port A pull-up bit
pub use PU0_W as PU3_W;
///Field `PU4` writer - Port A pull-up bit
pub use PU0_W as PU4_W;
///Field `PU5` writer - Port A pull-up bit
pub use PU0_W as PU5_W;
///Field `PU6` writer - Port A pull-up bit
pub use PU0_W as PU6_W;
///Field `PU7` writer - Port A pull-up bit
pub use PU0_W as PU7_W;
///Field `PU8` writer - Port A pull-up bit
pub use PU0_W as PU8_W;
///Field `PU9` writer - Port A pull-up bit
pub use PU0_W as PU9_W;
///Field `PU10` writer - Port A pull-up bit
pub use PU0_W as PU10_W;
///Field `PU11` writer - Port A pull-up bit
pub use PU0_W as PU11_W;
///Field `PU12` writer - Port A pull-up bit
pub use PU0_W as PU12_W;
///Field `PU13` writer - Port A pull-up bit
pub use PU0_W as PU13_W;
///Field `PU15` writer - Port A pull-up bit 15 When set, this bit activates the pull-up on PA15 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD15 bit is also set.
pub use PU0_W as PU15_W;
impl R {
    ///Bit 0 - Port A pull-up bit
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port A pull-up bit
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port A pull-up bit
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port A pull-up bit
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port A pull-up bit
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port A pull-up bit
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port A pull-up bit
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port A pull-up bit
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port A pull-up bit
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port A pull-up bit
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port A pull-up bit
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port A pull-up bit
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port A pull-up bit
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port A pull-up bit
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Port A pull-up bit 15 When set, this bit activates the pull-up on PA15 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD15 bit is also set.
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRA")
            .field("pu0", &self.pu0())
            .field("pu1", &self.pu1())
            .field("pu2", &self.pu2())
            .field("pu3", &self.pu3())
            .field("pu4", &self.pu4())
            .field("pu5", &self.pu5())
            .field("pu6", &self.pu6())
            .field("pu7", &self.pu7())
            .field("pu8", &self.pu8())
            .field("pu9", &self.pu9())
            .field("pu10", &self.pu10())
            .field("pu11", &self.pu11())
            .field("pu12", &self.pu12())
            .field("pu13", &self.pu13())
            .field("pu15", &self.pu15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port A pull-up bit
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<PUCRArs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - Port A pull-up bit
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<PUCRArs> {
        PU1_W::new(self, 1)
    }
    ///Bit 2 - Port A pull-up bit
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<PUCRArs> {
        PU2_W::new(self, 2)
    }
    ///Bit 3 - Port A pull-up bit
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<PUCRArs> {
        PU3_W::new(self, 3)
    }
    ///Bit 4 - Port A pull-up bit
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<PUCRArs> {
        PU4_W::new(self, 4)
    }
    ///Bit 5 - Port A pull-up bit
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W<PUCRArs> {
        PU5_W::new(self, 5)
    }
    ///Bit 6 - Port A pull-up bit
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<PUCRArs> {
        PU6_W::new(self, 6)
    }
    ///Bit 7 - Port A pull-up bit
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<PUCRArs> {
        PU7_W::new(self, 7)
    }
    ///Bit 8 - Port A pull-up bit
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W<PUCRArs> {
        PU8_W::new(self, 8)
    }
    ///Bit 9 - Port A pull-up bit
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W<PUCRArs> {
        PU9_W::new(self, 9)
    }
    ///Bit 10 - Port A pull-up bit
    #[inline(always)]
    pub fn pu10(&mut self) -> PU10_W<PUCRArs> {
        PU10_W::new(self, 10)
    }
    ///Bit 11 - Port A pull-up bit
    #[inline(always)]
    pub fn pu11(&mut self) -> PU11_W<PUCRArs> {
        PU11_W::new(self, 11)
    }
    ///Bit 12 - Port A pull-up bit
    #[inline(always)]
    pub fn pu12(&mut self) -> PU12_W<PUCRArs> {
        PU12_W::new(self, 12)
    }
    ///Bit 13 - Port A pull-up bit
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W<PUCRArs> {
        PU13_W::new(self, 13)
    }
    ///Bit 15 - Port A pull-up bit 15 When set, this bit activates the pull-up on PA15 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD15 bit is also set.
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W<PUCRArs> {
        PU15_W::new(self, 15)
    }
}
/**PWR port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:PUCRA)*/
pub struct PUCRArs;
impl crate::RegisterSpec for PUCRArs {
    type Ux = u32;
}
///`read()` method returns [`pucra::R`](R) reader structure
impl crate::Readable for PUCRArs {}
///`write(|w| ..)` method takes [`pucra::W`](W) writer structure
impl crate::Writable for PUCRArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRA to value 0
impl crate::Resettable for PUCRArs {}
