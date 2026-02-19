///Register `TCCR0` reader
pub type R = crate::R<TCCR0rs>;
///Register `TCCR0` writer
pub type W = crate::W<TCCR0rs>;
///Field `LPRX_TOCNT` reader - Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles).
pub type LPRX_TOCNT_R = crate::FieldReader<u16>;
///Field `LPRX_TOCNT` writer - Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles).
pub type LPRX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HSTX_TOCNT` reader - High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one LINE data transmission *Â (1Â +Â 10%)
pub type HSTX_TOCNT_R = crate::FieldReader<u16>;
///Field `HSTX_TOCNT` writer - High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one LINE data transmission *Â (1Â +Â 10%)
pub type HSTX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles).
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one LINE data transmission *Â (1Â +Â 10%)
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR0")
            .field("lprx_tocnt", &self.lprx_tocnt())
            .field("hstx_tocnt", &self.hstx_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles).
    #[inline(always)]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W<TCCR0rs> {
        LPRX_TOCNT_W::new(self, 0)
    }
    ///Bits 16:31 - High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â¥ the time of one LINE data transmission *Â (1Â +Â 10%)
    #[inline(always)]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W<TCCR0rs> {
        HSTX_TOCNT_W::new(self, 16)
    }
}
/**DSI Host timeout counter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`tccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:TCCR0)*/
pub struct TCCR0rs;
impl crate::RegisterSpec for TCCR0rs {
    type Ux = u32;
}
///`read()` method returns [`tccr0::R`](R) reader structure
impl crate::Readable for TCCR0rs {}
///`write(|w| ..)` method takes [`tccr0::W`](W) writer structure
impl crate::Writable for TCCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR0 to value 0
impl crate::Resettable for TCCR0rs {}
