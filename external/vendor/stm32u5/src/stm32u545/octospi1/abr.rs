///Register `ABR` reader
pub type R = crate::R<ABRrs>;
///Register `ABR` writer
pub type W = crate::W<ABRrs>;
///Field `ALTERNATE` reader - Alternate bytes
pub type ALTERNATE_R = crate::FieldReader<u32>;
///Field `ALTERNATE` writer - Alternate bytes
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Alternate bytes
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ABR")
            .field("alternate", &self.alternate())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Alternate bytes
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W<ABRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
/**alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`abr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OCTOSPI1:ABR)*/
pub struct ABRrs;
impl crate::RegisterSpec for ABRrs {
    type Ux = u32;
}
///`read()` method returns [`abr::R`](R) reader structure
impl crate::Readable for ABRrs {}
///`write(|w| ..)` method takes [`abr::W`](W) writer structure
impl crate::Writable for ABRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets ABR to value 0
impl crate::Resettable for ABRrs {}
