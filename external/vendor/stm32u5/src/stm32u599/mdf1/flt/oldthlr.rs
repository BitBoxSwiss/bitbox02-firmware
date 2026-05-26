///Register `OLDTHLR` reader
pub type R = crate::R<OLDTHLRrs>;
///Register `OLDTHLR` writer
pub type W = crate::W<OLDTHLRrs>;
///Field `OLDTHL` reader - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type OLDTHL_R = crate::FieldReader<u32>;
///Field `OLDTHL` writer - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type OLDTHL_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn oldthl(&self) -> OLDTHL_R {
        OLDTHL_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OLDTHLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<OLDTHLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`oldthlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oldthlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OLDTHLRrs;
impl crate::RegisterSpec for OLDTHLRrs {
    type Ux = u32;
}
///`read()` method returns [`oldthlr::R`](R) reader structure
impl crate::Readable for OLDTHLRrs {}
///`write(|w| ..)` method takes [`oldthlr::W`](W) writer structure
impl crate::Writable for OLDTHLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLDTHLR to value 0
impl crate::Resettable for OLDTHLRrs {}
