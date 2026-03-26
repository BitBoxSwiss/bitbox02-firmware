///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `RELOAD` reader - Counter reload value
pub type RELOAD_R = crate::FieldReader<u16>;
///Field `RELOAD` writer - Counter reload value
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `FELIM` reader - Frequency error limit
pub type FELIM_R = crate::FieldReader;
///Field `FELIM` writer - Frequency error limit
pub type FELIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**SYNC divider

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCDIV {
    ///0: SYNC not divided
    Div1 = 0,
    ///1: SYNC divided by 2
    Div2 = 1,
    ///2: SYNC divided by 4
    Div4 = 2,
    ///3: SYNC divided by 8
    Div8 = 3,
    ///4: SYNC divided by 16
    Div16 = 4,
    ///5: SYNC divided by 32
    Div32 = 5,
    ///6: SYNC divided by 64
    Div64 = 6,
    ///7: SYNC divided by 128
    Div128 = 7,
}
impl From<SYNCDIV> for u8 {
    #[inline(always)]
    fn from(variant: SYNCDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCDIV {
    type Ux = u8;
}
impl crate::IsEnum for SYNCDIV {}
///Field `SYNCDIV` reader - SYNC divider
pub type SYNCDIV_R = crate::FieldReader<SYNCDIV>;
impl SYNCDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCDIV {
        match self.bits {
            0 => SYNCDIV::Div1,
            1 => SYNCDIV::Div2,
            2 => SYNCDIV::Div4,
            3 => SYNCDIV::Div8,
            4 => SYNCDIV::Div16,
            5 => SYNCDIV::Div32,
            6 => SYNCDIV::Div64,
            7 => SYNCDIV::Div128,
            _ => unreachable!(),
        }
    }
    ///SYNC not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYNCDIV::Div1
    }
    ///SYNC divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SYNCDIV::Div2
    }
    ///SYNC divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SYNCDIV::Div4
    }
    ///SYNC divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SYNCDIV::Div8
    }
    ///SYNC divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SYNCDIV::Div16
    }
    ///SYNC divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SYNCDIV::Div32
    }
    ///SYNC divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SYNCDIV::Div64
    }
    ///SYNC divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SYNCDIV::Div128
    }
}
///Field `SYNCDIV` writer - SYNC divider
pub type SYNCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SYNCDIV, crate::Safe>;
impl<'a, REG> SYNCDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYNC not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div1)
    }
    ///SYNC divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div2)
    }
    ///SYNC divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div4)
    }
    ///SYNC divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div8)
    }
    ///SYNC divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div16)
    }
    ///SYNC divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div32)
    }
    ///SYNC divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div64)
    }
    ///SYNC divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::Div128)
    }
}
/**SYNC signal source selection

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSRC {
    ///0: GPIO AF (crs_sync_in_1) selected as SYNC signal source
    GpioAf = 0,
    ///1: LSE (crs_sync_in_2) selected as SYNC signal source
    Lse = 1,
    ///2: USB SOF (crs_sync_in_3) selected as SYNC signal source
    UsbSof = 2,
}
impl From<SYNCSRC> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCSRC {
    type Ux = u8;
}
impl crate::IsEnum for SYNCSRC {}
///Field `SYNCSRC` reader - SYNC signal source selection
pub type SYNCSRC_R = crate::FieldReader<SYNCSRC>;
impl SYNCSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCSRC> {
        match self.bits {
            0 => Some(SYNCSRC::GpioAf),
            1 => Some(SYNCSRC::Lse),
            2 => Some(SYNCSRC::UsbSof),
            _ => None,
        }
    }
    ///GPIO AF (crs_sync_in_1) selected as SYNC signal source
    #[inline(always)]
    pub fn is_gpio_af(&self) -> bool {
        *self == SYNCSRC::GpioAf
    }
    ///LSE (crs_sync_in_2) selected as SYNC signal source
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SYNCSRC::Lse
    }
    ///USB SOF (crs_sync_in_3) selected as SYNC signal source
    #[inline(always)]
    pub fn is_usb_sof(&self) -> bool {
        *self == SYNCSRC::UsbSof
    }
}
///Field `SYNCSRC` writer - SYNC signal source selection
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCSRC>;
impl<'a, REG> SYNCSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GPIO AF (crs_sync_in_1) selected as SYNC signal source
    #[inline(always)]
    pub fn gpio_af(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::GpioAf)
    }
    ///LSE (crs_sync_in_2) selected as SYNC signal source
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::Lse)
    }
    ///USB SOF (crs_sync_in_3) selected as SYNC signal source
    #[inline(always)]
    pub fn usb_sof(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::UsbSof)
    }
}
/**SYNC polarity selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCPOL {
    ///0: SYNC active on rising edge
    RisingEdge = 0,
    ///1: SYNC active on falling edge
    FallingEdge = 1,
}
impl From<SYNCPOL> for bool {
    #[inline(always)]
    fn from(variant: SYNCPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCPOL` reader - SYNC polarity selection
pub type SYNCPOL_R = crate::BitReader<SYNCPOL>;
impl SYNCPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCPOL {
        match self.bits {
            false => SYNCPOL::RisingEdge,
            true => SYNCPOL::FallingEdge,
        }
    }
    ///SYNC active on rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SYNCPOL::RisingEdge
    }
    ///SYNC active on falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SYNCPOL::FallingEdge
    }
}
///Field `SYNCPOL` writer - SYNC polarity selection
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG, SYNCPOL>;
impl<'a, REG> SYNCPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SYNC active on rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL::RisingEdge)
    }
    ///SYNC active on falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL::FallingEdge)
    }
}
impl R {
    ///Bits 0:15 - Counter reload value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Frequency error limit
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - SYNC divider
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:29 - SYNC signal source selection
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - SYNC polarity selection
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("syncpol", &self.syncpol())
            .field("syncsrc", &self.syncsrc())
            .field("syncdiv", &self.syncdiv())
            .field("felim", &self.felim())
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter reload value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<CFGRrs> {
        RELOAD_W::new(self, 0)
    }
    ///Bits 16:23 - Frequency error limit
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W<CFGRrs> {
        FELIM_W::new(self, 16)
    }
    ///Bits 24:26 - SYNC divider
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<CFGRrs> {
        SYNCDIV_W::new(self, 24)
    }
    ///Bits 28:29 - SYNC signal source selection
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<CFGRrs> {
        SYNCSRC_W::new(self, 28)
    }
    ///Bit 31 - SYNC polarity selection
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CFGRrs> {
        SYNCPOL_W::new(self, 31)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#CRS:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0x2022_bb7f
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
