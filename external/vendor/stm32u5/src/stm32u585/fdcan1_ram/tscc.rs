///Register `TSCC` reader
pub type R = crate::R<TSCCrs>;
///Register `TSCC` writer
pub type W = crate::W<TSCCrs>;
///Field `TSS` reader - Timestamp Select
pub type TSS_R = crate::FieldReader;
///Field `TSS` writer - Timestamp Select
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCP` reader - Timestamp Counter Prescaler
pub type TCP_R = crate::FieldReader;
///Field `TCP` writer - Timestamp Counter Prescaler
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Timestamp Select
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:19 - Timestamp Counter Prescaler
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCC")
            .field("tcp", &self.tcp())
            .field("tss", &self.tss())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timestamp Select
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<TSCCrs> {
        TSS_W::new(self, 0)
    }
    ///Bits 16:19 - Timestamp Counter Prescaler
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W<TSCCrs> {
        TCP_W::new(self, 16)
    }
}
/**FDCAN Timestamp Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TSCC)*/
pub struct TSCCrs;
impl crate::RegisterSpec for TSCCrs {
    type Ux = u32;
}
///`read()` method returns [`tscc::R`](R) reader structure
impl crate::Readable for TSCCrs {}
///`write(|w| ..)` method takes [`tscc::W`](W) writer structure
impl crate::Writable for TSCCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCC to value 0
impl crate::Resettable for TSCCrs {}
