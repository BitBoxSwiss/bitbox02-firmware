///Register `DFLTRSFR` reader
pub type R = crate::R<DFLTRSFRrs>;
///Register `DFLTRSFR` writer
pub type W = crate::W<DFLTRSFRrs>;
///Field `RSFLTBYP` reader - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type RSFLTBYP_R = crate::BitReader;
///Field `RSFLTBYP` writer - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type RSFLTBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSFLTD` reader - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type RSFLTD_R = crate::BitReader;
///Field `RSFLTD` writer - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type RSFLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPFBYP` reader - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type HPFBYP_R = crate::BitReader;
///Field `HPFBYP` writer - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type HPFBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPFC` reader - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type HPFC_R = crate::FieldReader;
///Field `HPFC` writer - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type HPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn rsfltbyp(&self) -> RSFLTBYP_R {
        RSFLTBYP_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn rsfltd(&self) -> RSFLTD_R {
        RSFLTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn hpfbyp(&self) -> HPFBYP_R {
        HPFBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn hpfc(&self) -> HPFC_R {
        HPFC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTRSFR")
            .field("rsfltbyp", &self.rsfltbyp())
            .field("rsfltd", &self.rsfltd())
            .field("hpfbyp", &self.hpfbyp())
            .field("hpfc", &self.hpfc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn rsfltbyp(&mut self) -> RSFLTBYP_W<DFLTRSFRrs> {
        RSFLTBYP_W::new(self, 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn rsfltd(&mut self) -> RSFLTD_W<DFLTRSFRrs> {
        RSFLTD_W::new(self, 4)
    }
    ///Bit 7 - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn hpfbyp(&mut self) -> HPFBYP_W<DFLTRSFRrs> {
        HPFBYP_W::new(self, 7)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn hpfc(&mut self) -> HPFC_W<DFLTRSFRrs> {
        HPFC_W::new(self, 8)
    }
}
/**This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`dfltrsfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltrsfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTRSFRrs;
impl crate::RegisterSpec for DFLTRSFRrs {
    type Ux = u32;
}
///`read()` method returns [`dfltrsfr::R`](R) reader structure
impl crate::Readable for DFLTRSFRrs {}
///`write(|w| ..)` method takes [`dfltrsfr::W`](W) writer structure
impl crate::Writable for DFLTRSFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLTRSFR to value 0
impl crate::Resettable for DFLTRSFRrs {}
