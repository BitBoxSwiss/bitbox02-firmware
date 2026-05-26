///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**ADRDYIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE {
    ///0: ADRDY interrupt disabled
    Disabled = 0,
    ///1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    Enabled = 1,
}
impl From<ADRDYIE> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDYIE` reader - ADRDYIE
pub type ADRDYIE_R = crate::BitReader<ADRDYIE>;
impl ADRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYIE {
        match self.bits {
            false => ADRDYIE::Disabled,
            true => ADRDYIE::Enabled,
        }
    }
    ///ADRDY interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRDYIE::Disabled
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRDYIE::Enabled
    }
}
///Field `ADRDYIE` writer - ADRDYIE
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYIE>;
impl<'a, REG> ADRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADRDY interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Disabled)
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Enabled)
    }
}
/**EOSMPIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE {
    ///0: EOSMP interrupt disabled
    Disabled = 0,
    ///1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    Enabled = 1,
}
impl From<EOSMPIE> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMPIE` reader - EOSMPIE
pub type EOSMPIE_R = crate::BitReader<EOSMPIE>;
impl EOSMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPIE {
        match self.bits {
            false => EOSMPIE::Disabled,
            true => EOSMPIE::Enabled,
        }
    }
    ///EOSMP interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE::Disabled
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE::Enabled
    }
}
///Field `EOSMPIE` writer - EOSMPIE
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOSMP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Disabled)
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Enabled)
    }
}
/**EOCIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    ///0: EOC interrupt disabled
    Disabled = 0,
    ///1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    Enabled = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCIE` reader - EOCIE
pub type EOCIE_R = crate::BitReader<EOCIE>;
impl EOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCIE {
        match self.bits {
            false => EOCIE::Disabled,
            true => EOCIE::Enabled,
        }
    }
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE::Enabled
    }
}
///Field `EOCIE` writer - EOCIE
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
/**EOSIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE {
    ///0: EOS interrupt disabled
    Disabled = 0,
    ///1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    Enabled = 1,
}
impl From<EOSIE> for bool {
    #[inline(always)]
    fn from(variant: EOSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSIE` reader - EOSIE
pub type EOSIE_R = crate::BitReader<EOSIE>;
impl EOSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSIE {
        match self.bits {
            false => EOSIE::Disabled,
            true => EOSIE::Enabled,
        }
    }
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSIE::Disabled
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSIE::Enabled
    }
}
///Field `EOSIE` writer - EOSIE
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSIE>;
impl<'a, REG> EOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Disabled)
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Enabled)
    }
}
/**OVRIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    ///0: Overrun interrupt disabled
    Disabled = 0,
    ///1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    Enabled = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRIE` reader - OVRIE
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::Disabled,
            true => OVRIE::Enabled,
        }
    }
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE::Disabled
    }
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE::Enabled
    }
}
///Field `OVRIE` writer - OVRIE
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Disabled)
    }
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Enabled)
    }
}
/**Analog watchdog %s interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1IE {
    ///0: Analog watchdog x interrupt disabled
    Disabled = 0,
    ///1: Analog watchdog x interrupt enabled
    Enabled = 1,
}
impl From<AWD1IE> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDIE(1-3)` reader - Analog watchdog %s interrupt enable
pub type AWDIE_R = crate::BitReader<AWD1IE>;
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1IE {
        match self.bits {
            false => AWD1IE::Disabled,
            true => AWD1IE::Enabled,
        }
    }
    ///Analog watchdog x interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1IE::Disabled
    }
    ///Analog watchdog x interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1IE::Enabled
    }
}
///Field `AWDIE(1-3)` writer - Analog watchdog %s interrupt enable
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWD1IE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog x interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::Disabled)
    }
    ///Analog watchdog x interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::Enabled)
    }
}
/**EOCALIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALIE {
    ///0: End of calibration interrupt disabled
    Disabled = 0,
    ///1: End of calibration interrupt enabled
    Enabled = 1,
}
impl From<EOCALIE> for bool {
    #[inline(always)]
    fn from(variant: EOCALIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCALIE` reader - EOCALIE
pub type EOCALIE_R = crate::BitReader<EOCALIE>;
impl EOCALIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCALIE {
        match self.bits {
            false => EOCALIE::Disabled,
            true => EOCALIE::Enabled,
        }
    }
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCALIE::Disabled
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCALIE::Enabled
    }
}
///Field `EOCALIE` writer - EOCALIE
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCALIE>;
impl<'a, REG> EOCALIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALIE::Disabled)
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALIE::Enabled)
    }
}
/**LDORDYIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDORDYIE {
    ///0: LDO ready interrupt disabled
    Disabled = 0,
    ///1: LDO ready interrupt enabled. An interrupt is generated when the LDO output is ready.
    Enabled = 1,
}
impl From<LDORDYIE> for bool {
    #[inline(always)]
    fn from(variant: LDORDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LDORDYIE` reader - LDORDYIE
pub type LDORDYIE_R = crate::BitReader<LDORDYIE>;
impl LDORDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDORDYIE {
        match self.bits {
            false => LDORDYIE::Disabled,
            true => LDORDYIE::Enabled,
        }
    }
    ///LDO ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LDORDYIE::Disabled
    }
    ///LDO ready interrupt enabled. An interrupt is generated when the LDO output is ready.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LDORDYIE::Enabled
    }
}
///Field `LDORDYIE` writer - LDORDYIE
pub type LDORDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LDORDYIE>;
impl<'a, REG> LDORDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LDO ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LDORDYIE::Disabled)
    }
    ///LDO ready interrupt enabled. An interrupt is generated when the LDO output is ready.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LDORDYIE::Enabled)
    }
}
impl R {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Analog watchdog (1-3) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1IE` field.</div>
    #[inline(always)]
    pub fn awdie(&self, n: u8) -> AWDIE_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWDIE_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog (1-3) interrupt enable
    #[inline(always)]
    pub fn awdie_iter(&self) -> impl Iterator<Item = AWDIE_R> + '_ {
        (0..3).map(move |n| AWDIE_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable
    #[inline(always)]
    pub fn awd1ie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable
    #[inline(always)]
    pub fn awd2ie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable
    #[inline(always)]
    pub fn awd3ie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - EOCALIE
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LDORDYIE
    #[inline(always)]
    pub fn ldordyie(&self) -> LDORDYIE_R {
        LDORDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("ldordyie", &self.ldordyie())
            .field("eocalie", &self.eocalie())
            .field("awd1ie", &self.awd1ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd3ie", &self.awd3ie())
            .field("ovrie", &self.ovrie())
            .field("eosie", &self.eosie())
            .field("eocie", &self.eocie())
            .field("eosmpie", &self.eosmpie())
            .field("adrdyie", &self.adrdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Analog watchdog (1-3) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1IE` field.</div>
    #[inline(always)]
    pub fn awdie(&mut self, n: u8) -> AWDIE_W<IERrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWDIE_W::new(self, n + 7)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 9)
    }
    ///Bit 11 - EOCALIE
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W<IERrs> {
        EOCALIE_W::new(self, 11)
    }
    ///Bit 12 - LDORDYIE
    #[inline(always)]
    pub fn ldordyie(&mut self) -> LDORDYIE_W<IERrs> {
        LDORDYIE_W::new(self, 12)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC4:IER)*/
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
