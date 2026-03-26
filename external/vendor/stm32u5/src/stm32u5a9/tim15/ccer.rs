///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
/**Capture/Compare %s output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E {
    ///0: Capture disabled
    Disabled = 0,
    ///1: Capture enabled
    Enabled = 1,
}
impl From<CC1E> for bool {
    #[inline(always)]
    fn from(variant: CC1E) -> Self {
        variant as u8 != 0
    }
}
///Field `CCE(1-2)` reader - Capture/Compare %s output enable
pub type CCE_R = crate::BitReader<CC1E>;
impl CCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1E {
        match self.bits {
            false => CC1E::Disabled,
            true => CC1E::Enabled,
        }
    }
    ///Capture disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1E::Disabled
    }
    ///Capture enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1E::Enabled
    }
}
///Field `CCE(1-2)` writer - Capture/Compare %s output enable
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG, CC1E>;
impl<'a, REG> CCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::Disabled)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::Enabled)
    }
}
/**Capture/Compare %s output Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P {
    ///0: Noninverted/rising edge
    RisingEdge = 0,
    ///1: Inverted/falling edge
    FallingEdge = 1,
}
impl From<CC1P> for bool {
    #[inline(always)]
    fn from(variant: CC1P) -> Self {
        variant as u8 != 0
    }
}
///Field `CCP(1-2)` reader - Capture/Compare %s output Polarity
pub type CCP_R = crate::BitReader<CC1P>;
impl CCP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1P {
        match self.bits {
            false => CC1P::RisingEdge,
            true => CC1P::FallingEdge,
        }
    }
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CC1P::RisingEdge
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CC1P::FallingEdge
    }
}
///Field `CCP(1-2)` writer - Capture/Compare %s output Polarity
pub type CCP_W<'a, REG> = crate::BitWriter<'a, REG, CC1P>;
impl<'a, REG> CCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::RisingEdge)
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::FallingEdge)
    }
}
/**Capture/Compare %s complementary output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE {
    ///0: Complementary output disabled
    Disabled = 0,
    ///1: Complementary output enabled
    Enabled = 1,
}
impl From<CC1NE> for bool {
    #[inline(always)]
    fn from(variant: CC1NE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCNE(1-1)` reader - Capture/Compare %s complementary output enable
pub type CCNE_R = crate::BitReader<CC1NE>;
impl CCNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NE {
        match self.bits {
            false => CC1NE::Disabled,
            true => CC1NE::Enabled,
        }
    }
    ///Complementary output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1NE::Disabled
    }
    ///Complementary output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1NE::Enabled
    }
}
///Field `CCNE(1-1)` writer - Capture/Compare %s complementary output enable
pub type CCNE_W<'a, REG> = crate::BitWriter<'a, REG, CC1NE>;
impl<'a, REG> CCNE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Complementary output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::Disabled)
    }
    ///Complementary output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::Enabled)
    }
}
/**Capture/Compare %s output Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP {
    ///0: OCxN active high
    ActiveHigh = 0,
    ///1: OCxN active low
    ActiveLow = 1,
}
impl From<CC1NP> for bool {
    #[inline(always)]
    fn from(variant: CC1NP) -> Self {
        variant as u8 != 0
    }
}
///Field `CCNP(1-2)` reader - Capture/Compare %s output Polarity
pub type CCNP_R = crate::BitReader<CC1NP>;
impl CCNP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NP {
        match self.bits {
            false => CC1NP::ActiveHigh,
            true => CC1NP::ActiveLow,
        }
    }
    ///OCxN active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CC1NP::ActiveHigh
    }
    ///OCxN active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CC1NP::ActiveLow
    }
}
///Field `CCNP(1-2)` writer - Capture/Compare %s output Polarity
pub type CCNP_W<'a, REG> = crate::BitWriter<'a, REG, CC1NP>;
impl<'a, REG> CCNP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OCxN active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::ActiveHigh)
    }
    ///OCxN active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::ActiveLow)
    }
}
impl R {
    ///Capture/Compare (1-2) output enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1E` field.</div>
    #[inline(always)]
    pub fn cce(&self, n: u8) -> CCE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCE_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-2) output enable
    #[inline(always)]
    pub fn cce_iter(&self) -> impl Iterator<Item = CCE_R> + '_ {
        (0..2).map(move |n| CCE_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CCE_R {
        CCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Capture/Compare (1-2) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1P` field.</div>
    #[inline(always)]
    pub fn ccp(&self, n: u8) -> CCP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCP_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-2) output Polarity
    #[inline(always)]
    pub fn ccp_iter(&self) -> impl Iterator<Item = CCP_R> + '_ {
        (0..2).map(move |n| CCP_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Capture/Compare (1-1) complementary output enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1NE` field.</div>
    #[inline(always)]
    pub fn ccne(&self, n: u8) -> CCNE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCNE_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) complementary output enable
    #[inline(always)]
    pub fn ccne_iter(&self) -> impl Iterator<Item = CCNE_R> + '_ {
        (0..1).map(move |n| CCNE_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0))
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CCNE_R {
        CCNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Capture/Compare (1-2) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1NP` field.</div>
    #[inline(always)]
    pub fn ccnp(&self, n: u8) -> CCNP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCNP_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-2) output Polarity
    #[inline(always)]
    pub fn ccnp_iter(&self) -> impl Iterator<Item = CCNP_R> + '_ {
        (0..2).map(move |n| CCNP_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc1np", &self.cc1np())
            .field("cc2np", &self.cc2np())
            .field("cc1p", &self.cc1p())
            .field("cc2p", &self.cc2p())
            .field("cc1e", &self.cc1e())
            .field("cc2e", &self.cc2e())
            .field("cc1ne", &self.cc1ne())
            .finish()
    }
}
impl W {
    ///Capture/Compare (1-2) output enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1E` field.</div>
    #[inline(always)]
    pub fn cce(&mut self, n: u8) -> CCE_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCE_W::new(self, n * 4)
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 4)
    }
    ///Capture/Compare (1-2) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1P` field.</div>
    #[inline(always)]
    pub fn ccp(&mut self, n: u8) -> CCP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCP_W::new(self, n * 4 + 1)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 1)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 5)
    }
    ///Capture/Compare (1-1) complementary output enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1NE` field.</div>
    #[inline(always)]
    pub fn ccne(&mut self, n: u8) -> CCNE_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCNE_W::new(self, n * 0 + 2)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CCNE_W<CCERrs> {
        CCNE_W::new(self, 2)
    }
    ///Capture/Compare (1-2) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1NP` field.</div>
    #[inline(always)]
    pub fn ccnp(&mut self, n: u8) -> CCNP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCNP_W::new(self, n * 4 + 3)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 3)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 7)
    }
}
/**capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TIM15:CCER)*/
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u32;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {}
