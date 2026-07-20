///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRS {
    ///0: Interrupt rising edge did not occur
    DidNotOccur = 0,
    ///1: Interrupt rising edge occurred
    Occurred = 1,
}
impl From<IRS> for bool {
    #[inline(always)]
    fn from(variant: IRS) -> Self {
        variant as u8 != 0
    }
}
///Field `IRS` reader - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
pub type IRS_R = crate::BitReader<IRS>;
impl IRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRS {
        match self.bits {
            false => IRS::DidNotOccur,
            true => IRS::Occurred,
        }
    }
    ///Interrupt rising edge did not occur
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == IRS::DidNotOccur
    }
    ///Interrupt rising edge occurred
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == IRS::Occurred
    }
}
///Field `IRS` writer - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
pub type IRS_W<'a, REG> = crate::BitWriter<'a, REG, IRS>;
impl<'a, REG> IRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt rising edge did not occur
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut crate::W<REG> {
        self.variant(IRS::DidNotOccur)
    }
    ///Interrupt rising edge occurred
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(IRS::Occurred)
    }
}
/**Interrupt high-level status The flag is set by hardware and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILS {
    ///0: Interrupt high-level did not occur
    DidNotOccur = 0,
    ///1: Interrupt high-level occurred
    Occurred = 1,
}
impl From<ILS> for bool {
    #[inline(always)]
    fn from(variant: ILS) -> Self {
        variant as u8 != 0
    }
}
///Field `ILS` reader - Interrupt high-level status The flag is set by hardware and reset by software.
pub type ILS_R = crate::BitReader<ILS>;
impl ILS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILS {
        match self.bits {
            false => ILS::DidNotOccur,
            true => ILS::Occurred,
        }
    }
    ///Interrupt high-level did not occur
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == ILS::DidNotOccur
    }
    ///Interrupt high-level occurred
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ILS::Occurred
    }
}
///Field `ILS` writer - Interrupt high-level status The flag is set by hardware and reset by software.
pub type ILS_W<'a, REG> = crate::BitWriter<'a, REG, ILS>;
impl<'a, REG> ILS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt high-level did not occur
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut crate::W<REG> {
        self.variant(ILS::DidNotOccur)
    }
    ///Interrupt high-level occurred
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(ILS::Occurred)
    }
}
/**Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFS {
    ///0: Interrupt falling edge did not occur
    DidNotOccur = 0,
    ///1: Interrupt falling edge occurred
    Occurred = 1,
}
impl From<IFS> for bool {
    #[inline(always)]
    fn from(variant: IFS) -> Self {
        variant as u8 != 0
    }
}
///Field `IFS` reader - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
pub type IFS_R = crate::BitReader<IFS>;
impl IFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IFS {
        match self.bits {
            false => IFS::DidNotOccur,
            true => IFS::Occurred,
        }
    }
    ///Interrupt falling edge did not occur
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == IFS::DidNotOccur
    }
    ///Interrupt falling edge occurred
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == IFS::Occurred
    }
}
///Field `IFS` writer - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
pub type IFS_W<'a, REG> = crate::BitWriter<'a, REG, IFS>;
impl<'a, REG> IFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt falling edge did not occur
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut crate::W<REG> {
        self.variant(IFS::DidNotOccur)
    }
    ///Interrupt falling edge occurred
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(IFS::Occurred)
    }
}
/**Interrupt rising edge detection enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN {
    ///0: Interrupt rising edge detection request disabled
    Disabled = 0,
    ///1: Interrupt rising edge detection request enabled
    Enabled = 1,
}
impl From<IREN> for bool {
    #[inline(always)]
    fn from(variant: IREN) -> Self {
        variant as u8 != 0
    }
}
///Field `IREN` reader - Interrupt rising edge detection enable bit
pub type IREN_R = crate::BitReader<IREN>;
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IREN {
        match self.bits {
            false => IREN::Disabled,
            true => IREN::Enabled,
        }
    }
    ///Interrupt rising edge detection request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN::Disabled
    }
    ///Interrupt rising edge detection request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN::Enabled
    }
}
///Field `IREN` writer - Interrupt rising edge detection enable bit
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt rising edge detection request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Disabled)
    }
    ///Interrupt rising edge detection request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Enabled)
    }
}
/**Interrupt high-level detection enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILEN {
    ///0: Interrupt high-level detection request disabled
    Disabled = 0,
    ///1: Interrupt high-level detection request enabled
    Enabled = 1,
}
impl From<ILEN> for bool {
    #[inline(always)]
    fn from(variant: ILEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ILEN` reader - Interrupt high-level detection enable bit
pub type ILEN_R = crate::BitReader<ILEN>;
impl ILEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILEN {
        match self.bits {
            false => ILEN::Disabled,
            true => ILEN::Enabled,
        }
    }
    ///Interrupt high-level detection request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILEN::Disabled
    }
    ///Interrupt high-level detection request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILEN::Enabled
    }
}
///Field `ILEN` writer - Interrupt high-level detection enable bit
pub type ILEN_W<'a, REG> = crate::BitWriter<'a, REG, ILEN>;
impl<'a, REG> ILEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt high-level detection request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ILEN::Disabled)
    }
    ///Interrupt high-level detection request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ILEN::Enabled)
    }
}
/**Interrupt falling edge detection enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFEN {
    ///0: Interrupt falling edge detection request disabled
    Disabled = 0,
    ///1: Interrupt falling edge detection request enabled
    Enabled = 1,
}
impl From<IFEN> for bool {
    #[inline(always)]
    fn from(variant: IFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IFEN` reader - Interrupt falling edge detection enable bit
pub type IFEN_R = crate::BitReader<IFEN>;
impl IFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IFEN {
        match self.bits {
            false => IFEN::Disabled,
            true => IFEN::Enabled,
        }
    }
    ///Interrupt falling edge detection request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IFEN::Disabled
    }
    ///Interrupt falling edge detection request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IFEN::Enabled
    }
}
///Field `IFEN` writer - Interrupt falling edge detection enable bit
pub type IFEN_W<'a, REG> = crate::BitWriter<'a, REG, IFEN>;
impl<'a, REG> IFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt falling edge detection request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IFEN::Disabled)
    }
    ///Interrupt falling edge detection request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IFEN::Enabled)
    }
}
/**FIFO empty. Read-only bit that provides the status of the FIFO

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEMPT {
    ///0: FIFO not empty
    NotEmpty = 0,
    ///1: FIFO empty
    Empty = 1,
}
impl From<FEMPT> for bool {
    #[inline(always)]
    fn from(variant: FEMPT) -> Self {
        variant as u8 != 0
    }
}
///Field `FEMPT` reader - FIFO empty. Read-only bit that provides the status of the FIFO
pub type FEMPT_R = crate::BitReader<FEMPT>;
impl FEMPT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEMPT {
        match self.bits {
            false => FEMPT::NotEmpty,
            true => FEMPT::Empty,
        }
    }
    ///FIFO not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == FEMPT::NotEmpty
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FEMPT::Empty
    }
}
impl R {
    ///Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software.
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt rising edge detection enable bit
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt high-level detection enable bit
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt falling edge detection enable bit
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FIFO empty. Read-only bit that provides the status of the FIFO
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("irs", &self.irs())
            .field("ils", &self.ils())
            .field("ifs", &self.ifs())
            .field("iren", &self.iren())
            .field("ilen", &self.ilen())
            .field("ifen", &self.ifen())
            .field("fempt", &self.fempt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W<SRrs> {
        IRS_W::new(self, 0)
    }
    ///Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software.
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W<SRrs> {
        ILS_W::new(self, 1)
    }
    ///Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set.
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W<SRrs> {
        IFS_W::new(self, 2)
    }
    ///Bit 3 - Interrupt rising edge detection enable bit
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<SRrs> {
        IREN_W::new(self, 3)
    }
    ///Bit 4 - Interrupt high-level detection enable bit
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W<SRrs> {
        ILEN_W::new(self, 4)
    }
    ///Bit 5 - Interrupt falling edge detection enable bit
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W<SRrs> {
        IFEN_W::new(self, 5)
    }
}
/**status and interrupt register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FMC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0x40
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x40;
}
