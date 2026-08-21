///Register `BSMXCR` reader
pub type R = crate::R<BSMXCRrs>;
///Register `BSMXCR` writer
pub type W = crate::W<BSMXCRrs>;
///Field `BSSEL` reader - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BSSEL_R = crate::FieldReader;
///Field `BSSEL` writer - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BSMXACTIVE` reader - BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\[4:0\] can only be updated when the BSMXACTIVE is set . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set in order update BSSEL\[4:0\] field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured.
pub type BSMXACTIVE_R = crate::BitReader;
impl R {
    ///Bits 0:4 - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bssel(&self) -> BSSEL_R {
        BSSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 31 - BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\[4:0\] can only be updated when the BSMXACTIVE is set . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set in order update BSSEL\[4:0\] field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn bsmxactive(&self) -> BSMXACTIVE_R {
        BSMXACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSMXCR")
            .field("bssel", &self.bssel())
            .field("bsmxactive", &self.bsmxactive())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\[x\]_F having the higher index number. - 00000: The bitstream bs\[0\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\[0\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\[1\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\[1\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\[15\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\[15\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bssel(&mut self) -> BSSEL_W<BSMXCRrs> {
        BSSEL_W::new(self, 0)
    }
}
/**This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`bsmxcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmxcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BSMXCRrs;
impl crate::RegisterSpec for BSMXCRrs {
    type Ux = u32;
}
///`read()` method returns [`bsmxcr::R`](R) reader structure
impl crate::Readable for BSMXCRrs {}
///`write(|w| ..)` method takes [`bsmxcr::W`](W) writer structure
impl crate::Writable for BSMXCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSMXCR to value 0
impl crate::Resettable for BSMXCRrs {}
