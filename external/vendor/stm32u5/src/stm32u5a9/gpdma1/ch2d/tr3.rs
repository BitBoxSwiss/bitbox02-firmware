///Register `TR3` reader
pub type R = crate::R<TR3rs>;
///Register `TR3` writer
pub type W = crate::W<TR3rs>;
///Field `SAO` reader - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\] for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.SINC=1. Note: A source address offset must be aligned with the programmed data width of a source burst (c.f. SAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type SAO_R = crate::FieldReader<u16>;
///Field `SAO` writer - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\] for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.SINC=1. Note: A source address offset must be aligned with the programmed data width of a source burst (c.f. SAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type SAO_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `DAO` reader - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\] for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.DINC=1. Note: A destination address offset must be aligned with the programmed data width of a destination burst (c.f. DAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\] is not applied.
pub type DAO_R = crate::FieldReader<u16>;
///Field `DAO` writer - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\] for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.DINC=1. Note: A destination address offset must be aligned with the programmed data width of a destination burst (c.f. DAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\] is not applied.
pub type DAO_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\] for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.SINC=1. Note: A source address offset must be aligned with the programmed data width of a source burst (c.f. SAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\] for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.DINC=1. Note: A destination address offset must be aligned with the programmed data width of a destination burst (c.f. DAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\] is not applied.
    #[inline(always)]
    pub fn dao(&self) -> DAO_R {
        DAO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR3")
            .field("sao", &self.sao())
            .field("dao", &self.dao())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\] for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.SINC=1. Note: A source address offset must be aligned with the programmed data width of a source burst (c.f. SAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sao(&mut self) -> SAO_W<TR3rs> {
        SAO_W::new(self, 0)
    }
    ///Bits 16:28 - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\] for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode i.e. if GPDMA_CxTR1.DINC=1. Note: A destination address offset must be aligned with the programmed data width of a destination burst (c.f. DAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. Note: When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\] is not applied.
    #[inline(always)]
    pub fn dao(&mut self) -> DAO_W<TR3rs> {
        DAO_W::new(self, 16)
    }
}
/**GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`tr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TR3rs;
impl crate::RegisterSpec for TR3rs {
    type Ux = u32;
}
///`read()` method returns [`tr3::R`](R) reader structure
impl crate::Readable for TR3rs {}
///`write(|w| ..)` method takes [`tr3::W`](W) writer structure
impl crate::Writable for TR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR3 to value 0
impl crate::Resettable for TR3rs {}
