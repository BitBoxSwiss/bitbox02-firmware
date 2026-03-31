///Register `SITF0CR` reader
pub type R = crate::R<SITF0CRrs>;
///Register `SITF0CR` writer
pub type W = crate::W<SITF0CRrs>;
///Field `SITFEN` reader - SITFEN
pub type SITFEN_R = crate::BitReader;
///Field `SITFEN` writer - SITFEN
pub type SITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCKSRC` reader - SCKSRC
pub type SCKSRC_R = crate::FieldReader;
///Field `SCKSRC` writer - SCKSRC
pub type SCKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SITFMOD` reader - SITFMOD
pub type SITFMOD_R = crate::FieldReader;
///Field `SITFMOD` writer - SITFMOD
pub type SITFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STH` reader - STH
pub type STH_R = crate::FieldReader;
///Field `STH` writer - STH
pub type STH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SITFACTIVE` reader - SITFACTIVE
pub type SITFACTIVE_R = crate::BitReader;
///Field `SITFACTIVE` writer - SITFACTIVE
pub type SITFACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SITFEN
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - SCKSRC
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - SITFMOD
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:12 - STH
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 31 - SITFACTIVE
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SITF0CR")
            .field("sitfactive", &self.sitfactive())
            .field("sth", &self.sth())
            .field("sitfmod", &self.sitfmod())
            .field("scksrc", &self.scksrc())
            .field("sitfen", &self.sitfen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SITFEN
    #[inline(always)]
    pub fn sitfen(&mut self) -> SITFEN_W<SITF0CRrs> {
        SITFEN_W::new(self, 0)
    }
    ///Bits 1:2 - SCKSRC
    #[inline(always)]
    pub fn scksrc(&mut self) -> SCKSRC_W<SITF0CRrs> {
        SCKSRC_W::new(self, 1)
    }
    ///Bits 4:5 - SITFMOD
    #[inline(always)]
    pub fn sitfmod(&mut self) -> SITFMOD_W<SITF0CRrs> {
        SITFMOD_W::new(self, 4)
    }
    ///Bits 8:12 - STH
    #[inline(always)]
    pub fn sth(&mut self) -> STH_W<SITF0CRrs> {
        STH_W::new(self, 8)
    }
    ///Bit 31 - SITFACTIVE
    #[inline(always)]
    pub fn sitfactive(&mut self) -> SITFACTIVE_W<SITF0CRrs> {
        SITFACTIVE_W::new(self, 31)
    }
}
/**ADF serial interface control register 0

You can [`read`](crate::Reg::read) this register and get [`sitf0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SITF0CR)*/
pub struct SITF0CRrs;
impl crate::RegisterSpec for SITF0CRrs {
    type Ux = u32;
}
///`read()` method returns [`sitf0cr::R`](R) reader structure
impl crate::Readable for SITF0CRrs {}
///`write(|w| ..)` method takes [`sitf0cr::W`](W) writer structure
impl crate::Writable for SITF0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SITF0CR to value 0x1f00
impl crate::Resettable for SITF0CRrs {
    const RESET_VALUE: u32 = 0x1f00;
}
