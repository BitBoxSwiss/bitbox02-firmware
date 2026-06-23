///Register `TISEL` reader
pub type R = crate::R<TISELrs>;
///Register `TISEL` writer
pub type W = crate::W<TISELrs>;
///Field `TI1SEL` reader - Selects tim_ti3\[0..15\] input
pub type TI1SEL_R = crate::FieldReader;
///Field `TI1SEL` writer - Selects tim_ti3\[0..15\] input
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI2SEL` reader - Selects tim_ti3\[0..15\] input
pub type TI2SEL_R = crate::FieldReader;
///Field `TI2SEL` writer - Selects tim_ti3\[0..15\] input
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI3SEL` reader - Selects tim_ti3\[0..15\] input
pub type TI3SEL_R = crate::FieldReader;
///Field `TI3SEL` writer - Selects tim_ti3\[0..15\] input
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI4SEL` reader - Selects tim_ti4\[0..15\] input
pub type TI4SEL_R = crate::FieldReader;
///Field `TI4SEL` writer - Selects tim_ti4\[0..15\] input
pub type TI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Selects tim_ti3\[0..15\] input
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Selects tim_ti3\[0..15\] input
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Selects tim_ti3\[0..15\] input
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - Selects tim_ti4\[0..15\] input
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TISEL")
            .field("ti4sel", &self.ti4sel())
            .field("ti3sel", &self.ti3sel())
            .field("ti2sel", &self.ti2sel())
            .field("ti1sel", &self.ti1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Selects tim_ti3\[0..15\] input
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    ///Bits 8:11 - Selects tim_ti3\[0..15\] input
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TISELrs> {
        TI2SEL_W::new(self, 8)
    }
    ///Bits 16:19 - Selects tim_ti3\[0..15\] input
    #[inline(always)]
    pub fn ti3sel(&mut self) -> TI3SEL_W<TISELrs> {
        TI3SEL_W::new(self, 16)
    }
    ///Bits 24:27 - Selects tim_ti4\[0..15\] input
    #[inline(always)]
    pub fn ti4sel(&mut self) -> TI4SEL_W<TISELrs> {
        TI4SEL_W::new(self, 24)
    }
}
/**timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TIM1:TISEL)*/
pub struct TISELrs;
impl crate::RegisterSpec for TISELrs {
    type Ux = u32;
}
///`read()` method returns [`tisel::R`](R) reader structure
impl crate::Readable for TISELrs {}
///`write(|w| ..)` method takes [`tisel::W`](W) writer structure
impl crate::Writable for TISELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TISEL to value 0
impl crate::Resettable for TISELrs {}
