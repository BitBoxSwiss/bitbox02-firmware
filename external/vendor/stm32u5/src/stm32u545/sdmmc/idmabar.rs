///Register `IDMABAR` reader
pub type R = crate::R<IDMABARrs>;
///Register `IDMABAR` writer
pub type W = crate::W<IDMABARrs>;
///Field `IDMABA` reader - Word aligned Linked list memory base address
pub type IDMABA_R = crate::FieldReader<u32>;
///Field `IDMABA` writer - Word aligned Linked list memory base address
pub type IDMABA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Word aligned Linked list memory base address
    #[inline(always)]
    pub fn idmaba(&self) -> IDMABA_R {
        IDMABA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMABAR")
            .field("idmaba", &self.idmaba())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Word aligned Linked list memory base address
    #[inline(always)]
    pub fn idmaba(&mut self) -> IDMABA_W<IDMABARrs> {
        IDMABA_W::new(self, 2)
    }
}
/**linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`idmabar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:IDMABAR)*/
pub struct IDMABARrs;
impl crate::RegisterSpec for IDMABARrs {
    type Ux = u32;
}
///`read()` method returns [`idmabar::R`](R) reader structure
impl crate::Readable for IDMABARrs {}
///`write(|w| ..)` method takes [`idmabar::W`](W) writer structure
impl crate::Writable for IDMABARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABAR to value 0
impl crate::Resettable for IDMABARrs {}
