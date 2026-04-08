///Register `TCCR4` reader
pub type R = crate::R<TCCR4rs>;
///Register `TCCR4` writer
pub type W = crate::W<TCCR4rs>;
///Field `LPWR_TOCNT` reader - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPWR_TOCNT_R = crate::FieldReader<u16>;
///Field `LPWR_TOCNT` writer - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LPWR_TOCNT_R {
        LPWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR4")
            .field("lpwr_tocnt", &self.lpwr_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn lpwr_tocnt(&mut self) -> LPWR_TOCNT_W<TCCR4rs> {
        LPWR_TOCNT_W::new(self, 0)
    }
}
/**DSI Host timeout counter configuration register 4

You can [`read`](crate::Reg::read) this register and get [`tccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:TCCR4)*/
pub struct TCCR4rs;
impl crate::RegisterSpec for TCCR4rs {
    type Ux = u32;
}
///`read()` method returns [`tccr4::R`](R) reader structure
impl crate::Readable for TCCR4rs {}
///`write(|w| ..)` method takes [`tccr4::W`](W) writer structure
impl crate::Writable for TCCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR4 to value 0
impl crate::Resettable for TCCR4rs {}
