///Register `VVBPCR` reader
pub type R = crate::R<VVBPCRrs>;
///Register `VVBPCR` writer
pub type W = crate::W<VVBPCRrs>;
///Field `VBP` reader - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
pub type VBP_R = crate::FieldReader<u16>;
///Field `VBP` writer - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVBPCR").field("vbp", &self.vbp()).finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W<VVBPCRrs> {
        VBP_W::new(self, 0)
    }
}
/**DSI Host video VBP configuration register

You can [`read`](crate::Reg::read) this register and get [`vvbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:VVBPCR)*/
pub struct VVBPCRrs;
impl crate::RegisterSpec for VVBPCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvbpcr::R`](R) reader structure
impl crate::Readable for VVBPCRrs {}
///`write(|w| ..)` method takes [`vvbpcr::W`](W) writer structure
impl crate::Writable for VVBPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VVBPCR to value 0
impl crate::Resettable for VVBPCRrs {}
