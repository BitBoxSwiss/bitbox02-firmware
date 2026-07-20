///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `CKSEL` reader - Clock selector
pub type CKSEL_R = crate::BitReader;
///Field `CKSEL` writer - Clock selector
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKPOL` reader - Clock Polarity
pub type CKPOL_R = crate::FieldReader;
///Field `CKPOL` writer - Clock Polarity
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKFLT` reader - Configurable digital filter for external clock
pub type CKFLT_R = crate::FieldReader;
///Field `CKFLT` writer - Configurable digital filter for external clock
pub type CKFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRGFLT` reader - Configurable digital filter for trigger
pub type TRGFLT_R = crate::FieldReader;
///Field `TRGFLT` writer - Configurable digital filter for trigger
pub type TRGFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRESC` reader - Clock prescaler
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - Clock prescaler
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGSEL` reader - Trigger selector
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - Trigger selector
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGEN` reader - Trigger enable and polarity
pub type TRIGEN_R = crate::FieldReader;
///Field `TRIGEN` writer - Trigger enable and polarity
pub type TRIGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIMOUT` reader - Timeout enable
pub type TIMOUT_R = crate::BitReader;
///Field `TIMOUT` writer - Timeout enable
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVE` reader - Waveform shape
pub type WAVE_R = crate::BitReader;
///Field `WAVE` writer - Waveform shape
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVPOL` reader - Waveform shape polarity
pub type WAVPOL_R = crate::BitReader;
///Field `WAVPOL` writer - Waveform shape polarity
pub type WAVPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRELOAD` reader - Registers update mode
pub type PRELOAD_R = crate::BitReader;
///Field `PRELOAD` writer - Registers update mode
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTMODE` reader - counter mode enabled
pub type COUNTMODE_R = crate::BitReader;
///Field `COUNTMODE` writer - counter mode enabled
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENC` reader - Encoder mode enable
pub type ENC_R = crate::BitReader;
///Field `ENC` writer - Encoder mode enable
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock selector
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Clock Polarity
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Configurable digital filter for external clock
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - Configurable digital filter for trigger
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 9:11 - Clock prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 13:15 - Trigger selector
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:18 - Trigger enable and polarity
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Timeout enable
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Waveform shape
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Waveform shape polarity
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Registers update mode
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - counter mode enabled
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Encoder mode enable
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("enc", &self.enc())
            .field("countmode", &self.countmode())
            .field("preload", &self.preload())
            .field("wavpol", &self.wavpol())
            .field("wave", &self.wave())
            .field("timout", &self.timout())
            .field("trigen", &self.trigen())
            .field("trigsel", &self.trigsel())
            .field("presc", &self.presc())
            .field("trgflt", &self.trgflt())
            .field("ckflt", &self.ckflt())
            .field("ckpol", &self.ckpol())
            .field("cksel", &self.cksel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock selector
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<CFGRrs> {
        CKSEL_W::new(self, 0)
    }
    ///Bits 1:2 - Clock Polarity
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<CFGRrs> {
        CKPOL_W::new(self, 1)
    }
    ///Bits 3:4 - Configurable digital filter for external clock
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W<CFGRrs> {
        CKFLT_W::new(self, 3)
    }
    ///Bits 6:7 - Configurable digital filter for trigger
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W<CFGRrs> {
        TRGFLT_W::new(self, 6)
    }
    ///Bits 9:11 - Clock prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<CFGRrs> {
        PRESC_W::new(self, 9)
    }
    ///Bits 13:15 - Trigger selector
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<CFGRrs> {
        TRIGSEL_W::new(self, 13)
    }
    ///Bits 17:18 - Trigger enable and polarity
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<CFGRrs> {
        TRIGEN_W::new(self, 17)
    }
    ///Bit 19 - Timeout enable
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W<CFGRrs> {
        TIMOUT_W::new(self, 19)
    }
    ///Bit 20 - Waveform shape
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W<CFGRrs> {
        WAVE_W::new(self, 20)
    }
    ///Bit 21 - Waveform shape polarity
    #[inline(always)]
    pub fn wavpol(&mut self) -> WAVPOL_W<CFGRrs> {
        WAVPOL_W::new(self, 21)
    }
    ///Bit 22 - Registers update mode
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W<CFGRrs> {
        PRELOAD_W::new(self, 22)
    }
    ///Bit 23 - counter mode enabled
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W<CFGRrs> {
        COUNTMODE_W::new(self, 23)
    }
    ///Bit 24 - Encoder mode enable
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W<CFGRrs> {
        ENC_W::new(self, 24)
    }
}
/**Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPTIM4:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
