///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
pub type LATENCY_R = crate::FieldReader;
///Field `LATENCY` writer - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRFTEN` reader - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPM` reader - Low-power read mode This bit puts the Flash memory in low-power read mode.
pub type LPM_R = crate::BitReader;
///Field `LPM` writer - Low-power read mode This bit puts the Flash memory in low-power read mode.
pub type LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDREQ1` reader - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
pub type PDREQ1_R = crate::BitReader;
///Field `PDREQ1` writer - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
pub type PDREQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDREQ2` reader - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
pub type PDREQ2_R = crate::BitReader;
///Field `PDREQ2` writer - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
pub type PDREQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_PD` reader - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
pub type SLEEP_PD_R = crate::BitReader;
///Field `SLEEP_PD` writer - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode.
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
    #[inline(always)]
    pub fn pdreq1(&self) -> PDREQ1_R {
        PDREQ1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
    #[inline(always)]
    pub fn pdreq2(&self) -> PDREQ2_R {
        PDREQ2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("lpm", &self.lpm())
            .field("pdreq1", &self.pdreq1())
            .field("pdreq2", &self.pdreq2())
            .field("sleep_pd", &self.sleep_pd())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode.
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W<ACRrs> {
        LPM_W::new(self, 11)
    }
    ///Bit 12 - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
    #[inline(always)]
    pub fn pdreq1(&mut self) -> PDREQ1_W<ACRrs> {
        PDREQ1_W::new(self, 12)
    }
    ///Bit 13 - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
    #[inline(always)]
    pub fn pdreq2(&mut self) -> PDREQ2_W<ACRrs> {
        PDREQ2_W::new(self, 13)
    }
    ///Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
}
/**FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {}
