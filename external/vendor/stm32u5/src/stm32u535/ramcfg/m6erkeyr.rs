///Register `M6ERKEYR` reader
pub type R = crate::R<M6ERKEYRrs>;
///Register `M6ERKEYR` writer
pub type W = crate::W<M6ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M6ERKEYR").finish()
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<M6ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`m6erkeyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6erkeyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RAMCFG:M6ERKEYR)*/
pub struct M6ERKEYRrs;
impl crate::RegisterSpec for M6ERKEYRrs {
    type Ux = u32;
}
///`read()` method returns [`m6erkeyr::R`](R) reader structure
impl crate::Readable for M6ERKEYRrs {}
///`write(|w| ..)` method takes [`m6erkeyr::W`](W) writer structure
impl crate::Writable for M6ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M6ERKEYR to value 0
impl crate::Resettable for M6ERKEYRrs {}
