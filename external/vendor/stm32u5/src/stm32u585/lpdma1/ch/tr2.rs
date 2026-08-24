///Register `TR2` reader
pub type R = crate::R<TR2rs>;
///Register `TR2` writer
pub type W = crate::W<TR2rs>;
///Field `REQSEL` reader - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\] value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_R = crate::FieldReader;
///Field `REQSEL` writer - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\] value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SWREQ` reader - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
pub type SWREQ_R = crate::BitReader;
///Field `SWREQ` writer - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREQ` reader - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_R = crate::BitReader;
///Field `BREQ` writer - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\] = 0b00 or 0b11, these TRIGM\[1:0\] bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\] = 0b01 or respectively TRIGPOL\[1:0\] = 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\] is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\] or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
pub type TRIGM_R = crate::FieldReader;
///Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\] = 0b00 or 0b11, these TRIGM\[1:0\] bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\] = 0b01 or respectively TRIGPOL\[1:0\] = 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\] is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\] or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\] = 00.
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\] = 00.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
pub type TRIGPOL_R = crate::FieldReader;
///Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
pub type TCEM_R = crate::FieldReader;
///Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:4 - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\] value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 9 - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\] = 0b00 or 0b11, these TRIGM\[1:0\] bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\] = 0b01 or respectively TRIGPOL\[1:0\] = 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\] is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\] or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:20 - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\] = 00.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
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
    ///Bits 0:4 - DMA hardware request selection These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per request. Note: The user must not assign a same input hardware request (same REQSEL\[4:0\] value) to different active DMA channels (LPDMA_CxCR.EN = 1 and LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<TR2rs> {
        REQSEL_W::new(self, 0)
    }
    ///Bit 9 - software request This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<TR2rs> {
        SWREQ_W::new(self, 9)
    }
    ///Bit 11 - block hardware request If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&mut self) -> BREQ_W<TR2rs> {
        BREQ_W::new(self, 11)
    }
    ///Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\] = 0b00 or 0b11, these TRIGM\[1:0\] bits are ignored. Else, a DMA transfer is conditioned by at least one trigger hit: The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\] = 0b01 or respectively TRIGPOL\[1:0\] = 0b10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[4:0\] is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of TRIGSEL\[4:0\] or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled (LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun.
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W<TR2rs> {
        TRIGM_W::new(self, 14)
    }
    ///Bits 16:20 - trigger event input selection These bits select the trigger event input of the LPDMA transfer (as per Programmed LPDMA1 trigger), with an active trigger event if TRIGPOL\[1:0\] = 00.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[4:0\].
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    ///Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with LPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
    #[inline(always)]
    pub fn tcem(&mut self) -> TCEM_W<TR2rs> {
        TCEM_W::new(self, 30)
    }
}
/**LPDMA channel 0 transfer register 2

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
