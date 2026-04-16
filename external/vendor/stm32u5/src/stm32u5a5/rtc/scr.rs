///Register `SCR` writer
pub type W = crate::W<SCRrs>;
/**CALRAF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRAF {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<CALRAF> for bool {
    #[inline(always)]
    fn from(variant: CALRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `CALRAF` writer - CALRAF
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG, CALRAF>;
impl<'a, REG> CALRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CALRAF::Clear)
    }
}
///Field `CALRBF` writer - CALRBF
pub use CALRAF_W as CALRBF_W;
///Field `CWUTF` writer - CWUTF
pub use CALRAF_W as CWUTF_W;
///Field `CTSF` writer - CTSF
pub use CALRAF_W as CTSF_W;
///Field `CTSOVF` writer - CTSOVF
pub use CALRAF_W as CTSOVF_W;
///Field `CITSF` writer - CITSF
pub use CALRAF_W as CITSF_W;
///Field `CSSRUF` writer - CSSRUF
pub use CALRAF_W as CSSRUF_W;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CALRAF
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W<SCRrs> {
        CALRAF_W::new(self, 0)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W<SCRrs> {
        CALRBF_W::new(self, 1)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W<SCRrs> {
        CWUTF_W::new(self, 2)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W<SCRrs> {
        CTSF_W::new(self, 3)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W<SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W<SCRrs> {
        CITSF_W::new(self, 5)
    }
    ///Bit 6 - CSSRUF
    #[inline(always)]
    pub fn cssruf(&mut self) -> CSSRUF_W<SCRrs> {
        CSSRUF_W::new(self, 6)
    }
}
/**RTC status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RTC:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
