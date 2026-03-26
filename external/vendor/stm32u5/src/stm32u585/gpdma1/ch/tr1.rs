///Register `TR1` reader
pub type R = crate::R<TR1rs>;
///Register `TR1` writer
pub type W = crate::W<TR1rs>;
///Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_R = crate::FieldReader;
///Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINC` reader - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_R = crate::BitReader;
///Field `SINC` writer - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBL_1` reader - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
pub type SBL_1_R = crate::FieldReader;
///Field `SBL_1` writer - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
pub type SBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PAM` reader - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer - Case 2: If destination data width < source data width 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note:
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer - Case 2: If destination data width < source data width 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note:
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SBX` reader - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
pub type SBX_R = crate::BitReader;
///Field `SBX` writer - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
pub type SBX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAP` reader - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type SAP_R = crate::BitReader;
///Field `SAP` writer - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type SAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSEC` reader - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure.
pub type SSEC_R = crate::BitReader;
///Field `SSEC` writer - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure.
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
pub type DDW_LOG2_R = crate::FieldReader;
///Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
pub type DDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type DINC_R = crate::BitReader;
///Field `DINC` writer - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBL_1` reader - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
pub type DBL_1_R = crate::FieldReader;
///Field `DBL_1` writer - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
pub type DBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DBX` reader - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
pub type DBX_R = crate::BitReader;
///Field `DBX` writer - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
pub type DBX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHX` reader - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
pub type DHX_R = crate::BitReader;
///Field `DHX` writer - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
pub type DHX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAP` reader - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type DAP_R = crate::BitReader;
///Field `DAP` writer - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type DAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEC` reader - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure.
pub type DSEC_R = crate::BitReader;
///Field `DSEC` writer - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure.
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn sbl_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 11:12 - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer - Case 2: If destination data width < source data width 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note:
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
    #[inline(always)]
    pub fn sbx(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn sap(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure.
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn dbl_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    #[inline(always)]
    pub fn dbx(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
    #[inline(always)]
    pub fn dhx(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure.
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
            .field("sbl_1", &self.sbl_1())
            .field("pam", &self.pam())
            .field("sbx", &self.sbx())
            .field("sap", &self.sap())
            .field("ssec", &self.ssec())
            .field("ddw_log2", &self.ddw_log2())
            .field("dinc", &self.dinc())
            .field("dbl_1", &self.dbl_1())
            .field("dbx", &self.dbx())
            .field("dhx", &self.dhx())
            .field("dap", &self.dap())
            .field("dsec", &self.dsec())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    ///Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<TR1rs> {
        SINC_W::new(self, 3)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn sbl_1(&mut self) -> SBL_1_W<TR1rs> {
        SBL_1_W::new(self, 4)
    }
    ///Bits 11:12 - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer - Case 2: If destination data width < source data width 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note:
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<TR1rs> {
        PAM_W::new(self, 11)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
    #[inline(always)]
    pub fn sbx(&mut self) -> SBX_W<TR1rs> {
        SBX_W::new(self, 13)
    }
    ///Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn sap(&mut self) -> SAP_W<TR1rs> {
        SAP_W::new(self, 14)
    }
    ///Bit 15 - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure.
    #[inline(always)]
    pub fn ssec(&mut self) -> SSEC_W<TR1rs> {
        SSEC_W::new(self, 15)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    ///Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<TR1rs> {
        DINC_W::new(self, 19)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn dbl_1(&mut self) -> DBL_1_W<TR1rs> {
        DBL_1_W::new(self, 20)
    }
    ///Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    #[inline(always)]
    pub fn dbx(&mut self) -> DBX_W<TR1rs> {
        DBX_W::new(self, 26)
    }
    ///Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
    #[inline(always)]
    pub fn dhx(&mut self) -> DHX_W<TR1rs> {
        DHX_W::new(self, 27)
    }
    ///Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn dap(&mut self) -> DAP_W<TR1rs> {
        DAP_W::new(self, 30)
    }
    ///Bit 31 - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure.
    #[inline(always)]
    pub fn dsec(&mut self) -> DSEC_W<TR1rs> {
        DSEC_W::new(self, 31)
    }
}
/**GPDMA channel 0 transfer register 1

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
