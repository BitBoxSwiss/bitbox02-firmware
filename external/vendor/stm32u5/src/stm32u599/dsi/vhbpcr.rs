///Register `VHBPCR` reader
pub type R = crate::R<VHBPCRrs>;
///Register `VHBPCR` writer
pub type W = crate::W<VHBPCRrs>;
///Field `HBP` reader - Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles.
pub type HBP_R = crate::FieldReader<u16>;
///Field `HBP` writer - Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles.
pub type HBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles.
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VHBPCR").field("hbp", &self.hbp()).finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles.
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W<VHBPCRrs> {
        HBP_W::new(self, 0)
    }
}
/**DSI Host video HBP configuration register

You can [`read`](crate::Reg::read) this register and get [`vhbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VHBPCR)*/
pub struct VHBPCRrs;
impl crate::RegisterSpec for VHBPCRrs {
    type Ux = u32;
}
///`read()` method returns [`vhbpcr::R`](R) reader structure
impl crate::Readable for VHBPCRrs {}
///`write(|w| ..)` method takes [`vhbpcr::W`](W) writer structure
impl crate::Writable for VHBPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VHBPCR to value 0
impl crate::Resettable for VHBPCRrs {}
