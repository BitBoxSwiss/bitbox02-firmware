///Register `ICSCR1` reader
pub type R = crate::R<ICSCR1rs>;
///Register `ICSCR1` writer
pub type W = crate::W<ICSCR1rs>;
///Field `MSICAL3` reader - MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\[4:0\] and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL3_R = crate::FieldReader;
///Field `MSICAL2` reader - MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\[4:0\] and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL2_R = crate::FieldReader;
///Field `MSICAL1` reader - MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\[4:0\] and the factory calibration trim value MSIRC1\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL1_R = crate::FieldReader;
///Field `MSICAL0` reader - MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\[4:0\] and the factory-programmed calibration trim value MSIRC0\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL0_R = crate::FieldReader;
/**MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIBIAS {
    ///0: MSI bias continuous mode (clock accuracy fast settling time)
    Continuous = 0,
    ///1: MSI bias sampling mode when the regulator is in range 4, or when the device is in Stop 1 or Stop 2 (ultra-low-power mode)
    Sampling = 1,
}
impl From<MSIBIAS> for bool {
    #[inline(always)]
    fn from(variant: MSIBIAS) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIBIAS` reader - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
pub type MSIBIAS_R = crate::BitReader<MSIBIAS>;
impl MSIBIAS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIBIAS {
        match self.bits {
            false => MSIBIAS::Continuous,
            true => MSIBIAS::Sampling,
        }
    }
    ///MSI bias continuous mode (clock accuracy fast settling time)
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MSIBIAS::Continuous
    }
    ///MSI bias sampling mode when the regulator is in range 4, or when the device is in Stop 1 or Stop 2 (ultra-low-power mode)
    #[inline(always)]
    pub fn is_sampling(&self) -> bool {
        *self == MSIBIAS::Sampling
    }
}
///Field `MSIBIAS` writer - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
pub type MSIBIAS_W<'a, REG> = crate::BitWriter<'a, REG, MSIBIAS>;
impl<'a, REG> MSIBIAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI bias continuous mode (clock accuracy fast settling time)
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(MSIBIAS::Continuous)
    }
    ///MSI bias sampling mode when the regulator is in range 4, or when the device is in Stop 1 or Stop 2 (ultra-low-power mode)
    #[inline(always)]
    pub fn sampling(self) -> &'a mut crate::W<REG> {
        self.variant(MSIBIAS::Sampling)
    }
}
/**MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\] and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL {
    ///0: MSIS/MSIK ranges provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR
    Csr = 0,
    ///1: MSIS/MSIK ranges provided by MSISRANGE\[3:0\] and MSIKRANGE\[3:0\] in RCC_ICSCR1
    Icscr1 = 1,
}
impl From<MSIRGSEL> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRGSEL` reader - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\] and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR.
pub type MSIRGSEL_R = crate::BitReader<MSIRGSEL>;
impl MSIRGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIRGSEL {
        match self.bits {
            false => MSIRGSEL::Csr,
            true => MSIRGSEL::Icscr1,
        }
    }
    ///MSIS/MSIK ranges provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR
    #[inline(always)]
    pub fn is_csr(&self) -> bool {
        *self == MSIRGSEL::Csr
    }
    ///MSIS/MSIK ranges provided by MSISRANGE\[3:0\] and MSIKRANGE\[3:0\] in RCC_ICSCR1
    #[inline(always)]
    pub fn is_icscr1(&self) -> bool {
        *self == MSIRGSEL::Icscr1
    }
}
///Field `MSIRGSEL` writer - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\] and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR.
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSIRGSEL>;
impl<'a, REG> MSIRGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSIS/MSIK ranges provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR
    #[inline(always)]
    pub fn csr(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Csr)
    }
    ///MSIS/MSIK ranges provided by MSISRANGE\[3:0\] and MSIKRANGE\[3:0\] in RCC_ICSCR1
    #[inline(always)]
    pub fn icscr1(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Icscr1)
    }
}
/**MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).

Value on reset: 4*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIKRANGE {
    ///0: Range 0 around 48 MHz
    F48mhz = 0,
    ///1: Range 1 around 24 MHz
    F24mhz = 1,
    ///2: Range 2 around 16 MHz
    F16mhz = 2,
    ///3: Range 3 around 12 MHz
    F12mhz = 3,
    ///4: Range 4 around 4 MHz
    F4mhz = 4,
    ///5: Range 5 around 2 MHz
    F2mhz = 5,
    ///6: Range 6 around 1.33 MHz
    F1_333mhz = 6,
    ///7: Range 7 around 1 MHz
    F1mhz = 7,
    ///8: Range 8 around 3.072 MHz
    F3_072mhz = 8,
    ///9: Range 9 around 1.536 MHz
    F1_536mhz = 9,
    ///10: Range 10 around 1.024 MHz
    F1_024mhz = 10,
    ///11: Range 11 around 768 kHz
    F768kHz = 11,
    ///12: Range 12 around 400 kHz
    F400kHz = 12,
    ///13: Range 13 around 200 kHz
    F200kHz = 13,
    ///14: Range 14 around 133 kHz
    F133kHz = 14,
    ///15: Range 15 around 100 kHz
    F100kHz = 15,
}
impl From<MSIKRANGE> for u8 {
    #[inline(always)]
    fn from(variant: MSIKRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIKRANGE {
    type Ux = u8;
}
impl crate::IsEnum for MSIKRANGE {}
///Field `MSIKRANGE` reader - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
pub type MSIKRANGE_R = crate::FieldReader<MSIKRANGE>;
impl MSIKRANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIKRANGE {
        match self.bits {
            0 => MSIKRANGE::F48mhz,
            1 => MSIKRANGE::F24mhz,
            2 => MSIKRANGE::F16mhz,
            3 => MSIKRANGE::F12mhz,
            4 => MSIKRANGE::F4mhz,
            5 => MSIKRANGE::F2mhz,
            6 => MSIKRANGE::F1_333mhz,
            7 => MSIKRANGE::F1mhz,
            8 => MSIKRANGE::F3_072mhz,
            9 => MSIKRANGE::F1_536mhz,
            10 => MSIKRANGE::F1_024mhz,
            11 => MSIKRANGE::F768kHz,
            12 => MSIKRANGE::F400kHz,
            13 => MSIKRANGE::F200kHz,
            14 => MSIKRANGE::F133kHz,
            15 => MSIKRANGE::F100kHz,
            _ => unreachable!(),
        }
    }
    ///Range 0 around 48 MHz
    #[inline(always)]
    pub fn is_f_48mhz(&self) -> bool {
        *self == MSIKRANGE::F48mhz
    }
    ///Range 1 around 24 MHz
    #[inline(always)]
    pub fn is_f_24mhz(&self) -> bool {
        *self == MSIKRANGE::F24mhz
    }
    ///Range 2 around 16 MHz
    #[inline(always)]
    pub fn is_f_16mhz(&self) -> bool {
        *self == MSIKRANGE::F16mhz
    }
    ///Range 3 around 12 MHz
    #[inline(always)]
    pub fn is_f_12mhz(&self) -> bool {
        *self == MSIKRANGE::F12mhz
    }
    ///Range 4 around 4 MHz
    #[inline(always)]
    pub fn is_f_4mhz(&self) -> bool {
        *self == MSIKRANGE::F4mhz
    }
    ///Range 5 around 2 MHz
    #[inline(always)]
    pub fn is_f_2mhz(&self) -> bool {
        *self == MSIKRANGE::F2mhz
    }
    ///Range 6 around 1.33 MHz
    #[inline(always)]
    pub fn is_f_1_333mhz(&self) -> bool {
        *self == MSIKRANGE::F1_333mhz
    }
    ///Range 7 around 1 MHz
    #[inline(always)]
    pub fn is_f_1mhz(&self) -> bool {
        *self == MSIKRANGE::F1mhz
    }
    ///Range 8 around 3.072 MHz
    #[inline(always)]
    pub fn is_f_3_072mhz(&self) -> bool {
        *self == MSIKRANGE::F3_072mhz
    }
    ///Range 9 around 1.536 MHz
    #[inline(always)]
    pub fn is_f_1_536mhz(&self) -> bool {
        *self == MSIKRANGE::F1_536mhz
    }
    ///Range 10 around 1.024 MHz
    #[inline(always)]
    pub fn is_f_1_024mhz(&self) -> bool {
        *self == MSIKRANGE::F1_024mhz
    }
    ///Range 11 around 768 kHz
    #[inline(always)]
    pub fn is_f_768k_hz(&self) -> bool {
        *self == MSIKRANGE::F768kHz
    }
    ///Range 12 around 400 kHz
    #[inline(always)]
    pub fn is_f_400k_hz(&self) -> bool {
        *self == MSIKRANGE::F400kHz
    }
    ///Range 13 around 200 kHz
    #[inline(always)]
    pub fn is_f_200k_hz(&self) -> bool {
        *self == MSIKRANGE::F200kHz
    }
    ///Range 14 around 133 kHz
    #[inline(always)]
    pub fn is_f_133k_hz(&self) -> bool {
        *self == MSIKRANGE::F133kHz
    }
    ///Range 15 around 100 kHz
    #[inline(always)]
    pub fn is_f_100k_hz(&self) -> bool {
        *self == MSIKRANGE::F100kHz
    }
}
///Field `MSIKRANGE` writer - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
pub type MSIKRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSIKRANGE, crate::Safe>;
impl<'a, REG> MSIKRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Range 0 around 48 MHz
    #[inline(always)]
    pub fn f_48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F48mhz)
    }
    ///Range 1 around 24 MHz
    #[inline(always)]
    pub fn f_24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F24mhz)
    }
    ///Range 2 around 16 MHz
    #[inline(always)]
    pub fn f_16mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F16mhz)
    }
    ///Range 3 around 12 MHz
    #[inline(always)]
    pub fn f_12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F12mhz)
    }
    ///Range 4 around 4 MHz
    #[inline(always)]
    pub fn f_4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F4mhz)
    }
    ///Range 5 around 2 MHz
    #[inline(always)]
    pub fn f_2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F2mhz)
    }
    ///Range 6 around 1.33 MHz
    #[inline(always)]
    pub fn f_1_333mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F1_333mhz)
    }
    ///Range 7 around 1 MHz
    #[inline(always)]
    pub fn f_1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F1mhz)
    }
    ///Range 8 around 3.072 MHz
    #[inline(always)]
    pub fn f_3_072mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F3_072mhz)
    }
    ///Range 9 around 1.536 MHz
    #[inline(always)]
    pub fn f_1_536mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F1_536mhz)
    }
    ///Range 10 around 1.024 MHz
    #[inline(always)]
    pub fn f_1_024mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F1_024mhz)
    }
    ///Range 11 around 768 kHz
    #[inline(always)]
    pub fn f_768k_hz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F768kHz)
    }
    ///Range 12 around 400 kHz
    #[inline(always)]
    pub fn f_400k_hz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F400kHz)
    }
    ///Range 13 around 200 kHz
    #[inline(always)]
    pub fn f_200k_hz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F200kHz)
    }
    ///Range 14 around 133 kHz
    #[inline(always)]
    pub fn f_133k_hz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F133kHz)
    }
    ///Range 15 around 100 kHz
    #[inline(always)]
    pub fn f_100k_hz(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKRANGE::F100kHz)
    }
}
///Field `MSISRANGE` reader - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
pub use MSIKRANGE_R as MSISRANGE_R;
///Field `MSISRANGE` writer - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
pub use MSIKRANGE_W as MSISRANGE_W;
impl R {
    ///Bits 0:4 - MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\[4:0\] and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical3(&self) -> MSICAL3_R {
        MSICAL3_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\[4:0\] and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical2(&self) -> MSICAL2_R {
        MSICAL2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\[4:0\] and the factory calibration trim value MSIRC1\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical1(&self) -> MSICAL1_R {
        MSICAL1_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\[4:0\] and the factory-programmed calibration trim value MSIRC0\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical0(&self) -> MSICAL0_R {
        MSICAL0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bit 22 - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
    #[inline(always)]
    pub fn msibias(&self) -> MSIBIAS_R {
        MSIBIAS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\] and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR.
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
    #[inline(always)]
    pub fn msikrange(&self) -> MSIKRANGE_R {
        MSIKRANGE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR1")
            .field("msical3", &self.msical3())
            .field("msical2", &self.msical2())
            .field("msical1", &self.msical1())
            .field("msical0", &self.msical0())
            .field("msibias", &self.msibias())
            .field("msirgsel", &self.msirgsel())
            .field("msikrange", &self.msikrange())
            .field("msisrange", &self.msisrange())
            .finish()
    }
}
impl W {
    ///Bit 22 - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
    #[inline(always)]
    pub fn msibias(&mut self) -> MSIBIAS_W<ICSCR1rs> {
        MSIBIAS_W::new(self, 22)
    }
    ///Bit 23 - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\] and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\] and MSIKSRANGE\[3:0\] in RCC_CSR.
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<ICSCR1rs> {
        MSIRGSEL_W::new(self, 23)
    }
    ///Bits 24:27 - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
    #[inline(always)]
    pub fn msikrange(&mut self) -> MSIKRANGE_W<ICSCR1rs> {
        MSIKRANGE_W::new(self, 24)
    }
    ///Bits 28:31 - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
    #[inline(always)]
    pub fn msisrange(&mut self) -> MSISRANGE_W<ICSCR1rs> {
        MSISRANGE_W::new(self, 28)
    }
}
/**RCC internal clock sources calibration register 1

You can [`read`](crate::Reg::read) this register and get [`icscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:ICSCR1)*/
pub struct ICSCR1rs;
impl crate::RegisterSpec for ICSCR1rs {
    type Ux = u32;
}
///`read()` method returns [`icscr1::R`](R) reader structure
impl crate::Readable for ICSCR1rs {}
///`write(|w| ..)` method takes [`icscr1::W`](W) writer structure
impl crate::Writable for ICSCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSCR1 to value 0x4400_0000
impl crate::Resettable for ICSCR1rs {
    const RESET_VALUE: u32 = 0x4400_0000;
}
