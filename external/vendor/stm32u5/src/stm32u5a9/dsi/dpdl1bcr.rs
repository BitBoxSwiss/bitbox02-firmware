///Register `DPDL1BCR` reader
pub type R = crate::R<DPDL1BCRrs>;
///Register `DPDL1BCR` writer
pub type W = crate::W<DPDL1BCRrs>;
///Field `BC` reader - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
pub type BC_R = crate::FieldReader;
///Field `BC` writer - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
pub type BC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPDL1BCR").field("bc", &self.bc()).finish()
    }
}
impl W {
    ///Bits 0:4 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W<DPDL1BCRrs> {
        BC_W::new(self, 0)
    }
}
/**DSI D-PHY data lane 1 band control register

You can [`read`](crate::Reg::read) this register and get [`dpdl1bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdl1bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DPDL1BCR)*/
pub struct DPDL1BCRrs;
impl crate::RegisterSpec for DPDL1BCRrs {
    type Ux = u32;
}
///`read()` method returns [`dpdl1bcr::R`](R) reader structure
impl crate::Readable for DPDL1BCRrs {}
///`write(|w| ..)` method takes [`dpdl1bcr::W`](W) writer structure
impl crate::Writable for DPDL1BCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPDL1BCR to value 0
impl crate::Resettable for DPDL1BCRrs {}
