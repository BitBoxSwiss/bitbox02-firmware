///Register `PIR` reader
pub type R = crate::R<PIRrs>;
///Register `PIR` writer
pub type W = crate::W<PIRrs>;
///Field `INTERVAL` reader - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
pub type INTERVAL_R = crate::FieldReader<u16>;
///Field `INTERVAL` writer - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIR")
            .field("interval", &self.interval())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W<PIRrs> {
        INTERVAL_W::new(self, 0)
    }
}
/**HSPI polling interval register

You can [`read`](crate::Reg::read) this register and get [`pir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:PIR)*/
pub struct PIRrs;
impl crate::RegisterSpec for PIRrs {
    type Ux = u32;
}
///`read()` method returns [`pir::R`](R) reader structure
impl crate::Readable for PIRrs {}
///`write(|w| ..)` method takes [`pir::W`](W) writer structure
impl crate::Writable for PIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIR to value 0
impl crate::Resettable for PIRrs {}
