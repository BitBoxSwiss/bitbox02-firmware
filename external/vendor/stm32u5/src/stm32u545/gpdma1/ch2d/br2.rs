///Register `BR2` reader
pub type R = crate::R<BR2rs>;
///Register `BR2` writer
pub type W = crate::W<BR2rs>;
///Field `BRSAO` reader - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BRSAO_R = crate::FieldReader<u16>;
///Field `BRSAO` writer - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BRSAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BRDAO` reader - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BRDAO_R = crate::FieldReader<u16>;
///Field `BRDAO` writer - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BRDAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn brsao(&self) -> BRSAO_R {
        BRSAO_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn brdao(&self) -> BRDAO_R {
        BRDAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BR2")
            .field("brsao", &self.brsao())
            .field("brdao", &self.brdao())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\[2:0\] vs GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn brsao(&mut self) -> BRSAO_W<BR2rs> {
        BRSAO_W::new(self, 0)
    }
    ///Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\[15:0\] is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\[2:0\] vs GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn brdao(&mut self) -> BRDAO_W<BR2rs> {
        BRDAO_W::new(self, 16)
    }
}
/**GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`br2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BR2rs;
impl crate::RegisterSpec for BR2rs {
    type Ux = u32;
}
///`read()` method returns [`br2::R`](R) reader structure
impl crate::Readable for BR2rs {}
///`write(|w| ..)` method takes [`br2::W`](W) writer structure
impl crate::Writable for BR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BR2 to value 0
impl crate::Resettable for BR2rs {}
