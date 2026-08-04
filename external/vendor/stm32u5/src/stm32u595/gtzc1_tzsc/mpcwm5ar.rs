///Register `MPCWM5AR` reader
pub type R = crate::R<MPCWM5ARrs>;
///Register `MPCWM5AR` writer
pub type W = crate::W<MPCWM5ARrs>;
///Field `SUBA_START` reader - Start of sub-region A
pub type SUBA_START_R = crate::FieldReader<u16>;
///Field `SUBA_START` writer - Start of sub-region A
pub type SUBA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `SUBA_LENGTH` reader - Length of sub-region A
pub type SUBA_LENGTH_R = crate::FieldReader<u16>;
///Field `SUBA_LENGTH` writer - Length of sub-region A
pub type SUBA_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    pub fn suba_start(&self) -> SUBA_START_R {
        SUBA_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    pub fn suba_length(&self) -> SUBA_LENGTH_R {
        SUBA_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM5AR")
            .field("suba_start", &self.suba_start())
            .field("suba_length", &self.suba_length())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    pub fn suba_start(&mut self) -> SUBA_START_W<MPCWM5ARrs> {
        SUBA_START_W::new(self, 0)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    pub fn suba_length(&mut self) -> SUBA_LENGTH_W<MPCWM5ARrs> {
        SUBA_LENGTH_W::new(self, 16)
    }
}
/**TZSC memory 5 sub-region A watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm5ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm5ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GTZC1_TZSC:MPCWM5AR)*/
pub struct MPCWM5ARrs;
impl crate::RegisterSpec for MPCWM5ARrs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm5ar::R`](R) reader structure
impl crate::Readable for MPCWM5ARrs {}
///`write(|w| ..)` method takes [`mpcwm5ar::W`](W) writer structure
impl crate::Writable for MPCWM5ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM5AR to value 0
impl crate::Resettable for MPCWM5ARrs {}
