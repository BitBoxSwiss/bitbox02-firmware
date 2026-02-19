///Register `WVPCR` reader
pub type R = crate::R<WVPCRrs>;
///Register `WVPCR` writer
pub type W = crate::W<WVPCRrs>;
///Field `WVSTPOS` reader - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\] must be ≤ AAH\[10:0\] bits (programmed in LTDC_AWCR register).
pub type WVSTPOS_R = crate::FieldReader<u16>;
///Field `WVSTPOS` writer - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\] must be ≤ AAH\[10:0\] bits (programmed in LTDC_AWCR register).
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `WVSPPOS` reader - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\] must be ≥ AVBP\[10:0\] bits + 1 (programmed in LTDC_BPCR register).
pub type WVSPPOS_R = crate::FieldReader<u16>;
///Field `WVSPPOS` writer - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\] must be ≥ AVBP\[10:0\] bits + 1 (programmed in LTDC_BPCR register).
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\] must be ≤ AAH\[10:0\] bits (programmed in LTDC_AWCR register).
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\] must be ≥ AVBP\[10:0\] bits + 1 (programmed in LTDC_BPCR register).
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WVPCR")
            .field("wvstpos", &self.wvstpos())
            .field("wvsppos", &self.wvsppos())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\] must be ≤ AAH\[10:0\] bits (programmed in LTDC_AWCR register).
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    ///Bits 16:26 - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\] must be ≥ AVBP\[10:0\] bits + 1 (programmed in LTDC_BPCR register).
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
/**LTDC layer 1 window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`wvpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wvpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WVPCRrs;
impl crate::RegisterSpec for WVPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wvpcr::R`](R) reader structure
impl crate::Readable for WVPCRrs {}
///`write(|w| ..)` method takes [`wvpcr::W`](W) writer structure
impl crate::Writable for WVPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WVPCR to value 0
impl crate::Resettable for WVPCRrs {}
