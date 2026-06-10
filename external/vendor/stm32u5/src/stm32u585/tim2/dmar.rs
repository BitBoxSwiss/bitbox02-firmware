///Register `DMAR` reader
pub type R = crate::R<DMARrs>;
///Register `DMAR` writer
pub type W = crate::W<DMARrs>;
///Field `ETRSEL` reader - DMA register for burst accesses
pub type ETRSEL_R = crate::FieldReader<u32>;
///Field `ETRSEL` writer - DMA register for burst accesses
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA register for burst accesses
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAR")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA register for burst accesses
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<DMARrs> {
        ETRSEL_W::new(self, 0)
    }
}
/**DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TIM2:DMAR)*/
pub struct DMARrs;
impl crate::RegisterSpec for DMARrs {
    type Ux = u32;
}
///`read()` method returns [`dmar::R`](R) reader structure
impl crate::Readable for DMARrs {}
///`write(|w| ..)` method takes [`dmar::W`](W) writer structure
impl crate::Writable for DMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAR to value 0
impl crate::Resettable for DMARrs {}
