///Register `DFLTISR` reader
pub type R = crate::R<DFLTISRrs>;
///Register `DFLTISR` writer
pub type W = crate::W<DFLTISRrs>;
///Field `FTHF` reader - FTHF
pub type FTHF_R = crate::BitReader;
///Field `DOVRF` reader - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
pub type DOVRF_R = crate::BitReader;
///Field `DOVRF` writer - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
pub type DOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSDRF` reader - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
pub type SSDRF_R = crate::BitReader;
///Field `SSDRF` writer - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
pub type SSDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEF` reader - RXFIFO Not Empty flag Set and cleared by hardware according to the RXFIFO level. - 0: Reading 0 means that the RXFIFO is empty. - 1: Reading 1 means that the RXFIFO is not empty.
pub type RXNEF_R = crate::BitReader;
///Field `OLDF` reader - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
pub type OLDF_R = crate::BitReader;
///Field `OLDF` writer - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
pub type OLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THLF` reader - Low threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the low threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was higher than OLDTHL when the last OLD event occurred. - 1: The signal was lower than OLDTHL when the last OLD event occurred.
pub type THLF_R = crate::BitReader;
///Field `THHF` reader - High threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the high threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was lower than OLDTHH when the last OLD event occurred. - 1: The signal was higher than OLDTHH when the last OLD event occurred.
pub type THHF_R = crate::BitReader;
///Field `SSOVRF` reader - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
pub type SSOVRF_R = crate::BitReader;
///Field `SSOVRF` writer - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
pub type SSOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDF` reader - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
pub type SCDF_R = crate::BitReader;
///Field `SCDF` writer - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
pub type SCDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATF` reader - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
pub type SATF_R = crate::BitReader;
///Field `SATF` writer - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
pub type SATF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABF` reader - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
pub type CKABF_R = crate::BitReader;
///Field `CKABF` writer - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
pub type CKABF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOVRF` reader - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
pub type RFOVRF_R = crate::BitReader;
///Field `RFOVRF` writer - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
pub type RFOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FTHF
    #[inline(always)]
    pub fn fthf(&self) -> FTHF_R {
        FTHF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn dovrf(&self) -> DOVRF_R {
        DOVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
    #[inline(always)]
    pub fn ssdrf(&self) -> SSDRF_R {
        SSDRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RXFIFO Not Empty flag Set and cleared by hardware according to the RXFIFO level. - 0: Reading 0 means that the RXFIFO is empty. - 1: Reading 1 means that the RXFIFO is not empty.
    #[inline(always)]
    pub fn rxnef(&self) -> RXNEF_R {
        RXNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
    #[inline(always)]
    pub fn oldf(&self) -> OLDF_R {
        OLDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Low threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the low threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was higher than OLDTHL when the last OLD event occurred. - 1: The signal was lower than OLDTHL when the last OLD event occurred.
    #[inline(always)]
    pub fn thlf(&self) -> THLF_R {
        THLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the high threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was lower than OLDTHH when the last OLD event occurred. - 1: The signal was higher than OLDTHH when the last OLD event occurred.
    #[inline(always)]
    pub fn thhf(&self) -> THHF_R {
        THHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn ssovrf(&self) -> SSOVRF_R {
        SSOVRF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn satf(&self) -> SATF_R {
        SATF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn rfovrf(&self) -> RFOVRF_R {
        RFOVRF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTISR")
            .field("fthf", &self.fthf())
            .field("dovrf", &self.dovrf())
            .field("ssdrf", &self.ssdrf())
            .field("rxnef", &self.rxnef())
            .field("oldf", &self.oldf())
            .field("thlf", &self.thlf())
            .field("thhf", &self.thhf())
            .field("ssovrf", &self.ssovrf())
            .field("scdf", &self.scdf())
            .field("satf", &self.satf())
            .field("ckabf", &self.ckabf())
            .field("rfovrf", &self.rfovrf())
            .finish()
    }
}
impl W {
    ///Bit 1 - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn dovrf(&mut self) -> DOVRF_W<DFLTISRrs> {
        DOVRF_W::new(self, 1)
    }
    ///Bit 2 - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
    #[inline(always)]
    pub fn ssdrf(&mut self) -> SSDRF_W<DFLTISRrs> {
        SSDRF_W::new(self, 2)
    }
    ///Bit 4 - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
    #[inline(always)]
    pub fn oldf(&mut self) -> OLDF_W<DFLTISRrs> {
        OLDF_W::new(self, 4)
    }
    ///Bit 7 - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn ssovrf(&mut self) -> SSOVRF_W<DFLTISRrs> {
        SSOVRF_W::new(self, 7)
    }
    ///Bit 8 - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn scdf(&mut self) -> SCDF_W<DFLTISRrs> {
        SCDF_W::new(self, 8)
    }
    ///Bit 9 - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn satf(&mut self) -> SATF_W<DFLTISRrs> {
        SATF_W::new(self, 9)
    }
    ///Bit 10 - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn ckabf(&mut self) -> CKABF_W<DFLTISRrs> {
        CKABF_W::new(self, 10)
    }
    ///Bit 11 - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn rfovrf(&mut self) -> RFOVRF_W<DFLTISRrs> {
        RFOVRF_W::new(self, 11)
    }
}
/**MDF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`dfltisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTISRrs;
impl crate::RegisterSpec for DFLTISRrs {
    type Ux = u32;
}
///`read()` method returns [`dfltisr::R`](R) reader structure
impl crate::Readable for DFLTISRrs {}
///`write(|w| ..)` method takes [`dfltisr::W`](W) writer structure
impl crate::Writable for DFLTISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLTISR to value 0
impl crate::Resettable for DFLTISRrs {}
