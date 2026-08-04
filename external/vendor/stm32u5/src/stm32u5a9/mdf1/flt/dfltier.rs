///Register `DFLTIER` reader
pub type R = crate::R<DFLTIERrs>;
///Register `DFLTIER` writer
pub type W = crate::W<DFLTIERrs>;
///Field `FTHIE` reader - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
pub type FTHIE_R = crate::BitReader;
///Field `FTHIE` writer - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
pub type FTHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOVRIE` reader - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
pub type DOVRIE_R = crate::BitReader;
///Field `DOVRIE` writer - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
pub type DOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSDRIE` reader - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
pub type SSDRIE_R = crate::BitReader;
///Field `SSDRIE` writer - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
pub type SSDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OLDIE` reader - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
pub type OLDIE_R = crate::BitReader;
///Field `OLDIE` writer - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
pub type OLDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSOVRIE` reader - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
pub type SSOVRIE_R = crate::BitReader;
///Field `SSOVRIE` writer - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
pub type SSOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDIE` reader - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
pub type SCDIE_R = crate::BitReader;
///Field `SCDIE` writer - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATIE` reader - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
pub type SATIE_R = crate::BitReader;
///Field `SATIE` writer - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
pub type SATIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABIE` reader - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
pub type CKABIE_R = crate::BitReader;
///Field `CKABIE` writer - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOVRIE` reader - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
pub type RFOVRIE_R = crate::BitReader;
///Field `RFOVRIE` writer - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
pub type RFOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
    #[inline(always)]
    pub fn ssdrie(&self) -> SSDRIE_R {
        SSDRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
    #[inline(always)]
    pub fn oldie(&self) -> OLDIE_R {
        OLDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
    #[inline(always)]
    pub fn ssovrie(&self) -> SSOVRIE_R {
        SSOVRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTIER")
            .field("fthie", &self.fthie())
            .field("dovrie", &self.dovrie())
            .field("ssdrie", &self.ssdrie())
            .field("oldie", &self.oldie())
            .field("ssovrie", &self.ssovrie())
            .field("scdie", &self.scdie())
            .field("satie", &self.satie())
            .field("ckabie", &self.ckabie())
            .field("rfovrie", &self.rfovrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
    #[inline(always)]
    pub fn fthie(&mut self) -> FTHIE_W<DFLTIERrs> {
        FTHIE_W::new(self, 0)
    }
    ///Bit 1 - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
    #[inline(always)]
    pub fn dovrie(&mut self) -> DOVRIE_W<DFLTIERrs> {
        DOVRIE_W::new(self, 1)
    }
    ///Bit 2 - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
    #[inline(always)]
    pub fn ssdrie(&mut self) -> SSDRIE_W<DFLTIERrs> {
        SSDRIE_W::new(self, 2)
    }
    ///Bit 4 - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
    #[inline(always)]
    pub fn oldie(&mut self) -> OLDIE_W<DFLTIERrs> {
        OLDIE_W::new(self, 4)
    }
    ///Bit 7 - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
    #[inline(always)]
    pub fn ssovrie(&mut self) -> SSOVRIE_W<DFLTIERrs> {
        SSOVRIE_W::new(self, 7)
    }
    ///Bit 8 - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W<DFLTIERrs> {
        SCDIE_W::new(self, 8)
    }
    ///Bit 9 - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
    #[inline(always)]
    pub fn satie(&mut self) -> SATIE_W<DFLTIERrs> {
        SATIE_W::new(self, 9)
    }
    ///Bit 10 - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<DFLTIERrs> {
        CKABIE_W::new(self, 10)
    }
    ///Bit 11 - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
    #[inline(always)]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<DFLTIERrs> {
        RFOVRIE_W::new(self, 11)
    }
}
/**This register is used for allowing or not the events to generate an interrupt.

You can [`read`](crate::Reg::read) this register and get [`dfltier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTIERrs;
impl crate::RegisterSpec for DFLTIERrs {
    type Ux = u32;
}
///`read()` method returns [`dfltier::R`](R) reader structure
impl crate::Readable for DFLTIERrs {}
///`write(|w| ..)` method takes [`dfltier::W`](W) writer structure
impl crate::Writable for DFLTIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLTIER to value 0
impl crate::Resettable for DFLTIERrs {}
