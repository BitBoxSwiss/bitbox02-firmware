///Register `WISR` reader
pub type R = crate::R<WISRrs>;
///Field `TEIF` reader - Tearing effect interrupt flag This bit is set when a tearing effect event occurs.
pub type TEIF_R = crate::BitReader;
///Field `ERIF` reader - End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished.
pub type ERIF_R = crate::BitReader;
///Field `BUSY` reader - Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing.
pub type BUSY_R = crate::BitReader;
///Field `PLLLS` reader - PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked.
pub type PLLLS_R = crate::BitReader;
///Field `PLLLIF` reader - PLL lock interrupt flag This bit is set when the PLL becomes locked.
pub type PLLLIF_R = crate::BitReader;
///Field `PLLUIF` reader - PLL unlock interrupt flag This bit is set when the PLL becomes unlocked.
pub type PLLUIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Tearing effect interrupt flag This bit is set when a tearing effect event occurs.
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished.
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked.
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL lock interrupt flag This bit is set when the PLL becomes locked.
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL unlock interrupt flag This bit is set when the PLL becomes unlocked.
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WISR")
            .field("teif", &self.teif())
            .field("erif", &self.erif())
            .field("busy", &self.busy())
            .field("pllls", &self.pllls())
            .field("plllif", &self.plllif())
            .field("plluif", &self.plluif())
            .finish()
    }
}
/**DSI Wrapper interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`wisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:WISR)*/
pub struct WISRrs;
impl crate::RegisterSpec for WISRrs {
    type Ux = u32;
}
///`read()` method returns [`wisr::R`](R) reader structure
impl crate::Readable for WISRrs {}
///`reset()` method sets WISR to value 0
impl crate::Resettable for WISRrs {}
