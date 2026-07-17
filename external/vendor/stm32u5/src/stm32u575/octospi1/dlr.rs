///Register `DLR` reader
pub type R = crate::R<DLRrs>;
///Register `DLR` writer
pub type W = crate::W<DLRrs>;
///Field `DL` reader - Data length
pub type DL_R = crate::FieldReader<u32>;
///Field `DL` writer - Data length
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Data length
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLR").field("dl", &self.dl()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data length
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W<DLRrs> {
        DL_W::new(self, 0)
    }
}
/**data length register

You can [`read`](crate::Reg::read) this register and get [`dlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OCTOSPI1:DLR)*/
pub struct DLRrs;
impl crate::RegisterSpec for DLRrs {
    type Ux = u32;
}
///`read()` method returns [`dlr::R`](R) reader structure
impl crate::Readable for DLRrs {}
///`write(|w| ..)` method takes [`dlr::W`](W) writer structure
impl crate::Writable for DLRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets DLR to value 0
impl crate::Resettable for DLRrs {}
