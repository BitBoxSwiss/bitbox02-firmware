///Register `SWTRGR` writer
pub type W = crate::W<SWTRGRrs>;
/**DAC channel%s software trigger

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG1 {
    ///0: No trigger
    NoTrigger = 0,
    ///1: Trigger
    Trigger = 1,
}
impl From<SWTRIG1> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG(1-2)` writer - DAC channel%s software trigger
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG1>;
impl<'a, REG> SWTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No trigger
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::NoTrigger)
    }
    ///Trigger
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::Trigger)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SWTRGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///DAC channel(1-2) software trigger
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SWTRIG1` field.</div>
    #[inline(always)]
    pub fn swtrig(&mut self, n: u8) -> SWTRIG_W<SWTRGRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SWTRIG_W::new(self, n)
    }
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG_W<SWTRGRrs> {
        SWTRIG_W::new(self, 0)
    }
    ///Bit 1 - DAC channel2 software trigger
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG_W<SWTRGRrs> {
        SWTRIG_W::new(self, 1)
    }
}
/**DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DAC1:SWTRGR)*/
pub struct SWTRGRrs;
impl crate::RegisterSpec for SWTRGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure
impl crate::Writable for SWTRGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWTRGR to value 0
impl crate::Resettable for SWTRGRrs {}
