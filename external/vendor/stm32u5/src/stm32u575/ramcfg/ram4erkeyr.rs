///Register `RAM4ERKEYR` writer
pub type W = crate::W<RAM4ERKEYRrs>;
///Field `ERASEKEY` writer - ERASEKEY
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<RAM4ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ERASEKEY
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<RAM4ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram4erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM4ERKEYR)*/
pub struct RAM4ERKEYRrs;
impl crate::RegisterSpec for RAM4ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ram4erkeyr::W`](W) writer structure
impl crate::Writable for RAM4ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM4ERKEYR to value 0
impl crate::Resettable for RAM4ERKEYRrs {}
