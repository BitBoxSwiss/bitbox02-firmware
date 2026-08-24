///Register `DLENR` reader
pub type R = crate::R<DLENRrs>;
///Register `DLENR` writer
pub type W = crate::W<DLENRrs>;
///Field `DATALENGTH` reader - Data length value
pub type DATALENGTH_R = crate::FieldReader<u32>;
///Field `DATALENGTH` writer - Data length value
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - Data length value
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLENR")
            .field("datalength", &self.datalength())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Data length value
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W<DLENRrs> {
        DATALENGTH_W::new(self, 0)
    }
}
/**data length register

You can [`read`](crate::Reg::read) this register and get [`dlenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DLENR)*/
pub struct DLENRrs;
impl crate::RegisterSpec for DLENRrs {
    type Ux = u32;
}
///`read()` method returns [`dlenr::R`](R) reader structure
impl crate::Readable for DLENRrs {}
///`write(|w| ..)` method takes [`dlenr::W`](W) writer structure
impl crate::Writable for DLENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLENR to value 0
impl crate::Resettable for DLENRrs {}
