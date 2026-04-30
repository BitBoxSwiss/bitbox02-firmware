///Register `VCCR` reader
pub type R = crate::R<VCCRrs>;
///Register `VCCR` writer
pub type W = crate::W<VCCRrs>;
///Field `NUMC` reader - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
pub type NUMC_R = crate::FieldReader<u16>;
///Field `NUMC` writer - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
pub type NUMC_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCCR").field("numc", &self.numc()).finish()
    }
}
impl W {
    ///Bits 0:12 - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
    #[inline(always)]
    pub fn numc(&mut self) -> NUMC_W<VCCRrs> {
        NUMC_W::new(self, 0)
    }
}
/**DSI Host video chunks configuration register

You can [`read`](crate::Reg::read) this register and get [`vccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:VCCR)*/
pub struct VCCRrs;
impl crate::RegisterSpec for VCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vccr::R`](R) reader structure
impl crate::Readable for VCCRrs {}
///`write(|w| ..)` method takes [`vccr::W`](W) writer structure
impl crate::Writable for VCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VCCR to value 0
impl crate::Resettable for VCCRrs {}
