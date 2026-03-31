///Register `TXBC` reader
pub type R = crate::R<TXBCrs>;
///Register `TXBC` writer
pub type W = crate::W<TXBCrs>;
///Field `TFQM` reader - Tx FIFO/Queue Mode
pub type TFQM_R = crate::BitReader;
///Field `TFQM` writer - Tx FIFO/Queue Mode
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBC").field("tfqm", &self.tfqm()).finish()
    }
}
impl W {
    ///Bit 24 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W<TXBCrs> {
        TFQM_W::new(self, 24)
    }
}
/**FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FDCAN1_RAM:TXBC)*/
pub struct TXBCrs;
impl crate::RegisterSpec for TXBCrs {
    type Ux = u32;
}
///`read()` method returns [`txbc::R`](R) reader structure
impl crate::Readable for TXBCrs {}
///`write(|w| ..)` method takes [`txbc::W`](W) writer structure
impl crate::Writable for TXBCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBC to value 0
impl crate::Resettable for TXBCrs {}
