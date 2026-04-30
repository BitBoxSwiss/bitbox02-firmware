///Register `ICR_output` writer
pub type W = crate::W<ICR_OUTPUTrs>;
///Field `CC1IF` writer - Capture/compare 1 clear flag
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMCF` writer - Autoreload match Clear Flag
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP1OKCF` writer - Compare register 1 update OK Clear Flag
pub type CMP1OKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKCF` writer - Autoreload register update OK Clear Flag
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPCF` writer - Direction change to UP Clear Flag
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNCF` writer - Direction change to down Clear Flag
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UECF` writer - Update event clear flag
pub type UECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKCF` writer - Repetition register update OK clear flag
pub type REPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2CF` writer - Capture/compare 2 clear flag
pub type CC2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP2OKCF` writer - Compare register 2 update OK clear flag
pub type CMP2OKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIEROKCF` writer - Interrupt enable register update OK clear flag
pub type DIEROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR_OUTPUTrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 clear flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<ICR_OUTPUTrs> {
        CC1IF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Clear Flag
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<ICR_OUTPUTrs> {
        ARRMCF_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Clear Flag
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<ICR_OUTPUTrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    ///Bit 3 - Compare register 1 update OK Clear Flag
    #[inline(always)]
    pub fn cmp1okcf(&mut self) -> CMP1OKCF_W<ICR_OUTPUTrs> {
        CMP1OKCF_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK Clear Flag
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<ICR_OUTPUTrs> {
        ARROKCF_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Clear Flag
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<ICR_OUTPUTrs> {
        UPCF_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Clear Flag
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<ICR_OUTPUTrs> {
        DOWNCF_W::new(self, 6)
    }
    ///Bit 7 - Update event clear flag
    #[inline(always)]
    pub fn uecf(&mut self) -> UECF_W<ICR_OUTPUTrs> {
        UECF_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK clear flag
    #[inline(always)]
    pub fn repokcf(&mut self) -> REPOKCF_W<ICR_OUTPUTrs> {
        REPOKCF_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 clear flag
    #[inline(always)]
    pub fn cc2cf(&mut self) -> CC2CF_W<ICR_OUTPUTrs> {
        CC2CF_W::new(self, 9)
    }
    ///Bit 19 - Compare register 2 update OK clear flag
    #[inline(always)]
    pub fn cmp2okcf(&mut self) -> CMP2OKCF_W<ICR_OUTPUTrs> {
        CMP2OKCF_W::new(self, 19)
    }
    ///Bit 24 - Interrupt enable register update OK clear flag
    #[inline(always)]
    pub fn dierokcf(&mut self) -> DIEROKCF_W<ICR_OUTPUTrs> {
        DIEROKCF_W::new(self, 24)
    }
}
/**Interrupt Clear Register (output mode)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LPTIM1:ICR_output)*/
pub struct ICR_OUTPUTrs;
impl crate::RegisterSpec for ICR_OUTPUTrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr_output::W`](W) writer structure
impl crate::Writable for ICR_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR_output to value 0
impl crate::Resettable for ICR_OUTPUTrs {}
