///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `IN1SEL` reader - LPTIM input 1 selection
pub type IN1SEL_R = crate::FieldReader;
///Field `IN1SEL` writer - LPTIM input 1 selection
pub type IN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IN2SEL` reader - LPTIM input 2 selection
pub type IN2SEL_R = crate::FieldReader;
///Field `IN2SEL` writer - LPTIM input 2 selection
pub type IN2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1SEL` reader - LPTIM input capture 1 selection
pub type IC1SEL_R = crate::FieldReader;
///Field `IC1SEL` writer - LPTIM input capture 1 selection
pub type IC1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2SEL` reader - LPTIM input capture 2 selection
pub type IC2SEL_R = crate::FieldReader;
///Field `IC2SEL` writer - LPTIM input capture 2 selection
pub type IC2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - LPTIM input 1 selection
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - LPTIM input 2 selection
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:17 - LPTIM input capture 1 selection
    #[inline(always)]
    pub fn ic1sel(&self) -> IC1SEL_R {
        IC1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - LPTIM input capture 2 selection
    #[inline(always)]
    pub fn ic2sel(&self) -> IC2SEL_R {
        IC2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("ic2sel", &self.ic2sel())
            .field("ic1sel", &self.ic1sel())
            .field("in2sel", &self.in2sel())
            .field("in1sel", &self.in1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - LPTIM input 1 selection
    #[inline(always)]
    pub fn in1sel(&mut self) -> IN1SEL_W<CFGR2rs> {
        IN1SEL_W::new(self, 0)
    }
    ///Bits 4:5 - LPTIM input 2 selection
    #[inline(always)]
    pub fn in2sel(&mut self) -> IN2SEL_W<CFGR2rs> {
        IN2SEL_W::new(self, 4)
    }
    ///Bits 16:17 - LPTIM input capture 1 selection
    #[inline(always)]
    pub fn ic1sel(&mut self) -> IC1SEL_W<CFGR2rs> {
        IC1SEL_W::new(self, 16)
    }
    ///Bits 20:21 - LPTIM input capture 2 selection
    #[inline(always)]
    pub fn ic2sel(&mut self) -> IC2SEL_W<CFGR2rs> {
        IC2SEL_W::new(self, 20)
    }
}
/**LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LPTIM4:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
