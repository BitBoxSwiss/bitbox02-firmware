///Register `ALR%sBINR` reader
pub type R = crate::R<ALRBINRrs>;
///Register `ALR%sBINR` writer
pub type W = crate::W<ALRBINRrs>;
///Field `SS` reader - Synchronous counter alarm value in Binary mode
pub type SS_R = crate::FieldReader<u32>;
///Field `SS` writer - Synchronous counter alarm value in Binary mode
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRBINR").field("ss", &self.ss()).finish()
    }
}
impl W {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<ALRBINRrs> {
        SS_W::new(self, 0)
    }
}
/**Alarm %s binary mode register

You can [`read`](crate::Reg::read) this register and get [`alrbinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrbinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RTC:ALR[A]BINR)*/
pub struct ALRBINRrs;
impl crate::RegisterSpec for ALRBINRrs {
    type Ux = u32;
}
///`read()` method returns [`alrbinr::R`](R) reader structure
impl crate::Readable for ALRBINRrs {}
///`write(|w| ..)` method takes [`alrbinr::W`](W) writer structure
impl crate::Writable for ALRBINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALR%sBINR to value 0
impl crate::Resettable for ALRBINRrs {}
