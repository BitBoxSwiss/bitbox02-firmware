///Register `DPCBCR` reader
pub type R = crate::R<DPCBCRrs>;
///Register `DPCBCR` writer
pub type W = crate::W<DPCBCRrs>;
///Field `BC` reader - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
pub type BC_R = crate::FieldReader;
///Field `BC` writer - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
pub type BC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 3:7 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPCBCR").field("bc", &self.bc()).finish()
    }
}
impl W {
    ///Bits 3:7 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W<DPCBCRrs> {
        BC_W::new(self, 3)
    }
}
/**DSI D-PHY clock band control register

You can [`read`](crate::Reg::read) this register and get [`dpcbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpcbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DPCBCR)*/
pub struct DPCBCRrs;
impl crate::RegisterSpec for DPCBCRrs {
    type Ux = u32;
}
///`read()` method returns [`dpcbcr::R`](R) reader structure
impl crate::Readable for DPCBCRrs {}
///`write(|w| ..)` method takes [`dpcbcr::W`](W) writer structure
impl crate::Writable for DPCBCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPCBCR to value 0
impl crate::Resettable for DPCBCRrs {}
