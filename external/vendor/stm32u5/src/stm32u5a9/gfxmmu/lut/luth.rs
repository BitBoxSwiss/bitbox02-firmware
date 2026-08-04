///Register `LUTH` reader
pub type R = crate::R<LUTHrs>;
///Register `LUTH` writer
pub type W = crate::W<LUTHrs>;
///Field `LO` reader - Line offset Line offset of line number x (i.e. offset of block 0 of line x)
pub type LO_R = crate::FieldReader<u32>;
///Field `LO` writer - Line offset Line offset of line number x (i.e. offset of block 0 of line x)
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 4:21 - Line offset Line offset of line number x (i.e. offset of block 0 of line x)
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits >> 4) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUTH").field("lo", &self.lo()).finish()
    }
}
impl W {
    ///Bits 4:21 - Line offset Line offset of line number x (i.e. offset of block 0 of line x)
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<LUTHrs> {
        LO_W::new(self, 4)
    }
}
/**Graphic MMU LUT entry x high

You can [`read`](crate::Reg::read) this register and get [`luth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`luth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUTHrs;
impl crate::RegisterSpec for LUTHrs {
    type Ux = u32;
}
///`read()` method returns [`luth::R`](R) reader structure
impl crate::Readable for LUTHrs {}
///`write(|w| ..)` method takes [`luth::W`](W) writer structure
impl crate::Writable for LUTHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LUTH to value 0
impl crate::Resettable for LUTHrs {}
