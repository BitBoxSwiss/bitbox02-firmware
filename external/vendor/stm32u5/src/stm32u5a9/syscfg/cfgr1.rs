///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANASWVDD` reader - GPIO analog switch control voltage selection
pub type ANASWVDD_R = crate::BitReader;
///Field `ANASWVDD` writer - GPIO analog switch control voltage selection
pub type ANASWVDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB6_FMP` reader - PB6_FMP
pub type PB6_FMP_R = crate::BitReader;
///Field `PB6_FMP` writer - PB6_FMP
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB7_FMP` reader - PB7_FMP
pub type PB7_FMP_R = crate::BitReader;
///Field `PB7_FMP` writer - PB7_FMP
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB8_FMP` reader - PB8_FMP
pub type PB8_FMP_R = crate::BitReader;
///Field `PB8_FMP` writer - PB8_FMP
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB9_FMP` reader - PB9_FMP
pub type PB9_FMP_R = crate::BitReader;
///Field `PB9_FMP` writer - PB9_FMP
pub type PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENDCAP` reader - ENDCAP
pub type ENDCAP_R = crate::FieldReader;
///Field `ENDCAP` writer - ENDCAP
pub type ENDCAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO analog switch control voltage selection
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - PB6_FMP
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PB7_FMP
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PB8_FMP
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PB9_FMP
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:25 - ENDCAP
    #[inline(always)]
    pub fn endcap(&self) -> ENDCAP_R {
        ENDCAP_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("endcap", &self.endcap())
            .field("pb9_fmp", &self.pb9_fmp())
            .field("pb8_fmp", &self.pb8_fmp())
            .field("pb7_fmp", &self.pb7_fmp())
            .field("pb6_fmp", &self.pb6_fmp())
            .field("anaswvdd", &self.anaswvdd())
            .field("boosten", &self.boosten())
            .finish()
    }
}
impl W {
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - GPIO analog switch control voltage selection
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<CFGR1rs> {
        ANASWVDD_W::new(self, 9)
    }
    ///Bit 16 - PB6_FMP
    #[inline(always)]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<CFGR1rs> {
        PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - PB7_FMP
    #[inline(always)]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<CFGR1rs> {
        PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - PB8_FMP
    #[inline(always)]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<CFGR1rs> {
        PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - PB9_FMP
    #[inline(always)]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W<CFGR1rs> {
        PB9_FMP_W::new(self, 19)
    }
    ///Bits 24:25 - ENDCAP
    #[inline(always)]
    pub fn endcap(&mut self) -> ENDCAP_W<CFGR1rs> {
        ENDCAP_W::new(self, 24)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
