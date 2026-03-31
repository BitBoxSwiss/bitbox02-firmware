///Register `VVACR` reader
pub type R = crate::R<VVACRrs>;
///Register `VVACR` writer
pub type W = crate::W<VVACRrs>;
///Field `VA` reader - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines.
pub type VA_R = crate::FieldReader<u16>;
///Field `VA` writer - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines.
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVACR").field("va", &self.va()).finish()
    }
}
impl W {
    ///Bits 0:13 - Vertical active duration This fields configures the vertical active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn va(&mut self) -> VA_W<VVACRrs> {
        VA_W::new(self, 0)
    }
}
/**DSI Host video VA configuration register

You can [`read`](crate::Reg::read) this register and get [`vvacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VVACR)*/
pub struct VVACRrs;
impl crate::RegisterSpec for VVACRrs {
    type Ux = u32;
}
///`read()` method returns [`vvacr::R`](R) reader structure
impl crate::Readable for VVACRrs {}
///`write(|w| ..)` method takes [`vvacr::W`](W) writer structure
impl crate::Writable for VVACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VVACR to value 0
impl crate::Resettable for VVACRrs {}
