///Register `CFG1` reader
pub type R = crate::R<CFG1rs>;
///Register `CFG1` writer
pub type W = crate::W<CFG1rs>;
///Field `DSIZE` reader - Number of bits in at single SPI data frame
pub type DSIZE_R = crate::FieldReader;
///Field `DSIZE` writer - Number of bits in at single SPI data frame
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**threshold level

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTHLV {
    ///0: 1 frame
    OneFrame = 0,
    ///1: 2 frames
    TwoFrames = 1,
    ///2: 3 frames
    ThreeFrames = 2,
    ///3: 4 frames
    FourFrames = 3,
    ///4: 5 frames
    FiveFrames = 4,
    ///5: 6 frames
    SixFrames = 5,
    ///6: 7 frames
    SevenFrames = 6,
    ///7: 8 frames
    EightFrames = 7,
    ///8: 9 frames
    NineFrames = 8,
    ///9: 10 frames
    TenFrames = 9,
    ///10: 11 frames
    ElevenFrames = 10,
    ///11: 12 frames
    TwelveFrames = 11,
    ///12: 13 frames
    ThirteenFrames = 12,
    ///13: 14 frames
    FourteenFrames = 13,
    ///14: 15 frames
    FifteenFrames = 14,
    ///15: 16 frames
    SixteenFrames = 15,
}
impl From<FTHLV> for u8 {
    #[inline(always)]
    fn from(variant: FTHLV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTHLV {
    type Ux = u8;
}
impl crate::IsEnum for FTHLV {}
///Field `FTHLV` reader - threshold level
pub type FTHLV_R = crate::FieldReader<FTHLV>;
impl FTHLV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTHLV {
        match self.bits {
            0 => FTHLV::OneFrame,
            1 => FTHLV::TwoFrames,
            2 => FTHLV::ThreeFrames,
            3 => FTHLV::FourFrames,
            4 => FTHLV::FiveFrames,
            5 => FTHLV::SixFrames,
            6 => FTHLV::SevenFrames,
            7 => FTHLV::EightFrames,
            8 => FTHLV::NineFrames,
            9 => FTHLV::TenFrames,
            10 => FTHLV::ElevenFrames,
            11 => FTHLV::TwelveFrames,
            12 => FTHLV::ThirteenFrames,
            13 => FTHLV::FourteenFrames,
            14 => FTHLV::FifteenFrames,
            15 => FTHLV::SixteenFrames,
            _ => unreachable!(),
        }
    }
    ///1 frame
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == FTHLV::OneFrame
    }
    ///2 frames
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == FTHLV::TwoFrames
    }
    ///3 frames
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == FTHLV::ThreeFrames
    }
    ///4 frames
    #[inline(always)]
    pub fn is_four_frames(&self) -> bool {
        *self == FTHLV::FourFrames
    }
    ///5 frames
    #[inline(always)]
    pub fn is_five_frames(&self) -> bool {
        *self == FTHLV::FiveFrames
    }
    ///6 frames
    #[inline(always)]
    pub fn is_six_frames(&self) -> bool {
        *self == FTHLV::SixFrames
    }
    ///7 frames
    #[inline(always)]
    pub fn is_seven_frames(&self) -> bool {
        *self == FTHLV::SevenFrames
    }
    ///8 frames
    #[inline(always)]
    pub fn is_eight_frames(&self) -> bool {
        *self == FTHLV::EightFrames
    }
    ///9 frames
    #[inline(always)]
    pub fn is_nine_frames(&self) -> bool {
        *self == FTHLV::NineFrames
    }
    ///10 frames
    #[inline(always)]
    pub fn is_ten_frames(&self) -> bool {
        *self == FTHLV::TenFrames
    }
    ///11 frames
    #[inline(always)]
    pub fn is_eleven_frames(&self) -> bool {
        *self == FTHLV::ElevenFrames
    }
    ///12 frames
    #[inline(always)]
    pub fn is_twelve_frames(&self) -> bool {
        *self == FTHLV::TwelveFrames
    }
    ///13 frames
    #[inline(always)]
    pub fn is_thirteen_frames(&self) -> bool {
        *self == FTHLV::ThirteenFrames
    }
    ///14 frames
    #[inline(always)]
    pub fn is_fourteen_frames(&self) -> bool {
        *self == FTHLV::FourteenFrames
    }
    ///15 frames
    #[inline(always)]
    pub fn is_fifteen_frames(&self) -> bool {
        *self == FTHLV::FifteenFrames
    }
    ///16 frames
    #[inline(always)]
    pub fn is_sixteen_frames(&self) -> bool {
        *self == FTHLV::SixteenFrames
    }
}
///Field `FTHLV` writer - threshold level
pub type FTHLV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FTHLV, crate::Safe>;
impl<'a, REG> FTHLV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 frame
    #[inline(always)]
    pub fn one_frame(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::OneFrame)
    }
    ///2 frames
    #[inline(always)]
    pub fn two_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::TwoFrames)
    }
    ///3 frames
    #[inline(always)]
    pub fn three_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::ThreeFrames)
    }
    ///4 frames
    #[inline(always)]
    pub fn four_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FourFrames)
    }
    ///5 frames
    #[inline(always)]
    pub fn five_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FiveFrames)
    }
    ///6 frames
    #[inline(always)]
    pub fn six_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::SixFrames)
    }
    ///7 frames
    #[inline(always)]
    pub fn seven_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::SevenFrames)
    }
    ///8 frames
    #[inline(always)]
    pub fn eight_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::EightFrames)
    }
    ///9 frames
    #[inline(always)]
    pub fn nine_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::NineFrames)
    }
    ///10 frames
    #[inline(always)]
    pub fn ten_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::TenFrames)
    }
    ///11 frames
    #[inline(always)]
    pub fn eleven_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::ElevenFrames)
    }
    ///12 frames
    #[inline(always)]
    pub fn twelve_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::TwelveFrames)
    }
    ///13 frames
    #[inline(always)]
    pub fn thirteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::ThirteenFrames)
    }
    ///14 frames
    #[inline(always)]
    pub fn fourteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FourteenFrames)
    }
    ///15 frames
    #[inline(always)]
    pub fn fifteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FifteenFrames)
    }
    ///16 frames
    #[inline(always)]
    pub fn sixteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::SixteenFrames)
    }
}
/**Behavior of slave transmitter at underrun condition

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRCFG {
    ///0: Slave sends a constant underrun pattern
    Constant = 0,
    ///1: Slave repeats last received data frame from master
    RepeatReceived = 1,
}
impl From<UDRCFG> for bool {
    #[inline(always)]
    fn from(variant: UDRCFG) -> Self {
        variant as u8 != 0
    }
}
///Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition
pub type UDRCFG_R = crate::BitReader<UDRCFG>;
impl UDRCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDRCFG {
        match self.bits {
            false => UDRCFG::Constant,
            true => UDRCFG::RepeatReceived,
        }
    }
    ///Slave sends a constant underrun pattern
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == UDRCFG::Constant
    }
    ///Slave repeats last received data frame from master
    #[inline(always)]
    pub fn is_repeat_received(&self) -> bool {
        *self == UDRCFG::RepeatReceived
    }
}
///Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition
pub type UDRCFG_W<'a, REG> = crate::BitWriter<'a, REG, UDRCFG>;
impl<'a, REG> UDRCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave sends a constant underrun pattern
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG::Constant)
    }
    ///Slave repeats last received data frame from master
    #[inline(always)]
    pub fn repeat_received(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG::RepeatReceived)
    }
}
/**Rx DMA stream enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    ///0: Rx buffer DMA disabled
    Disabled = 0,
    ///1: Rx buffer DMA enabled
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - Rx DMA stream enable
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
///Field `RXDMAEN` writer - Rx DMA stream enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
/**Tx DMA stream enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    ///0: Tx buffer DMA disabled
    Disabled = 0,
    ///1: Tx buffer DMA enabled
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - Tx DMA stream enable
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
///Field `TXDMAEN` writer - Tx DMA stream enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
///Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared
pub type CRCSIZE_R = crate::FieldReader;
///Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared
pub type CRCSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Hardware CRC computation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    ///0: CRC calculation disabled
    Disabled = 0,
    ///1: CRC calculation enabled
    Enabled = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEN` reader - Hardware CRC computation enable
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::Disabled,
            true => CRCEN::Enabled,
        }
    }
    ///CRC calculation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN::Disabled
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN::Enabled
    }
}
///Field `CRCEN` writer - Hardware CRC computation enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Disabled)
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Enabled)
    }
}
/**Master baud rate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBR {
    ///0: f_spi_ker_ck / 2
    Div2 = 0,
    ///1: f_spi_ker_ck / 4
    Div4 = 1,
    ///2: f_spi_ker_ck / 8
    Div8 = 2,
    ///3: f_spi_ker_ck / 16
    Div16 = 3,
    ///4: f_spi_ker_ck / 32
    Div32 = 4,
    ///5: f_spi_ker_ck / 64
    Div64 = 5,
    ///6: f_spi_ker_ck / 128
    Div128 = 6,
    ///7: f_spi_ker_ck / 256
    Div256 = 7,
}
impl From<MBR> for u8 {
    #[inline(always)]
    fn from(variant: MBR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBR {
    type Ux = u8;
}
impl crate::IsEnum for MBR {}
///Field `MBR` reader - Master baud rate
pub type MBR_R = crate::FieldReader<MBR>;
impl MBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MBR {
        match self.bits {
            0 => MBR::Div2,
            1 => MBR::Div4,
            2 => MBR::Div8,
            3 => MBR::Div16,
            4 => MBR::Div32,
            5 => MBR::Div64,
            6 => MBR::Div128,
            7 => MBR::Div256,
            _ => unreachable!(),
        }
    }
    ///f_spi_ker_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MBR::Div2
    }
    ///f_spi_ker_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MBR::Div4
    }
    ///f_spi_ker_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MBR::Div8
    }
    ///f_spi_ker_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MBR::Div16
    }
    ///f_spi_ker_ck / 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MBR::Div32
    }
    ///f_spi_ker_ck / 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MBR::Div64
    }
    ///f_spi_ker_ck / 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MBR::Div128
    }
    ///f_spi_ker_ck / 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == MBR::Div256
    }
}
///Field `MBR` writer - Master baud rate
pub type MBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MBR, crate::Safe>;
impl<'a, REG> MBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///f_spi_ker_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div2)
    }
    ///f_spi_ker_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div4)
    }
    ///f_spi_ker_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div8)
    }
    ///f_spi_ker_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div16)
    }
    ///f_spi_ker_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div32)
    }
    ///f_spi_ker_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div64)
    }
    ///f_spi_ker_ck / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div128)
    }
    ///f_spi_ker_ck / 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div256)
    }
}
/**BPASS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPASS {
    ///0: Bypass is disabled
    Disabled = 0,
    ///1: Bypass is enabled
    Enabled = 1,
}
impl From<BPASS> for bool {
    #[inline(always)]
    fn from(variant: BPASS) -> Self {
        variant as u8 != 0
    }
}
///Field `BPASS` reader - BPASS
pub type BPASS_R = crate::BitReader<BPASS>;
impl BPASS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BPASS {
        match self.bits {
            false => BPASS::Disabled,
            true => BPASS::Enabled,
        }
    }
    ///Bypass is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BPASS::Disabled
    }
    ///Bypass is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BPASS::Enabled
    }
}
///Field `BPASS` writer - BPASS
pub type BPASS_W<'a, REG> = crate::BitWriter<'a, REG, BPASS>;
impl<'a, REG> BPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BPASS::Disabled)
    }
    ///Bypass is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BPASS::Enabled)
    }
}
impl R {
    ///Bits 0:4 - Number of bits in at single SPI data frame
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - threshold level
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Behavior of slave transmitter at underrun condition
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Rx DMA stream enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx DMA stream enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - Length of CRC frame to be transacted and compared
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - Hardware CRC computation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 28:30 - Master baud rate
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - BPASS
    #[inline(always)]
    pub fn bpass(&self) -> BPASS_R {
        BPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("bpass", &self.bpass())
            .field("mbr", &self.mbr())
            .field("crcen", &self.crcen())
            .field("crcsize", &self.crcsize())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("udrcfg", &self.udrcfg())
            .field("fthlv", &self.fthlv())
            .field("dsize", &self.dsize())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Number of bits in at single SPI data frame
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W<CFG1rs> {
        DSIZE_W::new(self, 0)
    }
    ///Bits 5:8 - threshold level
    #[inline(always)]
    pub fn fthlv(&mut self) -> FTHLV_W<CFG1rs> {
        FTHLV_W::new(self, 5)
    }
    ///Bit 9 - Behavior of slave transmitter at underrun condition
    #[inline(always)]
    pub fn udrcfg(&mut self) -> UDRCFG_W<CFG1rs> {
        UDRCFG_W::new(self, 9)
    }
    ///Bit 14 - Rx DMA stream enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFG1rs> {
        RXDMAEN_W::new(self, 14)
    }
    ///Bit 15 - Tx DMA stream enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFG1rs> {
        TXDMAEN_W::new(self, 15)
    }
    ///Bits 16:20 - Length of CRC frame to be transacted and compared
    #[inline(always)]
    pub fn crcsize(&mut self) -> CRCSIZE_W<CFG1rs> {
        CRCSIZE_W::new(self, 16)
    }
    ///Bit 22 - Hardware CRC computation enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<CFG1rs> {
        CRCEN_W::new(self, 22)
    }
    ///Bits 28:30 - Master baud rate
    #[inline(always)]
    pub fn mbr(&mut self) -> MBR_W<CFG1rs> {
        MBR_W::new(self, 28)
    }
    ///Bit 31 - BPASS
    #[inline(always)]
    pub fn bpass(&mut self) -> BPASS_W<CFG1rs> {
        BPASS_W::new(self, 31)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SPI1:CFG1)*/
pub struct CFG1rs;
impl crate::RegisterSpec for CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`cfg1::R`](R) reader structure
impl crate::Readable for CFG1rs {}
///`write(|w| ..)` method takes [`cfg1::W`](W) writer structure
impl crate::Writable for CFG1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG1 to value 0x0007_0007
impl crate::Resettable for CFG1rs {
    const RESET_VALUE: u32 = 0x0007_0007;
}
