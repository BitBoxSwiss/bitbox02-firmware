///Register `M5ERKEYR` reader
pub type R = crate::R<M5ERKEYRrs>;
///Register `M5ERKEYR` writer
pub type W = crate::W<M5ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5ERKEYR").finish()
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<M5ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`m5erkeyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5erkeyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RAMCFG:M5ERKEYR)*/
pub struct M5ERKEYRrs;
impl crate::RegisterSpec for M5ERKEYRrs {
    type Ux = u32;
}
///`read()` method returns [`m5erkeyr::R`](R) reader structure
impl crate::Readable for M5ERKEYRrs {}
///`write(|w| ..)` method takes [`m5erkeyr::W`](W) writer structure
impl crate::Writable for M5ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5ERKEYR to value 0
impl crate::Resettable for M5ERKEYRrs {}
