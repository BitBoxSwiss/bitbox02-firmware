///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `PECF` writer - Parity error clear flag
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECF` writer - Framing error clear flag
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NECF` writer - Noise detected clear flag
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORECF` writer - Overrun error clear flag
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLECF` writer - Idle line detected clear flag
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` writer - Transmission complete clear flag
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - CTS clear flag
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMCF` writer - Character match clear flag
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn necf(&mut self) -> NECF_W<ICRrs> {
        NECF_W::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<ICRrs> {
        CMCF_W::new(self, 17)
    }
}
/**Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPUART1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
