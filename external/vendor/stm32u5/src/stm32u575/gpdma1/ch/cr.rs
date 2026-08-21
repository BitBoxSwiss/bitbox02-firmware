///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
pub type EN_R = crate::BitReader;
///Field `EN` writer - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET` writer - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in ).
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in .
pub type SUSP_R = crate::BitReader;
///Field `SUSP` writer - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in .
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - transfer complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - half transfer complete interrupt enable
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - half transfer complete interrupt enable
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEIE` reader - data transfer error interrupt enable
pub type DTEIE_R = crate::BitReader;
///Field `DTEIE` writer - data transfer error interrupt enable
pub type DTEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULEIE` reader - update link transfer error interrupt enable
pub type ULEIE_R = crate::BitReader;
///Field `ULEIE` writer - update link transfer error interrupt enable
pub type ULEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USEIE` reader - user setting error interrupt enable
pub type USEIE_R = crate::BitReader;
///Field `USEIE` writer - user setting error interrupt enable
pub type USEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPIE` reader - completed suspension interrupt enable
pub type SUSPIE_R = crate::BitReader;
///Field `SUSPIE` writer - completed suspension interrupt enable
pub type SUSPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOIE` reader - trigger overrun interrupt enable
pub type TOIE_R = crate::BitReader;
///Field `TOIE` writer - trigger overrun interrupt enable
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSM` reader - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\[15:0\] = 0 and GPDMA_CxBR1.BRC\[10:0\] = 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
pub type LSM_R = crate::BitReader;
///Field `LSM` writer - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\[15:0\] = 0 and GPDMA_CxBR1.BRC\[10:0\] = 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LAP` reader - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
pub type LAP_R = crate::BitReader;
///Field `LAP` writer - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
pub type LAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIO` reader - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type PRIO_R = crate::FieldReader;
///Field `PRIO` writer - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in .
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\[15:0\] = 0 and GPDMA_CxBR1.BRC\[10:0\] = 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    #[inline(always)]
    pub fn lap(&self) -> LAP_R {
        LAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("susp", &self.susp())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("dteie", &self.dteie())
            .field("uleie", &self.uleie())
            .field("useie", &self.useie())
            .field("suspie", &self.suspie())
            .field("toie", &self.toie())
            .field("lsm", &self.lsm())
            .field("lap", &self.lap())
            .field("prio", &self.prio())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in ).
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<CRrs> {
        RESET_W::new(self, 1)
    }
    ///Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in .
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<CRrs> {
        SUSP_W::new(self, 2)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 8)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<CRrs> {
        HTIE_W::new(self, 9)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&mut self) -> DTEIE_W<CRrs> {
        DTEIE_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&mut self) -> ULEIE_W<CRrs> {
        ULEIE_W::new(self, 11)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&mut self) -> USEIE_W<CRrs> {
        USEIE_W::new(self, 12)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&mut self) -> SUSPIE_W<CRrs> {
        SUSPIE_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<CRrs> {
        TOIE_W::new(self, 14)
    }
    ///Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\[15:0\] = 0 and GPDMA_CxBR1.BRC\[10:0\] = 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W<CRrs> {
        LSM_W::new(self, 16)
    }
    ///Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.
    #[inline(always)]
    pub fn lap(&mut self) -> LAP_W<CRrs> {
        LAP_W::new(self, 17)
    }
    ///Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<CRrs> {
        PRIO_W::new(self, 22)
    }
}
/**GPDMA channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
