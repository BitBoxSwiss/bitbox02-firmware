///Register `TSCV` reader
pub type R = crate::R<TSCVrs>;
///Register `TSCV` writer
pub type W = crate::W<TSCVrs>;
///Field `TSC` reader - Timestamp Counter
pub type TSC_R = crate::FieldReader<u16>;
///Field `TSC` writer - Timestamp Counter
pub type TSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timestamp Counter
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCV").field("tsc", &self.tsc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Timestamp Counter
    #[inline(always)]
    pub fn tsc(&mut self) -> TSC_W<TSCVrs> {
        TSC_W::new(self, 0)
    }
}
/**FDCAN Timestamp Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FDCAN1_RAM:TSCV)*/
pub struct TSCVrs;
impl crate::RegisterSpec for TSCVrs {
    type Ux = u32;
}
///`read()` method returns [`tscv::R`](R) reader structure
impl crate::Readable for TSCVrs {}
///`write(|w| ..)` method takes [`tscv::W`](W) writer structure
impl crate::Writable for TSCVrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCV to value 0
impl crate::Resettable for TSCVrs {}
