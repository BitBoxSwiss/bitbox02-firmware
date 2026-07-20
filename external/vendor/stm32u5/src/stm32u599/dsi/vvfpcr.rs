///Register `VVFPCR` reader
pub type R = crate::R<VVFPCRrs>;
///Register `VVFPCR` writer
pub type W = crate::W<VVFPCRrs>;
///Field `VFP` reader - Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines.
pub type VFP_R = crate::FieldReader<u16>;
///Field `VFP` writer - Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines.
pub type VFP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVFPCR").field("vfp", &self.vfp()).finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W<VVFPCRrs> {
        VFP_W::new(self, 0)
    }
}
/**DSI Host video VFP configuration register

You can [`read`](crate::Reg::read) this register and get [`vvfpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvfpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VVFPCR)*/
pub struct VVFPCRrs;
impl crate::RegisterSpec for VVFPCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvfpcr::R`](R) reader structure
impl crate::Readable for VVFPCRrs {}
///`write(|w| ..)` method takes [`vvfpcr::W`](W) writer structure
impl crate::Writable for VVFPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VVFPCR to value 0
impl crate::Resettable for VVFPCRrs {}
