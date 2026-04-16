///Register `FGOR` reader
pub type R = crate::R<FGORrs>;
///Register `FGOR` writer
pub type W = crate::W<FGORrs>;
///Field `LO` reader - Line offset
pub type LO_R = crate::FieldReader<u16>;
///Field `LO` writer - Line offset
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Line offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FGOR").field("lo", &self.lo()).finish()
    }
}
impl W {
    ///Bits 0:15 - Line offset
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<FGORrs> {
        LO_W::new(self, 0)
    }
}
/**foreground offset register

You can [`read`](crate::Reg::read) this register and get [`fgor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGOR)*/
pub struct FGORrs;
impl crate::RegisterSpec for FGORrs {
    type Ux = u32;
}
///`read()` method returns [`fgor::R`](R) reader structure
impl crate::Readable for FGORrs {}
///`write(|w| ..)` method takes [`fgor::W`](W) writer structure
impl crate::Writable for FGORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FGOR to value 0
impl crate::Resettable for FGORrs {}
