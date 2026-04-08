///Register `GTPR` reader
pub type R = crate::R<GTPRrs>;
///Register `GTPR` writer
pub type W = crate::W<GTPRrs>;
///Field `PSC` reader - Prescaler value
pub type PSC_R = crate::FieldReader;
///Field `PSC` writer - Prescaler value
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `GT` reader - Guard time value
pub type GT_R = crate::FieldReader;
///Field `GT` writer - Guard time value
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Guard time value
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPR")
            .field("gt", &self.gt())
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<GTPRrs> {
        PSC_W::new(self, 0)
    }
    ///Bits 8:15 - Guard time value
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W<GTPRrs> {
        GT_W::new(self, 8)
    }
}
/**Guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#USART1:GTPR)*/
pub struct GTPRrs;
impl crate::RegisterSpec for GTPRrs {
    type Ux = u32;
}
///`read()` method returns [`gtpr::R`](R) reader structure
impl crate::Readable for GTPRrs {}
///`write(|w| ..)` method takes [`gtpr::W`](W) writer structure
impl crate::Writable for GTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPR to value 0
impl crate::Resettable for GTPRrs {}
