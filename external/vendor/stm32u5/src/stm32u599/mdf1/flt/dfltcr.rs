///Register `DFLTCR` reader
pub type R = crate::R<DFLTCRrs>;
///Register `DFLTCR` writer
pub type W = crate::W<DFLTCRrs>;
///Field `DFLTEN` writer - Digital Filter Enable Set and cleared by software. This bit is used to control the start of acquisition of the corresponding digital filter path. The behavior of this bit depends on ACQMOD and external events. or the acquisition starts when the proper trigger event occurs if ACQMOD = 01x . The serial or parallel interface delivering the samples shall be enabled as well. - 0: The acquisition is stopped immediately - 1: The acquisition is immediately started if ACQMOD = 00x or 1xx ,
pub type DFLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTH` reader - RXFIFO Threshold selection Set and cleared by software.
pub type FTH_R = crate::BitReader;
///Field `FTH` writer - RXFIFO Threshold selection Set and cleared by software.
pub type FTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACQMOD` reader - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACQMOD_R = crate::FieldReader;
///Field `ACQMOD` writer - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACQMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRGSENS` reader - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSENS_R = crate::BitReader;
///Field `TRGSENS` writer - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGSRC` reader - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\] is selected ... - 1111: mdf_trg\[13\] is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSRC_R = crate::FieldReader;
///Field `TRGSRC` writer - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\] is selected ... - 1111: mdf_trg\[13\] is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SNPSFMT` reader - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\] of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SNPSFMT_R = crate::BitReader;
///Field `SNPSFMT` writer - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\] of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SNPSFMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBDIS` reader - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type NBDIS_R = crate::FieldReader;
///Field `NBDIS` writer - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type NBDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DFLTRUN` reader - Digital filter Run Status Flag Set and cleared by hardware. This bit indicates if the digital filter is running or not. - 0: The digital filter is not running, and ready to accept a new trigger event - 1: The digital filter is running
pub type DFLTRUN_R = crate::BitReader;
///Field `DFLTACTIVE` reader - Digital filter Active Flag Set and cleared by hardware. This bit indicates if the digital filter is active: can be running or waiting for events. - 0: The digital filter is not active, and can be re-enabled again (via DFLTEN bit) if needed - 1: The digital filter is active
pub type DFLTACTIVE_R = crate::BitReader;
impl R {
    ///Bit 1 - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO Threshold selection Set and cleared by software.
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\] is selected ... - 1111: mdf_trg\[13\] is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\] of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn snpsfmt(&self) -> SNPSFMT_R {
        SNPSFMT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:27 - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - Digital filter Run Status Flag Set and cleared by hardware. This bit indicates if the digital filter is running or not. - 0: The digital filter is not running, and ready to accept a new trigger event - 1: The digital filter is running
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Digital filter Active Flag Set and cleared by hardware. This bit indicates if the digital filter is active: can be running or waiting for events. - 0: The digital filter is not active, and can be re-enabled again (via DFLTEN bit) if needed - 1: The digital filter is active
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTCR")
            .field("dmaen", &self.dmaen())
            .field("fth", &self.fth())
            .field("acqmod", &self.acqmod())
            .field("trgsens", &self.trgsens())
            .field("trgsrc", &self.trgsrc())
            .field("snpsfmt", &self.snpsfmt())
            .field("nbdis", &self.nbdis())
            .field("dfltrun", &self.dfltrun())
            .field("dfltactive", &self.dfltactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Digital Filter Enable Set and cleared by software. This bit is used to control the start of acquisition of the corresponding digital filter path. The behavior of this bit depends on ACQMOD and external events. or the acquisition starts when the proper trigger event occurs if ACQMOD = 01x . The serial or parallel interface delivering the samples shall be enabled as well. - 0: The acquisition is stopped immediately - 1: The acquisition is immediately started if ACQMOD = 00x or 1xx ,
    #[inline(always)]
    pub fn dflten(&mut self) -> DFLTEN_W<DFLTCRrs> {
        DFLTEN_W::new(self, 0)
    }
    ///Bit 1 - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<DFLTCRrs> {
        DMAEN_W::new(self, 1)
    }
    ///Bit 2 - RXFIFO Threshold selection Set and cleared by software.
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<DFLTCRrs> {
        FTH_W::new(self, 2)
    }
    ///Bits 4:6 - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acqmod(&mut self) -> ACQMOD_W<DFLTCRrs> {
        ACQMOD_W::new(self, 4)
    }
    ///Bit 8 - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn trgsens(&mut self) -> TRGSENS_W<DFLTCRrs> {
        TRGSENS_W::new(self, 8)
    }
    ///Bits 12:15 - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\] is selected ... - 1111: mdf_trg\[13\] is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W<DFLTCRrs> {
        TRGSRC_W::new(self, 12)
    }
    ///Bit 16 - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\] of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn snpsfmt(&mut self) -> SNPSFMT_W<DFLTCRrs> {
        SNPSFMT_W::new(self, 16)
    }
    ///Bits 20:27 - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn nbdis(&mut self) -> NBDIS_W<DFLTCRrs> {
        NBDIS_W::new(self, 20)
    }
}
/**This register is used to control the digital filter x.

You can [`read`](crate::Reg::read) this register and get [`dfltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTCRrs;
impl crate::RegisterSpec for DFLTCRrs {
    type Ux = u32;
}
///`read()` method returns [`dfltcr::R`](R) reader structure
impl crate::Readable for DFLTCRrs {}
///`write(|w| ..)` method takes [`dfltcr::W`](W) writer structure
impl crate::Writable for DFLTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLTCR to value 0
impl crate::Resettable for DFLTCRrs {}
