///Register `VPCR` reader
pub type R = crate::R<VPCRrs>;
///Register `VPCR` writer
pub type W = crate::W<VPCRrs>;
///Field `VPSIZE` reader - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
pub type VPSIZE_R = crate::FieldReader<u16>;
///Field `VPSIZE` writer - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
pub type VPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPCR")
            .field("vpsize", &self.vpsize())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
    #[inline(always)]
    pub fn vpsize(&mut self) -> VPSIZE_W<VPCRrs> {
        VPSIZE_W::new(self, 0)
    }
}
/**DSI Host video packet configuration register

You can [`read`](crate::Reg::read) this register and get [`vpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VPCR)*/
pub struct VPCRrs;
impl crate::RegisterSpec for VPCRrs {
    type Ux = u32;
}
///`read()` method returns [`vpcr::R`](R) reader structure
impl crate::Readable for VPCRrs {}
///`write(|w| ..)` method takes [`vpcr::W`](W) writer structure
impl crate::Writable for VPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VPCR to value 0
impl crate::Resettable for VPCRrs {}
