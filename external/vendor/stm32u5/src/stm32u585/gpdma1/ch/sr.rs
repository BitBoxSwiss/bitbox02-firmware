///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `IDLEF` reader - idle flag This idle flag is de-asserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state).
pub type IDLEF_R = crate::BitReader;
///Field `TCF` reader - transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\[1:0\]).
pub type TCF_R = crate::BitReader;
///Field `HTF` reader - half transfer flag An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\[1:0\]). An half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\[15:0\]/2) has been transferred to the destination. An half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\[10:0\]+1)/2)) has been transferred to the destination.
pub type HTF_R = crate::BitReader;
///Field `DTEF` reader - data transfer error flag
pub type DTEF_R = crate::BitReader;
///Field `ULEF` reader - update link transfer error flag
pub type ULEF_R = crate::BitReader;
///Field `USEF` reader - user setting error flag
pub type USEF_R = crate::BitReader;
///Field `SUSPF` reader - completed suspension flag
pub type SUSPF_R = crate::BitReader;
///Field `TOF` reader - trigger overrun flag
pub type TOF_R = crate::BitReader;
///Field `FIFOL` reader - monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\[1:0\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\[7:0\], additionally to GPDMA_CxBR1.BDNT\[15:0\] and GPDMA_CxBR1.BRC\[10:0\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1).
pub type FIFOL_R = crate::FieldReader;
impl R {
    ///Bit 0 - idle flag This idle flag is de-asserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state).
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\[1:0\]).
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer flag An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\[1:0\]). An half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\[15:0\]/2) has been transferred to the destination. An half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\[10:0\]+1)/2)) has been transferred to the destination.
    #[inline(always)]
    pub fn htf(&self) -> HTF_R {
        HTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error flag
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error flag
    #[inline(always)]
    pub fn ulef(&self) -> ULEF_R {
        ULEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error flag
    #[inline(always)]
    pub fn usef(&self) -> USEF_R {
        USEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension flag
    #[inline(always)]
    pub fn suspf(&self) -> SUSPF_R {
        SUSPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\[1:0\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\[7:0\], additionally to GPDMA_CxBR1.BDNT\[15:0\] and GPDMA_CxBR1.BRC\[10:0\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1).
    #[inline(always)]
    pub fn fifol(&self) -> FIFOL_R {
        FIFOL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("idlef", &self.idlef())
            .field("tcf", &self.tcf())
            .field("htf", &self.htf())
            .field("dtef", &self.dtef())
            .field("ulef", &self.ulef())
            .field("usef", &self.usef())
            .field("suspf", &self.suspf())
            .field("tof", &self.tof())
            .field("fifol", &self.fifol())
            .finish()
    }
}
/**GPDMA channel 0 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
