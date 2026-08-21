///Register `TCCR3` reader
pub type R = crate::R<TCCR3rs>;
///Register `TCCR3` writer
pub type W = crate::W<TCCR3rs>;
///Field `HSWR_TOCNT` reader - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type HSWR_TOCNT_R = crate::FieldReader<u16>;
///Field `HSWR_TOCNT` writer - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type HSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PM` reader - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready.
pub type PM_R = crate::BitReader;
///Field `PM` writer - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready.
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 24 - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready.
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR3")
            .field("hswr_tocnt", &self.hswr_tocnt())
            .field("pm", &self.pm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<TCCR3rs> {
        HSWR_TOCNT_W::new(self, 0)
    }
    ///Bit 24 - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready.
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<TCCR3rs> {
        PM_W::new(self, 24)
    }
}
/**DSI Host timeout counter configuration register 3

You can [`read`](crate::Reg::read) this register and get [`tccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:TCCR3)*/
pub struct TCCR3rs;
impl crate::RegisterSpec for TCCR3rs {
    type Ux = u32;
}
///`read()` method returns [`tccr3::R`](R) reader structure
impl crate::Readable for TCCR3rs {}
///`write(|w| ..)` method takes [`tccr3::W`](W) writer structure
impl crate::Writable for TCCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR3 to value 0
impl crate::Resettable for TCCR3rs {}
