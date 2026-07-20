///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**End of acquisition interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOAIE {
    ///0: End of acquisition interrupt disabled
    Disabled = 0,
    ///1: End of acquisition interrupt enabled
    Enabled = 1,
}
impl From<EOAIE> for bool {
    #[inline(always)]
    fn from(variant: EOAIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOAIE` reader - End of acquisition interrupt enable
pub type EOAIE_R = crate::BitReader<EOAIE>;
impl EOAIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOAIE {
        match self.bits {
            false => EOAIE::Disabled,
            true => EOAIE::Enabled,
        }
    }
    ///End of acquisition interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOAIE::Disabled
    }
    ///End of acquisition interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOAIE::Enabled
    }
}
///Field `EOAIE` writer - End of acquisition interrupt enable
pub type EOAIE_W<'a, REG> = crate::BitWriter<'a, REG, EOAIE>;
impl<'a, REG> EOAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of acquisition interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOAIE::Disabled)
    }
    ///End of acquisition interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOAIE::Enabled)
    }
}
/**Max count error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCEIE {
    ///0: Max count error interrupt disabled
    Disabled = 0,
    ///1: Max count error interrupt enabled
    Enabled = 1,
}
impl From<MCEIE> for bool {
    #[inline(always)]
    fn from(variant: MCEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `MCEIE` reader - Max count error interrupt enable
pub type MCEIE_R = crate::BitReader<MCEIE>;
impl MCEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCEIE {
        match self.bits {
            false => MCEIE::Disabled,
            true => MCEIE::Enabled,
        }
    }
    ///Max count error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEIE::Disabled
    }
    ///Max count error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEIE::Enabled
    }
}
///Field `MCEIE` writer - Max count error interrupt enable
pub type MCEIE_W<'a, REG> = crate::BitWriter<'a, REG, MCEIE>;
impl<'a, REG> MCEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Max count error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEIE::Disabled)
    }
    ///Max count error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEIE::Enabled)
    }
}
impl R {
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("mceie", &self.mceie())
            .field("eoaie", &self.eoaie())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&mut self) -> EOAIE_W<IERrs> {
        EOAIE_W::new(self, 0)
    }
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&mut self) -> MCEIE_W<IERrs> {
        MCEIE_W::new(self, 1)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TSC:IER)*/
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
