///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**RXP interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<RXPIE> for bool {
    #[inline(always)]
    fn from(variant: RXPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXPIE` reader - RXP interrupt enable
pub type RXPIE_R = crate::BitReader<RXPIE>;
impl RXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXPIE {
        match self.bits {
            false => RXPIE::Disabled,
            true => RXPIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXPIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXPIE::Enabled
    }
}
///Field `RXPIE` writer - RXP interrupt enable
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG, RXPIE>;
impl<'a, REG> RXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE::Enabled)
    }
}
///Field `TXPIE` reader - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
pub use RXPIE_R as TXPIE_R;
///Field `DXPIE` reader - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
pub use RXPIE_R as DXPIE_R;
///Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable
pub use RXPIE_R as EOTIE_R;
///Field `TXTFIE` reader - TXTFIE interrupt enable
pub use RXPIE_R as TXTFIE_R;
///Field `UDRIE` reader - UDR interrupt enable
pub use RXPIE_R as UDRIE_R;
///Field `OVRIE` reader - OVR interrupt enable
pub use RXPIE_R as OVRIE_R;
///Field `CRCEIE` reader - CRC error interrupt enable
pub use RXPIE_R as CRCEIE_R;
///Field `TIFREIE` reader - TIFRE interrupt enable
pub use RXPIE_R as TIFREIE_R;
///Field `MODFIE` reader - mode Fault interrupt enable
pub use RXPIE_R as MODFIE_R;
///Field `TXPIE` writer - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
pub use RXPIE_W as TXPIE_W;
///Field `DXPIE` writer - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
pub use RXPIE_W as DXPIE_W;
///Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable
pub use RXPIE_W as EOTIE_W;
///Field `TXTFIE` writer - TXTFIE interrupt enable
pub use RXPIE_W as TXTFIE_W;
///Field `UDRIE` writer - UDR interrupt enable
pub use RXPIE_W as UDRIE_W;
///Field `OVRIE` writer - OVR interrupt enable
pub use RXPIE_W as OVRIE_W;
///Field `CRCEIE` writer - CRC error interrupt enable
pub use RXPIE_W as CRCEIE_W;
///Field `TIFREIE` writer - TIFRE interrupt enable
pub use RXPIE_W as TIFREIE_W;
///Field `MODFIE` writer - mode Fault interrupt enable
pub use RXPIE_W as MODFIE_W;
impl R {
    ///Bit 0 - RXP interrupt enable
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mode Fault interrupt enable
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rxpie", &self.rxpie())
            .field("txpie", &self.txpie())
            .field("dxpie", &self.dxpie())
            .field("eotie", &self.eotie())
            .field("txtfie", &self.txtfie())
            .field("udrie", &self.udrie())
            .field("ovrie", &self.ovrie())
            .field("crceie", &self.crceie())
            .field("tifreie", &self.tifreie())
            .field("modfie", &self.modfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXP interrupt enable
    #[inline(always)]
    pub fn rxpie(&mut self) -> RXPIE_W<IERrs> {
        RXPIE_W::new(self, 0)
    }
    ///Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    pub fn txpie(&mut self) -> TXPIE_W<IERrs> {
        TXPIE_W::new(self, 1)
    }
    ///Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    pub fn dxpie(&mut self) -> DXPIE_W<IERrs> {
        DXPIE_W::new(self, 2)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    pub fn eotie(&mut self) -> EOTIE_W<IERrs> {
        EOTIE_W::new(self, 3)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    pub fn txtfie(&mut self) -> TXTFIE_W<IERrs> {
        TXTFIE_W::new(self, 4)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    pub fn udrie(&mut self) -> UDRIE_W<IERrs> {
        UDRIE_W::new(self, 5)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 6)
    }
    ///Bit 7 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W<IERrs> {
        CRCEIE_W::new(self, 7)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    pub fn tifreie(&mut self) -> TIFREIE_W<IERrs> {
        TIFREIE_W::new(self, 8)
    }
    ///Bit 9 - mode Fault interrupt enable
    #[inline(always)]
    pub fn modfie(&mut self) -> MODFIE_W<IERrs> {
        MODFIE_W::new(self, 9)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
