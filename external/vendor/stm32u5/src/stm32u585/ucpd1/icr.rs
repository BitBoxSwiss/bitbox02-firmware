///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register.

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
///Field `TXMSGDISCCF` writer - Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register.
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
///Field `TXMSGSENTCF` writer - Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as TXMSGSENTCF_W;
///Field `TXMSGABTCF` writer - Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as TXMSGABTCF_W;
///Field `HRSTDISCCF` writer - Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as HRSTDISCCF_W;
///Field `HRSTSENTCF` writer - Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as HRSTSENTCF_W;
///Field `TXUNDCF` writer - Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as TXUNDCF_W;
///Field `RXORDDETCF` writer - Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as RXORDDETCF_W;
///Field `RXHRSTDETCF` writer - Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as RXHRSTDETCF_W;
///Field `RXOVRCF` writer - Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as RXOVRCF_W;
///Field `RXMSGENDCF` writer - Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as RXMSGENDCF_W;
///Field `TYPECEVT1CF` writer - Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register
pub use TXMSGDISCCF_W as TYPECEVT1CF_W;
///Field `TYPECEVT2CF` writer - Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register
pub use TXMSGDISCCF_W as TYPECEVT2CF_W;
///Field `FRSEVTCF` writer - FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the UCPD_SR register.
pub use TXMSGDISCCF_W as FRSEVTCF_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register.
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    ///Bit 2 - Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the UCPD_SR register.
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    ///Bit 3 - Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the UCPD_SR register.
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    ///Bit 4 - Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the UCPD_SR register.
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    ///Bit 5 - Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the UCPD_SR register.
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    ///Bit 6 - Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the UCPD_SR register.
    #[inline(always)]
    pub fn txundcf(&mut self) -> TXUNDCF_W<ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    ///Bit 9 - Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the UCPD_SR register.
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    ///Bit 10 - Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the UCPD_SR register.
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    ///Bit 11 - Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the UCPD_SR register.
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    ///Bit 12 - Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the UCPD_SR register.
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    ///Bit 14 - Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    ///Bit 15 - Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    ///Bit 20 - FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the UCPD_SR register.
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
/**UCPD interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#UCPD1:ICR)*/
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
