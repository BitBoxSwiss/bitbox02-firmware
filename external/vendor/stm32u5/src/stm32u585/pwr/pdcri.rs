///Register `PDCRI` reader
pub type R = crate::R<PDCRIrs>;
///Register `PDCRI` writer
pub type W = crate::W<PDCRIrs>;
/**Port I pull-down bit

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
///Field `PD0` reader - Port I pull-down bit
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
///Field `PD0` writer - Port I pull-down bit
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
///Field `PD1` reader - Port I pull-down bit
pub use PD0_R as PD1_R;
///Field `PD2` reader - Port I pull-down bit
pub use PD0_R as PD2_R;
///Field `PD3` reader - Port I pull-down bit
pub use PD0_R as PD3_R;
///Field `PD4` reader - Port I pull-down bit
pub use PD0_R as PD4_R;
///Field `PD5` reader - Port I pull-down bit
pub use PD0_R as PD5_R;
///Field `PD6` reader - Port I pull-down bit
pub use PD0_R as PD6_R;
///Field `PD7` reader - Port I pull-down bit
pub use PD0_R as PD7_R;
///Field `PD1` writer - Port I pull-down bit
pub use PD0_W as PD1_W;
///Field `PD2` writer - Port I pull-down bit
pub use PD0_W as PD2_W;
///Field `PD3` writer - Port I pull-down bit
pub use PD0_W as PD3_W;
///Field `PD4` writer - Port I pull-down bit
pub use PD0_W as PD4_W;
///Field `PD5` writer - Port I pull-down bit
pub use PD0_W as PD5_W;
///Field `PD6` writer - Port I pull-down bit
pub use PD0_W as PD6_W;
///Field `PD7` writer - Port I pull-down bit
pub use PD0_W as PD7_W;
impl R {
    ///Bit 0 - Port I pull-down bit
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port I pull-down bit
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port I pull-down bit
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port I pull-down bit
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port I pull-down bit
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port I pull-down bit
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port I pull-down bit
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port I pull-down bit
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRI")
            .field("pd0", &self.pd0())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .field("pd3", &self.pd3())
            .field("pd4", &self.pd4())
            .field("pd5", &self.pd5())
            .field("pd6", &self.pd6())
            .field("pd7", &self.pd7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port I pull-down bit
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<PDCRIrs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1 - Port I pull-down bit
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<PDCRIrs> {
        PD1_W::new(self, 1)
    }
    ///Bit 2 - Port I pull-down bit
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PDCRIrs> {
        PD2_W::new(self, 2)
    }
    ///Bit 3 - Port I pull-down bit
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<PDCRIrs> {
        PD3_W::new(self, 3)
    }
    ///Bit 4 - Port I pull-down bit
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<PDCRIrs> {
        PD4_W::new(self, 4)
    }
    ///Bit 5 - Port I pull-down bit
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<PDCRIrs> {
        PD5_W::new(self, 5)
    }
    ///Bit 6 - Port I pull-down bit
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<PDCRIrs> {
        PD6_W::new(self, 6)
    }
    ///Bit 7 - Port I pull-down bit
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<PDCRIrs> {
        PD7_W::new(self, 7)
    }
}
/**PWR port I pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:PDCRI)*/
pub struct PDCRIrs;
impl crate::RegisterSpec for PDCRIrs {
    type Ux = u32;
}
///`read()` method returns [`pdcri::R`](R) reader structure
impl crate::Readable for PDCRIrs {}
///`write(|w| ..)` method takes [`pdcri::W`](W) writer structure
impl crate::Writable for PDCRIrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRI to value 0
impl crate::Resettable for PDCRIrs {}
