///Register `OLDCR` reader
pub type R = crate::R<OLDCRrs>;
///Register `OLDCR` writer
pub type W = crate::W<OLDCRrs>;
///Field `OLDEN` reader - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
pub type OLDEN_R = crate::BitReader;
///Field `OLDEN` writer - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
pub type OLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THINB` reader - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type THINB_R = crate::BitReader;
///Field `THINB` writer - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type THINB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKOLD` reader - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\] = 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\] = 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BKOLD_R = crate::FieldReader;
///Field `BKOLD` writer - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\] = 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\] = 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BKOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACICN` reader - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICN_R = crate::FieldReader;
///Field `ACICN` writer - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ACICD` reader - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICD_R = crate::FieldReader;
///Field `ACICD` writer - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACICD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OLDACTIVE` reader - OLD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the OLD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the OLDACTIVE is set to , please refer to Section 1.4.15: Register protection for details. The delay between a transition on OLDEN and a transition on OLDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The OLD is not active, and can be configured if needed - 1: The OLD is active, and protected fields cannot be configured.
pub type OLDACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
    #[inline(always)]
    pub fn olden(&self) -> OLDEN_R {
        OLDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn thinb(&self) -> THINB_R {
        THINB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:7 - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\] = 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\] = 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bkold(&self) -> BKOLD_R {
        BKOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:13 - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acicn(&self) -> ACICN_R {
        ACICN_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 17:21 - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acicd(&self) -> ACICD_R {
        ACICD_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 31 - OLD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the OLD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the OLDACTIVE is set to , please refer to Section 1.4.15: Register protection for details. The delay between a transition on OLDEN and a transition on OLDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The OLD is not active, and can be configured if needed - 1: The OLD is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn oldactive(&self) -> OLDACTIVE_R {
        OLDACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OLDCR")
            .field("olden", &self.olden())
            .field("thinb", &self.thinb())
            .field("bkold", &self.bkold())
            .field("acicn", &self.acicn())
            .field("acicd", &self.acicd())
            .field("oldactive", &self.oldactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode.
    #[inline(always)]
    pub fn olden(&mut self) -> OLDEN_W<OLDCRrs> {
        OLDEN_W::new(self, 0)
    }
    ///Bit 1 - Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn thinb(&mut self) -> THINB_W<OLDCRrs> {
        THINB_W::new(self, 1)
    }
    ///Bits 4:7 - Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\[i\] = 0: Break signal (mdf_break\[i\]) is not assigned to threshold event BKOLD\[i\] = 1: Break signal (mdf_break\[i\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bkold(&mut self) -> BKOLD_W<OLDCRrs> {
        BKOLD_W::new(self, 4)
    }
    ///Bits 12:13 - OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acicn(&mut self) -> ACICN_W<OLDCRrs> {
        ACICN_W::new(self, 12)
    }
    ///Bits 17:21 - OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\[2:0\] = 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acicd(&mut self) -> ACICD_W<OLDCRrs> {
        ACICD_W::new(self, 17)
    }
}
/**This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`oldcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oldcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OLDCRrs;
impl crate::RegisterSpec for OLDCRrs {
    type Ux = u32;
}
///`read()` method returns [`oldcr::R`](R) reader structure
impl crate::Readable for OLDCRrs {}
///`write(|w| ..)` method takes [`oldcr::W`](W) writer structure
impl crate::Writable for OLDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLDCR to value 0
impl crate::Resettable for OLDCRrs {}
