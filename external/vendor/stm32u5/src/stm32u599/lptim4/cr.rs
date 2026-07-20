///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ENABLE` reader - LPTIM Enable
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - LPTIM Enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNGSTRT` reader - LPTIM start in single mode
pub type SNGSTRT_R = crate::BitReader;
///Field `SNGSTRT` writer - LPTIM start in single mode
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTSTRT` reader - Timer start in continuous mode
pub type CNTSTRT_R = crate::BitReader;
///Field `CNTSTRT` writer - Timer start in continuous mode
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTRST` reader - Counter reset
pub type COUNTRST_R = crate::BitReader;
///Field `COUNTRST` writer - Counter reset
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTARE` reader - Reset after read enable
pub type RSTARE_R = crate::BitReader;
///Field `RSTARE` writer - Reset after read enable
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPTIM Enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM start in single mode
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer start in continuous mode
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Counter reset
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Reset after read enable
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rstare", &self.rstare())
            .field("countrst", &self.countrst())
            .field("cntstrt", &self.cntstrt())
            .field("sngstrt", &self.sngstrt())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM Enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - LPTIM start in single mode
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    ///Bit 2 - Timer start in continuous mode
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    ///Bit 3 - Counter reset
    #[inline(always)]
    pub fn countrst(&mut self) -> COUNTRST_W<CRrs> {
        COUNTRST_W::new(self, 3)
    }
    ///Bit 4 - Reset after read enable
    #[inline(always)]
    pub fn rstare(&mut self) -> RSTARE_W<CRrs> {
        RSTARE_W::new(self, 4)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LPTIM4:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
