///Register `SITFCR` reader
pub type R = crate::R<SITFCRrs>;
///Register `SITFCR` writer
pub type W = crate::W<SITFCRrs>;
///Field `SITFEN` reader - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
pub type SITFEN_R = crate::BitReader;
///Field `SITFEN` writer - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
pub type SITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCKSRC` reader - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCKSRC_R = crate::FieldReader;
///Field `SCKSRC` writer - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SITFMOD` reader - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SITFMOD_R = crate::FieldReader;
///Field `SITFMOD` writer - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SITFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STH` reader - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\] lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type STH_R = crate::FieldReader;
///Field `STH` writer - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\] lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type STH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SITFACTIVE` reader - Serial interface Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when the SITFACTIVE is set , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SITFEN and a transition on SITFACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The serial interface is not active, and can be configured if needed - 1: The serial interface is active, and protected fields cannot be configured.
pub type SITFACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:12 - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\] lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 31 - Serial interface Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when the SITFACTIVE is set , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SITFEN and a transition on SITFACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The serial interface is not active, and can be configured if needed - 1: The serial interface is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SITFCR")
            .field("sitfen", &self.sitfen())
            .field("scksrc", &self.scksrc())
            .field("sitfmod", &self.sitfmod())
            .field("sth", &self.sth())
            .field("sitfactive", &self.sitfactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled
    #[inline(always)]
    pub fn sitfen(&mut self) -> SITFEN_W<SITFCRrs> {
        SITFEN_W::new(self, 0)
    }
    ///Bits 1:2 - Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn scksrc(&mut self) -> SCKSRC_W<SITFCRrs> {
        SCKSRC_W::new(self, 1)
    }
    ///Bits 4:5 - Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn sitfmod(&mut self) -> SITFMOD_W<SITFCRrs> {
        SITFMOD_W::new(self, 4)
    }
    ///Bits 8:12 - Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\[4:0\] lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn sth(&mut self) -> STH_W<SITFCRrs> {
        STH_W::new(self, 8)
    }
}
/**This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`sitfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SITFCRrs;
impl crate::RegisterSpec for SITFCRrs {
    type Ux = u32;
}
///`read()` method returns [`sitfcr::R`](R) reader structure
impl crate::Readable for SITFCRrs {}
///`write(|w| ..)` method takes [`sitfcr::W`](W) writer structure
impl crate::Writable for SITFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SITFCR to value 0x1f00
impl crate::Resettable for SITFCRrs {
    const RESET_VALUE: u32 = 0x1f00;
}
