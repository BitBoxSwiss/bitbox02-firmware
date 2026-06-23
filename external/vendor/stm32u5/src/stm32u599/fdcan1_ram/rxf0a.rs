///Register `RXF0A` reader
pub type R = crate::R<RXF0Ars>;
///Register `RXF0A` writer
pub type W = crate::W<RXF0Ars>;
///Field `F0AI` reader - Rx FIFO 0 Acknowledge Index
pub type F0AI_R = crate::FieldReader;
///Field `F0AI` writer - Rx FIFO 0 Acknowledge Index
pub type F0AI_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Rx FIFO 0 Acknowledge Index
    #[inline(always)]
    pub fn f0ai(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0A").field("f0ai", &self.f0ai()).finish()
    }
}
impl W {
    ///Bits 0:2 - Rx FIFO 0 Acknowledge Index
    #[inline(always)]
    pub fn f0ai(&mut self) -> F0AI_W<RXF0Ars> {
        F0AI_W::new(self, 0)
    }
}
/**CAN Rx FIFO 0 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FDCAN1_RAM:RXF0A)*/
pub struct RXF0Ars;
impl crate::RegisterSpec for RXF0Ars {
    type Ux = u32;
}
///`read()` method returns [`rxf0a::R`](R) reader structure
impl crate::Readable for RXF0Ars {}
///`write(|w| ..)` method takes [`rxf0a::W`](W) writer structure
impl crate::Writable for RXF0Ars {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXF0A to value 0
impl crate::Resettable for RXF0Ars {}
