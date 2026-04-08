///Register `LIPCR` reader
pub type R = crate::R<LIPCRrs>;
///Register `LIPCR` writer
pub type W = crate::W<LIPCRrs>;
///Field `LIPOS` reader - line interrupt position These bits configure the line interrupt position.
pub type LIPOS_R = crate::FieldReader<u16>;
///Field `LIPOS` writer - line interrupt position These bits configure the line interrupt position.
pub type LIPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - line interrupt position These bits configure the line interrupt position.
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIPCR")
            .field("lipos", &self.lipos())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - line interrupt position These bits configure the line interrupt position.
    #[inline(always)]
    pub fn lipos(&mut self) -> LIPOS_W<LIPCRrs> {
        LIPOS_W::new(self, 0)
    }
}
/**LTDC line interrupt position configuration register

You can [`read`](crate::Reg::read) this register and get [`lipcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LIPCR)*/
pub struct LIPCRrs;
impl crate::RegisterSpec for LIPCRrs {
    type Ux = u32;
}
///`read()` method returns [`lipcr::R`](R) reader structure
impl crate::Readable for LIPCRrs {}
///`write(|w| ..)` method takes [`lipcr::W`](W) writer structure
impl crate::Writable for LIPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LIPCR to value 0
impl crate::Resettable for LIPCRrs {}
