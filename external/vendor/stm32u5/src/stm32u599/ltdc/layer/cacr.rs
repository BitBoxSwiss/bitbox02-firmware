///Register `CACR` reader
pub type R = crate::R<CACRrs>;
///Register `CACR` writer
pub type W = crate::W<CACRrs>;
///Field `CONSTA` reader - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
pub type CONSTA_R = crate::FieldReader;
///Field `CONSTA` writer - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
pub type CONSTA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR")
            .field("consta", &self.consta())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W<CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
/**LTDC layer 1 constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACRrs;
impl crate::RegisterSpec for CACRrs {
    type Ux = u32;
}
///`read()` method returns [`cacr::R`](R) reader structure
impl crate::Readable for CACRrs {}
///`write(|w| ..)` method takes [`cacr::W`](W) writer structure
impl crate::Writable for CACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CACR to value 0xff
impl crate::Resettable for CACRrs {
    const RESET_VALUE: u32 = 0xff;
}
