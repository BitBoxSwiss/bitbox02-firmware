///Register `SHIFTR` writer
pub type W = crate::W<SHIFTRrs>;
///Field `SUBFS` writer - Subtract a fraction of a second
pub type SUBFS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16, crate::Safe>;
/**Add one second

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1SW {
    ///1: Add one second to the clock/calendar
    Add1 = 1,
}
impl From<ADD1SW> for bool {
    #[inline(always)]
    fn from(variant: ADD1SW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1S` writer - Add one second
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG, ADD1SW>;
impl<'a, REG> ADD1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Add one second to the clock/calendar
    #[inline(always)]
    pub fn add1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1SW::Add1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SHIFTRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:14 - Subtract a fraction of a second
    #[inline(always)]
    pub fn subfs(&mut self) -> SUBFS_W<SHIFTRrs> {
        SUBFS_W::new(self, 0)
    }
    ///Bit 31 - Add one second
    #[inline(always)]
    pub fn add1s(&mut self) -> ADD1S_W<SHIFTRrs> {
        ADD1S_W::new(self, 31)
    }
}
/**shift control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RTC:SHIFTR)*/
pub struct SHIFTRrs;
impl crate::RegisterSpec for SHIFTRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`shiftr::W`](W) writer structure
impl crate::Writable for SHIFTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHIFTR to value 0
impl crate::Resettable for SHIFTRrs {}
