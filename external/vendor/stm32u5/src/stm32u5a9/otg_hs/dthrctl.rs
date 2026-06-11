///Register `DTHRCTL` reader
pub type R = crate::R<DTHRCTLrs>;
///Register `DTHRCTL` writer
pub type W = crate::W<DTHRCTLrs>;
///Field `NONISOTHREN` reader - Nonisochronous IN endpoints threshold enable When this bit is set, the core enables thresholding for nonisochronous IN endpoints.
pub type NONISOTHREN_R = crate::BitReader;
///Field `NONISOTHREN` writer - Nonisochronous IN endpoints threshold enable When this bit is set, the core enables thresholding for nonisochronous IN endpoints.
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOTHREN` reader - ISO IN endpoint threshold enable When this bit is set, the core enables thresholding for isochronous IN endpoints.
pub type ISOTHREN_R = crate::BitReader;
///Field `ISOTHREN` writer - ISO IN endpoint threshold enable When this bit is set, the core enables thresholding for isochronous IN endpoints.
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTHRLEN` reader - Transmit threshold length This field specifies the transmit thresholding size in 32-bit words. This field specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmitting on the USB. The threshold length has to be at least eight 32-bit words. This field controls both isochronous and nonisochronous IN endpoint thresholds. The recommended value for TXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
pub type TXTHRLEN_R = crate::FieldReader<u16>;
///Field `TXTHRLEN` writer - Transmit threshold length This field specifies the transmit thresholding size in 32-bit words. This field specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmitting on the USB. The threshold length has to be at least eight 32-bit words. This field controls both isochronous and nonisochronous IN endpoint thresholds. The recommended value for TXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `RXTHREN` reader - Receive threshold enable When this bit is set, the core enables thresholding in the receive direction.
pub type RXTHREN_R = crate::BitReader;
///Field `RXTHREN` writer - Receive threshold enable When this bit is set, the core enables thresholding in the receive direction.
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTHRLEN` reader - Receive threshold length This field specifies the receive thresholding size in 32-bit words. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight 32-bit words. The recommended value for RXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
pub type RXTHRLEN_R = crate::FieldReader<u16>;
///Field `RXTHRLEN` writer - Receive threshold length This field specifies the receive thresholding size in 32-bit words. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight 32-bit words. The recommended value for RXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `ARPEN` reader - Arbiter parking enable This bit controls internal DMA arbiter parking for IN endpoints. When thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default parking is enabled.
pub type ARPEN_R = crate::BitReader;
///Field `ARPEN` writer - Arbiter parking enable This bit controls internal DMA arbiter parking for IN endpoints. When thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default parking is enabled.
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable When this bit is set, the core enables thresholding for nonisochronous IN endpoints.
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ISO IN endpoint threshold enable When this bit is set, the core enables thresholding for isochronous IN endpoints.
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:10 - Transmit threshold length This field specifies the transmit thresholding size in 32-bit words. This field specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmitting on the USB. The threshold length has to be at least eight 32-bit words. This field controls both isochronous and nonisochronous IN endpoint thresholds. The recommended value for TXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bit 16 - Receive threshold enable When this bit is set, the core enables thresholding in the receive direction.
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:25 - Receive threshold length This field specifies the receive thresholding size in 32-bit words. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight 32-bit words. The recommended value for RXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    ///Bit 27 - Arbiter parking enable This bit controls internal DMA arbiter parking for IN endpoints. When thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default parking is enabled.
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTHRCTL")
            .field("nonisothren", &self.nonisothren())
            .field("isothren", &self.isothren())
            .field("txthrlen", &self.txthrlen())
            .field("rxthren", &self.rxthren())
            .field("rxthrlen", &self.rxthrlen())
            .field("arpen", &self.arpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable When this bit is set, the core enables thresholding for nonisochronous IN endpoints.
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<DTHRCTLrs> {
        NONISOTHREN_W::new(self, 0)
    }
    ///Bit 1 - ISO IN endpoint threshold enable When this bit is set, the core enables thresholding for isochronous IN endpoints.
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W<DTHRCTLrs> {
        ISOTHREN_W::new(self, 1)
    }
    ///Bits 2:10 - Transmit threshold length This field specifies the transmit thresholding size in 32-bit words. This field specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmitting on the USB. The threshold length has to be at least eight 32-bit words. This field controls both isochronous and nonisochronous IN endpoint thresholds. The recommended value for TXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<DTHRCTLrs> {
        TXTHRLEN_W::new(self, 2)
    }
    ///Bit 16 - Receive threshold enable When this bit is set, the core enables thresholding in the receive direction.
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W<DTHRCTLrs> {
        RXTHREN_W::new(self, 16)
    }
    ///Bits 17:25 - Receive threshold length This field specifies the receive thresholding size in 32-bit words. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight 32-bit words. The recommended value for RXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_GAHBCFG).
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<DTHRCTLrs> {
        RXTHRLEN_W::new(self, 17)
    }
    ///Bit 27 - Arbiter parking enable This bit controls internal DMA arbiter parking for IN endpoints. When thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default parking is enabled.
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W<DTHRCTLrs> {
        ARPEN_W::new(self, 27)
    }
}
/**OTG device threshold control register

You can [`read`](crate::Reg::read) this register and get [`dthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DTHRCTL)*/
pub struct DTHRCTLrs;
impl crate::RegisterSpec for DTHRCTLrs {
    type Ux = u32;
}
///`read()` method returns [`dthrctl::R`](R) reader structure
impl crate::Readable for DTHRCTLrs {}
///`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure
impl crate::Writable for DTHRCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTHRCTL to value 0
impl crate::Resettable for DTHRCTLrs {}
