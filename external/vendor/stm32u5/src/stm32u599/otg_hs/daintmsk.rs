///Register `DAINTMSK` reader
pub type R = crate::R<DAINTMSKrs>;
///Register `DAINTMSK` writer
pub type W = crate::W<DAINTMSKrs>;
///Field `IEPM` reader - IEPM
pub type IEPM_R = crate::FieldReader<u16>;
///Field `IEPM` writer - IEPM
pub type IEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `OEPM` reader - OEPM
pub type OEPM_R = crate::FieldReader<u16>;
///Field `OEPM` writer - OEPM
pub type OEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IEPM
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OEPM
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINTMSK")
            .field("iepm", &self.iepm())
            .field("oepm", &self.oepm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IEPM
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W<DAINTMSKrs> {
        IEPM_W::new(self, 0)
    }
    ///Bits 16:31 - OEPM
    #[inline(always)]
    pub fn oepm(&mut self) -> OEPM_W<DAINTMSKrs> {
        OEPM_W::new(self, 16)
    }
}
/**The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.

You can [`read`](crate::Reg::read) this register and get [`daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:DAINTMSK)*/
pub struct DAINTMSKrs;
impl crate::RegisterSpec for DAINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`daintmsk::R`](R) reader structure
impl crate::Readable for DAINTMSKrs {}
///`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure
impl crate::Writable for DAINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAINTMSK to value 0
impl crate::Resettable for DAINTMSKrs {}
