///Register `IVR2` reader
pub type R = crate::R<IVR2rs>;
///Register `IVR2` writer
pub type W = crate::W<IVR2rs>;
///Field `IVI` reader - Initialization vector input, bits \[95:64\]
pub type IVI_R = crate::FieldReader<u32>;
///Field `IVI` writer - Initialization vector input, bits \[95:64\]
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Initialization vector input, bits \[95:64\]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR2").field("ivi", &self.ivi()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector input, bits \[95:64\]
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<IVR2rs> {
        IVI_W::new(self, 0)
    }
}
/**initialization vector register 2

You can [`read`](crate::Reg::read) this register and get [`ivr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#AES:IVR2)*/
pub struct IVR2rs;
impl crate::RegisterSpec for IVR2rs {
    type Ux = u32;
}
///`read()` method returns [`ivr2::R`](R) reader structure
impl crate::Readable for IVR2rs {}
///`write(|w| ..)` method takes [`ivr2::W`](W) writer structure
impl crate::Writable for IVR2rs {
    type Safety = crate::Safe;
}
///`reset()` method sets IVR2 to value 0
impl crate::Resettable for IVR2rs {}
