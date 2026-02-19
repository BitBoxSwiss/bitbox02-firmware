///Register `DVR` reader
pub type R = crate::R<DVRrs>;
///Register `DVR` writer
pub type W = crate::W<DVRrs>;
///Field `DV` reader - Default value This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped.
pub type DV_R = crate::FieldReader<u32>;
///Field `DV` writer - Default value This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped.
pub type DV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Default value This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped.
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVR").field("dv", &self.dv()).finish()
    }
}
impl W {
    ///Bits 0:31 - Default value This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped.
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W<DVRrs> {
        DV_W::new(self, 0)
    }
}
/**GFXMMU default value register

You can [`read`](crate::Reg::read) this register and get [`dvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GFXMMU:DVR)*/
pub struct DVRrs;
impl crate::RegisterSpec for DVRrs {
    type Ux = u32;
}
///`read()` method returns [`dvr::R`](R) reader structure
impl crate::Readable for DVRrs {}
///`write(|w| ..)` method takes [`dvr::W`](W) writer structure
impl crate::Writable for DVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVR to value 0
impl crate::Resettable for DVRrs {}
