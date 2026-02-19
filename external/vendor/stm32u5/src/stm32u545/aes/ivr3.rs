///Register `IVR3` reader
pub type R = crate::R<IVR3rs>;
///Register `IVR3` writer
pub type W = crate::W<IVR3rs>;
///Field `IVI` reader - Initialization vector input, bits \[127:96\]
pub type IVI_R = crate::FieldReader<u32>;
///Field `IVI` writer - Initialization vector input, bits \[127:96\]
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Initialization vector input, bits \[127:96\]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR3").field("ivi", &self.ivi()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector input, bits \[127:96\]
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<IVR3rs> {
        IVI_W::new(self, 0)
    }
}
/**initialization vector register 3

You can [`read`](crate::Reg::read) this register and get [`ivr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#AES:IVR3)*/
pub struct IVR3rs;
impl crate::RegisterSpec for IVR3rs {
    type Ux = u32;
}
///`read()` method returns [`ivr3::R`](R) reader structure
impl crate::Readable for IVR3rs {}
///`write(|w| ..)` method takes [`ivr3::W`](W) writer structure
impl crate::Writable for IVR3rs {
    type Safety = crate::Safe;
}
///`reset()` method sets IVR3 to value 0
impl crate::Resettable for IVR3rs {}
