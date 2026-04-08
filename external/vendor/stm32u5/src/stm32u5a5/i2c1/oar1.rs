///Register `OAR1` reader
pub type R = crate::R<OAR1rs>;
///Register `OAR1` writer
pub type W = crate::W<OAR1rs>;
///Field `OA1` reader - Interface address
pub type OA1_R = crate::FieldReader<u16>;
///Field `OA1` writer - Interface address
pub type OA1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
/**Own Address 1 10-bit mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA1MODE {
    ///0: Own address 1 is a 7-bit address
    Bit7 = 0,
    ///1: Own address 1 is a 10-bit address
    Bit10 = 1,
}
impl From<OA1MODE> for bool {
    #[inline(always)]
    fn from(variant: OA1MODE) -> Self {
        variant as u8 != 0
    }
}
///Field `OA1MODE` reader - Own Address 1 10-bit mode
pub type OA1MODE_R = crate::BitReader<OA1MODE>;
impl OA1MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OA1MODE {
        match self.bits {
            false => OA1MODE::Bit7,
            true => OA1MODE::Bit10,
        }
    }
    ///Own address 1 is a 7-bit address
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == OA1MODE::Bit7
    }
    ///Own address 1 is a 10-bit address
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == OA1MODE::Bit10
    }
}
///Field `OA1MODE` writer - Own Address 1 10-bit mode
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG, OA1MODE>;
impl<'a, REG> OA1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Own address 1 is a 7-bit address
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(OA1MODE::Bit7)
    }
    ///Own address 1 is a 10-bit address
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(OA1MODE::Bit10)
    }
}
/**Own Address 1 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA1EN {
    ///0: Own address 1 disabled. The received slave address OA1 is NACKed
    Disabled = 0,
    ///1: Own address 1 enabled. The received slave address OA1 is ACKed
    Enabled = 1,
}
impl From<OA1EN> for bool {
    #[inline(always)]
    fn from(variant: OA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OA1EN` reader - Own Address 1 enable
pub type OA1EN_R = crate::BitReader<OA1EN>;
impl OA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OA1EN {
        match self.bits {
            false => OA1EN::Disabled,
            true => OA1EN::Enabled,
        }
    }
    ///Own address 1 disabled. The received slave address OA1 is NACKed
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA1EN::Disabled
    }
    ///Own address 1 enabled. The received slave address OA1 is ACKed
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA1EN::Enabled
    }
}
///Field `OA1EN` writer - Own Address 1 enable
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG, OA1EN>;
impl<'a, REG> OA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Own address 1 disabled. The received slave address OA1 is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OA1EN::Disabled)
    }
    ///Own address 1 enabled. The received slave address OA1 is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OA1EN::Enabled)
    }
}
impl R {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Own Address 1 10-bit mode
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Own Address 1 enable
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR1")
            .field("oa1", &self.oa1())
            .field("oa1mode", &self.oa1mode())
            .field("oa1en", &self.oa1en())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W<OAR1rs> {
        OA1_W::new(self, 0)
    }
    ///Bit 10 - Own Address 1 10-bit mode
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W<OAR1rs> {
        OA1MODE_W::new(self, 10)
    }
    ///Bit 15 - Own Address 1 enable
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W<OAR1rs> {
        OA1EN_W::new(self, 15)
    }
}
/**Own address register 1

You can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#I2C1:OAR1)*/
pub struct OAR1rs;
impl crate::RegisterSpec for OAR1rs {
    type Ux = u32;
}
///`read()` method returns [`oar1::R`](R) reader structure
impl crate::Readable for OAR1rs {}
///`write(|w| ..)` method takes [`oar1::W`](W) writer structure
impl crate::Writable for OAR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OAR1 to value 0
impl crate::Resettable for OAR1rs {}
