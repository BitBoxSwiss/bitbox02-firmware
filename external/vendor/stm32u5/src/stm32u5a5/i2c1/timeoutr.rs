///Register `TIMEOUTR` reader
pub type R = crate::R<TIMEOUTRrs>;
///Register `TIMEOUTR` writer
pub type W = crate::W<TIMEOUTRrs>;
///Field `TIMEOUTA` reader - Bus timeout A
pub type TIMEOUTA_R = crate::FieldReader<u16>;
///Field `TIMEOUTA` writer - Bus timeout A
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
/**Idle clock timeout detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIDLE {
    ///0: TIMEOUTA is used to detect SCL low timeout
    Disabled = 0,
    ///1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
    Enabled = 1,
}
impl From<TIDLE> for bool {
    #[inline(always)]
    fn from(variant: TIDLE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIDLE` reader - Idle clock timeout detection
pub type TIDLE_R = crate::BitReader<TIDLE>;
impl TIDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIDLE {
        match self.bits {
            false => TIDLE::Disabled,
            true => TIDLE::Enabled,
        }
    }
    ///TIMEOUTA is used to detect SCL low timeout
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIDLE::Disabled
    }
    ///TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIDLE::Enabled
    }
}
///Field `TIDLE` writer - Idle clock timeout detection
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG, TIDLE>;
impl<'a, REG> TIDLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIMEOUTA is used to detect SCL low timeout
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIDLE::Disabled)
    }
    ///TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIDLE::Enabled)
    }
}
/**Clock timeout enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTEN {
    ///0: SCL timeout detection is disabled
    Disabled = 0,
    ///1: SCL timeout detection is enabled
    Enabled = 1,
}
impl From<TIMOUTEN> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMOUTEN` reader - Clock timeout enable
pub type TIMOUTEN_R = crate::BitReader<TIMOUTEN>;
impl TIMOUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMOUTEN {
        match self.bits {
            false => TIMOUTEN::Disabled,
            true => TIMOUTEN::Enabled,
        }
    }
    ///SCL timeout detection is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUTEN::Disabled
    }
    ///SCL timeout detection is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUTEN::Enabled
    }
}
///Field `TIMOUTEN` writer - Clock timeout enable
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUTEN>;
impl<'a, REG> TIMOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SCL timeout detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTEN::Disabled)
    }
    ///SCL timeout detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTEN::Enabled)
    }
}
///Field `TIMEOUTB` reader - Bus timeout B
pub type TIMEOUTB_R = crate::FieldReader<u16>;
///Field `TIMEOUTB` writer - Bus timeout B
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
/**Extended clock timeout enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTEN {
    ///0: Extended clock timeout detection is disabled
    Disabled = 0,
    ///1: Extended clock timeout detection is enabled
    Enabled = 1,
}
impl From<TEXTEN> for bool {
    #[inline(always)]
    fn from(variant: TEXTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXTEN` reader - Extended clock timeout enable
pub type TEXTEN_R = crate::BitReader<TEXTEN>;
impl TEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEXTEN {
        match self.bits {
            false => TEXTEN::Disabled,
            true => TEXTEN::Enabled,
        }
    }
    ///Extended clock timeout detection is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEXTEN::Disabled
    }
    ///Extended clock timeout detection is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEXTEN::Enabled
    }
}
///Field `TEXTEN` writer - Extended clock timeout enable
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG, TEXTEN>;
impl<'a, REG> TEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Extended clock timeout detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTEN::Disabled)
    }
    ///Extended clock timeout detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTEN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUTR")
            .field("timeouta", &self.timeouta())
            .field("tidle", &self.tidle())
            .field("timouten", &self.timouten())
            .field("timeoutb", &self.timeoutb())
            .field("texten", &self.texten())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<TIMEOUTRrs> {
        TIMEOUTA_W::new(self, 0)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W<TIMEOUTRrs> {
        TIDLE_W::new(self, 12)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&mut self) -> TIMOUTEN_W<TIMEOUTRrs> {
        TIMOUTEN_W::new(self, 15)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<TIMEOUTRrs> {
        TIMEOUTB_W::new(self, 16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&mut self) -> TEXTEN_W<TIMEOUTRrs> {
        TEXTEN_W::new(self, 31)
    }
}
/**Status register 1

You can [`read`](crate::Reg::read) this register and get [`timeoutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#I2C1:TIMEOUTR)*/
pub struct TIMEOUTRrs;
impl crate::RegisterSpec for TIMEOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`timeoutr::R`](R) reader structure
impl crate::Readable for TIMEOUTRrs {}
///`write(|w| ..)` method takes [`timeoutr::W`](W) writer structure
impl crate::Writable for TIMEOUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMEOUTR to value 0
impl crate::Resettable for TIMEOUTRrs {}
