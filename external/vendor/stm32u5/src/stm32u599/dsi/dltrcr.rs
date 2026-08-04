///Register `DLTRCR` reader
pub type R = crate::R<DLTRCRrs>;
///Register `DLTRCR` writer
pub type W = crate::W<DLTRCRrs>;
///Field `MRD_TIME` reader - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
pub type MRD_TIME_R = crate::FieldReader<u16>;
///Field `MRD_TIME` writer - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
pub type MRD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLTRCR")
            .field("mrd_time", &self.mrd_time())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<DLTRCRrs> {
        MRD_TIME_W::new(self, 0)
    }
}
/**DSI Host data lane timer read configuration register

You can [`read`](crate::Reg::read) this register and get [`dltrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dltrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DLTRCR)*/
pub struct DLTRCRrs;
impl crate::RegisterSpec for DLTRCRrs {
    type Ux = u32;
}
///`read()` method returns [`dltrcr::R`](R) reader structure
impl crate::Readable for DLTRCRrs {}
///`write(|w| ..)` method takes [`dltrcr::W`](W) writer structure
impl crate::Writable for DLTRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLTRCR to value 0
impl crate::Resettable for DLTRCRrs {}
