///Register `BCCR` reader
pub type R = crate::R<BCCRrs>;
///Register `BCCR` writer
pub type W = crate::W<BCCRrs>;
///Field `BCBLUE` reader - background color blue value These bits configure the background blue value.
pub type BCBLUE_R = crate::FieldReader;
///Field `BCBLUE` writer - background color blue value These bits configure the background blue value.
pub type BCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BCGREEN` reader - background color green value These bits configure the background green value.
pub type BCGREEN_R = crate::FieldReader;
///Field `BCGREEN` writer - background color green value These bits configure the background green value.
pub type BCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BCRED` reader - background color red value These bits configure the background red value.
pub type BCRED_R = crate::FieldReader;
///Field `BCRED` writer - background color red value These bits configure the background red value.
pub type BCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - background color blue value These bits configure the background blue value.
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - background color green value These bits configure the background green value.
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - background color red value These bits configure the background red value.
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCCR")
            .field("bcblue", &self.bcblue())
            .field("bcgreen", &self.bcgreen())
            .field("bcred", &self.bcred())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - background color blue value These bits configure the background blue value.
    #[inline(always)]
    pub fn bcblue(&mut self) -> BCBLUE_W<BCCRrs> {
        BCBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - background color green value These bits configure the background green value.
    #[inline(always)]
    pub fn bcgreen(&mut self) -> BCGREEN_W<BCCRrs> {
        BCGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - background color red value These bits configure the background red value.
    #[inline(always)]
    pub fn bcred(&mut self) -> BCRED_W<BCCRrs> {
        BCRED_W::new(self, 16)
    }
}
/**LTDC background color configuration register

You can [`read`](crate::Reg::read) this register and get [`bccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:BCCR)*/
pub struct BCCRrs;
impl crate::RegisterSpec for BCCRrs {
    type Ux = u32;
}
///`read()` method returns [`bccr::R`](R) reader structure
impl crate::Readable for BCCRrs {}
///`write(|w| ..)` method takes [`bccr::W`](W) writer structure
impl crate::Writable for BCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCR to value 0
impl crate::Resettable for BCCRrs {}
