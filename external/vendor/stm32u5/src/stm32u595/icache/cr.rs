///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHEINV` writer - CACHEINV
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAYSEL` reader - WAYSEL
pub type WAYSEL_R = crate::BitReader;
///Field `WAYSEL` writer - WAYSEL
pub type WAYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HITMEN` reader - HITMEN
pub type HITMEN_R = crate::BitReader;
///Field `HITMEN` writer - HITMEN
pub type HITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISSMEN` reader - MISSMEN
pub type MISSMEN_R = crate::BitReader;
///Field `MISSMEN` writer - MISSMEN
pub type MISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HITMRST` reader - HITMRST
pub type HITMRST_R = crate::BitReader;
///Field `HITMRST` writer - HITMRST
pub type HITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISSMRST` reader - MISSMRST
pub type MISSMRST_R = crate::BitReader;
///Field `MISSMRST` writer - MISSMRST
pub type MISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - WAYSEL
    #[inline(always)]
    pub fn waysel(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - HITMEN
    #[inline(always)]
    pub fn hitmen(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MISSMEN
    #[inline(always)]
    pub fn missmen(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HITMRST
    #[inline(always)]
    pub fn hitmrst(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MISSMRST
    #[inline(always)]
    pub fn missmrst(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("waysel", &self.waysel())
            .field("hitmen", &self.hitmen())
            .field("missmen", &self.missmen())
            .field("hitmrst", &self.hitmrst())
            .field("missmrst", &self.missmrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - CACHEINV
    #[inline(always)]
    pub fn cacheinv(&mut self) -> CACHEINV_W<CRrs> {
        CACHEINV_W::new(self, 1)
    }
    ///Bit 2 - WAYSEL
    #[inline(always)]
    pub fn waysel(&mut self) -> WAYSEL_W<CRrs> {
        WAYSEL_W::new(self, 2)
    }
    ///Bit 16 - HITMEN
    #[inline(always)]
    pub fn hitmen(&mut self) -> HITMEN_W<CRrs> {
        HITMEN_W::new(self, 16)
    }
    ///Bit 17 - MISSMEN
    #[inline(always)]
    pub fn missmen(&mut self) -> MISSMEN_W<CRrs> {
        MISSMEN_W::new(self, 17)
    }
    ///Bit 18 - HITMRST
    #[inline(always)]
    pub fn hitmrst(&mut self) -> HITMRST_W<CRrs> {
        HITMRST_W::new(self, 18)
    }
    ///Bit 19 - MISSMRST
    #[inline(always)]
    pub fn missmrst(&mut self) -> MISSMRST_W<CRrs> {
        MISSMRST_W::new(self, 19)
    }
}
/**ICACHE control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ICACHE:CR)*/
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
///`reset()` method sets CR to value 0x04
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x04;
}
