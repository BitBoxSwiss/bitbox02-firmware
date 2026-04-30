///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**Computation complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFIE {
    ///0: Interrupt is disabled (masked)
    Disabled = 0,
    ///1: Interrupt is enabled (not masked)
    Enabled = 1,
}
impl From<CCFIE> for bool {
    #[inline(always)]
    fn from(variant: CCFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCFIE` reader - Computation complete flag
pub type CCFIE_R = crate::BitReader<CCFIE>;
impl CCFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCFIE {
        match self.bits {
            false => CCFIE::Disabled,
            true => CCFIE::Enabled,
        }
    }
    ///Interrupt is disabled (masked)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCFIE::Disabled
    }
    ///Interrupt is enabled (not masked)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCFIE::Enabled
    }
}
///Field `CCFIE` writer - Computation complete flag
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG, CCFIE>;
impl<'a, REG> CCFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled (masked)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCFIE::Disabled)
    }
    ///Interrupt is enabled (not masked)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCFIE::Enabled)
    }
}
///Field `RWEIE` reader - Read or write error interrupt flag
pub use CCFIE_R as RWEIE_R;
///Field `KEIE` reader - Key error interrupt flag
pub use CCFIE_R as KEIE_R;
///Field `RNGEIE` reader - Key error interrupt flag
pub use CCFIE_R as RNGEIE_R;
///Field `RWEIE` writer - Read or write error interrupt flag
pub use CCFIE_W as RWEIE_W;
///Field `KEIE` writer - Key error interrupt flag
pub use CCFIE_W as KEIE_W;
///Field `RNGEIE` writer - Key error interrupt flag
pub use CCFIE_W as RNGEIE_W;
impl R {
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt flag
    #[inline(always)]
    pub fn rweie(&self) -> RWEIE_R {
        RWEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt flag
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Key error interrupt flag
    #[inline(always)]
    pub fn rngeie(&self) -> RNGEIE_R {
        RNGEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("ccfie", &self.ccfie())
            .field("rngeie", &self.rngeie())
            .field("keie", &self.keie())
            .field("rweie", &self.rweie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<IERrs> {
        CCFIE_W::new(self, 0)
    }
    ///Bit 1 - Read or write error interrupt flag
    #[inline(always)]
    pub fn rweie(&mut self) -> RWEIE_W<IERrs> {
        RWEIE_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt flag
    #[inline(always)]
    pub fn keie(&mut self) -> KEIE_W<IERrs> {
        KEIE_W::new(self, 2)
    }
    ///Bit 3 - Key error interrupt flag
    #[inline(always)]
    pub fn rngeie(&mut self) -> RNGEIE_W<IERrs> {
        RNGEIE_W::new(self, 3)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#AES:IER)*/
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
