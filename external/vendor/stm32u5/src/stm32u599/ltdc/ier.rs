///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**line interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIE {
    ///0: Line interrupt disabled
    Disabled = 0,
    ///1: Line interrupt enabled
    Enabled = 1,
}
impl From<LIE> for bool {
    #[inline(always)]
    fn from(variant: LIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LIE` reader - line interrupt enable This bit is set and cleared by software.
pub type LIE_R = crate::BitReader<LIE>;
impl LIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LIE {
        match self.bits {
            false => LIE::Disabled,
            true => LIE::Enabled,
        }
    }
    ///Line interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LIE::Disabled
    }
    ///Line interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LIE::Enabled
    }
}
///Field `LIE` writer - line interrupt enable This bit is set and cleared by software.
pub type LIE_W<'a, REG> = crate::BitWriter<'a, REG, LIE>;
impl<'a, REG> LIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Line interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LIE::Disabled)
    }
    ///Line interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LIE::Enabled)
    }
}
/**FIFO underrun interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUIE {
    ///0: FIFO underrun interrupt disabled
    Disabled = 0,
    ///1: FIFO underrun interrupt enabled
    Enabled = 1,
}
impl From<FUIE> for bool {
    #[inline(always)]
    fn from(variant: FUIE) -> Self {
        variant as u8 != 0
    }
}
///Field `FUIE` reader - FIFO underrun interrupt enable This bit is set and cleared by software.
pub type FUIE_R = crate::BitReader<FUIE>;
impl FUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FUIE {
        match self.bits {
            false => FUIE::Disabled,
            true => FUIE::Enabled,
        }
    }
    ///FIFO underrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FUIE::Disabled
    }
    ///FIFO underrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FUIE::Enabled
    }
}
///Field `FUIE` writer - FIFO underrun interrupt enable This bit is set and cleared by software.
pub type FUIE_W<'a, REG> = crate::BitWriter<'a, REG, FUIE>;
impl<'a, REG> FUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO underrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FUIE::Disabled)
    }
    ///FIFO underrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FUIE::Enabled)
    }
}
/**transfer error interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIE {
    ///0: Transfer error interrupt disabled
    Disabled = 0,
    ///1: Transfer error interrupt enabled
    Enabled = 1,
}
impl From<TERRIE> for bool {
    #[inline(always)]
    fn from(variant: TERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TERRIE` reader - transfer error interrupt enable This bit is set and cleared by software.
pub type TERRIE_R = crate::BitReader<TERRIE>;
impl TERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TERRIE {
        match self.bits {
            false => TERRIE::Disabled,
            true => TERRIE::Enabled,
        }
    }
    ///Transfer error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TERRIE::Disabled
    }
    ///Transfer error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TERRIE::Enabled
    }
}
///Field `TERRIE` writer - transfer error interrupt enable This bit is set and cleared by software.
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG, TERRIE>;
impl<'a, REG> TERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE::Disabled)
    }
    ///Transfer error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE::Enabled)
    }
}
/**register reload interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIE {
    ///0: Register reload interrupt disabled
    Disabled = 0,
    ///1: Register reload interrupt enabled
    Enabled = 1,
}
impl From<RRIE> for bool {
    #[inline(always)]
    fn from(variant: RRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RRIE` reader - register reload interrupt enable This bit is set and cleared by software.
pub type RRIE_R = crate::BitReader<RRIE>;
impl RRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRIE {
        match self.bits {
            false => RRIE::Disabled,
            true => RRIE::Enabled,
        }
    }
    ///Register reload interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRIE::Disabled
    }
    ///Register reload interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRIE::Enabled
    }
}
///Field `RRIE` writer - register reload interrupt enable This bit is set and cleared by software.
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG, RRIE>;
impl<'a, REG> RRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register reload interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRIE::Disabled)
    }
    ///Register reload interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRIE::Enabled)
    }
}
impl R {
    ///Bit 0 - line interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - register reload interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("lie", &self.lie())
            .field("fuie", &self.fuie())
            .field("terrie", &self.terrie())
            .field("rrie", &self.rrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - line interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W<IERrs> {
        LIE_W::new(self, 0)
    }
    ///Bit 1 - FIFO underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W<IERrs> {
        FUIE_W::new(self, 1)
    }
    ///Bit 2 - transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W<IERrs> {
        TERRIE_W::new(self, 2)
    }
    ///Bit 3 - register reload interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<IERrs> {
        RRIE_W::new(self, 3)
    }
}
/**LTDC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
