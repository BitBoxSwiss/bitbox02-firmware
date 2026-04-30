///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGSEL {
    ///0: LDO selected
    Ldo = 0,
    ///1: SMPS selected
    Smps = 1,
}
impl From<REGSEL> for bool {
    #[inline(always)]
    fn from(variant: REGSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `REGSEL` reader - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
pub type REGSEL_R = crate::BitReader<REGSEL>;
impl REGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGSEL {
        match self.bits {
            false => REGSEL::Ldo,
            true => REGSEL::Smps,
        }
    }
    ///LDO selected
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == REGSEL::Ldo
    }
    ///SMPS selected
    #[inline(always)]
    pub fn is_smps(&self) -> bool {
        *self == REGSEL::Smps
    }
}
///Field `REGSEL` writer - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
pub type REGSEL_W<'a, REG> = crate::BitWriter<'a, REG, REGSEL>;
impl<'a, REG> REGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LDO selected
    #[inline(always)]
    pub fn ldo(self) -> &'a mut crate::W<REG> {
        self.variant(REGSEL::Ldo)
    }
    ///SMPS selected
    #[inline(always)]
    pub fn smps(self) -> &'a mut crate::W<REG> {
        self.variant(REGSEL::Smps)
    }
}
/**Fast soft start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTEN {
    ///0: LDO/SMPS fast startup disabled (limited inrush current)
    Disabled = 0,
    ///1: LDO/SMPS fast startup enabled
    Enabled = 1,
}
impl From<FSTEN> for bool {
    #[inline(always)]
    fn from(variant: FSTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FSTEN` reader - Fast soft start
pub type FSTEN_R = crate::BitReader<FSTEN>;
impl FSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSTEN {
        match self.bits {
            false => FSTEN::Disabled,
            true => FSTEN::Enabled,
        }
    }
    ///LDO/SMPS fast startup disabled (limited inrush current)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTEN::Disabled
    }
    ///LDO/SMPS fast startup enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTEN::Enabled
    }
}
///Field `FSTEN` writer - Fast soft start
pub type FSTEN_W<'a, REG> = crate::BitWriter<'a, REG, FSTEN>;
impl<'a, REG> FSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LDO/SMPS fast startup disabled (limited inrush current)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTEN::Disabled)
    }
    ///LDO/SMPS fast startup enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTEN::Enabled)
    }
}
impl R {
    ///Bit 1 - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fast soft start
    #[inline(always)]
    pub fn fsten(&self) -> FSTEN_R {
        FSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("regsel", &self.regsel())
            .field("fsten", &self.fsten())
            .finish()
    }
}
impl W {
    ///Bit 1 - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
    #[inline(always)]
    pub fn regsel(&mut self) -> REGSEL_W<CR3rs> {
        REGSEL_W::new(self, 1)
    }
    ///Bit 2 - Fast soft start
    #[inline(always)]
    pub fn fsten(&mut self) -> FSTEN_W<CR3rs> {
        FSTEN_W::new(self, 2)
    }
}
/**PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
