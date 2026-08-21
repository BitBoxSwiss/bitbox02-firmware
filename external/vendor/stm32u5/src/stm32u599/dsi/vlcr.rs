///Register `VLCR` reader
pub type R = crate::R<VLCRrs>;
///Register `VLCR` writer
pub type W = crate::W<VLCRrs>;
///Field `HLINE` reader - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
pub type HLINE_R = crate::FieldReader<u16>;
///Field `HLINE` writer - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
pub type HLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VLCR")
            .field("hline", &self.hline())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
    #[inline(always)]
    pub fn hline(&mut self) -> HLINE_W<VLCRrs> {
        HLINE_W::new(self, 0)
    }
}
/**DSI Host video line configuration register

You can [`read`](crate::Reg::read) this register and get [`vlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VLCR)*/
pub struct VLCRrs;
impl crate::RegisterSpec for VLCRrs {
    type Ux = u32;
}
///`read()` method returns [`vlcr::R`](R) reader structure
impl crate::Readable for VLCRrs {}
///`write(|w| ..)` method takes [`vlcr::W`](W) writer structure
impl crate::Writable for VLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VLCR to value 0
impl crate::Resettable for VLCRrs {}
