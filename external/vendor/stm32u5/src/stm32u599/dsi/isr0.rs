///Register `ISR0` reader
pub type R = crate::R<ISR0rs>;
///Field `AE0` reader - Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report.
pub type AE0_R = crate::BitReader;
///Field `AE1` reader - Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report.
pub type AE1_R = crate::BitReader;
///Field `AE2` reader - Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report.
pub type AE2_R = crate::BitReader;
///Field `AE3` reader - Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report.
pub type AE3_R = crate::BitReader;
///Field `AE4` reader - Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report.
pub type AE4_R = crate::BitReader;
///Field `AE5` reader - Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report.
pub type AE5_R = crate::BitReader;
///Field `AE6` reader - Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report.
pub type AE6_R = crate::BitReader;
///Field `AE7` reader - Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report.
pub type AE7_R = crate::BitReader;
///Field `AE8` reader - Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report.
pub type AE8_R = crate::BitReader;
///Field `AE9` reader - Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report.
pub type AE9_R = crate::BitReader;
///Field `AE10` reader - Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report.
pub type AE10_R = crate::BitReader;
///Field `AE11` reader - Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report.
pub type AE11_R = crate::BitReader;
///Field `AE12` reader - Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report.
pub type AE12_R = crate::BitReader;
///Field `AE13` reader - Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report.
pub type AE13_R = crate::BitReader;
///Field `AE14` reader - Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report.
pub type AE14_R = crate::BitReader;
///Field `AE15` reader - Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report.
pub type AE15_R = crate::BitReader;
///Field `PE0` reader - PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0.
pub type PE0_R = crate::BitReader;
///Field `PE1` reader - PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0.
pub type PE1_R = crate::BitReader;
///Field `PE2` reader - PHY error 2 This bit indicates the ErrControl error from lane 0.
pub type PE2_R = crate::BitReader;
///Field `PE3` reader - PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0.
pub type PE3_R = crate::BitReader;
///Field `PE4` reader - PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0.
pub type PE4_R = crate::BitReader;
impl R {
    ///Bit 0 - Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report.
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report.
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report.
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report.
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report.
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report.
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report.
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report.
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report.
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report.
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report.
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report.
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report.
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report.
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report.
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report.
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0.
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0.
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PHY error 2 This bit indicates the ErrControl error from lane 0.
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0.
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0.
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR0")
            .field("ae0", &self.ae0())
            .field("ae1", &self.ae1())
            .field("ae2", &self.ae2())
            .field("ae3", &self.ae3())
            .field("ae4", &self.ae4())
            .field("ae5", &self.ae5())
            .field("ae6", &self.ae6())
            .field("ae7", &self.ae7())
            .field("ae8", &self.ae8())
            .field("ae9", &self.ae9())
            .field("ae10", &self.ae10())
            .field("ae11", &self.ae11())
            .field("ae12", &self.ae12())
            .field("ae13", &self.ae13())
            .field("ae14", &self.ae14())
            .field("ae15", &self.ae15())
            .field("pe0", &self.pe0())
            .field("pe1", &self.pe1())
            .field("pe2", &self.pe2())
            .field("pe3", &self.pe3())
            .field("pe4", &self.pe4())
            .finish()
    }
}
/**DSI Host interrupt and status register 0

You can [`read`](crate::Reg::read) this register and get [`isr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:ISR0)*/
pub struct ISR0rs;
impl crate::RegisterSpec for ISR0rs {
    type Ux = u32;
}
///`read()` method returns [`isr0::R`](R) reader structure
impl crate::Readable for ISR0rs {}
///`reset()` method sets ISR0 to value 0
impl crate::Resettable for ISR0rs {}
