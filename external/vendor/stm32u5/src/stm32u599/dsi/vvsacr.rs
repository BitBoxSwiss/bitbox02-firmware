///Register `VVSACR` reader
pub type R = crate::R<VVSACRrs>;
///Register `VVSACR` writer
pub type W = crate::W<VVSACRrs>;
///Field `VSA` reader - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
pub type VSA_R = crate::FieldReader<u16>;
///Field `VSA` writer - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
pub type VSA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVSACR").field("vsa", &self.vsa()).finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vsa(&mut self) -> VSA_W<VVSACRrs> {
        VSA_W::new(self, 0)
    }
}
/**DSI Host video VSA configuration register

You can [`read`](crate::Reg::read) this register and get [`vvsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VVSACR)*/
pub struct VVSACRrs;
impl crate::RegisterSpec for VVSACRrs {
    type Ux = u32;
}
///`read()` method returns [`vvsacr::R`](R) reader structure
impl crate::Readable for VVSACRrs {}
///`write(|w| ..)` method takes [`vvsacr::W`](W) writer structure
impl crate::Writable for VVSACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VVSACR to value 0
impl crate::Resettable for VVSACRrs {}
