///Register `BR1` reader
pub type R = crate::R<BR1rs>;
///Register `BR1` writer
pub type W = crate::W<BR1rs>;
///Field `BNDT` reader - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\[1\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
pub type BNDT_R = crate::FieldReader<u16>;
///Field `BNDT` writer - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\[1\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BRC` reader - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\[10:0\] = BNDT\[15:0\] = 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer.
pub type BRC_R = crate::FieldReader<u16>;
///Field `BRC` writer - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\[10:0\] = BNDT\[15:0\] = 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer.
pub type BRC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `SDEC` reader - source address decrement
pub type SDEC_R = crate::BitReader;
///Field `SDEC` writer - source address decrement
pub type SDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDEC` reader - destination address decrement
pub type DDEC_R = crate::BitReader;
///Field `DDEC` writer - destination address decrement
pub type DDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRSDEC` reader - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer.
pub type BRSDEC_R = crate::BitReader;
///Field `BRSDEC` writer - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer.
pub type BRSDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRDDEC` reader - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer.
pub type BRDDEC_R = crate::BitReader;
///Field `BRDDEC` writer - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer.
pub type BRDDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\[1\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:26 - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\[10:0\] = BNDT\[15:0\] = 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer.
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 28 - source address decrement
    #[inline(always)]
    pub fn sdec(&self) -> SDEC_R {
        SDEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - destination address decrement
    #[inline(always)]
    pub fn ddec(&self) -> DDEC_R {
        DDEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer.
    #[inline(always)]
    pub fn brsdec(&self) -> BRSDEC_R {
        BRSDEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer.
    #[inline(always)]
    pub fn brddec(&self) -> BRDDEC_R {
        BRDDEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BR1")
            .field("bndt", &self.bndt())
            .field("brc", &self.brc())
            .field("sdec", &self.sdec())
            .field("ddec", &self.ddec())
            .field("brsdec", &self.brsdec())
            .field("brddec", &self.brddec())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\[1\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\[2:0\] versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<BR1rs> {
        BNDT_W::new(self, 0)
    }
    ///Bits 16:26 - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\[10:0\] = BNDT\[15:0\] = 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\[15:0\] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer.
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W<BR1rs> {
        BRC_W::new(self, 16)
    }
    ///Bit 28 - source address decrement
    #[inline(always)]
    pub fn sdec(&mut self) -> SDEC_W<BR1rs> {
        SDEC_W::new(self, 28)
    }
    ///Bit 29 - destination address decrement
    #[inline(always)]
    pub fn ddec(&mut self) -> DDEC_W<BR1rs> {
        DDEC_W::new(self, 29)
    }
    ///Bit 30 - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer.
    #[inline(always)]
    pub fn brsdec(&mut self) -> BRSDEC_W<BR1rs> {
        BRSDEC_W::new(self, 30)
    }
    ///Bit 31 - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer.
    #[inline(always)]
    pub fn brddec(&mut self) -> BRDDEC_W<BR1rs> {
        BRDDEC_W::new(self, 31)
    }
}
/**GPDMA channel 12 alternate block register 1

You can [`read`](crate::Reg::read) this register and get [`br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BR1rs;
impl crate::RegisterSpec for BR1rs {
    type Ux = u32;
}
///`read()` method returns [`br1::R`](R) reader structure
impl crate::Readable for BR1rs {}
///`write(|w| ..)` method takes [`br1::W`](W) writer structure
impl crate::Writable for BR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BR1 to value 0
impl crate::Resettable for BR1rs {}
