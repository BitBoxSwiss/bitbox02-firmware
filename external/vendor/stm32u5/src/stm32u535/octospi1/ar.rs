///Register `AR` reader
pub type R = crate::R<ARrs>;
///Register `AR` writer
pub type W = crate::W<ARrs>;
///Field `ADDRESS` reader - ADDRESS
pub type ADDRESS_R = crate::FieldReader<u32>;
///Field `ADDRESS` writer - ADDRESS
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - ADDRESS
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR")
            .field("address", &self.address())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ADDRESS
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<ARrs> {
        ADDRESS_W::new(self, 0)
    }
}
/**address register

You can [`read`](crate::Reg::read) this register and get [`ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#OCTOSPI1:AR)*/
pub struct ARrs;
impl crate::RegisterSpec for ARrs {
    type Ux = u32;
}
///`read()` method returns [`ar::R`](R) reader structure
impl crate::Readable for ARrs {}
///`write(|w| ..)` method takes [`ar::W`](W) writer structure
impl crate::Writable for ARrs {
    type Safety = crate::Safe;
}
///`reset()` method sets AR to value 0
impl crate::Resettable for ARrs {}
