///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
/**Update interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    ///0: Update interrupt disabled
    Disabled = 0,
    ///1: Update interrupt enabled
    Enabled = 1,
}
impl From<UIE> for bool {
    #[inline(always)]
    fn from(variant: UIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader<UIE>;
impl UIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIE {
        match self.bits {
            false => UIE::Disabled,
            true => UIE::Enabled,
        }
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UIE::Disabled
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UIE::Enabled
    }
}
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Disabled)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Enabled)
    }
}
/**Capture/Compare %s interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE {
    ///0: CCx interrupt disabled
    Disabled = 0,
    ///1: CCx interrupt enabled
    Enabled = 1,
}
impl From<CC1IE> for bool {
    #[inline(always)]
    fn from(variant: CC1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCIE(1-1)` reader - Capture/Compare %s interrupt enable
pub type CCIE_R = crate::BitReader<CC1IE>;
impl CCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IE {
        match self.bits {
            false => CC1IE::Disabled,
            true => CC1IE::Enabled,
        }
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1IE::Disabled
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1IE::Enabled
    }
}
///Field `CCIE(1-1)` writer - Capture/Compare %s interrupt enable
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE>;
impl<'a, REG> CCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::Disabled)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::Enabled)
    }
}
/**COM interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIE {
    ///0: COM interrupt disabled
    Disabled = 0,
    ///1: COM interrupt enabled
    Enabled = 1,
}
impl From<COMIE> for bool {
    #[inline(always)]
    fn from(variant: COMIE) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIE` reader - COM interrupt enable
pub type COMIE_R = crate::BitReader<COMIE>;
impl COMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMIE {
        match self.bits {
            false => COMIE::Disabled,
            true => COMIE::Enabled,
        }
    }
    ///COM interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMIE::Disabled
    }
    ///COM interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMIE::Enabled
    }
}
///Field `COMIE` writer - COM interrupt enable
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG, COMIE>;
impl<'a, REG> COMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///COM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMIE::Disabled)
    }
    ///COM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMIE::Enabled)
    }
}
/**Break interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIE {
    ///0: Break interrupt disabled
    Disabled = 0,
    ///1: Break interrupt enabled
    Enabled = 1,
}
impl From<BIE> for bool {
    #[inline(always)]
    fn from(variant: BIE) -> Self {
        variant as u8 != 0
    }
}
///Field `BIE` reader - Break interrupt enable
pub type BIE_R = crate::BitReader<BIE>;
impl BIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIE {
        match self.bits {
            false => BIE::Disabled,
            true => BIE::Enabled,
        }
    }
    ///Break interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BIE::Disabled
    }
    ///Break interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BIE::Enabled
    }
}
///Field `BIE` writer - Break interrupt enable
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG, BIE>;
impl<'a, REG> BIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIE::Disabled)
    }
    ///Break interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIE::Enabled)
    }
}
/**Update DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE {
    ///0: Update DMA request disabled
    Disabled = 0,
    ///1: Update DMA request enabled
    Enabled = 1,
}
impl From<UDE> for bool {
    #[inline(always)]
    fn from(variant: UDE) -> Self {
        variant as u8 != 0
    }
}
///Field `UDE` reader - Update DMA request enable
pub type UDE_R = crate::BitReader<UDE>;
impl UDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDE {
        match self.bits {
            false => UDE::Disabled,
            true => UDE::Enabled,
        }
    }
    ///Update DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDE::Disabled
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDE::Enabled
    }
}
///Field `UDE` writer - Update DMA request enable
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Disabled)
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Enabled)
    }
}
/**Capture/Compare %s DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1DE {
    ///0: CCx DMA request disabled
    Disabled = 0,
    ///1: CCx DMA request enabled
    Enabled = 1,
}
impl From<CC1DE> for bool {
    #[inline(always)]
    fn from(variant: CC1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCDE(1-1)` reader - Capture/Compare %s DMA request enable
pub type CCDE_R = crate::BitReader<CC1DE>;
impl CCDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1DE {
        match self.bits {
            false => CC1DE::Disabled,
            true => CC1DE::Enabled,
        }
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1DE::Disabled
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1DE::Enabled
    }
}
///Field `CCDE(1-1)` writer - Capture/Compare %s DMA request enable
pub type CCDE_W<'a, REG> = crate::BitWriter<'a, REG, CC1DE>;
impl<'a, REG> CCDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::Disabled)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::Enabled)
    }
}
///Field `COMDE` reader - COM DMA request enable
pub type COMDE_R = crate::BitReader;
///Field `COMDE` writer - COM DMA request enable
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Capture/Compare (1-1) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IE` field.</div>
    #[inline(always)]
    pub fn ccie(&self, n: u8) -> CCIE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIE_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) interrupt enable
    #[inline(always)]
    pub fn ccie_iter(&self) -> impl Iterator<Item = CCIE_R> + '_ {
        (0..1).map(move |n| CCIE_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Capture/Compare (1-1) DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1DE` field.</div>
    #[inline(always)]
    pub fn ccde(&self, n: u8) -> CCDE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCDE_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) DMA request enable
    #[inline(always)]
    pub fn ccde_iter(&self) -> impl Iterator<Item = CCDE_R> + '_ {
        (0..1).map(move |n| CCDE_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("comde", &self.comde())
            .field("cc1de", &self.cc1de())
            .field("ude", &self.ude())
            .field("bie", &self.bie())
            .field("comie", &self.comie())
            .field("cc1ie", &self.cc1ie())
            .field("uie", &self.uie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Capture/Compare (1-1) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IE` field.</div>
    #[inline(always)]
    pub fn ccie(&mut self, n: u8) -> CCIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIE_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 1)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<DIERrs> {
        BIE_W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Capture/Compare (1-1) DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1DE` field.</div>
    #[inline(always)]
    pub fn ccde(&mut self, n: u8) -> CCDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCDE_W::new(self, n * 0 + 9)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 9)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W<DIERrs> {
        COMDE_W::new(self, 13)
    }
}
/**DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TIM16:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
