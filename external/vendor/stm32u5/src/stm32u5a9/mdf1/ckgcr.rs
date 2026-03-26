///Register `CKGCR` reader
pub type R = crate::R<CKGCRrs>;
///Register `CKGCR` writer
pub type W = crate::W<CKGCRrs>;
///Field `CKGDEN` reader - CKGDEN
pub type CKGDEN_R = crate::BitReader;
///Field `CKGDEN` writer - CKGDEN
pub type CKGDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCK0EN` reader - CCK0EN
pub type CCK0EN_R = crate::BitReader;
///Field `CCK0EN` writer - CCK0EN
pub type CCK0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCK1EN` reader - CCK1EN
pub type CCK1EN_R = crate::BitReader;
///Field `CCK1EN` writer - CCK1EN
pub type CCK1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKGMOD` reader - CKGMOD
pub type CKGMOD_R = crate::BitReader;
///Field `CKGMOD` writer - CKGMOD
pub type CKGMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCK0DIR` reader - CCK0DIR
pub type CCK0DIR_R = crate::BitReader;
///Field `CCK0DIR` writer - CCK0DIR
pub type CCK0DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCK1DIR` reader - CCK1DIR
pub type CCK1DIR_R = crate::BitReader;
///Field `CCK1DIR` writer - CCK1DIR
pub type CCK1DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGSENS` reader - TRGSENS
pub type TRGSENS_R = crate::BitReader;
///Field `TRGSENS` writer - TRGSENS
pub type TRGSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGSRC` reader - TRGSRC
pub type TRGSRC_R = crate::FieldReader;
///Field `TRGSRC` writer - TRGSRC
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CCKDIV` reader - CCKDIV
pub type CCKDIV_R = crate::FieldReader;
///Field `CCKDIV` writer - CCKDIV
pub type CCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PROCDIV` reader - PROCDIV
pub type PROCDIV_R = crate::FieldReader;
///Field `PROCDIV` writer - PROCDIV
pub type PROCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CKGACTIVE` reader - CKGACTIVE
pub type CKGACTIVE_R = crate::BitReader;
///Field `CKGACTIVE` writer - CKGACTIVE
pub type CKGACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CKGDEN
    #[inline(always)]
    pub fn ckgden(&self) -> CKGDEN_R {
        CKGDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCK0EN
    #[inline(always)]
    pub fn cck0en(&self) -> CCK0EN_R {
        CCK0EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CCK1EN
    #[inline(always)]
    pub fn cck1en(&self) -> CCK1EN_R {
        CCK1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CKGMOD
    #[inline(always)]
    pub fn ckgmod(&self) -> CKGMOD_R {
        CKGMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CCK0DIR
    #[inline(always)]
    pub fn cck0dir(&self) -> CCK0DIR_R {
        CCK0DIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CCK1DIR
    #[inline(always)]
    pub fn cck1dir(&self) -> CCK1DIR_R {
        CCK1DIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - TRGSENS
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - TRGSRC
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - CCKDIV
    #[inline(always)]
    pub fn cckdiv(&self) -> CCKDIV_R {
        CCKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:30 - PROCDIV
    #[inline(always)]
    pub fn procdiv(&self) -> PROCDIV_R {
        PROCDIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    ///Bit 31 - CKGACTIVE
    #[inline(always)]
    pub fn ckgactive(&self) -> CKGACTIVE_R {
        CKGACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKGCR")
            .field("ckgden", &self.ckgden())
            .field("cck0en", &self.cck0en())
            .field("cck1en", &self.cck1en())
            .field("ckgmod", &self.ckgmod())
            .field("cck0dir", &self.cck0dir())
            .field("cck1dir", &self.cck1dir())
            .field("trgsens", &self.trgsens())
            .field("trgsrc", &self.trgsrc())
            .field("cckdiv", &self.cckdiv())
            .field("procdiv", &self.procdiv())
            .field("ckgactive", &self.ckgactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - CKGDEN
    #[inline(always)]
    pub fn ckgden(&mut self) -> CKGDEN_W<CKGCRrs> {
        CKGDEN_W::new(self, 0)
    }
    ///Bit 1 - CCK0EN
    #[inline(always)]
    pub fn cck0en(&mut self) -> CCK0EN_W<CKGCRrs> {
        CCK0EN_W::new(self, 1)
    }
    ///Bit 2 - CCK1EN
    #[inline(always)]
    pub fn cck1en(&mut self) -> CCK1EN_W<CKGCRrs> {
        CCK1EN_W::new(self, 2)
    }
    ///Bit 4 - CKGMOD
    #[inline(always)]
    pub fn ckgmod(&mut self) -> CKGMOD_W<CKGCRrs> {
        CKGMOD_W::new(self, 4)
    }
    ///Bit 5 - CCK0DIR
    #[inline(always)]
    pub fn cck0dir(&mut self) -> CCK0DIR_W<CKGCRrs> {
        CCK0DIR_W::new(self, 5)
    }
    ///Bit 6 - CCK1DIR
    #[inline(always)]
    pub fn cck1dir(&mut self) -> CCK1DIR_W<CKGCRrs> {
        CCK1DIR_W::new(self, 6)
    }
    ///Bit 8 - TRGSENS
    #[inline(always)]
    pub fn trgsens(&mut self) -> TRGSENS_W<CKGCRrs> {
        TRGSENS_W::new(self, 8)
    }
    ///Bits 12:15 - TRGSRC
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W<CKGCRrs> {
        TRGSRC_W::new(self, 12)
    }
    ///Bits 16:19 - CCKDIV
    #[inline(always)]
    pub fn cckdiv(&mut self) -> CCKDIV_W<CKGCRrs> {
        CCKDIV_W::new(self, 16)
    }
    ///Bits 24:30 - PROCDIV
    #[inline(always)]
    pub fn procdiv(&mut self) -> PROCDIV_W<CKGCRrs> {
        PROCDIV_W::new(self, 24)
    }
    ///Bit 31 - CKGACTIVE
    #[inline(always)]
    pub fn ckgactive(&mut self) -> CKGACTIVE_W<CKGCRrs> {
        CKGACTIVE_W::new(self, 31)
    }
}
/**MDF clock generator control register

You can [`read`](crate::Reg::read) this register and get [`ckgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#MDF1:CKGCR)*/
pub struct CKGCRrs;
impl crate::RegisterSpec for CKGCRrs {
    type Ux = u32;
}
///`read()` method returns [`ckgcr::R`](R) reader structure
impl crate::Readable for CKGCRrs {}
///`write(|w| ..)` method takes [`ckgcr::W`](W) writer structure
impl crate::Writable for CKGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKGCR to value 0
impl crate::Resettable for CKGCRrs {}
