///Register `PSC` reader
pub type R = crate::R<PSCrs>;
///Register `PSC` writer
pub type W = crate::W<PSCrs>;
///Field `PSC` reader - Prescaler value The counter clock frequency (ftim_cnt_ck) is equal to ftim_psc_ck / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in âreset modeâ).
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - Prescaler value The counter clock frequency (ftim_cnt_ck) is equal to ftim_psc_ck / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in âreset modeâ).
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Prescaler value The counter clock frequency (ftim_cnt_ck) is equal to ftim_psc_ck / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in âreset modeâ).
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSC").field("psc", &self.psc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value The counter clock frequency (ftim_cnt_ck) is equal to ftim_psc_ck / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in âreset modeâ).
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM1 prescaler

You can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TIM1:PSC)*/
pub struct PSCrs;
impl crate::RegisterSpec for PSCrs {
    type Ux = u16;
}
///`read()` method returns [`psc::R`](R) reader structure
impl crate::Readable for PSCrs {}
///`write(|w| ..)` method takes [`psc::W`](W) writer structure
impl crate::Writable for PSCrs {
    type Safety = crate::Safe;
}
///`reset()` method sets PSC to value 0
impl crate::Resettable for PSCrs {}
