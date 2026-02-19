///Register `PDCRB` reader
pub type R = crate::R<PDCRBrs>;
///Register `PDCRB` writer
pub type W = crate::W<PDCRBrs>;
/**

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0 {
    ///0: Pull-down disabled
    Disabled = 0,
    ///1: Pull-down enabled
    Enabled = 1,
}
impl From<PD0> for bool {
    #[inline(always)]
    fn from(variant: PD0) -> Self {
        variant as u8 != 0
    }
}
///Field `PD0` reader -
pub type PD0_R = crate::BitReader<PD0>;
impl PD0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PD0 {
        match self.bits {
            false => PD0::Disabled,
            true => PD0::Enabled,
        }
    }
    ///Pull-down disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD0::Disabled
    }
    ///Pull-down enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD0::Enabled
    }
}
///Field `PD0` writer -
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG, PD0>;
impl<'a, REG> PD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pull-down disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD0::Disabled)
    }
    ///Pull-down enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD0::Enabled)
    }
}
///Field `PD1` reader -
pub use PD0_R as PD1_R;
///Field `PD2` reader -
pub use PD0_R as PD2_R;
///Field `PD3` reader -
pub use PD0_R as PD3_R;
///Field `PD5` reader -
pub use PD0_R as PD5_R;
///Field `PD6` reader -
pub use PD0_R as PD6_R;
///Field `PD7` reader -
pub use PD0_R as PD7_R;
///Field `PD8` reader -
pub use PD0_R as PD8_R;
///Field `PD9` reader -
pub use PD0_R as PD9_R;
///Field `PD10` reader -
pub use PD0_R as PD10_R;
///Field `PD11` reader -
pub use PD0_R as PD11_R;
///Field `PD12` reader -
pub use PD0_R as PD12_R;
///Field `PD13` reader -
pub use PD0_R as PD13_R;
///Field `PD14` reader -
pub use PD0_R as PD14_R;
///Field `PD15` reader -
pub use PD0_R as PD15_R;
///Field `PD1` writer -
pub use PD0_W as PD1_W;
///Field `PD2` writer -
pub use PD0_W as PD2_W;
///Field `PD3` writer -
pub use PD0_W as PD3_W;
///Field `PD5` writer -
pub use PD0_W as PD5_W;
///Field `PD6` writer -
pub use PD0_W as PD6_W;
///Field `PD7` writer -
pub use PD0_W as PD7_W;
///Field `PD8` writer -
pub use PD0_W as PD8_W;
///Field `PD9` writer -
pub use PD0_W as PD9_W;
///Field `PD10` writer -
pub use PD0_W as PD10_W;
///Field `PD11` writer -
pub use PD0_W as PD11_W;
///Field `PD12` writer -
pub use PD0_W as PD12_W;
///Field `PD13` writer -
pub use PD0_W as PD13_W;
///Field `PD14` writer -
pub use PD0_W as PD14_W;
///Field `PD15` writer -
pub use PD0_W as PD15_W;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRB")
            .field("pd0", &self.pd0())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .field("pd3", &self.pd3())
            .field("pd5", &self.pd5())
            .field("pd6", &self.pd6())
            .field("pd7", &self.pd7())
            .field("pd8", &self.pd8())
            .field("pd9", &self.pd9())
            .field("pd10", &self.pd10())
            .field("pd11", &self.pd11())
            .field("pd12", &self.pd12())
            .field("pd13", &self.pd13())
            .field("pd14", &self.pd14())
            .field("pd15", &self.pd15())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<PDCRBrs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<PDCRBrs> {
        PD1_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PDCRBrs> {
        PD2_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<PDCRBrs> {
        PD3_W::new(self, 3)
    }
    ///Bit 5
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<PDCRBrs> {
        PD5_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<PDCRBrs> {
        PD6_W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<PDCRBrs> {
        PD7_W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<PDCRBrs> {
        PD8_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<PDCRBrs> {
        PD9_W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<PDCRBrs> {
        PD10_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W<PDCRBrs> {
        PD11_W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W<PDCRBrs> {
        PD12_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<PDCRBrs> {
        PD13_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<PDCRBrs> {
        PD14_W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<PDCRBrs> {
        PD15_W::new(self, 15)
    }
}
/**PWR port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#PWR:PDCRB)*/
pub struct PDCRBrs;
impl crate::RegisterSpec for PDCRBrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrb::R`](R) reader structure
impl crate::Readable for PDCRBrs {}
///`write(|w| ..)` method takes [`pdcrb::W`](W) writer structure
impl crate::Writable for PDCRBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRB to value 0
impl crate::Resettable for PDCRBrs {}
