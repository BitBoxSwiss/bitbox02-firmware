///Register `RX_ORDSETR` reader
pub type R = crate::R<RX_ORDSETRrs>;
/**RXORDSET

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXORDSET {
    ///0: SOP code detected in receiver
    Sop = 0,
    ///1: SOP' code detected in receiver
    Sopprime = 1,
    ///2: SOP'' code detected in receiver
    SopdoublePrime = 2,
    ///3: SOP'_Debug detected in receiver
    SopprimeDebug = 3,
    ///4: SOP''_Debug detected in receiver
    SopdoublePrimeDebug = 4,
    ///5: Cable Reset detected in receiver
    CableReset = 5,
    ///6: SOP extension #1 detected in receiver
    Sopextension1 = 6,
    ///7: SOP extension #2 detected in receiver
    Sopextension2 = 7,
}
impl From<RXORDSET> for u8 {
    #[inline(always)]
    fn from(variant: RXORDSET) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXORDSET {
    type Ux = u8;
}
impl crate::IsEnum for RXORDSET {}
///Field `RXORDSET` reader - RXORDSET
pub type RXORDSET_R = crate::FieldReader<RXORDSET>;
impl RXORDSET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXORDSET {
        match self.bits {
            0 => RXORDSET::Sop,
            1 => RXORDSET::Sopprime,
            2 => RXORDSET::SopdoublePrime,
            3 => RXORDSET::SopprimeDebug,
            4 => RXORDSET::SopdoublePrimeDebug,
            5 => RXORDSET::CableReset,
            6 => RXORDSET::Sopextension1,
            7 => RXORDSET::Sopextension2,
            _ => unreachable!(),
        }
    }
    ///SOP code detected in receiver
    #[inline(always)]
    pub fn is_sop(&self) -> bool {
        *self == RXORDSET::Sop
    }
    ///SOP' code detected in receiver
    #[inline(always)]
    pub fn is_sopprime(&self) -> bool {
        *self == RXORDSET::Sopprime
    }
    ///SOP'' code detected in receiver
    #[inline(always)]
    pub fn is_sopdouble_prime(&self) -> bool {
        *self == RXORDSET::SopdoublePrime
    }
    ///SOP'_Debug detected in receiver
    #[inline(always)]
    pub fn is_sopprime_debug(&self) -> bool {
        *self == RXORDSET::SopprimeDebug
    }
    ///SOP''_Debug detected in receiver
    #[inline(always)]
    pub fn is_sopdouble_prime_debug(&self) -> bool {
        *self == RXORDSET::SopdoublePrimeDebug
    }
    ///Cable Reset detected in receiver
    #[inline(always)]
    pub fn is_cable_reset(&self) -> bool {
        *self == RXORDSET::CableReset
    }
    ///SOP extension #1 detected in receiver
    #[inline(always)]
    pub fn is_sopextension1(&self) -> bool {
        *self == RXORDSET::Sopextension1
    }
    ///SOP extension #2 detected in receiver
    #[inline(always)]
    pub fn is_sopextension2(&self) -> bool {
        *self == RXORDSET::Sopextension2
    }
}
/**RXSOP3OF4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSOP3OF4 {
    ///0: 4 correct K-codes out of 4
    AllCorrect = 0,
    ///1: 3 correct K-codes out of 4
    OneIncorrect = 1,
}
impl From<RXSOP3OF4> for bool {
    #[inline(always)]
    fn from(variant: RXSOP3OF4) -> Self {
        variant as u8 != 0
    }
}
///Field `RXSOP3OF4` reader - RXSOP3OF4
pub type RXSOP3OF4_R = crate::BitReader<RXSOP3OF4>;
impl RXSOP3OF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXSOP3OF4 {
        match self.bits {
            false => RXSOP3OF4::AllCorrect,
            true => RXSOP3OF4::OneIncorrect,
        }
    }
    ///4 correct K-codes out of 4
    #[inline(always)]
    pub fn is_all_correct(&self) -> bool {
        *self == RXSOP3OF4::AllCorrect
    }
    ///3 correct K-codes out of 4
    #[inline(always)]
    pub fn is_one_incorrect(&self) -> bool {
        *self == RXSOP3OF4::OneIncorrect
    }
}
/**RXSOPKINVALID

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXSOPKINVALID {
    ///0: No K-code corrupted
    Valid = 0,
    ///1: First K-code corrupted
    FirstCorrupted = 1,
    ///2: Second K-code corrupted
    SecondCorrupted = 2,
    ///3: Third K-code corrupted
    ThirdCorrupted = 3,
    ///4: Fourth K-code corrupted
    FourthCorrupted = 4,
}
impl From<RXSOPKINVALID> for u8 {
    #[inline(always)]
    fn from(variant: RXSOPKINVALID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXSOPKINVALID {
    type Ux = u8;
}
impl crate::IsEnum for RXSOPKINVALID {}
///Field `RXSOPKINVALID` reader - RXSOPKINVALID
pub type RXSOPKINVALID_R = crate::FieldReader<RXSOPKINVALID>;
impl RXSOPKINVALID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXSOPKINVALID> {
        match self.bits {
            0 => Some(RXSOPKINVALID::Valid),
            1 => Some(RXSOPKINVALID::FirstCorrupted),
            2 => Some(RXSOPKINVALID::SecondCorrupted),
            3 => Some(RXSOPKINVALID::ThirdCorrupted),
            4 => Some(RXSOPKINVALID::FourthCorrupted),
            _ => None,
        }
    }
    ///No K-code corrupted
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == RXSOPKINVALID::Valid
    }
    ///First K-code corrupted
    #[inline(always)]
    pub fn is_first_corrupted(&self) -> bool {
        *self == RXSOPKINVALID::FirstCorrupted
    }
    ///Second K-code corrupted
    #[inline(always)]
    pub fn is_second_corrupted(&self) -> bool {
        *self == RXSOPKINVALID::SecondCorrupted
    }
    ///Third K-code corrupted
    #[inline(always)]
    pub fn is_third_corrupted(&self) -> bool {
        *self == RXSOPKINVALID::ThirdCorrupted
    }
    ///Fourth K-code corrupted
    #[inline(always)]
    pub fn is_fourth_corrupted(&self) -> bool {
        *self == RXSOPKINVALID::FourthCorrupted
    }
}
impl R {
    ///Bits 0:2 - RXORDSET
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - RXSOP3OF4
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - RXSOPKINVALID
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDSETR")
            .field("rxordset", &self.rxordset())
            .field("rxsop3of4", &self.rxsop3of4())
            .field("rxsopkinvalid", &self.rxsopkinvalid())
            .finish()
    }
}
/**UCPD Rx Ordered Set Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordsetr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#UCPD1:RX_ORDSETR)*/
pub struct RX_ORDSETRrs;
impl crate::RegisterSpec for RX_ORDSETRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ordsetr::R`](R) reader structure
impl crate::Readable for RX_ORDSETRrs {}
///`reset()` method sets RX_ORDSETR to value 0
impl crate::Resettable for RX_ORDSETRrs {}
