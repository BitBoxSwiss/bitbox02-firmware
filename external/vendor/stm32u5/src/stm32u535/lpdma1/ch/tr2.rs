///Register `TR2` reader
pub type R = crate::R<TR2rs>;
///Register `TR2` writer
pub type W = crate::W<TR2rs>;
///Field `REQSEL` reader - DMA hardware request selection If the channel x is activated (i.e. LPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\] value) to different active DMA channels (i.e. if LPDMA_CxCR.EN=1 and LPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_R = crate::FieldReader;
///Field `REQSEL` writer - DMA hardware request selection If the channel x is activated (i.e. LPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\] value) to different active DMA channels (i.e. if LPDMA_CxCR.EN=1 and LPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SWREQ` reader - Software request When LPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\] is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\] is ignored.
pub type SWREQ_R = crate::BitReader;
///Field `SWREQ` writer - Software request When LPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\] is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\] is ignored.
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREQ` reader - BREQ
pub type BREQ_R = crate::BitReader;
///Field `BREQ` writer - BREQ
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGM` reader - Trigger mode: Defines the transfer granularity for its conditioning by the trigger. If the channel x is enabled (i.e. when LPDMA_CxCR.EN is asserted) with TRIGPOL\[1:0\]=00 or 11, these bits are ignored. Else, a DMA transfer is conditioned by (at least) one trigger hit, either at: - 00: at block level (for channel x=12 to 15: for each block if a 2D/repeated block is configured i.e. if LPDMA_CxBR1.BRC\[10:0\]! = 0): the first burst read of a/each block transfer is conditioned by one hit trigger. - 01: at 2D/repeated block level for channel x=12 to 15; same as 00 for channel x=0 to 11
pub type TRIGM_R = crate::FieldReader;
///Field `TRIGM` writer - Trigger mode: Defines the transfer granularity for its conditioning by the trigger. If the channel x is enabled (i.e. when LPDMA_CxCR.EN is asserted) with TRIGPOL\[1:0\]=00 or 11, these bits are ignored. Else, a DMA transfer is conditioned by (at least) one trigger hit, either at: - 00: at block level (for channel x=12 to 15: for each block if a 2D/repeated block is configured i.e. if LPDMA_CxBR1.BRC\[10:0\]! = 0): the first burst read of a/each block transfer is conditioned by one hit trigger. - 01: at 2D/repeated block level for channel x=12 to 15; same as 00 for channel x=0 to 11
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRIGSEL` reader - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\] !=00.
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\] !=00.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIGPOL` reader - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
pub type TRIGPOL_R = crate::FieldReader;
///Field `TRIGPOL` writer - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCEM` reader - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when LPDMA_CxBR1.BRC\[10:0\]= 0 and LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address LPDMA_CxLLR.LA\[15:2\] to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the LPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated.
pub type TCEM_R = crate::FieldReader;
///Field `TCEM` writer - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when LPDMA_CxBR1.BRC\[10:0\]= 0 and LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address LPDMA_CxLLR.LA\[15:2\] to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the LPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated.
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:4 - DMA hardware request selection If the channel x is activated (i.e. LPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\] value) to different active DMA channels (i.e. if LPDMA_CxCR.EN=1 and LPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 9 - Software request When LPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\] is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\] is ignored.
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - BREQ
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 14:15 - Trigger mode: Defines the transfer granularity for its conditioning by the trigger. If the channel x is enabled (i.e. when LPDMA_CxCR.EN is asserted) with TRIGPOL\[1:0\]=00 or 11, these bits are ignored. Else, a DMA transfer is conditioned by (at least) one trigger hit, either at: - 00: at block level (for channel x=12 to 15: for each block if a 2D/repeated block is configured i.e. if LPDMA_CxBR1.BRC\[10:0\]! = 0): the first burst read of a/each block transfer is conditioned by one hit trigger. - 01: at 2D/repeated block level for channel x=12 to 15; same as 00 for channel x=0 to 11
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:20 - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\] !=00.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:25 - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 30:31 - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when LPDMA_CxBR1.BRC\[10:0\]= 0 and LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address LPDMA_CxLLR.LA\[15:2\] to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the LPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated.
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR2")
            .field("reqsel", &self.reqsel())
            .field("swreq", &self.swreq())
            .field("breq", &self.breq())
            .field("trigm", &self.trigm())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("tcem", &self.tcem())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMA hardware request selection If the channel x is activated (i.e. LPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\] value) to different active DMA channels (i.e. if LPDMA_CxCR.EN=1 and LPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<TR2rs> {
        REQSEL_W::new(self, 0)
    }
    ///Bit 9 - Software request When LPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\] is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\] is ignored.
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<TR2rs> {
        SWREQ_W::new(self, 9)
    }
    ///Bit 11 - BREQ
    #[inline(always)]
    pub fn breq(&mut self) -> BREQ_W<TR2rs> {
        BREQ_W::new(self, 11)
    }
    ///Bits 14:15 - Trigger mode: Defines the transfer granularity for its conditioning by the trigger. If the channel x is enabled (i.e. when LPDMA_CxCR.EN is asserted) with TRIGPOL\[1:0\]=00 or 11, these bits are ignored. Else, a DMA transfer is conditioned by (at least) one trigger hit, either at: - 00: at block level (for channel x=12 to 15: for each block if a 2D/repeated block is configured i.e. if LPDMA_CxBR1.BRC\[10:0\]! = 0): the first burst read of a/each block transfer is conditioned by one hit trigger. - 01: at 2D/repeated block level for channel x=12 to 15; same as 00 for channel x=0 to 11
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W<TR2rs> {
        TRIGM_W::new(self, 14)
    }
    ///Bits 16:20 - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\] !=00.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bits 24:25 - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    ///Bits 30:31 - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when LPDMA_CxBR1.BRC\[10:0\]= 0 and LPDMA_CxBR1.BNDT\[15:0\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address LPDMA_CxLLR.LA\[15:2\] to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the LPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated.
    #[inline(always)]
    pub fn tcem(&mut self) -> TCEM_W<TR2rs> {
        TCEM_W::new(self, 30)
    }
}
/**LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TR2rs;
impl crate::RegisterSpec for TR2rs {
    type Ux = u32;
}
///`read()` method returns [`tr2::R`](R) reader structure
impl crate::Readable for TR2rs {}
///`write(|w| ..)` method takes [`tr2::W`](W) writer structure
impl crate::Writable for TR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR2 to value 0
impl crate::Resettable for TR2rs {}
