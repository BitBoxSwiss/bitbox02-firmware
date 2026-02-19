///Register `TCCR5` reader
pub type R = crate::R<TCCR5rs>;
///Register `TCCR5` writer
pub type W = crate::W<TCCR5rs>;
///Field `BTA_TOCNT` reader - Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the DâPHY enters the Stop state and causes no interrupts.
pub type BTA_TOCNT_R = crate::FieldReader<u16>;
///Field `BTA_TOCNT` writer - Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the DâPHY enters the Stop state and causes no interrupts.
pub type BTA_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the DâPHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn bta_tocnt(&self) -> BTA_TOCNT_R {
        BTA_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR5")
            .field("bta_tocnt", &self.bta_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the DâPHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn bta_tocnt(&mut self) -> BTA_TOCNT_W<TCCR5rs> {
        BTA_TOCNT_W::new(self, 0)
    }
}
/**DSI Host timeout counter configuration register 5

You can [`read`](crate::Reg::read) this register and get [`tccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:TCCR5)*/
pub struct TCCR5rs;
impl crate::RegisterSpec for TCCR5rs {
    type Ux = u32;
}
///`read()` method returns [`tccr5::R`](R) reader structure
impl crate::Readable for TCCR5rs {}
///`write(|w| ..)` method takes [`tccr5::W`](W) writer structure
impl crate::Writable for TCCR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR5 to value 0
impl crate::Resettable for TCCR5rs {}
