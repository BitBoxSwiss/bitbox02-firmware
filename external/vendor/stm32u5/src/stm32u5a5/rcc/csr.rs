///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.

Value on reset: 4*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIKSRANGE {
    ///4: Range 4 around 4 MHz
    F4mhz = 4,
    ///5: Range 5 around 2 MHz
    F2mhz = 5,
    ///6: Range 6 around 1.33 MHz
    F1_33mhz = 6,
    ///7: Range 7 around 1 MHz
    F1mhz = 7,
    ///8: Range 8 around 3.072 MHz
    F3_072mhz = 8,
}
impl From<MSIKSRANGE> for u8 {
    #[inline(always)]
    fn from(variant: MSIKSRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIKSRANGE {
    type Ux = u8;
}
impl crate::IsEnum for MSIKSRANGE {}
///Field `MSIKSRANGE` reader - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
pub type MSIKSRANGE_R = crate::FieldReader<MSIKSRANGE>;
impl MSIKSRANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSIKSRANGE> {
        match self.bits {
            4 => Some(MSIKSRANGE::F4mhz),
            5 => Some(MSIKSRANGE::F2mhz),
            6 => Some(MSIKSRANGE::F1_33mhz),
            7 => Some(MSIKSRANGE::F1mhz),
            8 => Some(MSIKSRANGE::F3_072mhz),
            _ => None,
        }
    }
    ///Range 4 around 4 MHz
    #[inline(always)]
    pub fn is_f_4mhz(&self) -> bool {
        *self == MSIKSRANGE::F4mhz
    }
    ///Range 5 around 2 MHz
    #[inline(always)]
    pub fn is_f_2mhz(&self) -> bool {
        *self == MSIKSRANGE::F2mhz
    }
    ///Range 6 around 1.33 MHz
    #[inline(always)]
    pub fn is_f_1_33mhz(&self) -> bool {
        *self == MSIKSRANGE::F1_33mhz
    }
    ///Range 7 around 1 MHz
    #[inline(always)]
    pub fn is_f_1mhz(&self) -> bool {
        *self == MSIKSRANGE::F1mhz
    }
    ///Range 8 around 3.072 MHz
    #[inline(always)]
    pub fn is_f_3_072mhz(&self) -> bool {
        *self == MSIKSRANGE::F3_072mhz
    }
}
///Field `MSIKSRANGE` writer - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
pub type MSIKSRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSIKSRANGE>;
impl<'a, REG> MSIKSRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Range 4 around 4 MHz
    #[inline(always)]
    pub fn f_4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKSRANGE::F4mhz)
    }
    ///Range 5 around 2 MHz
    #[inline(always)]
    pub fn f_2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKSRANGE::F2mhz)
    }
    ///Range 6 around 1.33 MHz
    #[inline(always)]
    pub fn f_1_33mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKSRANGE::F1_33mhz)
    }
    ///Range 7 around 1 MHz
    #[inline(always)]
    pub fn f_1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKSRANGE::F1mhz)
    }
    ///Range 8 around 3.072 MHz
    #[inline(always)]
    pub fn f_3_072mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKSRANGE::F3_072mhz)
    }
}
///Field `MSISSRANGE` reader - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
pub use MSIKSRANGE_R as MSISSRANGE_R;
///Field `MSISSRANGE` writer - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
pub use MSIKSRANGE_W as MSISSRANGE_W;
/**Remove reset flag This bit is set by software to clear the reset flags.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW {
    ///1: Clear the reset flags
    Clear = 1,
}
impl From<RMVFW> for bool {
    #[inline(always)]
    fn from(variant: RMVFW) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flag This bit is set by software to clear the reset flags.
pub type RMVF_R = crate::BitReader<RMVFW>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RMVFW> {
        match self.bits {
            true => Some(RMVFW::Clear),
            _ => None,
        }
    }
    ///Clear the reset flags
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW::Clear
    }
}
///Field `RMVF` writer - Remove reset flag This bit is set by software to clear the reset flags.
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVFW>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the reset flags
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVFW::Clear)
    }
}
/**Option-byte loader reset flag This bit is set by hardware when a reset from the option-byte loading occurs. It is cleared by�writing to the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTFR {
    ///0: No reset from option-byte loading occurred
    NotOccured = 0,
    ///1: Reset from option-byte loading occurred
    Occured = 1,
}
impl From<OBLRSTFR> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `OBLRSTF` reader - Option-byte loader reset flag This bit is set by hardware when a reset from the option-byte loading occurs. It is cleared by�writing to the RMVF bit.
pub type OBLRSTF_R = crate::BitReader<OBLRSTFR>;
impl OBLRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTFR {
        match self.bits {
            false => OBLRSTFR::NotOccured,
            true => OBLRSTFR::Occured,
        }
    }
    ///No reset from option-byte loading occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == OBLRSTFR::NotOccured
    }
    ///Reset from option-byte loading occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == OBLRSTFR::Occured
    }
}
/**NRST pin reset flag This bit is set by hardware when a reset from the NRST pin occurs. It is cleared by writing to�the RMVF bit.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTFR {
    ///0: No reset from NRST pin occurred
    NotOccured = 0,
    ///1: Reset from NRST pin occurred
    Occured = 1,
}
impl From<PINRSTFR> for bool {
    #[inline(always)]
    fn from(variant: PINRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `PINRSTF` reader - NRST pin reset flag This bit is set by hardware when a reset from the NRST pin occurs. It is cleared by writing to�the RMVF bit.
pub type PINRSTF_R = crate::BitReader<PINRSTFR>;
impl PINRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTFR {
        match self.bits {
            false => PINRSTFR::NotOccured,
            true => PINRSTFR::Occured,
        }
    }
    ///No reset from NRST pin occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == PINRSTFR::NotOccured
    }
    ///Reset from NRST pin occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == PINRSTFR::Occured
    }
}
/**Brownout reset or an exit from Shutdown mode reset flag This bit is set by hardware when a brownout reset or an exit from Shutdown mode reset occurs. It is cleared by writing to the RMVF bit.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTFR {
    ///0: No BOR/exit from Shutdown mode reset occurred
    NotOccured = 0,
    ///1: BOR/exit from Shutdown mode reset occurred
    Occured = 1,
}
impl From<BORRSTFR> for bool {
    #[inline(always)]
    fn from(variant: BORRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `BORRSTF` reader - Brownout reset or an exit from Shutdown mode reset flag This bit is set by hardware when a brownout reset or an exit from Shutdown mode reset occurs. It is cleared by writing to the RMVF bit.
pub type BORRSTF_R = crate::BitReader<BORRSTFR>;
impl BORRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORRSTFR {
        match self.bits {
            false => BORRSTFR::NotOccured,
            true => BORRSTFR::Occured,
        }
    }
    ///No BOR/exit from Shutdown mode reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == BORRSTFR::NotOccured
    }
    ///BOR/exit from Shutdown mode reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == BORRSTFR::Occured
    }
}
/**Software reset flag This bit is set by hardware when a software reset occurs. It is cleared by writing to RMVF.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTFR {
    ///0: No software reset occurred
    NotOccured = 0,
    ///1: Software reset occurred
    Occured = 1,
}
impl From<SFTRSTFR> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SFTRSTF` reader - Software reset flag This bit is set by hardware when a software reset occurs. It is cleared by writing to RMVF.
pub type SFTRSTF_R = crate::BitReader<SFTRSTFR>;
impl SFTRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFTRSTFR {
        match self.bits {
            false => SFTRSTFR::NotOccured,
            true => SFTRSTFR::Occured,
        }
    }
    ///No software reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == SFTRSTFR::NotOccured
    }
    ///Software reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == SFTRSTFR::Occured
    }
}
/**Independent watchdog reset flag This bit is set by hardware when an independent watchdog reset domain occurs. It is cleared by writing to the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTFR {
    ///0: No independent watchdog reset occurred
    NotOccured = 0,
    ///1: Independent watchdog reset occurred
    Occured = 1,
}
impl From<IWDGRSTFR> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDGRSTF` reader - Independent watchdog reset flag This bit is set by hardware when an independent watchdog reset domain occurs. It is cleared by writing to the RMVF bit.
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTFR>;
impl IWDGRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDGRSTFR {
        match self.bits {
            false => IWDGRSTFR::NotOccured,
            true => IWDGRSTFR::Occured,
        }
    }
    ///No independent watchdog reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == IWDGRSTFR::NotOccured
    }
    ///Independent watchdog reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == IWDGRSTFR::Occured
    }
}
/**Window watchdog reset flag This bit is set by hardware when a window watchdog reset occurs. It is cleared by writing to�the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTFR {
    ///0: No window watchdog reset occurred
    NotOccured = 0,
    ///1: Window watchdog reset occurred
    Occured = 1,
}
impl From<WWDGRSTFR> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDGRSTF` reader - Window watchdog reset flag This bit is set by hardware when a window watchdog reset occurs. It is cleared by writing to�the RMVF bit.
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTFR>;
impl WWDGRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDGRSTFR {
        match self.bits {
            false => WWDGRSTFR::NotOccured,
            true => WWDGRSTFR::Occured,
        }
    }
    ///No window watchdog reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == WWDGRSTFR::NotOccured
    }
    ///Window watchdog reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == WWDGRSTFR::Occured
    }
}
/**Low-power reset flag This bit is set by hardware when a reset occurs due to a Stop, Standby, or Shutdown mode entry, whereas the corresponding NRST_STOP, NRST_STBY, or NRST_SHDW option bit is cleared. This bit is cleared by writing to the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTFR {
    ///0: No illegal low-power mode reset occurred
    NotOccured = 0,
    ///1: Illegal low-power mode reset occurred
    Occured = 1,
}
impl From<LPWRRSTFR> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `LPWRRSTF` reader - Low-power reset flag This bit is set by hardware when a reset occurs due to a Stop, Standby, or Shutdown mode entry, whereas the corresponding NRST_STOP, NRST_STBY, or NRST_SHDW option bit is cleared. This bit is cleared by writing to the RMVF bit.
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTFR>;
impl LPWRRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPWRRSTFR {
        match self.bits {
            false => LPWRRSTFR::NotOccured,
            true => LPWRRSTFR::Occured,
        }
    }
    ///No illegal low-power mode reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == LPWRRSTFR::NotOccured
    }
    ///Illegal low-power mode reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == LPWRRSTFR::Occured
    }
}
impl R {
    ///Bits 8:11 - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
    #[inline(always)]
    pub fn msiksrange(&self) -> MSIKSRANGE_R {
        MSIKSRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
    #[inline(always)]
    pub fn msissrange(&self) -> MSISSRANGE_R {
        MSISSRANGE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 23 - Remove reset flag This bit is set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option-byte loader reset flag This bit is set by hardware when a reset from the option-byte loading occurs. It is cleared by�writing to the RMVF bit.
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - NRST pin reset flag This bit is set by hardware when a reset from the NRST pin occurs. It is cleared by writing to�the RMVF bit.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Brownout reset or an exit from Shutdown mode reset flag This bit is set by hardware when a brownout reset or an exit from Shutdown mode reset occurs. It is cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag This bit is set by hardware when a software reset occurs. It is cleared by writing to RMVF.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag This bit is set by hardware when an independent watchdog reset domain occurs. It is cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag This bit is set by hardware when a window watchdog reset occurs. It is cleared by writing to�the RMVF bit.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag This bit is set by hardware when a reset occurs due to a Stop, Standby, or Shutdown mode entry, whereas the corresponding NRST_STOP, NRST_STBY, or NRST_SHDW option bit is cleared. This bit is cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("msiksrange", &self.msiksrange())
            .field("msissrange", &self.msissrange())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("borrstf", &self.borrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bits 8:11 - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
    #[inline(always)]
    pub fn msiksrange(&mut self) -> MSIKSRANGE_W<CSRrs> {
        MSIKSRANGE_W::new(self, 8)
    }
    ///Bits 12:15 - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
    #[inline(always)]
    pub fn msissrange(&mut self) -> MSISSRANGE_W<CSRrs> {
        MSISSRANGE_W::new(self, 12)
    }
    ///Bit 23 - Remove reset flag This bit is set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**RCC control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x0c00_4400
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_4400;
}
