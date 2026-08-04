///Register `DPCSRCR` reader
pub type R = crate::R<DPCSRCRrs>;
///Register `DPCSRCR` writer
pub type W = crate::W<DPCSRCRrs>;
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
        f.debug_struct("DPCSRCR").field("src", &self.src()).finish()
    }
}
impl W {
    ///Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W<DPCSRCRrs> {
        SRC_W::new(self, 0)
    }
}
/**DSI D-PHY clock skew rate control register

You can [`read`](crate::Reg::read) this register and get [`dpcsrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpcsrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DPCSRCR)*/
pub struct DPCSRCRrs;
impl crate::RegisterSpec for DPCSRCRrs {
    type Ux = u32;
}
///`read()` method returns [`dpcsrcr::R`](R) reader structure
impl crate::Readable for DPCSRCRrs {}
///`write(|w| ..)` method takes [`dpcsrcr::W`](W) writer structure
impl crate::Writable for DPCSRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPCSRCR to value 0
impl crate::Resettable for DPCSRCRrs {}
