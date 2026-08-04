///Register `IDMABSIZER` reader
pub type R = crate::R<IDMABSIZERrs>;
///Register `IDMABSIZER` writer
pub type W = crate::W<IDMABSIZERrs>;
///Field `IDMABNDT` reader - Number of bytes per buffer
pub type IDMABNDT_R = crate::FieldReader<u16>;
///Field `IDMABNDT` writer - Number of bytes per buffer
pub type IDMABNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 5:16 - Number of bytes per buffer
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMABSIZER")
            .field("idmabndt", &self.idmabndt())
            .finish()
    }
}
impl W {
    ///Bits 5:16 - Number of bytes per buffer
    #[inline(always)]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<IDMABSIZERrs> {
        IDMABNDT_W::new(self, 5)
    }
}
/**buffer size register

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#SDMMC1:IDMABSIZER)*/
pub struct IDMABSIZERrs;
impl crate::RegisterSpec for IDMABSIZERrs {
    type Ux = u32;
}
///`read()` method returns [`idmabsizer::R`](R) reader structure
impl crate::Readable for IDMABSIZERrs {}
///`write(|w| ..)` method takes [`idmabsizer::W`](W) writer structure
impl crate::Writable for IDMABSIZERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABSIZER to value 0
impl crate::Resettable for IDMABSIZERrs {}
