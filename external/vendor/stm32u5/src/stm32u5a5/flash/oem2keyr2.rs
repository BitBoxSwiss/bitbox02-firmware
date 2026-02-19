///Register `OEM2KEYR2` writer
pub type W = crate::W<OEM2KEYR2rs>;
///Field `OEM2KEY` writer - OEM2 most significant bytes key
pub type OEM2KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OEM2KEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - OEM2 most significant bytes key
    #[inline(always)]
    pub fn oem2key(&mut self) -> OEM2KEY_W<OEM2KEYR2rs> {
        OEM2KEY_W::new(self, 0)
    }
}
/**FLASH OEM2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem2keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FLASH:OEM2KEYR2)*/
pub struct OEM2KEYR2rs;
impl crate::RegisterSpec for OEM2KEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`oem2keyr2::W`](W) writer structure
impl crate::Writable for OEM2KEYR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OEM2KEYR2 to value 0
impl crate::Resettable for OEM2KEYR2rs {}
