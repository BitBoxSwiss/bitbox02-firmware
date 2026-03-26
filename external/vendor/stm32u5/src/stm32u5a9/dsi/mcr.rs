///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
///Field `CMDM` reader - Command mode This bit configures the DSI Host in either video or command mode.
pub type CMDM_R = crate::BitReader;
///Field `CMDM` writer - Command mode This bit configures the DSI Host in either video or command mode.
pub type CMDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Command mode This bit configures the DSI Host in either video or command mode.
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR").field("cmdm", &self.cmdm()).finish()
    }
}
impl W {
    ///Bit 0 - Command mode This bit configures the DSI Host in either video or command mode.
    #[inline(always)]
    pub fn cmdm(&mut self) -> CMDM_W<MCRrs> {
        CMDM_W::new(self, 0)
    }
}
/**DSI Host mode configuration register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0x01
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0x01;
}
