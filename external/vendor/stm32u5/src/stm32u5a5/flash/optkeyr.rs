///Register `OPTKEYR` writer
pub type W = crate::W<OPTKEYRrs>;
///Field `OPTKEY` writer - Option byte key The following values must be written consecutively to unlock the FLASH_OPTR register allowing option byte programming/erasing operations: KEY1: 0x0819 2A3B KEY2: 0x4C5D 6E7F
pub type OPTKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OPTKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Option byte key The following values must be written consecutively to unlock the FLASH_OPTR register allowing option byte programming/erasing operations: KEY1: 0x0819 2A3B KEY2: 0x4C5D 6E7F
    #[inline(always)]
    pub fn optkey(&mut self) -> OPTKEY_W<OPTKEYRrs> {
        OPTKEY_W::new(self, 0)
    }
}
/**FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FLASH:OPTKEYR)*/
pub struct OPTKEYRrs;
impl crate::RegisterSpec for OPTKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure
impl crate::Writable for OPTKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTKEYR to value 0
impl crate::Resettable for OPTKEYRrs {}
