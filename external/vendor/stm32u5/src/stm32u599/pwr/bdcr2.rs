///Register `BDCR2` reader
pub type R = crate::R<BDCR2rs>;
///Register `BDCR2` writer
pub type W = crate::W<BDCR2rs>;
/**VBAT charging enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    ///0: VBAT battery charging disabled
    Disabled = 0,
    ///1: VBAT battery charging enabled
    Enabled = 1,
}
impl From<VBE> for bool {
    #[inline(always)]
    fn from(variant: VBE) -> Self {
        variant as u8 != 0
    }
}
///Field `VBE` reader - VBAT charging enable
pub type VBE_R = crate::BitReader<VBE>;
impl VBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBE {
        match self.bits {
            false => VBE::Disabled,
            true => VBE::Enabled,
        }
    }
    ///VBAT battery charging disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    ///VBAT battery charging enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE::Enabled
    }
}
///Field `VBE` writer - VBAT charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBAT battery charging disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    ///VBAT battery charging enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
/**VBAT charging resistor selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    ///0: Charge VBAT through a 5 kOhm resistor
    R5k = 0,
    ///1: Charge VBAT through a 1.5 kOhm resistor
    R1k5 = 1,
}
impl From<VBRS> for bool {
    #[inline(always)]
    fn from(variant: VBRS) -> Self {
        variant as u8 != 0
    }
}
///Field `VBRS` reader - VBAT charging resistor selection
pub type VBRS_R = crate::BitReader<VBRS>;
impl VBRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBRS {
        match self.bits {
            false => VBRS::R5k,
            true => VBRS::R1k5,
        }
    }
    ///Charge VBAT through a 5 kOhm resistor
    #[inline(always)]
    pub fn is_r_5k(&self) -> bool {
        *self == VBRS::R5k
    }
    ///Charge VBAT through a 1.5 kOhm resistor
    #[inline(always)]
    pub fn is_r_1k5(&self) -> bool {
        *self == VBRS::R1k5
    }
}
///Field `VBRS` writer - VBAT charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Charge VBAT through a 5 kOhm resistor
    #[inline(always)]
    pub fn r_5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R5k)
    }
    ///Charge VBAT through a 1.5 kOhm resistor
    #[inline(always)]
    pub fn r_1k5(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R1k5)
    }
}
impl R {
    ///Bit 0 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR2")
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .finish()
    }
}
impl W {
    ///Bit 0 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<BDCR2rs> {
        VBE_W::new(self, 0)
    }
    ///Bit 1 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<BDCR2rs> {
        VBRS_W::new(self, 1)
    }
}
/**PWR Backup domain control register 2

You can [`read`](crate::Reg::read) this register and get [`bdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#PWR:BDCR2)*/
pub struct BDCR2rs;
impl crate::RegisterSpec for BDCR2rs {
    type Ux = u32;
}
///`read()` method returns [`bdcr2::R`](R) reader structure
impl crate::Readable for BDCR2rs {}
///`write(|w| ..)` method takes [`bdcr2::W`](W) writer structure
impl crate::Writable for BDCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR2 to value 0
impl crate::Resettable for BDCR2rs {}
