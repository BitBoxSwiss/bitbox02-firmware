///Register `DOEPTSIZ7` reader
pub type R = crate::R<DOEPTSIZ7rs>;
///Register `DOEPTSIZ7` writer
pub type W = crate::W<DOEPTSIZ7rs>;
///Field `XFRSIZ` reader - XFRSIZ
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - XFRSIZ
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - PKTCNT
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - PKTCNT
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RXDPID_STUPCNT` reader - RXDPID_STUPCNT
pub type RXDPID_STUPCNT_R = crate::FieldReader;
///Field `RXDPID_STUPCNT` writer - RXDPID_STUPCNT
pub type RXDPID_STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:18 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - RXDPID_STUPCNT
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RXDPID_STUPCNT_R {
        RXDPID_STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ7")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DOEPTSIZ7rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ7rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - RXDPID_STUPCNT
    #[inline(always)]
    pub fn rxdpid_stupcnt(&mut self) -> RXDPID_STUPCNT_W<DOEPTSIZ7rs> {
        RXDPID_STUPCNT_W::new(self, 29)
    }
}
/**The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DOEPTSIZ7)*/
pub struct DOEPTSIZ7rs;
impl crate::RegisterSpec for DOEPTSIZ7rs {
    type Ux = u32;
}
///`read()` method returns [`doeptsiz7::R`](R) reader structure
impl crate::Readable for DOEPTSIZ7rs {}
///`write(|w| ..)` method takes [`doeptsiz7::W`](W) writer structure
impl crate::Writable for DOEPTSIZ7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPTSIZ7 to value 0
impl crate::Resettable for DOEPTSIZ7rs {}
