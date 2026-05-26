///Register `OLDTHHR` reader
pub type R = crate::R<OLDTHHRrs>;
///Register `OLDTHHR` writer
pub type W = crate::W<OLDTHHRrs>;
///Field `OLDTHH` reader - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
pub type OLDTHH_R = crate::FieldReader<u32>;
///Field `OLDTHH` writer - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
pub type OLDTHH_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
    #[inline(always)]
    pub fn oldthh(&self) -> OLDTHH_R {
        OLDTHH_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OLDTHHR")
            .field("oldthh", &self.oldthh())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
    #[inline(always)]
    pub fn oldthh(&mut self) -> OLDTHH_W<OLDTHHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
/**This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`oldthhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oldthhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OLDTHHRrs;
impl crate::RegisterSpec for OLDTHHRrs {
    type Ux = u32;
}
///`read()` method returns [`oldthhr::R`](R) reader structure
impl crate::Readable for OLDTHHRrs {}
///`write(|w| ..)` method takes [`oldthhr::W`](W) writer structure
impl crate::Writable for OLDTHHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLDTHHR to value 0
impl crate::Resettable for OLDTHHRrs {}
