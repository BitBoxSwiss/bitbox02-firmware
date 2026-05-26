///Register `BR1` reader
pub type R = crate::R<BR1rs>;
///Register `BR1` writer
pub type W = crate::W<BR1rs>;
///Field `BNDT` reader - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\] = 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BNDT_R = crate::FieldReader<u16>;
///Field `BNDT` writer - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\] = 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\] = 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BR1").field("bndt", &self.bndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\] = 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\] = 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\] versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<BR1rs> {
        BNDT_W::new(self, 0)
    }
}
/**LPDMA channel 0 block register 1

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
