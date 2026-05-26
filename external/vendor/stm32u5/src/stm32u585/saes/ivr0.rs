///Register `IVR0` reader
pub type R = crate::R<IVR0rs>;
///Register `IVR0` writer
pub type W = crate::W<IVR0rs>;
///Field `IVI` reader - Initialization vector input, bits \[31:0\]
pub type IVI_R = crate::FieldReader<u32>;
///Field `IVI` writer - Initialization vector input, bits \[31:0\]
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initialization vector input, bits \[31:0\]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR0").field("ivi", &self.ivi()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initialization vector input, bits \[31:0\]
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<IVR0rs> {
        IVI_W::new(self, 0)
    }
}
/**initialization vector register 0

You can [`read`](crate::Reg::read) this register and get [`ivr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:IVR0)*/
pub struct IVR0rs;
impl crate::RegisterSpec for IVR0rs {
    type Ux = u32;
}
///`read()` method returns [`ivr0::R`](R) reader structure
impl crate::Readable for IVR0rs {}
///`write(|w| ..)` method takes [`ivr0::W`](W) writer structure
impl crate::Writable for IVR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IVR0 to value 0
impl crate::Resettable for IVR0rs {}
