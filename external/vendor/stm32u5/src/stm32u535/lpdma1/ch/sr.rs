///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `IDLEF` reader - idle flag - 0: the channel is not in idle state - 1: the channel is in idle state This idle flag is de-asserted by hardware when the channel is enabled (i.e. is written 1 into LPDMA_CxCR.EN) with a valid channel configuration (i.e. no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (i.e. in suspended or disabled state).
pub type IDLEF_R = crate::BitReader;
///Field `TCF` reader - transfer complete flag - 0: no transfer complete event - 1: a transfer complete event occurred A transfer complete event is either a block transfer complete or a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode i.e. LPDMA_CxTR2.TCEM\[1:0\].
pub type TCF_R = crate::BitReader;
///Field `HTF` reader - half transfer flag - 0: no half transfer event - 1: an half transfer event occurred An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode i.e. LPDMA_CxTR2.TCEM\[1:0\]. An half block transfer occurs when half of the bytes of the source block size (i.e. rounded up integer of LPDMA_CxBR1.BNDT\[15:0\]/2) has been transferred to the destination. Half 2D/repeated block transfer occurs when half of the repeated blocks (i.e. rounded up integer of (LPDMA_CxBR1.BRC\[10:0\]+1)/2) have been transferred to the destination.
pub type HTF_R = crate::BitReader;
///Field `DTEF` reader - data transfer error flag - 0: no data transfer error event - 1: a master bus error event occurred on a data transfer
pub type DTEF_R = crate::BitReader;
///Field `ULEF` reader - update link transfer error flag - 0: no update link transfer error event - 1: a master bus error event occurred while updating a linked-list register from memory
pub type ULEF_R = crate::BitReader;
///Field `USEF` reader - user setting error flag - 0: no user setting error event - 1: a user setting error event occurred
pub type USEF_R = crate::BitReader;
///Field `SUSPF` reader - completed suspension flag - 0: no completed suspension event - 1: a completed suspension event occurred
pub type SUSPF_R = crate::BitReader;
impl R {
    ///Bit 0 - idle flag - 0: the channel is not in idle state - 1: the channel is in idle state This idle flag is de-asserted by hardware when the channel is enabled (i.e. is written 1 into LPDMA_CxCR.EN) with a valid channel configuration (i.e. no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (i.e. in suspended or disabled state).
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - transfer complete flag - 0: no transfer complete event - 1: a transfer complete event occurred A transfer complete event is either a block transfer complete or a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode i.e. LPDMA_CxTR2.TCEM\[1:0\].
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer flag - 0: no half transfer event - 1: an half transfer event occurred An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode i.e. LPDMA_CxTR2.TCEM\[1:0\]. An half block transfer occurs when half of the bytes of the source block size (i.e. rounded up integer of LPDMA_CxBR1.BNDT\[15:0\]/2) has been transferred to the destination. Half 2D/repeated block transfer occurs when half of the repeated blocks (i.e. rounded up integer of (LPDMA_CxBR1.BRC\[10:0\]+1)/2) have been transferred to the destination.
    #[inline(always)]
    pub fn htf(&self) -> HTF_R {
        HTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error flag - 0: no data transfer error event - 1: a master bus error event occurred on a data transfer
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error flag - 0: no update link transfer error event - 1: a master bus error event occurred while updating a linked-list register from memory
    #[inline(always)]
    pub fn ulef(&self) -> ULEF_R {
        ULEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error flag - 0: no user setting error event - 1: a user setting error event occurred
    #[inline(always)]
    pub fn usef(&self) -> USEF_R {
        USEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension flag - 0: no completed suspension event - 1: a completed suspension event occurred
    #[inline(always)]
    pub fn suspf(&self) -> SUSPF_R {
        SUSPF_R::new(((self.bits >> 13) & 1) != 0)
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
            .finish()
    }
}
/**channel x status register

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
