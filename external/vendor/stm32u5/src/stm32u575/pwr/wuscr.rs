///Register `WUSCR` writer
pub type W = crate::W<WUSCRrs>;
/**Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1 {
    ///1: Clear the WUFx flag in PWR_WUSR
    Clear = 1,
}
impl From<CWUF1> for bool {
    #[inline(always)]
    fn from(variant: CWUF1) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF1` writer - Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the WUFx flag in PWR_WUSR
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1::Clear)
    }
}
///Field `CWUF2` writer - Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
pub use CWUF1_W as CWUF2_W;
///Field `CWUF3` writer - Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
pub use CWUF1_W as CWUF3_W;
///Field `CWUF4` writer - Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
pub use CWUF1_W as CWUF4_W;
///Field `CWUF5` writer - Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
pub use CWUF1_W as CWUF5_W;
///Field `CWUF6` writer - Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
pub use CWUF1_W as CWUF6_W;
///Field `CWUF7` writer - Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
pub use CWUF1_W as CWUF7_W;
///Field `CWUF8` writer - Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
pub use CWUF1_W as CWUF8_W;
impl core::fmt::Debug for crate::generic::Reg<WUSCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<WUSCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<WUSCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<WUSCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W<WUSCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W<WUSCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 5 - Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf6(&mut self) -> CWUF6_W<WUSCRrs> {
        CWUF6_W::new(self, 5)
    }
    ///Bit 6 - Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf7(&mut self) -> CWUF7_W<WUSCRrs> {
        CWUF7_W::new(self, 6)
    }
    ///Bit 7 - Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf8(&mut self) -> CWUF8_W<WUSCRrs> {
        CWUF8_W::new(self, 7)
    }
}
/**PWR wakeup status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:WUSCR)*/
pub struct WUSCRrs;
impl crate::RegisterSpec for WUSCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wuscr::W`](W) writer structure
impl crate::Writable for WUSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUSCR to value 0
impl crate::Resettable for WUSCRrs {}
