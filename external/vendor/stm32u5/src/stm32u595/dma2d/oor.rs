///Register `OOR` reader
pub type R = crate::R<OORrs>;
///Register `OOR` writer
pub type W = crate::W<OORrs>;
///Field `LO` reader - Line Offset
pub type LO_R = crate::FieldReader<u16>;
///Field `LO` writer - Line Offset
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Line Offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OOR").field("lo", &self.lo()).finish()
    }
}
impl W {
    ///Bits 0:15 - Line Offset
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<OORrs> {
        LO_W::new(self, 0)
    }
}
/**output offset register

You can [`read`](crate::Reg::read) this register and get [`oor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DMA2D:OOR)*/
pub struct OORrs;
impl crate::RegisterSpec for OORrs {
    type Ux = u32;
}
///`read()` method returns [`oor::R`](R) reader structure
impl crate::Readable for OORrs {}
///`write(|w| ..)` method takes [`oor::W`](W) writer structure
impl crate::Writable for OORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OOR to value 0
impl crate::Resettable for OORrs {}
