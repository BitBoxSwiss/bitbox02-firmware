///Register `RQR` writer
pub type W = crate::W<RQRrs>;
/**Auto baud rate request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABRRQ {
    ///1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame
    Request = 1,
}
impl From<ABRRQ> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `ABRRQ` writer - Auto baud rate request
pub type ABRRQ_W<'a, REG> = crate::BitWriter<'a, REG, ABRRQ>;
impl<'a, REG> ABRRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame
    #[inline(always)]
    pub fn request(self) -> &'a mut crate::W<REG> {
        self.variant(ABRRQ::Request)
    }
}
/**Send break request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKRQ {
    ///1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    Break = 1,
}
impl From<SBKRQ> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKRQ` writer - Send break request
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG, SBKRQ>;
impl<'a, REG> SBKRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(SBKRQ::Break)
    }
}
/**Mute mode request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMRQ {
    ///1: Puts the USART in mute mode and sets the RWU flag
    Mute = 1,
}
impl From<MMRQ> for bool {
    #[inline(always)]
    fn from(variant: MMRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `MMRQ` writer - Mute mode request
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG, MMRQ>;
impl<'a, REG> MMRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Puts the USART in mute mode and sets the RWU flag
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(MMRQ::Mute)
    }
}
/**Receive data flush request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRQ {
    ///1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    Discard = 1,
}
impl From<RXFRQ> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFRQ` writer - Receive data flush request
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG, RXFRQ>;
impl<'a, REG> RXFRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(RXFRQ::Discard)
    }
}
/**Transmit data flush request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRQ {
    ///1: Set the TXE flags. This allows to discard the transmit data
    Discard = 1,
}
impl From<TXFRQ> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFRQ` writer - Transmit data flush request
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG, TXFRQ>;
impl<'a, REG> TXFRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set the TXE flags. This allows to discard the transmit data
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(TXFRQ::Discard)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RQRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Auto baud rate request
    #[inline(always)]
    pub fn abrrq(&mut self) -> ABRRQ_W<RQRrs> {
        ABRRQ_W::new(self, 0)
    }
    ///Bit 1 - Send break request
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W<RQRrs> {
        SBKRQ_W::new(self, 1)
    }
    ///Bit 2 - Mute mode request
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W<RQRrs> {
        MMRQ_W::new(self, 2)
    }
    ///Bit 3 - Receive data flush request
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W<RQRrs> {
        RXFRQ_W::new(self, 3)
    }
    ///Bit 4 - Transmit data flush request
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W<RQRrs> {
        TXFRQ_W::new(self, 4)
    }
}
/**Request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#USART1:RQR)*/
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rqr::W`](W) writer structure
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RQR to value 0
impl crate::Resettable for RQRrs {}
