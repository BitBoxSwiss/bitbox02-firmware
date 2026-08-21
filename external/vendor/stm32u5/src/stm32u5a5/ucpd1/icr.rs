///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**TXMSGDISCCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMSGDISCCFW {
    ///1: Clear flag in UCPD_SR
    Clear = 1,
}
impl From<TXMSGDISCCFW> for bool {
    #[inline(always)]
    fn from(variant: TXMSGDISCCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGDISCCF` writer - TXMSGDISCCF
pub type TXMSGDISCCF_W<'a, REG> = crate::BitWriter<'a, REG, TXMSGDISCCFW>;
impl<'a, REG> TXMSGDISCCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag in UCPD_SR
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGDISCCFW::Clear)
    }
}
///Field `TXMSGSENTCF` writer - TXMSGSENTCF
pub use TXMSGDISCCF_W as TXMSGSENTCF_W;
///Field `TXMSGABTCF` writer - TXMSGABTCF
pub use TXMSGDISCCF_W as TXMSGABTCF_W;
///Field `HRSTDISCCF` writer - HRSTDISCCF
pub use TXMSGDISCCF_W as HRSTDISCCF_W;
///Field `HRSTSENTCF` writer - HRSTSENTCF
pub use TXMSGDISCCF_W as HRSTSENTCF_W;
///Field `TXUNDCF` writer - TXUNDCF
pub use TXMSGDISCCF_W as TXUNDCF_W;
///Field `RXORDDETCF` writer - RXORDDETCF
pub use TXMSGDISCCF_W as RXORDDETCF_W;
///Field `RXHRSTDETCF` writer - RXHRSTDETCF
pub use TXMSGDISCCF_W as RXHRSTDETCF_W;
///Field `RXOVRCF` writer - RXOVRCF
pub use TXMSGDISCCF_W as RXOVRCF_W;
///Field `RXMSGENDCF` writer - RXMSGENDCF
pub use TXMSGDISCCF_W as RXMSGENDCF_W;
///Field `TYPECEVT1CF` writer - TYPECEVT1CF
pub use TXMSGDISCCF_W as TYPECEVT1CF_W;
///Field `TYPECEVT2CF` writer - TYPECEVT2CF
pub use TXMSGDISCCF_W as TYPECEVT2CF_W;
///Field `FRSEVTCF` writer - FRSEVTCF
pub use TXMSGDISCCF_W as FRSEVTCF_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - TXMSGDISCCF
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENTCF
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABTCF
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISCCF
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENTCF
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    ///Bit 6 - TXUNDCF
    #[inline(always)]
    pub fn txundcf(&mut self) -> TXUNDCF_W<ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    ///Bit 9 - RXORDDETCF
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDETCF
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    ///Bit 11 - RXOVRCF
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    ///Bit 12 - RXMSGENDCF
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    ///Bit 14 - TYPECEVT1CF
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2CF
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    ///Bit 20 - FRSEVTCF
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
/**UCPD Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#UCPD1:ICR)*/
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
