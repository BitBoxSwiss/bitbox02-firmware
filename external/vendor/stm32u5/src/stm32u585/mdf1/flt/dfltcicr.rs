///Register `DFLTCICR` reader
pub type R = crate::R<DFLTCICRrs>;
///Register `DFLTCICR` writer
pub type W = crate::W<DFLTCICRrs>;
///Field `DATSRC` reader - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DATSRC_R = crate::FieldReader;
///Field `DATSRC` writer - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DATSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CICMOD` reader - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\] is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type CICMOD_R = crate::FieldReader;
///Field `CICMOD` writer - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\] is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type CICMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCICD` reader - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type MCICD_R = crate::FieldReader<u16>;
///Field `MCICD` writer - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type MCICD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SCALE` reader - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\] field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
pub type SCALE_R = crate::FieldReader;
///Field `SCALE` writer - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\] field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1 - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\] is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:16 - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    ///Bits 20:25 - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\] field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTCICR")
            .field("datsrc", &self.datsrc())
            .field("cicmod", &self.cicmod())
            .field("mcicd", &self.mcicd())
            .field("scale", &self.scale())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn datsrc(&mut self) -> DATSRC_W<DFLTCICRrs> {
        DATSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\[2:0\] is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn cicmod(&mut self) -> CICMOD_W<DFLTCICRrs> {
        CICMOD_W::new(self, 4)
    }
    ///Bits 8:16 - CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn mcicd(&mut self) -> MCICD_W<DFLTCICRrs> {
        MCICD_W::new(self, 8)
    }
    ///Bits 20:25 - Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\[5:0\] field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<DFLTCICRrs> {
        SCALE_W::new(self, 20)
    }
}
/**This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`dfltcicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltcicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTCICRrs;
impl crate::RegisterSpec for DFLTCICRrs {
    type Ux = u32;
}
///`read()` method returns [`dfltcicr::R`](R) reader structure
impl crate::Readable for DFLTCICRrs {}
///`write(|w| ..)` method takes [`dfltcicr::W`](W) writer structure
impl crate::Writable for DFLTCICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLTCICR to value 0
impl crate::Resettable for DFLTCICRrs {}
