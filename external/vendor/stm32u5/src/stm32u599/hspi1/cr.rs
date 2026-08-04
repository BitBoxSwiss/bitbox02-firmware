///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` reader - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0.
pub type ABORT_R = crate::BitReader;
///Field `ABORT` writer - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0.
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA enable In Indirect mode, the DMA can be used to input or output data via HSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation.
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA enable In Indirect mode, the DMA can be used to input or output data via HSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCEN` reader - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\[1:0\]Â =Â 11) is selected. This bit enables the timeout counter.
pub type TCEN_R = crate::BitReader;
///Field `TCEN` writer - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\[1:0\]Â =Â 11) is selected. This bit enables the timeout counter.
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMM` reader - Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity
pub type DMM_R = crate::BitReader;
///Field `DMM` writer - Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity
pub type DMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSEL` reader - Memory select This bit is the mirror of bit 30. Refer to the description of MSEL\[1:0\] above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0.
pub type FSEL_R = crate::BitReader;
///Field `FSEL` writer - Memory select This bit is the mirror of bit 30. Refer to the description of MSEL\[1:0\] above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0.
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTHRES` reader - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in HSPI_SR, to be set. ... Note: If DMAENÂ =Â 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\[5:0\] value.
pub type FTHRES_R = crate::FieldReader;
///Field `FTHRES` writer - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in HSPI_SR, to be set. ... Note: If DMAENÂ =Â 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\[5:0\] value.
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TEIE` reader - Transfer error interrupt enable This bit enables the transfer error interrupt.
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error interrupt enable This bit enables the transfer error interrupt.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTIE` reader - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
pub type FTIE_R = crate::BitReader;
///Field `FTIE` writer - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMIE` reader - Status match interrupt enable This bit enables the status match interrupt.
pub type SMIE_R = crate::BitReader;
///Field `SMIE` writer - Status match interrupt enable This bit enables the status match interrupt.
pub type SMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOIE` reader - Timeout interrupt enable This bit enables the timeout interrupt.
pub type TOIE_R = crate::BitReader;
///Field `TOIE` writer - Timeout interrupt enable This bit enables the timeout interrupt.
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APMS` reader - Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match.
pub type APMS_R = crate::BitReader;
///Field `APMS` writer - Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match.
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMM` reader - Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode.
pub type PMM_R = crate::BitReader;
///Field `PMM` writer - Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode.
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMODE` reader - Functional mode This field defines the HSPI functional mode of operation. If DMAENÂ =Â 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\[1:0\] value. If FMODE\[1:0\] and FTHRES\[4:0\] are wrongly updated while DMAENÂ =Â 1, the DMA request signal automatically goes to inactive state.
pub type FMODE_R = crate::FieldReader;
///Field `FMODE` writer - Functional mode This field defines the HSPI functional mode of operation. If DMAENÂ =Â 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\[1:0\] value. If FMODE\[1:0\] and FTHRES\[4:0\] are wrongly updated while DMAENÂ =Â 1, the DMA request signal automatically goes to inactive state.
pub type FMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSEL` reader - Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in singleâmemory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO\[7:0\] 1x: data exchanged over IO\[15:8\] These bits are ignored when in dual-octal configuration (data on 8 bits and DMMÂ =Â 1) or 16âbit configuration (data exchanged over IO\[15:0\]).
pub type MSEL_R = crate::FieldReader;
///Field `MSEL` writer - Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in singleâmemory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO\[7:0\] 1x: data exchanged over IO\[15:8\] These bits are ignored when in dual-octal configuration (data on 8 bits and DMMÂ =Â 1) or 16âbit configuration (data exchanged over IO\[15:0\]).
pub type MSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0.
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA enable In Indirect mode, the DMA can be used to input or output data via HSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\[1:0\]Â =Â 11) is selected. This bit enables the timeout counter.
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Memory select This bit is the mirror of bit 30. Refer to the description of MSEL\[1:0\] above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0.
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in HSPI_SR, to be set. ... Note: If DMAENÂ =Â 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\[5:0\] value.
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Status match interrupt enable This bit enables the status match interrupt.
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timeout interrupt enable This bit enables the timeout interrupt.
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match.
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode.
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 28:29 - Functional mode This field defines the HSPI functional mode of operation. If DMAENÂ =Â 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\[1:0\] value. If FMODE\[1:0\] and FTHRES\[4:0\] are wrongly updated while DMAENÂ =Â 1, the DMA request signal automatically goes to inactive state.
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in singleâmemory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO\[7:0\] 1x: data exchanged over IO\[15:8\] These bits are ignored when in dual-octal configuration (data on 8 bits and DMMÂ =Â 1) or 16âbit configuration (data exchanged over IO\[15:0\]).
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("abort", &self.abort())
            .field("dmaen", &self.dmaen())
            .field("tcen", &self.tcen())
            .field("dmm", &self.dmm())
            .field("fsel", &self.fsel())
            .field("fthres", &self.fthres())
            .field("teie", &self.teie())
            .field("tcie", &self.tcie())
            .field("ftie", &self.ftie())
            .field("smie", &self.smie())
            .field("toie", &self.toie())
            .field("apms", &self.apms())
            .field("pmm", &self.pmm())
            .field("fmode", &self.fmode())
            .field("msel", &self.msel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0.
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 1)
    }
    ///Bit 2 - DMA enable In Indirect mode, the DMA can be used to input or output data via HSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 2)
    }
    ///Bit 3 - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\[1:0\]Â =Â 11) is selected. This bit enables the timeout counter.
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<CRrs> {
        TCEN_W::new(self, 3)
    }
    ///Bit 6 - Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity
    #[inline(always)]
    pub fn dmm(&mut self) -> DMM_W<CRrs> {
        DMM_W::new(self, 6)
    }
    ///Bit 7 - Memory select This bit is the mirror of bit 30. Refer to the description of MSEL\[1:0\] above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0.
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W<CRrs> {
        FSEL_W::new(self, 7)
    }
    ///Bits 8:13 - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in HSPI_SR, to be set. ... Note: If DMAENÂ =Â 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\[5:0\] value.
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W<CRrs> {
        FTHRES_W::new(self, 8)
    }
    ///Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 16)
    }
    ///Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 17)
    }
    ///Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W<CRrs> {
        FTIE_W::new(self, 18)
    }
    ///Bit 19 - Status match interrupt enable This bit enables the status match interrupt.
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W<CRrs> {
        SMIE_W::new(self, 19)
    }
    ///Bit 20 - Timeout interrupt enable This bit enables the timeout interrupt.
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<CRrs> {
        TOIE_W::new(self, 20)
    }
    ///Bit 22 - Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match.
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W<CRrs> {
        APMS_W::new(self, 22)
    }
    ///Bit 23 - Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode.
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W<CRrs> {
        PMM_W::new(self, 23)
    }
    ///Bits 28:29 - Functional mode This field defines the HSPI functional mode of operation. If DMAENÂ =Â 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\[1:0\] value. If FMODE\[1:0\] and FTHRES\[4:0\] are wrongly updated while DMAENÂ =Â 1, the DMA request signal automatically goes to inactive state.
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W<CRrs> {
        FMODE_W::new(self, 28)
    }
    ///Bits 30:31 - Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in singleâmemory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO\[7:0\] 1x: data exchanged over IO\[15:8\] These bits are ignored when in dual-octal configuration (data on 8 bits and DMMÂ =Â 1) or 16âbit configuration (data exchanged over IO\[15:0\]).
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<CRrs> {
        MSEL_W::new(self, 30)
    }
}
/**HSPI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
