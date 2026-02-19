///Register `HTR3` reader
pub type R = crate::R<HTR3rs>;
///Register `HTR3` writer
pub type W = crate::W<HTR3rs>;
///Field `HTR3` reader - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
pub type HTR3_R = crate::FieldReader<u32>;
///Field `HTR3` writer - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
pub type HTR3_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32, crate::Safe>;
impl R {
    ///Bits 0:24 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
    #[inline(always)]
    pub fn htr3(&self) -> HTR3_R {
        HTR3_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTR3").field("htr3", &self.htr3()).finish()
    }
}
impl W {
    ///Bits 0:24 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
    #[inline(always)]
    pub fn htr3(&mut self) -> HTR3_W<HTR3rs> {
        HTR3_W::new(self, 0)
    }
}
/**ADC watchdog higher threshold register 3

You can [`read`](crate::Reg::read) this register and get [`htr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:HTR3)*/
pub struct HTR3rs;
impl crate::RegisterSpec for HTR3rs {
    type Ux = u32;
}
///`read()` method returns [`htr3::R`](R) reader structure
impl crate::Readable for HTR3rs {}
///`write(|w| ..)` method takes [`htr3::W`](W) writer structure
impl crate::Writable for HTR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTR3 to value 0x01ff_ffff
impl crate::Resettable for HTR3rs {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
