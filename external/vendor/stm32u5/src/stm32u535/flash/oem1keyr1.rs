///Register `OEM1KEYR1` writer
pub type W = crate::W<OEM1KEYR1rs>;
///Field `OEM1KEY` writer - OEM1 least significant bytes key
pub type OEM1KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OEM1KEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - OEM1 least significant bytes key
    #[inline(always)]
    pub fn oem1key(&mut self) -> OEM1KEY_W<OEM1KEYR1rs> {
        OEM1KEY_W::new(self, 0)
    }
}
/**FLASH OEM1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem1keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FLASH:OEM1KEYR1)*/
pub struct OEM1KEYR1rs;
impl crate::RegisterSpec for OEM1KEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`oem1keyr1::W`](W) writer structure
impl crate::Writable for OEM1KEYR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OEM1KEYR1 to value 0
impl crate::Resettable for OEM1KEYR1rs {}
