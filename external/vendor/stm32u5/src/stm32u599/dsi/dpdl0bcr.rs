///Register `DPDL0BCR` reader
pub type R = crate::R<DPDL0BCRrs>;
///Register `DPDL0BCR` writer
pub type W = crate::W<DPDL0BCRrs>;
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
        f.debug_struct("DPDL0BCR").field("bc", &self.bc()).finish()
    }
}
impl W {
    ///Bits 0:4 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W<DPDL0BCRrs> {
        BC_W::new(self, 0)
    }
}
/**DSI D-PHY data lane 0 band control register

You can [`read`](crate::Reg::read) this register and get [`dpdl0bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdl0bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DPDL0BCR)*/
pub struct DPDL0BCRrs;
impl crate::RegisterSpec for DPDL0BCRrs {
    type Ux = u32;
}
///`read()` method returns [`dpdl0bcr::R`](R) reader structure
impl crate::Readable for DPDL0BCRrs {}
///`write(|w| ..)` method takes [`dpdl0bcr::W`](W) writer structure
impl crate::Writable for DPDL0BCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPDL0BCR to value 0
impl crate::Resettable for DPDL0BCRrs {}
