///Register `CLKCR` reader
pub type R = crate::R<CLKCRrs>;
///Register `CLKCR` writer
pub type W = crate::W<CLKCRrs>;
///Field `CLKDIV` reader - Clock divide factor
pub type CLKDIV_R = crate::FieldReader<u16>;
///Field `CLKDIV` writer - Clock divide factor
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PWRSAV` reader - Power saving configuration bit
pub type PWRSAV_R = crate::BitReader;
///Field `PWRSAV` writer - Power saving configuration bit
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIDBUS` reader - Wide bus mode enable bit
pub type WIDBUS_R = crate::FieldReader;
///Field `WIDBUS` writer - Wide bus mode enable bit
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NEGEDGE` reader - SDIO_CK dephasing selection bit
pub type NEGEDGE_R = crate::BitReader;
///Field `NEGEDGE` writer - SDIO_CK dephasing selection bit
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HWFC_EN` reader - HW Flow Control enable
pub type HWFC_EN_R = crate::BitReader;
///Field `HWFC_EN` writer - HW Flow Control enable
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDR` reader - Data rate signaling selection
pub type DDR_R = crate::BitReader;
///Field `DDR` writer - Data rate signaling selection
pub type DDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSSPEED` reader - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50, SDR104
pub type BUSSPEED_R = crate::BitReader;
///Field `BUSSPEED` writer - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50, SDR104
pub type BUSSPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SELCLKRX` reader - Receive clock selection
pub type SELCLKRX_R = crate::FieldReader;
///Field `SELCLKRX` writer - Receive clock selection
pub type SELCLKRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:9 - Clock divide factor
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 12 - Power saving configuration bit
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - Wide bus mode enable bit
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - SDIO_CK dephasing selection bit
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HW Flow Control enable
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Data rate signaling selection
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50, SDR104
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Receive clock selection
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCR")
            .field("selclkrx", &self.selclkrx())
            .field("busspeed", &self.busspeed())
            .field("ddr", &self.ddr())
            .field("hwfc_en", &self.hwfc_en())
            .field("negedge", &self.negedge())
            .field("widbus", &self.widbus())
            .field("pwrsav", &self.pwrsav())
            .field("clkdiv", &self.clkdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Clock divide factor
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    ///Bit 12 - Power saving configuration bit
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W<CLKCRrs> {
        PWRSAV_W::new(self, 12)
    }
    ///Bits 14:15 - Wide bus mode enable bit
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W<CLKCRrs> {
        WIDBUS_W::new(self, 14)
    }
    ///Bit 16 - SDIO_CK dephasing selection bit
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W<CLKCRrs> {
        NEGEDGE_W::new(self, 16)
    }
    ///Bit 17 - HW Flow Control enable
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<CLKCRrs> {
        HWFC_EN_W::new(self, 17)
    }
    ///Bit 18 - Data rate signaling selection
    #[inline(always)]
    pub fn ddr(&mut self) -> DDR_W<CLKCRrs> {
        DDR_W::new(self, 18)
    }
    ///Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50, SDR104
    #[inline(always)]
    pub fn busspeed(&mut self) -> BUSSPEED_W<CLKCRrs> {
        BUSSPEED_W::new(self, 19)
    }
    ///Bits 20:21 - Receive clock selection
    #[inline(always)]
    pub fn selclkrx(&mut self) -> SELCLKRX_W<CLKCRrs> {
        SELCLKRX_W::new(self, 20)
    }
}
/**clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SDMMC1:CLKCR)*/
pub struct CLKCRrs;
impl crate::RegisterSpec for CLKCRrs {
    type Ux = u32;
}
///`read()` method returns [`clkcr::R`](R) reader structure
impl crate::Readable for CLKCRrs {}
///`write(|w| ..)` method takes [`clkcr::W`](W) writer structure
impl crate::Writable for CLKCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLKCR to value 0
impl crate::Resettable for CLKCRrs {}
