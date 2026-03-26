///Register `CKCR` reader
pub type R = crate::R<CKCRrs>;
///Register `CKCR` writer
pub type W = crate::W<CKCRrs>;
///Field `CKBLUE` reader - color key blue value
pub type CKBLUE_R = crate::FieldReader;
///Field `CKBLUE` writer - color key blue value
pub type CKBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CKGREEN` reader - color key green value
pub type CKGREEN_R = crate::FieldReader;
///Field `CKGREEN` writer - color key green value
pub type CKGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CKRED` reader - color key red value
pub type CKRED_R = crate::FieldReader;
///Field `CKRED` writer - color key red value
pub type CKRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - color key blue value
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - color key green value
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - color key red value
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKCR")
            .field("ckblue", &self.ckblue())
            .field("ckgreen", &self.ckgreen())
            .field("ckred", &self.ckred())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - color key blue value
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W<CKCRrs> {
        CKBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - color key green value
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W<CKCRrs> {
        CKGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - color key red value
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W<CKCRrs> {
        CKRED_W::new(self, 16)
    }
}
/**LTDC layer 1 color keying configuration register

You can [`read`](crate::Reg::read) this register and get [`ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CKCRrs;
impl crate::RegisterSpec for CKCRrs {
    type Ux = u32;
}
///`read()` method returns [`ckcr::R`](R) reader structure
impl crate::Readable for CKCRrs {}
///`write(|w| ..)` method takes [`ckcr::W`](W) writer structure
impl crate::Writable for CKCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKCR to value 0
impl crate::Resettable for CKCRrs {}
