///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**AES enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Disable AES
    Disabled = 0,
    ///1: Enable AES
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - AES enable
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Disable AES
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Enable AES
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - AES enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable AES
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Enable AES
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Data type selection (for data in and data out to/from the cryptographic block)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATATYPE {
    ///0: Word
    None = 0,
    ///1: Half-word (16-bit)
    HalfWord = 1,
    ///2: Byte (8-bit)
    Byte = 2,
    ///3: Bit
    Bit = 3,
}
impl From<DATATYPE> for u8 {
    #[inline(always)]
    fn from(variant: DATATYPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATATYPE {
    type Ux = u8;
}
impl crate::IsEnum for DATATYPE {}
///Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)
pub type DATATYPE_R = crate::FieldReader<DATATYPE>;
impl DATATYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATATYPE {
        match self.bits {
            0 => DATATYPE::None,
            1 => DATATYPE::HalfWord,
            2 => DATATYPE::Byte,
            3 => DATATYPE::Bit,
            _ => unreachable!(),
        }
    }
    ///Word
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DATATYPE::None
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DATATYPE::HalfWord
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DATATYPE::Byte
    }
    ///Bit
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == DATATYPE::Bit
    }
}
///Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATATYPE, crate::Safe>;
impl<'a, REG> DATATYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Word
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::None)
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::HalfWord)
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::Byte)
    }
    ///Bit
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::Bit)
    }
}
/**AES operating mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Mode 1: encryption
    Mode1 = 0,
    ///1: Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    Mode2 = 1,
    ///2: Mode 3: decryption
    Mode3 = 2,
    ///3: Mode 4: key derivation then single decryption
    Mode4 = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE` reader - AES operating mode
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Mode1,
            1 => MODE::Mode2,
            2 => MODE::Mode3,
            3 => MODE::Mode4,
            _ => unreachable!(),
        }
    }
    ///Mode 1: encryption
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == MODE::Mode1
    }
    ///Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == MODE::Mode2
    }
    ///Mode 3: decryption
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == MODE::Mode3
    }
    ///Mode 4: key derivation then single decryption
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == MODE::Mode4
    }
}
///Field `MODE` writer - AES operating mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Mode 1: encryption
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode1)
    }
    ///Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode2)
    }
    ///Mode 3: decryption
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode3)
    }
    ///Mode 4: key derivation then single decryption
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode4)
    }
}
/**AES chaining mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMOD {
    ///0: Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    Ecb = 0,
    ///1: Cipher-block chaining (CBC)
    Cbc = 1,
    ///2: Counter mode (CTR)
    Ctr = 2,
    ///3: Galois counter mode (GCM) and Galois message authentication code (GMAC)
    Gcm = 3,
}
impl From<CHMOD> for u8 {
    #[inline(always)]
    fn from(variant: CHMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHMOD {
    type Ux = u8;
}
impl crate::IsEnum for CHMOD {}
///Field `CHMOD` reader - AES chaining mode
pub type CHMOD_R = crate::FieldReader<CHMOD>;
impl CHMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHMOD {
        match self.bits {
            0 => CHMOD::Ecb,
            1 => CHMOD::Cbc,
            2 => CHMOD::Ctr,
            3 => CHMOD::Gcm,
            _ => unreachable!(),
        }
    }
    ///Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CHMOD::Ecb
    }
    ///Cipher-block chaining (CBC)
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CHMOD::Cbc
    }
    ///Counter mode (CTR)
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == CHMOD::Ctr
    }
    ///Galois counter mode (GCM) and Galois message authentication code (GMAC)
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == CHMOD::Gcm
    }
}
///Field `CHMOD` writer - AES chaining mode
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CHMOD, crate::Safe>;
impl<'a, REG> CHMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ecb)
    }
    ///Cipher-block chaining (CBC)
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Cbc)
    }
    ///Counter mode (CTR)
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ctr)
    }
    ///Galois counter mode (GCM) and Galois message authentication code (GMAC)
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Gcm)
    }
}
/**Enable DMA management of data input phase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINEN {
    ///0: Disable DMA Input
    Disabled = 0,
    ///1: Enable DMA Input
    Enabled = 1,
}
impl From<DMAINEN> for bool {
    #[inline(always)]
    fn from(variant: DMAINEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAINEN` reader - Enable DMA management of data input phase
pub type DMAINEN_R = crate::BitReader<DMAINEN>;
impl DMAINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAINEN {
        match self.bits {
            false => DMAINEN::Disabled,
            true => DMAINEN::Enabled,
        }
    }
    ///Disable DMA Input
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAINEN::Disabled
    }
    ///Enable DMA Input
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINEN::Enabled
    }
}
///Field `DMAINEN` writer - Enable DMA management of data input phase
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAINEN>;
impl<'a, REG> DMAINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DMA Input
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINEN::Disabled)
    }
    ///Enable DMA Input
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINEN::Enabled)
    }
}
/**Enable DMA management of data output phase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAOUTEN {
    ///0: Disable DMA Output
    Disabled = 0,
    ///1: Enabled DMA Output
    Enabled = 1,
}
impl From<DMAOUTEN> for bool {
    #[inline(always)]
    fn from(variant: DMAOUTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAOUTEN` reader - Enable DMA management of data output phase
pub type DMAOUTEN_R = crate::BitReader<DMAOUTEN>;
impl DMAOUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAOUTEN {
        match self.bits {
            false => DMAOUTEN::Disabled,
            true => DMAOUTEN::Enabled,
        }
    }
    ///Disable DMA Output
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAOUTEN::Disabled
    }
    ///Enabled DMA Output
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAOUTEN::Enabled
    }
}
///Field `DMAOUTEN` writer - Enable DMA management of data output phase
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAOUTEN>;
impl<'a, REG> DMAOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DMA Output
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAOUTEN::Disabled)
    }
    ///Enabled DMA Output
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAOUTEN::Enabled)
    }
}
/**GCMPH

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GCMPH {
    ///0: Init phase
    Init = 0,
    ///1: Header phase
    Header = 1,
    ///2: Payload phase
    Payload = 2,
    ///3: Final Phase
    Final = 3,
}
impl From<GCMPH> for u8 {
    #[inline(always)]
    fn from(variant: GCMPH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GCMPH {
    type Ux = u8;
}
impl crate::IsEnum for GCMPH {}
///Field `GCMPH` reader - GCMPH
pub type GCMPH_R = crate::FieldReader<GCMPH>;
impl GCMPH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCMPH {
        match self.bits {
            0 => GCMPH::Init,
            1 => GCMPH::Header,
            2 => GCMPH::Payload,
            3 => GCMPH::Final,
            _ => unreachable!(),
        }
    }
    ///Init phase
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        *self == GCMPH::Init
    }
    ///Header phase
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == GCMPH::Header
    }
    ///Payload phase
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == GCMPH::Payload
    }
    ///Final Phase
    #[inline(always)]
    pub fn is_final(&self) -> bool {
        *self == GCMPH::Final
    }
}
///Field `GCMPH` writer - GCMPH
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GCMPH, crate::Safe>;
impl<'a, REG> GCMPH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Init phase
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Init)
    }
    ///Header phase
    #[inline(always)]
    pub fn header(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Header)
    }
    ///Payload phase
    #[inline(always)]
    pub fn payload(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Payload)
    }
    ///Final Phase
    #[inline(always)]
    pub fn final_(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Final)
    }
}
/**CHMOD_2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHMOD_2 {
    ///0: Mode as per CHMOD (ECB, CBC, CTR, GCM)
    Chmod = 0,
    ///1: Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    Ccm = 1,
}
impl From<CHMOD_2> for bool {
    #[inline(always)]
    fn from(variant: CHMOD_2) -> Self {
        variant as u8 != 0
    }
}
///Field `CHMOD_2` reader - CHMOD_2
pub type CHMOD_2_R = crate::BitReader<CHMOD_2>;
impl CHMOD_2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHMOD_2 {
        match self.bits {
            false => CHMOD_2::Chmod,
            true => CHMOD_2::Ccm,
        }
    }
    ///Mode as per CHMOD (ECB, CBC, CTR, GCM)
    #[inline(always)]
    pub fn is_chmod(&self) -> bool {
        *self == CHMOD_2::Chmod
    }
    ///Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == CHMOD_2::Ccm
    }
}
///Field `CHMOD_2` writer - CHMOD_2
pub type CHMOD_2_W<'a, REG> = crate::BitWriter<'a, REG, CHMOD_2>;
impl<'a, REG> CHMOD_2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mode as per CHMOD (ECB, CBC, CTR, GCM)
    #[inline(always)]
    pub fn chmod(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD_2::Chmod)
    }
    ///Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    #[inline(always)]
    pub fn ccm(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD_2::Ccm)
    }
}
/**KEYSIZE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYSIZE {
    ///0: 128
    Aes128 = 0,
    ///1: 256
    Aes256 = 1,
}
impl From<KEYSIZE> for bool {
    #[inline(always)]
    fn from(variant: KEYSIZE) -> Self {
        variant as u8 != 0
    }
}
///Field `KEYSIZE` reader - KEYSIZE
pub type KEYSIZE_R = crate::BitReader<KEYSIZE>;
impl KEYSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KEYSIZE {
        match self.bits {
            false => KEYSIZE::Aes128,
            true => KEYSIZE::Aes256,
        }
    }
    ///128
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEYSIZE::Aes128
    }
    ///256
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEYSIZE::Aes256
    }
}
///Field `KEYSIZE` writer - KEYSIZE
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG, KEYSIZE>;
impl<'a, REG> KEYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///128
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZE::Aes128)
    }
    ///256
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZE::Aes256)
    }
}
///Field `NPBLB` reader - NPBLB
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - NPBLB
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**KMOD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KMOD {
    ///0: Normal key mode. Key registers are freely usable
    NormalKey = 0,
    ///2: Shared key mode. If shared key mode is properly initialized in SAES peripheral, the AES peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in AES key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register
    SharedKey = 2,
}
impl From<KMOD> for u8 {
    #[inline(always)]
    fn from(variant: KMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KMOD {
    type Ux = u8;
}
impl crate::IsEnum for KMOD {}
///Field `KMOD` reader - KMOD
pub type KMOD_R = crate::FieldReader<KMOD>;
impl KMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<KMOD> {
        match self.bits {
            0 => Some(KMOD::NormalKey),
            2 => Some(KMOD::SharedKey),
            _ => None,
        }
    }
    ///Normal key mode. Key registers are freely usable
    #[inline(always)]
    pub fn is_normal_key(&self) -> bool {
        *self == KMOD::NormalKey
    }
    ///Shared key mode. If shared key mode is properly initialized in SAES peripheral, the AES peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in AES key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register
    #[inline(always)]
    pub fn is_shared_key(&self) -> bool {
        *self == KMOD::SharedKey
    }
}
///Field `KMOD` writer - KMOD
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, KMOD>;
impl<'a, REG> KMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal key mode. Key registers are freely usable
    #[inline(always)]
    pub fn normal_key(self) -> &'a mut crate::W<REG> {
        self.variant(KMOD::NormalKey)
    }
    ///Shared key mode. If shared key mode is properly initialized in SAES peripheral, the AES peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in AES key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register
    #[inline(always)]
    pub fn shared_key(self) -> &'a mut crate::W<REG> {
        self.variant(KMOD::SharedKey)
    }
}
///Field `IPRST` reader - IPRST
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - IPRST
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCMPH
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - CHMOD_2
    #[inline(always)]
    pub fn chmod_2(&self) -> CHMOD_2_R {
        CHMOD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - NPBLB
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("iprst", &self.iprst())
            .field("kmod", &self.kmod())
            .field("npblb", &self.npblb())
            .field("keysize", &self.keysize())
            .field("chmod_2", &self.chmod_2())
            .field("gcmph", &self.gcmph())
            .field("dmaouten", &self.dmaouten())
            .field("dmainen", &self.dmainen())
            .field("chmod", &self.chmod())
            .field("mode", &self.mode())
            .field("datatype", &self.datatype())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCMPH
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - CHMOD_2
    #[inline(always)]
    pub fn chmod_2(&mut self) -> CHMOD_2_W<CRrs> {
        CHMOD_2_W::new(self, 16)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bits 20:23 - NPBLB
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<CRrs> {
        NPBLB_W::new(self, 20)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#AES:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
