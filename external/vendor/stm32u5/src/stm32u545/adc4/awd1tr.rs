///Register `AWD1TR` reader
pub type R = crate::R<AWD1TRrs>;
///Register `AWD1TR` writer
pub type W = crate::W<AWD1TRrs>;
///Field `LT1` reader - LT1
pub type LT1_R = crate::FieldReader<u16>;
///Field `LT1` writer - LT1
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `HT1` reader - HT1
pub type HT1_R = crate::FieldReader<u16>;
///Field `HT1` writer - HT1
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - LT1
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - HT1
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD1TR")
            .field("ht1", &self.ht1())
            .field("lt1", &self.lt1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - LT1
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W<AWD1TRrs> {
        LT1_W::new(self, 0)
    }
    ///Bits 16:27 - HT1
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W<AWD1TRrs> {
        HT1_W::new(self, 16)
    }
}
/**ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC4:AWD1TR)*/
pub struct AWD1TRrs;
impl crate::RegisterSpec for AWD1TRrs {
    type Ux = u32;
}
///`read()` method returns [`awd1tr::R`](R) reader structure
impl crate::Readable for AWD1TRrs {}
///`write(|w| ..)` method takes [`awd1tr::W`](W) writer structure
impl crate::Writable for AWD1TRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD1TR to value 0x0fff_0000
impl crate::Resettable for AWD1TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
