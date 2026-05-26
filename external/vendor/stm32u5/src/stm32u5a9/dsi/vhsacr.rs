///Register `VHSACR` reader
pub type R = crate::R<VHSACRrs>;
///Register `VHSACR` writer
pub type W = crate::W<VHSACRrs>;
///Field `HSA` reader - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
pub type HSA_R = crate::FieldReader<u16>;
///Field `HSA` writer - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
pub type HSA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VHSACR").field("hsa", &self.hsa()).finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
    #[inline(always)]
    pub fn hsa(&mut self) -> HSA_W<VHSACRrs> {
        HSA_W::new(self, 0)
    }
}
/**DSI Host video HSA configuration register

You can [`read`](crate::Reg::read) this register and get [`vhsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:VHSACR)*/
pub struct VHSACRrs;
impl crate::RegisterSpec for VHSACRrs {
    type Ux = u32;
}
///`read()` method returns [`vhsacr::R`](R) reader structure
impl crate::Readable for VHSACRrs {}
///`write(|w| ..)` method takes [`vhsacr::W`](W) writer structure
impl crate::Writable for VHSACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VHSACR to value 0
impl crate::Resettable for VHSACRrs {}
