///Register `WPCCR` reader
pub type R = crate::R<WPCCRrs>;
///Register `WPCCR` writer
pub type W = crate::W<WPCCRrs>;
/**Instruction mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IMODE {
    ///0: No instruction
    NoInstruction = 0,
    ///1: Instruction on a single line
    SingleLine = 1,
    ///2: Instruction on two lines
    TwoLines = 2,
    ///3: Instruction on four lines
    FourLines = 3,
    ///4: Instruction on eight lines
    EightLines = 4,
}
impl From<IMODE> for u8 {
    #[inline(always)]
    fn from(variant: IMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IMODE {
    type Ux = u8;
}
impl crate::IsEnum for IMODE {}
///Field `IMODE` reader - Instruction mode
pub type IMODE_R = crate::FieldReader<IMODE>;
impl IMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IMODE> {
        match self.bits {
            0 => Some(IMODE::NoInstruction),
            1 => Some(IMODE::SingleLine),
            2 => Some(IMODE::TwoLines),
            3 => Some(IMODE::FourLines),
            4 => Some(IMODE::EightLines),
            _ => None,
        }
    }
    ///No instruction
    #[inline(always)]
    pub fn is_no_instruction(&self) -> bool {
        *self == IMODE::NoInstruction
    }
    ///Instruction on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == IMODE::SingleLine
    }
    ///Instruction on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == IMODE::TwoLines
    }
    ///Instruction on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == IMODE::FourLines
    }
    ///Instruction on eight lines
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == IMODE::EightLines
    }
}
///Field `IMODE` writer - Instruction mode
pub type IMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, IMODE>;
impl<'a, REG> IMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No instruction
    #[inline(always)]
    pub fn no_instruction(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::NoInstruction)
    }
    ///Instruction on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::SingleLine)
    }
    ///Instruction on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::TwoLines)
    }
    ///Instruction on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::FourLines)
    }
    ///Instruction on eight lines
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::EightLines)
    }
}
/**Instruction double transfer rate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDTR {
    ///0: DTR mode disabled for instruction phase
    Disabled = 0,
    ///1: DTR mode enabled for instruction phase
    Enabled = 1,
}
impl From<IDTR> for bool {
    #[inline(always)]
    fn from(variant: IDTR) -> Self {
        variant as u8 != 0
    }
}
///Field `IDTR` reader - Instruction double transfer rate
pub type IDTR_R = crate::BitReader<IDTR>;
impl IDTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDTR {
        match self.bits {
            false => IDTR::Disabled,
            true => IDTR::Enabled,
        }
    }
    ///DTR mode disabled for instruction phase
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDTR::Disabled
    }
    ///DTR mode enabled for instruction phase
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDTR::Enabled
    }
}
///Field `IDTR` writer - Instruction double transfer rate
pub type IDTR_W<'a, REG> = crate::BitWriter<'a, REG, IDTR>;
impl<'a, REG> IDTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTR mode disabled for instruction phase
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDTR::Disabled)
    }
    ///DTR mode enabled for instruction phase
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDTR::Enabled)
    }
}
/**Instruction size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISIZE {
    ///0: 8-bit instruction
    Bits8 = 0,
    ///1: 16-bit instruction
    Bits16 = 1,
    ///2: 24-bit instruction
    Bits24 = 2,
    ///3: 32-bit instruction
    Bits32 = 3,
}
impl From<ISIZE> for u8 {
    #[inline(always)]
    fn from(variant: ISIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISIZE {
    type Ux = u8;
}
impl crate::IsEnum for ISIZE {}
///Field `ISIZE` reader - Instruction size
pub type ISIZE_R = crate::FieldReader<ISIZE>;
impl ISIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ISIZE {
        match self.bits {
            0 => ISIZE::Bits8,
            1 => ISIZE::Bits16,
            2 => ISIZE::Bits24,
            3 => ISIZE::Bits32,
            _ => unreachable!(),
        }
    }
    ///8-bit instruction
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == ISIZE::Bits8
    }
    ///16-bit instruction
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ISIZE::Bits16
    }
    ///24-bit instruction
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == ISIZE::Bits24
    }
    ///32-bit instruction
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ISIZE::Bits32
    }
}
///Field `ISIZE` writer - Instruction size
pub type ISIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISIZE, crate::Safe>;
impl<'a, REG> ISIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit instruction
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(ISIZE::Bits8)
    }
    ///16-bit instruction
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(ISIZE::Bits16)
    }
    ///24-bit instruction
    #[inline(always)]
    pub fn bits24(self) -> &'a mut crate::W<REG> {
        self.variant(ISIZE::Bits24)
    }
    ///32-bit instruction
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(ISIZE::Bits32)
    }
}
/**Address mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADMODE {
    ///0: No address
    NoAddress = 0,
    ///1: Address on a single line
    SingleLine = 1,
    ///2: Address on two lines
    TwoLines = 2,
    ///3: Address on four lines
    FourLines = 3,
    ///4: Address on eight lines
    EightLines = 4,
}
impl From<ADMODE> for u8 {
    #[inline(always)]
    fn from(variant: ADMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADMODE {
    type Ux = u8;
}
impl crate::IsEnum for ADMODE {}
///Field `ADMODE` reader - Address mode
pub type ADMODE_R = crate::FieldReader<ADMODE>;
impl ADMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADMODE> {
        match self.bits {
            0 => Some(ADMODE::NoAddress),
            1 => Some(ADMODE::SingleLine),
            2 => Some(ADMODE::TwoLines),
            3 => Some(ADMODE::FourLines),
            4 => Some(ADMODE::EightLines),
            _ => None,
        }
    }
    ///No address
    #[inline(always)]
    pub fn is_no_address(&self) -> bool {
        *self == ADMODE::NoAddress
    }
    ///Address on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == ADMODE::SingleLine
    }
    ///Address on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == ADMODE::TwoLines
    }
    ///Address on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == ADMODE::FourLines
    }
    ///Address on eight lines
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == ADMODE::EightLines
    }
}
///Field `ADMODE` writer - Address mode
pub type ADMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADMODE>;
impl<'a, REG> ADMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No address
    #[inline(always)]
    pub fn no_address(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::NoAddress)
    }
    ///Address on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::SingleLine)
    }
    ///Address on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::TwoLines)
    }
    ///Address on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::FourLines)
    }
    ///Address on eight lines
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::EightLines)
    }
}
/**Address double transfer rate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDTR {
    ///0: DTR mode disabled for address phase
    Disabled = 0,
    ///1: DTR mode enabled for address phase
    Enabled = 1,
}
impl From<ADDTR> for bool {
    #[inline(always)]
    fn from(variant: ADDTR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDTR` reader - Address double transfer rate
pub type ADDTR_R = crate::BitReader<ADDTR>;
impl ADDTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDTR {
        match self.bits {
            false => ADDTR::Disabled,
            true => ADDTR::Enabled,
        }
    }
    ///DTR mode disabled for address phase
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDTR::Disabled
    }
    ///DTR mode enabled for address phase
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDTR::Enabled
    }
}
///Field `ADDTR` writer - Address double transfer rate
pub type ADDTR_W<'a, REG> = crate::BitWriter<'a, REG, ADDTR>;
impl<'a, REG> ADDTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTR mode disabled for address phase
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDTR::Disabled)
    }
    ///DTR mode enabled for address phase
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDTR::Enabled)
    }
}
/**Address size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADSIZE {
    ///0: 8-bit address
    Bits8 = 0,
    ///1: 16-bit address
    Bits16 = 1,
    ///2: 24-bit address
    Bits24 = 2,
    ///3: 32-bit address
    Bits32 = 3,
}
impl From<ADSIZE> for u8 {
    #[inline(always)]
    fn from(variant: ADSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADSIZE {
    type Ux = u8;
}
impl crate::IsEnum for ADSIZE {}
///Field `ADSIZE` reader - Address size
pub type ADSIZE_R = crate::FieldReader<ADSIZE>;
impl ADSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSIZE {
        match self.bits {
            0 => ADSIZE::Bits8,
            1 => ADSIZE::Bits16,
            2 => ADSIZE::Bits24,
            3 => ADSIZE::Bits32,
            _ => unreachable!(),
        }
    }
    ///8-bit address
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == ADSIZE::Bits8
    }
    ///16-bit address
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ADSIZE::Bits16
    }
    ///24-bit address
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == ADSIZE::Bits24
    }
    ///32-bit address
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ADSIZE::Bits32
    }
}
///Field `ADSIZE` writer - Address size
pub type ADSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADSIZE, crate::Safe>;
impl<'a, REG> ADSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit address
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bits8)
    }
    ///16-bit address
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bits16)
    }
    ///24-bit address
    #[inline(always)]
    pub fn bits24(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bits24)
    }
    ///32-bit address
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bits32)
    }
}
/**Alternate byte mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABMODE {
    ///0: No alternate bytes
    NoAlternateBytes = 0,
    ///1: Alternate bytes on a single line
    SingleLine = 1,
    ///2: Alternate bytes on two lines
    TwoLines = 2,
    ///3: Alternate bytes on four lines
    FourLines = 3,
    ///4: Alternate bytes on eight lines
    EightLines = 4,
}
impl From<ABMODE> for u8 {
    #[inline(always)]
    fn from(variant: ABMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABMODE {
    type Ux = u8;
}
impl crate::IsEnum for ABMODE {}
///Field `ABMODE` reader - Alternate byte mode
pub type ABMODE_R = crate::FieldReader<ABMODE>;
impl ABMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ABMODE> {
        match self.bits {
            0 => Some(ABMODE::NoAlternateBytes),
            1 => Some(ABMODE::SingleLine),
            2 => Some(ABMODE::TwoLines),
            3 => Some(ABMODE::FourLines),
            4 => Some(ABMODE::EightLines),
            _ => None,
        }
    }
    ///No alternate bytes
    #[inline(always)]
    pub fn is_no_alternate_bytes(&self) -> bool {
        *self == ABMODE::NoAlternateBytes
    }
    ///Alternate bytes on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == ABMODE::SingleLine
    }
    ///Alternate bytes on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == ABMODE::TwoLines
    }
    ///Alternate bytes on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == ABMODE::FourLines
    }
    ///Alternate bytes on eight lines
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == ABMODE::EightLines
    }
}
///Field `ABMODE` writer - Alternate byte mode
pub type ABMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ABMODE>;
impl<'a, REG> ABMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No alternate bytes
    #[inline(always)]
    pub fn no_alternate_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::NoAlternateBytes)
    }
    ///Alternate bytes on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::SingleLine)
    }
    ///Alternate bytes on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::TwoLines)
    }
    ///Alternate bytes on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::FourLines)
    }
    ///Alternate bytes on eight lines
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::EightLines)
    }
}
/**Alternate bytes double transfer rate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABDTR {
    ///0: DTR mode disabled for alternate bytes phase
    Disabled = 0,
    ///1: DTR mode enabled for alternate bytes phase
    Enabled = 1,
}
impl From<ABDTR> for bool {
    #[inline(always)]
    fn from(variant: ABDTR) -> Self {
        variant as u8 != 0
    }
}
///Field `ABDTR` reader - Alternate bytes double transfer rate
pub type ABDTR_R = crate::BitReader<ABDTR>;
impl ABDTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABDTR {
        match self.bits {
            false => ABDTR::Disabled,
            true => ABDTR::Enabled,
        }
    }
    ///DTR mode disabled for alternate bytes phase
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABDTR::Disabled
    }
    ///DTR mode enabled for alternate bytes phase
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABDTR::Enabled
    }
}
///Field `ABDTR` writer - Alternate bytes double transfer rate
pub type ABDTR_W<'a, REG> = crate::BitWriter<'a, REG, ABDTR>;
impl<'a, REG> ABDTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTR mode disabled for alternate bytes phase
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABDTR::Disabled)
    }
    ///DTR mode enabled for alternate bytes phase
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABDTR::Enabled)
    }
}
/**Alternate bytes size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABSIZE {
    ///0: 8-bit alternate bytes
    Bits8 = 0,
    ///1: 16-bit alternate bytes
    Bits16 = 1,
    ///2: 24-bit alternate bytes
    Bits24 = 2,
    ///3: 32-bit alternate bytes
    Bits32 = 3,
}
impl From<ABSIZE> for u8 {
    #[inline(always)]
    fn from(variant: ABSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABSIZE {
    type Ux = u8;
}
impl crate::IsEnum for ABSIZE {}
///Field `ABSIZE` reader - Alternate bytes size
pub type ABSIZE_R = crate::FieldReader<ABSIZE>;
impl ABSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABSIZE {
        match self.bits {
            0 => ABSIZE::Bits8,
            1 => ABSIZE::Bits16,
            2 => ABSIZE::Bits24,
            3 => ABSIZE::Bits32,
            _ => unreachable!(),
        }
    }
    ///8-bit alternate bytes
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == ABSIZE::Bits8
    }
    ///16-bit alternate bytes
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ABSIZE::Bits16
    }
    ///24-bit alternate bytes
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == ABSIZE::Bits24
    }
    ///32-bit alternate bytes
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ABSIZE::Bits32
    }
}
///Field `ABSIZE` writer - Alternate bytes size
pub type ABSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ABSIZE, crate::Safe>;
impl<'a, REG> ABSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit alternate bytes
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bits8)
    }
    ///16-bit alternate bytes
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bits16)
    }
    ///24-bit alternate bytes
    #[inline(always)]
    pub fn bits24(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bits24)
    }
    ///32-bit alternate bytes
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bits32)
    }
}
/**Data mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMODE {
    ///0: No data
    NoData = 0,
    ///1: Data on a single line
    SingleLine = 1,
    ///2: Data on two lines
    TwoLines = 2,
    ///3: Data on four lines
    FourLines = 3,
    ///4: Data on eight lines
    EightLines = 4,
}
impl From<DMODE> for u8 {
    #[inline(always)]
    fn from(variant: DMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMODE {
    type Ux = u8;
}
impl crate::IsEnum for DMODE {}
///Field `DMODE` reader - Data mode
pub type DMODE_R = crate::FieldReader<DMODE>;
impl DMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMODE> {
        match self.bits {
            0 => Some(DMODE::NoData),
            1 => Some(DMODE::SingleLine),
            2 => Some(DMODE::TwoLines),
            3 => Some(DMODE::FourLines),
            4 => Some(DMODE::EightLines),
            _ => None,
        }
    }
    ///No data
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == DMODE::NoData
    }
    ///Data on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == DMODE::SingleLine
    }
    ///Data on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == DMODE::TwoLines
    }
    ///Data on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == DMODE::FourLines
    }
    ///Data on eight lines
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == DMODE::EightLines
    }
}
///Field `DMODE` writer - Data mode
pub type DMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DMODE>;
impl<'a, REG> DMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No data
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::NoData)
    }
    ///Data on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::SingleLine)
    }
    ///Data on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::TwoLines)
    }
    ///Data on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::FourLines)
    }
    ///Data on eight lines
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::EightLines)
    }
}
/**alternate bytes double transfer rate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDTR {
    ///0: DTR mode disabled for data phase
    Disabled = 0,
    ///1: DTR mode enabled for data phase
    Enabled = 1,
}
impl From<DDTR> for bool {
    #[inline(always)]
    fn from(variant: DDTR) -> Self {
        variant as u8 != 0
    }
}
///Field `DDTR` reader - alternate bytes double transfer rate
pub type DDTR_R = crate::BitReader<DDTR>;
impl DDTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDTR {
        match self.bits {
            false => DDTR::Disabled,
            true => DDTR::Enabled,
        }
    }
    ///DTR mode disabled for data phase
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDTR::Disabled
    }
    ///DTR mode enabled for data phase
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDTR::Enabled
    }
}
///Field `DDTR` writer - alternate bytes double transfer rate
pub type DDTR_W<'a, REG> = crate::BitWriter<'a, REG, DDTR>;
impl<'a, REG> DDTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTR mode disabled for data phase
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDTR::Disabled)
    }
    ///DTR mode enabled for data phase
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDTR::Enabled)
    }
}
/**DQS enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DQSE {
    ///0: DQS disabled
    Disabled = 0,
    ///1: DQS enabled
    Enabled = 1,
}
impl From<DQSE> for bool {
    #[inline(always)]
    fn from(variant: DQSE) -> Self {
        variant as u8 != 0
    }
}
///Field `DQSE` reader - DQS enable
pub type DQSE_R = crate::BitReader<DQSE>;
impl DQSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DQSE {
        match self.bits {
            false => DQSE::Disabled,
            true => DQSE::Enabled,
        }
    }
    ///DQS disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DQSE::Disabled
    }
    ///DQS enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DQSE::Enabled
    }
}
///Field `DQSE` writer - DQS enable
pub type DQSE_W<'a, REG> = crate::BitWriter<'a, REG, DQSE>;
impl<'a, REG> DQSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DQS disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DQSE::Disabled)
    }
    ///DQS enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DQSE::Enabled)
    }
}
impl R {
    ///Bits 0:2 - Instruction mode
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Instruction double transfer rate
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Instruction size
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:10 - Address mode
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - Address double transfer rate
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Address size
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - Alternate byte mode
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Alternate bytes double transfer rate
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Alternate bytes size
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:26 - Data mode
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - alternate bytes double transfer rate
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DQS enable
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCCR")
            .field("imode", &self.imode())
            .field("idtr", &self.idtr())
            .field("isize", &self.isize())
            .field("admode", &self.admode())
            .field("addtr", &self.addtr())
            .field("abmode", &self.abmode())
            .field("abdtr", &self.abdtr())
            .field("absize", &self.absize())
            .field("dmode", &self.dmode())
            .field("ddtr", &self.ddtr())
            .field("dqse", &self.dqse())
            .field("adsize", &self.adsize())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Instruction mode
    #[inline(always)]
    pub fn imode(&mut self) -> IMODE_W<WPCCRrs> {
        IMODE_W::new(self, 0)
    }
    ///Bit 3 - Instruction double transfer rate
    #[inline(always)]
    pub fn idtr(&mut self) -> IDTR_W<WPCCRrs> {
        IDTR_W::new(self, 3)
    }
    ///Bits 4:5 - Instruction size
    #[inline(always)]
    pub fn isize(&mut self) -> ISIZE_W<WPCCRrs> {
        ISIZE_W::new(self, 4)
    }
    ///Bits 8:10 - Address mode
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W<WPCCRrs> {
        ADMODE_W::new(self, 8)
    }
    ///Bit 11 - Address double transfer rate
    #[inline(always)]
    pub fn addtr(&mut self) -> ADDTR_W<WPCCRrs> {
        ADDTR_W::new(self, 11)
    }
    ///Bits 12:13 - Address size
    #[inline(always)]
    pub fn adsize(&mut self) -> ADSIZE_W<WPCCRrs> {
        ADSIZE_W::new(self, 12)
    }
    ///Bits 16:18 - Alternate byte mode
    #[inline(always)]
    pub fn abmode(&mut self) -> ABMODE_W<WPCCRrs> {
        ABMODE_W::new(self, 16)
    }
    ///Bit 19 - Alternate bytes double transfer rate
    #[inline(always)]
    pub fn abdtr(&mut self) -> ABDTR_W<WPCCRrs> {
        ABDTR_W::new(self, 19)
    }
    ///Bits 20:21 - Alternate bytes size
    #[inline(always)]
    pub fn absize(&mut self) -> ABSIZE_W<WPCCRrs> {
        ABSIZE_W::new(self, 20)
    }
    ///Bits 24:26 - Data mode
    #[inline(always)]
    pub fn dmode(&mut self) -> DMODE_W<WPCCRrs> {
        DMODE_W::new(self, 24)
    }
    ///Bit 27 - alternate bytes double transfer rate
    #[inline(always)]
    pub fn ddtr(&mut self) -> DDTR_W<WPCCRrs> {
        DDTR_W::new(self, 27)
    }
    ///Bit 29 - DQS enable
    #[inline(always)]
    pub fn dqse(&mut self) -> DQSE_W<WPCCRrs> {
        DQSE_W::new(self, 29)
    }
}
/**wrap communication configuration register

You can [`read`](crate::Reg::read) this register and get [`wpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OCTOSPI1:WPCCR)*/
pub struct WPCCRrs;
impl crate::RegisterSpec for WPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`wpccr::R`](R) reader structure
impl crate::Readable for WPCCRrs {}
///`write(|w| ..)` method takes [`wpccr::W`](W) writer structure
impl crate::Writable for WPCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCCR to value 0
impl crate::Resettable for WPCCRrs {}
