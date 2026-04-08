///Register `DTIMER` reader
pub type R = crate::R<DTIMERrs>;
///Register `DTIMER` writer
pub type W = crate::W<DTIMERrs>;
///Field `DATATIME` reader - Data and R1b busy timeout period
pub type DATATIME_R = crate::FieldReader<u32>;
///Field `DATATIME` writer - Data and R1b busy timeout period
pub type DATATIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data and R1b busy timeout period
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTIMER")
            .field("datatime", &self.datatime())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Data and R1b busy timeout period
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W<DTIMERrs> {
        DATATIME_W::new(self, 0)
    }
}
/**data timer register

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DTIMER)*/
pub struct DTIMERrs;
impl crate::RegisterSpec for DTIMERrs {
    type Ux = u32;
}
///`read()` method returns [`dtimer::R`](R) reader structure
impl crate::Readable for DTIMERrs {}
///`write(|w| ..)` method takes [`dtimer::W`](W) writer structure
impl crate::Writable for DTIMERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTIMER to value 0
impl crate::Resettable for DTIMERrs {}
