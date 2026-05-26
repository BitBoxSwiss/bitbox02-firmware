///Register `TCCR2` reader
pub type R = crate::R<TCCR2rs>;
///Register `TCCR2` writer
pub type W = crate::W<TCCR2rs>;
///Field `LPRD_TOCNT` reader - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPRD_TOCNT_R = crate::FieldReader<u16>;
///Field `LPRD_TOCNT` writer - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPRD_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LPRD_TOCNT_R {
        LPRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR2")
            .field("lprd_tocnt", &self.lprd_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn lprd_tocnt(&mut self) -> LPRD_TOCNT_W<TCCR2rs> {
        LPRD_TOCNT_W::new(self, 0)
    }
}
/**DSI Host timeout counter configuration register 2

You can [`read`](crate::Reg::read) this register and get [`tccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:TCCR2)*/
pub struct TCCR2rs;
impl crate::RegisterSpec for TCCR2rs {
    type Ux = u32;
}
///`read()` method returns [`tccr2::R`](R) reader structure
impl crate::Readable for TCCR2rs {}
///`write(|w| ..)` method takes [`tccr2::W`](W) writer structure
impl crate::Writable for TCCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR2 to value 0
impl crate::Resettable for TCCR2rs {}
