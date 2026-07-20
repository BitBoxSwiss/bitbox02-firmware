///Register `TR1` reader
pub type R = crate::R<TR1rs>;
///Register `TR1` writer
pub type W = crate::W<TR1rs>;
///Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: a source block size must be a multiple of the source data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: A source burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxSAR\[2:0\] and address offset LPDMA_CxTR3.SAO\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_R = crate::FieldReader;
///Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: a source block size must be a multiple of the source data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: A source burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxSAR\[2:0\] and address offset LPDMA_CxTR3.SAO\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINC` reader - source incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe source address, pointed by DMA_CxSAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_R = crate::BitReader;
///Field `SINC` writer - source incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe source address, pointed by DMA_CxSAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAM` reader - PAM
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - PAM
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SSEC` reader - security attribute of the DMA transfer from the sourceThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer from the source is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
pub type SSEC_R = crate::BitReader;
///Field `SSEC` writer - security attribute of the DMA transfer from the sourceThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer from the source is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: A destination burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxDAR\[2:0\] and address offset LPDMA_CxTR3.DAO\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: When configured in packing mode (i.e. if PAM\[1\]=1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type DDW_LOG2_R = crate::FieldReader;
///Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: A destination burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxDAR\[2:0\] and address offset LPDMA_CxTR3.DAO\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: When configured in packing mode (i.e. if PAM\[1\]=1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type DDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - destination incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe destination address, pointed by DMA_CxDAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type DINC_R = crate::BitReader;
///Field `DINC` writer - destination incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe destination address, pointed by DMA_CxDAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEC` reader - security attribute of the DMA transfer to the destinationThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer to the destination is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
pub type DSEC_R = crate::BitReader;
///Field `DSEC` writer - security attribute of the DMA transfer to the destinationThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer to the destination is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - binary logarithm of the source data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: a source block size must be a multiple of the source data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: A source burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxSAR\[2:0\] and address offset LPDMA_CxTR3.SAO\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe source address, pointed by DMA_CxSAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 11:12 - PAM
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 15 - security attribute of the DMA transfer from the sourceThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer from the source is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: A destination burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxDAR\[2:0\] and address offset LPDMA_CxTR3.DAO\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: When configured in packing mode (i.e. if PAM\[1\]=1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe destination address, pointed by DMA_CxDAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 31 - security attribute of the DMA transfer to the destinationThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer to the destination is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR1")
            .field("sdw_log2", &self.sdw_log2())
            .field("sinc", &self.sinc())
            .field("pam", &self.pam())
            .field("ssec", &self.ssec())
            .field("ddw_log2", &self.ddw_log2())
            .field("dinc", &self.dinc())
            .field("dsec", &self.dsec())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - binary logarithm of the source data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: a source block size must be a multiple of the source data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: A source burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxSAR\[2:0\] and address offset LPDMA_CxTR3.SAO\[2:0\] vs SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    ///Bit 3 - source incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe source address, pointed by DMA_CxSAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<TR1rs> {
        SINC_W::new(self, 3)
    }
    ///Bits 11:12 - PAM
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<TR1rs> {
        PAM_W::new(self, 11)
    }
    ///Bit 15 - security attribute of the DMA transfer from the sourceThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer from the source is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
    #[inline(always)]
    pub fn ssec(&mut self) -> SSEC_W<TR1rs> {
        SSEC_W::new(self, 15)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes- 00: byte- 01: half-word (2 bytes)- 10: word (4 bytes)- 11: a user setting error is reported and no transfer is issued.Note: Setting a 8-byte data width is causing a user setting error to be reported and none transfer is issued.Note: A destination burst transfer must have an aligned address with its data width (c.f. start address LPDMA_CxDAR\[2:0\] and address offset LPDMA_CxTR3.DAO\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.Note: When configured in packing mode (i.e. if PAM\[1\]=1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (c.f. LPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    ///Bit 19 - destination incrementing burst- 0: fixed burst- 1: contiguously incremented burstThe destination address, pointed by DMA_CxDAR, is either kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<TR1rs> {
        DINC_W::new(self, 19)
    }
    ///Bit 31 - security attribute of the DMA transfer to the destinationThis is a secure register bit.This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx=1. A secure write is ignored when LPDMA_SECCFGR.SECx=0.When is de-asserted LPDMA_SECCFGR.SECx, this bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the DMA transfer to the destination is non-secure.If LPDMA_SECCFGR.SECx=1 (and a secure access):- 0: non-secure- 1: secure
    #[inline(always)]
    pub fn dsec(&mut self) -> DSEC_W<TR1rs> {
        DSEC_W::new(self, 31)
    }
}
/**LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TR1rs;
impl crate::RegisterSpec for TR1rs {
    type Ux = u32;
}
///`read()` method returns [`tr1::R`](R) reader structure
impl crate::Readable for TR1rs {}
///`write(|w| ..)` method takes [`tr1::W`](W) writer structure
impl crate::Writable for TR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR1 to value 0
impl crate::Resettable for TR1rs {}
