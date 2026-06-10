///Register `DPDL1SRCR` reader
pub type R = crate::R<DPDL1SRCRrs>;
///Register `DPDL1SRCR` writer
pub type W = crate::W<DPDL1SRCRrs>;
///Field `SRC` reader - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
pub type SRC_R = crate::FieldReader;
///Field `SRC` writer - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
pub type SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPDL1SRCR")
            .field("src", &self.src())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W<DPDL1SRCRrs> {
        SRC_W::new(self, 0)
    }
}
/**DSI D-PHY data lane 1 skew rate control register

You can [`read`](crate::Reg::read) this register and get [`dpdl1srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdl1srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DPDL1SRCR)*/
pub struct DPDL1SRCRrs;
impl crate::RegisterSpec for DPDL1SRCRrs {
    type Ux = u32;
}
///`read()` method returns [`dpdl1srcr::R`](R) reader structure
impl crate::Readable for DPDL1SRCRrs {}
///`write(|w| ..)` method takes [`dpdl1srcr::W`](W) writer structure
impl crate::Writable for DPDL1SRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPDL1SRCR to value 0
impl crate::Resettable for DPDL1SRCRrs {}
