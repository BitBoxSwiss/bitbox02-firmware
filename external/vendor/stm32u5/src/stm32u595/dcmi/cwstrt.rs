///Register `CWSTRT` reader
pub type R = crate::R<CWSTRTrs>;
///Register `CWSTRT` writer
pub type W = crate::W<CWSTRTrs>;
///Field `HOFFCNT` reader - Horizontal offset count
pub type HOFFCNT_R = crate::FieldReader<u16>;
///Field `HOFFCNT` writer - Horizontal offset count
pub type HOFFCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
///Field `VST` reader - Vertical start line count
pub type VST_R = crate::FieldReader<u16>;
///Field `VST` writer - Vertical start line count
pub type VST_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 0:13 - Horizontal offset count
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:28 - Vertical start line count
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWSTRT")
            .field("vst", &self.vst())
            .field("hoffcnt", &self.hoffcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Horizontal offset count
    #[inline(always)]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W<CWSTRTrs> {
        HOFFCNT_W::new(self, 0)
    }
    ///Bits 16:28 - Vertical start line count
    #[inline(always)]
    pub fn vst(&mut self) -> VST_W<CWSTRTrs> {
        VST_W::new(self, 16)
    }
}
/**crop window start

You can [`read`](crate::Reg::read) this register and get [`cwstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DCMI:CWSTRT)*/
pub struct CWSTRTrs;
impl crate::RegisterSpec for CWSTRTrs {
    type Ux = u32;
}
///`read()` method returns [`cwstrt::R`](R) reader structure
impl crate::Readable for CWSTRTrs {}
///`write(|w| ..)` method takes [`cwstrt::W`](W) writer structure
impl crate::Writable for CWSTRTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWSTRT to value 0
impl crate::Resettable for CWSTRTrs {}
