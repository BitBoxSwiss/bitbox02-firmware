///Register `WTCR` reader
pub type R = crate::R<WTCRrs>;
///Register `WTCR` writer
pub type W = crate::W<WTCRrs>;
///Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.
pub type DCYC_R = crate::FieldReader;
///Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTCR").field("dcyc", &self.dcyc()).finish()
    }
}
impl W {
    ///Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W<WTCRrs> {
        DCYC_W::new(self, 0)
    }
}
/**HSPI write timing configuration register

You can [`read`](crate::Reg::read) this register and get [`wtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WTCR)*/
pub struct WTCRrs;
impl crate::RegisterSpec for WTCRrs {
    type Ux = u32;
}
///`read()` method returns [`wtcr::R`](R) reader structure
impl crate::Readable for WTCRrs {}
///`write(|w| ..)` method takes [`wtcr::W`](W) writer structure
impl crate::Writable for WTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WTCR to value 0
impl crate::Resettable for WTCRrs {}
