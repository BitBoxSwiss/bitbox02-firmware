///Register `CFBLNR` reader
pub type R = crate::R<CFBLNRrs>;
///Register `CFBLNR` writer
pub type W = crate::W<CFBLNRrs>;
///Field `CFBLNBR` reader - frame buffer line number These bits define the number of lines in the frame buffer that corresponds to the active high width.
pub type CFBLNBR_R = crate::FieldReader<u16>;
///Field `CFBLNBR` writer - frame buffer line number These bits define the number of lines in the frame buffer that corresponds to the active high width.
pub type CFBLNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - frame buffer line number These bits define the number of lines in the frame buffer that corresponds to the active high width.
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFBLNR")
            .field("cfblnbr", &self.cfblnbr())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - frame buffer line number These bits define the number of lines in the frame buffer that corresponds to the active high width.
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W<CFBLNRrs> {
        CFBLNBR_W::new(self, 0)
    }
}
/**LTDC layer 1 color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`cfblnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfblnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFBLNRrs;
impl crate::RegisterSpec for CFBLNRrs {
    type Ux = u32;
}
///`read()` method returns [`cfblnr::R`](R) reader structure
impl crate::Readable for CFBLNRrs {}
///`write(|w| ..)` method takes [`cfblnr::W`](W) writer structure
impl crate::Writable for CFBLNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFBLNR to value 0
impl crate::Resettable for CFBLNRrs {}
