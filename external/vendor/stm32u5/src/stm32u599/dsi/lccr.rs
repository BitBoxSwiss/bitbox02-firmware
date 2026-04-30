///Register `LCCR` reader
pub type R = crate::R<LCCRrs>;
///Register `LCCR` writer
pub type W = crate::W<LCCRrs>;
///Field `CMDSIZE` reader - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
pub type CMDSIZE_R = crate::FieldReader<u16>;
///Field `CMDSIZE` writer - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
pub type CMDSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCCR")
            .field("cmdsize", &self.cmdsize())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
    #[inline(always)]
    pub fn cmdsize(&mut self) -> CMDSIZE_W<LCCRrs> {
        CMDSIZE_W::new(self, 0)
    }
}
/**DSI Host LTDC command configuration register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:LCCR)*/
pub struct LCCRrs;
impl crate::RegisterSpec for LCCRrs {
    type Ux = u32;
}
///`read()` method returns [`lccr::R`](R) reader structure
impl crate::Readable for LCCRrs {}
///`write(|w| ..)` method takes [`lccr::W`](W) writer structure
impl crate::Writable for LCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCCR to value 0
impl crate::Resettable for LCCRrs {}
