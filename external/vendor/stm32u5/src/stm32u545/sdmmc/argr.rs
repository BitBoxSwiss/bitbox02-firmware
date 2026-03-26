///Register `ARGR` reader
pub type R = crate::R<ARGRrs>;
///Register `ARGR` writer
pub type W = crate::W<ARGRrs>;
///Field `CMDARG` reader - Command argument
pub type CMDARG_R = crate::FieldReader<u32>;
///Field `CMDARG` writer - Command argument
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Command argument
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARGR")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Command argument
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W<ARGRrs> {
        CMDARG_W::new(self, 0)
    }
}
/**argument register

You can [`read`](crate::Reg::read) this register and get [`argr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:ARGR)*/
pub struct ARGRrs;
impl crate::RegisterSpec for ARGRrs {
    type Ux = u32;
}
///`read()` method returns [`argr::R`](R) reader structure
impl crate::Readable for ARGRrs {}
///`write(|w| ..)` method takes [`argr::W`](W) writer structure
impl crate::Writable for ARGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ARGR to value 0
impl crate::Resettable for ARGRrs {}
