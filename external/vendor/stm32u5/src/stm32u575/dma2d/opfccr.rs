///Register `OPFCCR` reader
pub type R = crate::R<OPFCCRrs>;
///Register `OPFCCR` writer
pub type W = crate::W<OPFCCRrs>;
/**Color mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM {
    ///0: ARGB8888
    Argb8888 = 0,
    ///1: RGB888
    Rgb888 = 1,
    ///2: RGB565
    Rgb565 = 2,
    ///3: ARGB1555
    Argb1555 = 3,
    ///4: ARGB4444
    Argb4444 = 4,
}
impl From<CM> for u8 {
    #[inline(always)]
    fn from(variant: CM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM {
    type Ux = u8;
}
impl crate::IsEnum for CM {}
///Field `CM` reader - Color mode
pub type CM_R = crate::FieldReader<CM>;
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CM> {
        match self.bits {
            0 => Some(CM::Argb8888),
            1 => Some(CM::Rgb888),
            2 => Some(CM::Rgb565),
            3 => Some(CM::Argb1555),
            4 => Some(CM::Argb4444),
            _ => None,
        }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM::Argb8888
    }
    ///RGB888
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM::Rgb888
    }
    ///RGB565
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM::Rgb565
    }
    ///ARGB1555
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM::Argb1555
    }
    ///ARGB4444
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM::Argb4444
    }
}
///Field `CM` writer - Color mode
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CM>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb4444)
    }
}
/**Swap Bytes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SB {
    ///0: Regular byte order
    Regular = 0,
    ///1: Bytes are swapped two by two
    SwapBytes = 1,
}
impl From<SB> for bool {
    #[inline(always)]
    fn from(variant: SB) -> Self {
        variant as u8 != 0
    }
}
///Field `SB` reader - Swap Bytes
pub type SB_R = crate::BitReader<SB>;
impl SB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SB {
        match self.bits {
            false => SB::Regular,
            true => SB::SwapBytes,
        }
    }
    ///Regular byte order
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == SB::Regular
    }
    ///Bytes are swapped two by two
    #[inline(always)]
    pub fn is_swap_bytes(&self) -> bool {
        *self == SB::SwapBytes
    }
}
///Field `SB` writer - Swap Bytes
pub type SB_W<'a, REG> = crate::BitWriter<'a, REG, SB>;
impl<'a, REG> SB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular byte order
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(SB::Regular)
    }
    ///Bytes are swapped two by two
    #[inline(always)]
    pub fn swap_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SB::SwapBytes)
    }
}
/**Alpha Inverted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI {
    ///0: Regular alpha
    RegularAlpha = 0,
    ///1: Inverted alpha
    InvertedAlpha = 1,
}
impl From<AI> for bool {
    #[inline(always)]
    fn from(variant: AI) -> Self {
        variant as u8 != 0
    }
}
///Field `AI` reader - Alpha Inverted
pub type AI_R = crate::BitReader<AI>;
impl AI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AI {
        match self.bits {
            false => AI::RegularAlpha,
            true => AI::InvertedAlpha,
        }
    }
    ///Regular alpha
    #[inline(always)]
    pub fn is_regular_alpha(&self) -> bool {
        *self == AI::RegularAlpha
    }
    ///Inverted alpha
    #[inline(always)]
    pub fn is_inverted_alpha(&self) -> bool {
        *self == AI::InvertedAlpha
    }
}
///Field `AI` writer - Alpha Inverted
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG, AI>;
impl<'a, REG> AI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular alpha
    #[inline(always)]
    pub fn regular_alpha(self) -> &'a mut crate::W<REG> {
        self.variant(AI::RegularAlpha)
    }
    ///Inverted alpha
    #[inline(always)]
    pub fn inverted_alpha(self) -> &'a mut crate::W<REG> {
        self.variant(AI::InvertedAlpha)
    }
}
/**Red Blue Swap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBS {
    ///0: No Red Blue Swap (RGB or ARGB)
    Regular = 0,
    ///1: Red Blue Swap (BGR or ABGR)
    Swap = 1,
}
impl From<RBS> for bool {
    #[inline(always)]
    fn from(variant: RBS) -> Self {
        variant as u8 != 0
    }
}
///Field `RBS` reader - Red Blue Swap
pub type RBS_R = crate::BitReader<RBS>;
impl RBS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RBS {
        match self.bits {
            false => RBS::Regular,
            true => RBS::Swap,
        }
    }
    ///No Red Blue Swap (RGB or ARGB)
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == RBS::Regular
    }
    ///Red Blue Swap (BGR or ABGR)
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == RBS::Swap
    }
}
///Field `RBS` writer - Red Blue Swap
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG, RBS>;
impl<'a, REG> RBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Red Blue Swap (RGB or ARGB)
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(RBS::Regular)
    }
    ///Red Blue Swap (BGR or ABGR)
    #[inline(always)]
    pub fn swap(self) -> &'a mut crate::W<REG> {
        self.variant(RBS::Swap)
    }
}
impl R {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
    ///Bit 9 - Swap Bytes
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Red Blue Swap
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPFCCR")
            .field("rbs", &self.rbs())
            .field("ai", &self.ai())
            .field("sb", &self.sb())
            .field("cm", &self.cm())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<OPFCCRrs> {
        CM_W::new(self, 0)
    }
    ///Bit 9 - Swap Bytes
    #[inline(always)]
    pub fn sb(&mut self) -> SB_W<OPFCCRrs> {
        SB_W::new(self, 9)
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W<OPFCCRrs> {
        AI_W::new(self, 20)
    }
    ///Bit 21 - Red Blue Swap
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W<OPFCCRrs> {
        RBS_W::new(self, 21)
    }
}
/**output PFC control register

You can [`read`](crate::Reg::read) this register and get [`opfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DMA2D:OPFCCR)*/
pub struct OPFCCRrs;
impl crate::RegisterSpec for OPFCCRrs {
    type Ux = u32;
}
///`read()` method returns [`opfccr::R`](R) reader structure
impl crate::Readable for OPFCCRrs {}
///`write(|w| ..)` method takes [`opfccr::W`](W) writer structure
impl crate::Writable for OPFCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPFCCR to value 0
impl crate::Resettable for OPFCCRrs {}
