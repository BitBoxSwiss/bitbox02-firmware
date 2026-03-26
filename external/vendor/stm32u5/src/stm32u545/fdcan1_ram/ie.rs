///Register `IE` reader
pub type R = crate::R<IErs>;
///Register `IE` writer
pub type W = crate::W<IErs>;
///Field `RF0NE` reader - Rx FIFO 0 New Message Enable
pub type RF0NE_R = crate::BitReader;
///Field `RF0NE` writer - Rx FIFO 0 New Message Enable
pub type RF0NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0FE` reader - Rx FIFO 0 Full Enable
pub type RF0FE_R = crate::BitReader;
///Field `RF0FE` writer - Rx FIFO 0 Full Enable
pub type RF0FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0LE` reader - Rx FIFO 0 Message Lost Enable
pub type RF0LE_R = crate::BitReader;
///Field `RF0LE` writer - Rx FIFO 0 Message Lost Enable
pub type RF0LE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1NE` reader - Rx FIFO 1 New Message Enable
pub type RF1NE_R = crate::BitReader;
///Field `RF1NE` writer - Rx FIFO 1 New Message Enable
pub type RF1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1FE` reader - Rx FIFO 1 Watermark Reached Enable
pub type RF1FE_R = crate::BitReader;
///Field `RF1FE` writer - Rx FIFO 1 Watermark Reached Enable
pub type RF1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1LE` reader - Rx FIFO 1 Message Lost Enable
pub type RF1LE_R = crate::BitReader;
///Field `RF1LE` writer - Rx FIFO 1 Message Lost Enable
pub type RF1LE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPME` reader - High Priority Message Enable
pub type HPME_R = crate::BitReader;
///Field `HPME` writer - High Priority Message Enable
pub type HPME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCE` reader - Transmission Completed Enable
pub type TCE_R = crate::BitReader;
///Field `TCE` writer - Transmission Completed Enable
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCFE` reader - Transmission Cancellation Finished Enable
pub type TCFE_R = crate::BitReader;
///Field `TCFE` writer - Transmission Cancellation Finished Enable
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFE` reader - Tx FIFO Empty Enable
pub type TEFE_R = crate::BitReader;
///Field `TEFE` writer - Tx FIFO Empty Enable
pub type TEFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFNE` reader - Tx Event FIFO New Entry Enable
pub type TEFNE_R = crate::BitReader;
///Field `TEFNE` writer - Tx Event FIFO New Entry Enable
pub type TEFNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFFE` reader - Tx Event FIFO Full Enable
pub type TEFFE_R = crate::BitReader;
///Field `TEFFE` writer - Tx Event FIFO Full Enable
pub type TEFFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFLE` reader - Tx Event FIFO Element Lost Enable
pub type TEFLE_R = crate::BitReader;
///Field `TEFLE` writer - Tx Event FIFO Element Lost Enable
pub type TEFLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSWE` reader - TSWE
pub type TSWE_R = crate::BitReader;
///Field `TSWE` writer - TSWE
pub type TSWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAFE` reader - Message RAM Access Failure Enable
pub type MRAFE_R = crate::BitReader;
///Field `MRAFE` writer - Message RAM Access Failure Enable
pub type MRAFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOOE` reader - Timeout Occurred Enable
pub type TOOE_R = crate::BitReader;
///Field `TOOE` writer - Timeout Occurred Enable
pub type TOOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELOE` reader - Error Logging Overflow Enable
pub type ELOE_R = crate::BitReader;
///Field `ELOE` writer - Error Logging Overflow Enable
pub type ELOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPE` reader - Error Passive Enable
pub type EPE_R = crate::BitReader;
///Field `EPE` writer - Error Passive Enable
pub type EPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWE` reader - Warning Status Enable
pub type EWE_R = crate::BitReader;
///Field `EWE` writer - Warning Status Enable
pub type EWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOE` reader - Bus_Off Status Enable
pub type BOE_R = crate::BitReader;
///Field `BOE` writer - Bus_Off Status Enable
pub type BOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDIE` reader - Watchdog Interrupt Enable
pub type WDIE_R = crate::BitReader;
///Field `WDIE` writer - Watchdog Interrupt Enable
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEAE` reader - Protocol Error in Arbitration Phase Enable
pub type PEAE_R = crate::BitReader;
///Field `PEAE` writer - Protocol Error in Arbitration Phase Enable
pub type PEAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEDE` reader - Protocol Error in Data Phase Enable
pub type PEDE_R = crate::BitReader;
///Field `PEDE` writer - Protocol Error in Data Phase Enable
pub type PEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARAE` reader - Access to Reserved Address Enable
pub type ARAE_R = crate::BitReader;
///Field `ARAE` writer - Access to Reserved Address Enable
pub type ARAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx FIFO 0 New Message Enable
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Full Enable
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Message Lost Enable
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 1 New Message Enable
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 Watermark Reached Enable
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Message Lost Enable
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High Priority Message Enable
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmission Completed Enable
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmission Cancellation Finished Enable
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx FIFO Empty Enable
    #[inline(always)]
    pub fn tefe(&self) -> TEFE_R {
        TEFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx Event FIFO New Entry Enable
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx Event FIFO Full Enable
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx Event FIFO Element Lost Enable
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TSWE
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Message RAM Access Failure Enable
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timeout Occurred Enable
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Error Logging Overflow Enable
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Error Passive Enable
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Warning Status Enable
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bus_Off Status Enable
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Watchdog Interrupt Enable
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Protocol Error in Arbitration Phase Enable
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Protocol Error in Data Phase Enable
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Access to Reserved Address Enable
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("rf0ne", &self.rf0ne())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tefe", &self.tefe())
            .field("tefne", &self.tefne())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message Enable
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W<IErs> {
        RF0NE_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 Full Enable
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W<IErs> {
        RF0FE_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 Message Lost Enable
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W<IErs> {
        RF0LE_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 1 New Message Enable
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W<IErs> {
        RF1NE_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 Watermark Reached Enable
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W<IErs> {
        RF1FE_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 Message Lost Enable
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W<IErs> {
        RF1LE_W::new(self, 5)
    }
    ///Bit 6 - High Priority Message Enable
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W<IErs> {
        HPME_W::new(self, 6)
    }
    ///Bit 7 - Transmission Completed Enable
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W<IErs> {
        TCE_W::new(self, 7)
    }
    ///Bit 8 - Transmission Cancellation Finished Enable
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W<IErs> {
        TCFE_W::new(self, 8)
    }
    ///Bit 9 - Tx FIFO Empty Enable
    #[inline(always)]
    pub fn tefe(&mut self) -> TEFE_W<IErs> {
        TEFE_W::new(self, 9)
    }
    ///Bit 10 - Tx Event FIFO New Entry Enable
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W<IErs> {
        TEFNE_W::new(self, 10)
    }
    ///Bit 11 - Tx Event FIFO Full Enable
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W<IErs> {
        TEFFE_W::new(self, 11)
    }
    ///Bit 12 - Tx Event FIFO Element Lost Enable
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W<IErs> {
        TEFLE_W::new(self, 12)
    }
    ///Bit 13 - TSWE
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W<IErs> {
        TSWE_W::new(self, 13)
    }
    ///Bit 14 - Message RAM Access Failure Enable
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W<IErs> {
        MRAFE_W::new(self, 14)
    }
    ///Bit 15 - Timeout Occurred Enable
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W<IErs> {
        TOOE_W::new(self, 15)
    }
    ///Bit 16 - Error Logging Overflow Enable
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W<IErs> {
        ELOE_W::new(self, 16)
    }
    ///Bit 17 - Error Passive Enable
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W<IErs> {
        EPE_W::new(self, 17)
    }
    ///Bit 18 - Warning Status Enable
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W<IErs> {
        EWE_W::new(self, 18)
    }
    ///Bit 19 - Bus_Off Status Enable
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W<IErs> {
        BOE_W::new(self, 19)
    }
    ///Bit 20 - Watchdog Interrupt Enable
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W<IErs> {
        WDIE_W::new(self, 20)
    }
    ///Bit 21 - Protocol Error in Arbitration Phase Enable
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W<IErs> {
        PEAE_W::new(self, 21)
    }
    ///Bit 22 - Protocol Error in Data Phase Enable
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W<IErs> {
        PEDE_W::new(self, 22)
    }
    ///Bit 23 - Access to Reserved Address Enable
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W<IErs> {
        ARAE_W::new(self, 23)
    }
}
/**FDCAN Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FDCAN1_RAM:IE)*/
pub struct IErs;
impl crate::RegisterSpec for IErs {
    type Ux = u32;
}
///`read()` method returns [`ie::R`](R) reader structure
impl crate::Readable for IErs {}
///`write(|w| ..)` method takes [`ie::W`](W) writer structure
impl crate::Writable for IErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IE to value 0
impl crate::Resettable for IErs {}
