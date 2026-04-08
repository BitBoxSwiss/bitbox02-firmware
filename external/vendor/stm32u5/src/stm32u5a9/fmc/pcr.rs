///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
/**Wait feature enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWAITEN {
    ///0: Wait feature disabled
    Disabled = 0,
    ///1: Wait feature enabled
    Enabled = 1,
}
impl From<PWAITEN> for bool {
    #[inline(always)]
    fn from(variant: PWAITEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PWAITEN` reader - Wait feature enable bit
pub type PWAITEN_R = crate::BitReader<PWAITEN>;
impl PWAITEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWAITEN {
        match self.bits {
            false => PWAITEN::Disabled,
            true => PWAITEN::Enabled,
        }
    }
    ///Wait feature disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWAITEN::Disabled
    }
    ///Wait feature enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWAITEN::Enabled
    }
}
///Field `PWAITEN` writer - Wait feature enable bit
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG, PWAITEN>;
impl<'a, REG> PWAITEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wait feature disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWAITEN::Disabled)
    }
    ///Wait feature enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWAITEN::Enabled)
    }
}
/**NAND Flash memory bank enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBKEN {
    ///0: Corresponding memory bank is disabled
    Disabled = 0,
    ///1: Corresponding memory bank is enabled
    Enabled = 1,
}
impl From<PBKEN> for bool {
    #[inline(always)]
    fn from(variant: PBKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PBKEN` reader - NAND Flash memory bank enable bit
pub type PBKEN_R = crate::BitReader<PBKEN>;
impl PBKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PBKEN {
        match self.bits {
            false => PBKEN::Disabled,
            true => PBKEN::Enabled,
        }
    }
    ///Corresponding memory bank is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PBKEN::Disabled
    }
    ///Corresponding memory bank is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PBKEN::Enabled
    }
}
///Field `PBKEN` writer - NAND Flash memory bank enable bit
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG, PBKEN>;
impl<'a, REG> PBKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Corresponding memory bank is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PBKEN::Disabled)
    }
    ///Corresponding memory bank is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PBKEN::Enabled)
    }
}
/**Memory type

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTYP {
    ///1: NAND Flash
    Nandflash = 1,
}
impl From<PTYP> for bool {
    #[inline(always)]
    fn from(variant: PTYP) -> Self {
        variant as u8 != 0
    }
}
///Field `PTYP` reader - Memory type
pub type PTYP_R = crate::BitReader<PTYP>;
impl PTYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTYP> {
        match self.bits {
            true => Some(PTYP::Nandflash),
            _ => None,
        }
    }
    ///NAND Flash
    #[inline(always)]
    pub fn is_nandflash(&self) -> bool {
        *self == PTYP::Nandflash
    }
}
///Field `PTYP` writer - Memory type
pub type PTYP_W<'a, REG> = crate::BitWriter<'a, REG, PTYP>;
impl<'a, REG> PTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NAND Flash
    #[inline(always)]
    pub fn nandflash(self) -> &'a mut crate::W<REG> {
        self.variant(PTYP::Nandflash)
    }
}
/**Data bus width

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWID {
    ///0: External memory device width 8 bits
    Bits8 = 0,
    ///1: External memory device width 16 bits
    Bits16 = 1,
}
impl From<PWID> for u8 {
    #[inline(always)]
    fn from(variant: PWID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWID {
    type Ux = u8;
}
impl crate::IsEnum for PWID {}
///Field `PWID` reader - Data bus width
pub type PWID_R = crate::FieldReader<PWID>;
impl PWID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWID> {
        match self.bits {
            0 => Some(PWID::Bits8),
            1 => Some(PWID::Bits16),
            _ => None,
        }
    }
    ///External memory device width 8 bits
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PWID::Bits8
    }
    ///External memory device width 16 bits
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PWID::Bits16
    }
}
///Field `PWID` writer - Data bus width
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWID>;
impl<'a, REG> PWID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///External memory device width 8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PWID::Bits8)
    }
    ///External memory device width 16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PWID::Bits16)
    }
}
/**ECC computation logic enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCEN {
    ///0: ECC logic is disabled and reset
    Disabled = 0,
    ///1: ECC logic is enabled
    Enabled = 1,
}
impl From<ECCEN> for bool {
    #[inline(always)]
    fn from(variant: ECCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCEN` reader - ECC computation logic enable bit
pub type ECCEN_R = crate::BitReader<ECCEN>;
impl ECCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCEN {
        match self.bits {
            false => ECCEN::Disabled,
            true => ECCEN::Enabled,
        }
    }
    ///ECC logic is disabled and reset
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCEN::Disabled
    }
    ///ECC logic is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCEN::Enabled
    }
}
///Field `ECCEN` writer - ECC computation logic enable bit
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG, ECCEN>;
impl<'a, REG> ECCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECC logic is disabled and reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCEN::Disabled)
    }
    ///ECC logic is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCEN::Enabled)
    }
}
///Field `TCLR` reader - CLE to RE delay
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - CLE to RE delay
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TAR` reader - ALE to RE delay
pub type TAR_R = crate::FieldReader;
///Field `TAR` writer - ALE to RE delay
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**ECC page size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCPS {
    ///0: ECC page size 256 bytes
    Bytes256 = 0,
    ///1: ECC page size 512 bytes
    Bytes512 = 1,
    ///2: ECC page size 1024 bytes
    Bytes1024 = 2,
    ///3: ECC page size 2048 bytes
    Bytes2048 = 3,
    ///4: ECC page size 4096 bytes
    Bytes4096 = 4,
    ///5: ECC page size 8192 bytes
    Bytes8192 = 5,
}
impl From<ECCPS> for u8 {
    #[inline(always)]
    fn from(variant: ECCPS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ECCPS {
    type Ux = u8;
}
impl crate::IsEnum for ECCPS {}
///Field `ECCPS` reader - ECC page size
pub type ECCPS_R = crate::FieldReader<ECCPS>;
impl ECCPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ECCPS> {
        match self.bits {
            0 => Some(ECCPS::Bytes256),
            1 => Some(ECCPS::Bytes512),
            2 => Some(ECCPS::Bytes1024),
            3 => Some(ECCPS::Bytes2048),
            4 => Some(ECCPS::Bytes4096),
            5 => Some(ECCPS::Bytes8192),
            _ => None,
        }
    }
    ///ECC page size 256 bytes
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == ECCPS::Bytes256
    }
    ///ECC page size 512 bytes
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == ECCPS::Bytes512
    }
    ///ECC page size 1024 bytes
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == ECCPS::Bytes1024
    }
    ///ECC page size 2048 bytes
    #[inline(always)]
    pub fn is_bytes2048(&self) -> bool {
        *self == ECCPS::Bytes2048
    }
    ///ECC page size 4096 bytes
    #[inline(always)]
    pub fn is_bytes4096(&self) -> bool {
        *self == ECCPS::Bytes4096
    }
    ///ECC page size 8192 bytes
    #[inline(always)]
    pub fn is_bytes8192(&self) -> bool {
        *self == ECCPS::Bytes8192
    }
}
///Field `ECCPS` writer - ECC page size
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ECCPS>;
impl<'a, REG> ECCPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ECC page size 256 bytes
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes256)
    }
    ///ECC page size 512 bytes
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes512)
    }
    ///ECC page size 1024 bytes
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes1024)
    }
    ///ECC page size 2048 bytes
    #[inline(always)]
    pub fn bytes2048(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes2048)
    }
    ///ECC page size 4096 bytes
    #[inline(always)]
    pub fn bytes4096(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes4096)
    }
    ///ECC page size 8192 bytes
    #[inline(always)]
    pub fn bytes8192(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes8192)
    }
}
impl R {
    ///Bit 1 - Wait feature enable bit
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NAND Flash memory bank enable bit
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Memory type
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Data bus width
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 9:12 - CLE to RE delay
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:16 - ALE to RE delay
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:19 - ECC page size
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("pwaiten", &self.pwaiten())
            .field("pbken", &self.pbken())
            .field("ptyp", &self.ptyp())
            .field("pwid", &self.pwid())
            .field("eccen", &self.eccen())
            .field("tclr", &self.tclr())
            .field("tar", &self.tar())
            .field("eccps", &self.eccps())
            .finish()
    }
}
impl W {
    ///Bit 1 - Wait feature enable bit
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - NAND Flash memory bank enable bit
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W<PCRrs> {
        PBKEN_W::new(self, 2)
    }
    ///Bit 3 - Memory type
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W<PCRrs> {
        PTYP_W::new(self, 3)
    }
    ///Bits 4:5 - Data bus width
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W<PCRrs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<PCRrs> {
        ECCEN_W::new(self, 6)
    }
    ///Bits 9:12 - CLE to RE delay
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<PCRrs> {
        TCLR_W::new(self, 9)
    }
    ///Bits 13:16 - ALE to RE delay
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<PCRrs> {
        TAR_W::new(self, 13)
    }
    ///Bits 17:19 - ECC page size
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W<PCRrs> {
        ECCPS_W::new(self, 17)
    }
}
/**NAND Flash control registers

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#FMC:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR to value 0x18
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x18;
}
