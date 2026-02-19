///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
///Field `TRIM1_NG_CCRPD` reader - TRIM1_NG_CCRPD
pub type TRIM1_NG_CCRPD_R = crate::FieldReader;
///Field `TRIM1_NG_CCRPD` writer - TRIM1_NG_CCRPD
pub type TRIM1_NG_CCRPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRIM1_NG_CC3A0` reader - TRIM1_NG_CC3A0
pub type TRIM1_NG_CC3A0_R = crate::FieldReader;
///Field `TRIM1_NG_CC3A0` writer - TRIM1_NG_CC3A0
pub type TRIM1_NG_CC3A0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRIM2_NG_CCRPD` reader - TRIM2_NG_CCRPD
pub type TRIM2_NG_CCRPD_R = crate::FieldReader;
///Field `TRIM2_NG_CCRPD` writer - TRIM2_NG_CCRPD
pub type TRIM2_NG_CCRPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `TRIM2_NG_CC3A0` reader - TRIM2_NG_CC3A0
pub type TRIM2_NG_CC3A0_R = crate::FieldReader;
///Field `TRIM2_NG_CC3A0` writer - TRIM2_NG_CC3A0
pub type TRIM2_NG_CC3A0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - TRIM1_NG_CCRPD
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 9:12 - TRIM1_NG_CC3A0
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 16:19 - TRIM2_NG_CCRPD
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 25:28 - TRIM2_NG_CC3A0
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("trim1_ng_ccrpd", &self.trim1_ng_ccrpd())
            .field("trim1_ng_cc3a0", &self.trim1_ng_cc3a0())
            .field("trim2_ng_ccrpd", &self.trim2_ng_ccrpd())
            .field("trim2_ng_cc3a0", &self.trim2_ng_cc3a0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TRIM1_NG_CCRPD
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W<CFGR3rs> {
        TRIM1_NG_CCRPD_W::new(self, 0)
    }
    ///Bits 9:12 - TRIM1_NG_CC3A0
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W<CFGR3rs> {
        TRIM1_NG_CC3A0_W::new(self, 9)
    }
    ///Bits 16:19 - TRIM2_NG_CCRPD
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W<CFGR3rs> {
        TRIM2_NG_CCRPD_W::new(self, 16)
    }
    ///Bits 25:28 - TRIM2_NG_CC3A0
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W<CFGR3rs> {
        TRIM2_NG_CC3A0_W::new(self, 25)
    }
}
/**UCPD configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#UCPD1:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
