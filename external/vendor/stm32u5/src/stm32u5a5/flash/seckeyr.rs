///Register `SECKEYR` writer
pub type W = crate::W<SECKEYRrs>;
///Field `SECKEY` writer - Flash memory secure key The following values must be written consecutively to unlock the FLASH_SECCR register, allowing the Flash memory secure programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB
pub type SECKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SECKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Flash memory secure key The following values must be written consecutively to unlock the FLASH_SECCR register, allowing the Flash memory secure programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB
    #[inline(always)]
    pub fn seckey(&mut self) -> SECKEY_W<SECKEYRrs> {
        SECKEY_W::new(self, 0)
    }
}
/**FLASH secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FLASH:SECKEYR)*/
pub struct SECKEYRrs;
impl crate::RegisterSpec for SECKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`seckeyr::W`](W) writer structure
impl crate::Writable for SECKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECKEYR to value 0
impl crate::Resettable for SECKEYRrs {}
