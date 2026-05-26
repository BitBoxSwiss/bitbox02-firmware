///Register `OPAMP1_CSR` reader
pub type R = crate::R<OPAMP1_CSRrs>;
///Register `OPAMP1_CSR` writer
pub type W = crate::W<OPAMP1_CSRrs>;
///Field `OPAEN` reader - OPAMP enable
pub type OPAEN_R = crate::BitReader;
///Field `OPAEN` writer - OPAMP enable
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPALPM` reader - OPAMP low-power mode The OPAMP must be disabled to change this configuration.
pub type OPALPM_R = crate::BitReader;
///Field `OPALPM` writer - OPAMP low-power mode The OPAMP must be disabled to change this configuration.
pub type OPALPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMODE` reader - OPAMP PGA mode 00 and 01: internal PGA disabled
pub type OPAMODE_R = crate::FieldReader;
///Field `OPAMODE` writer - OPAMP PGA mode 00 and 01: internal PGA disabled
pub type OPAMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PGA_GAIN` reader - OPAMP programmable amplifier gain value
pub type PGA_GAIN_R = crate::FieldReader;
///Field `PGA_GAIN` writer - OPAMP programmable amplifier gain value
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VM_SEL` reader - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: inverting input not externally connected
pub type VM_SEL_R = crate::FieldReader;
///Field `VM_SEL` writer - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: inverting input not externally connected
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VP_SEL` reader - Non-inverted input selection
pub type VP_SEL_R = crate::BitReader;
///Field `VP_SEL` writer - Non-inverted input selection
pub type VP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALON` reader - Calibration mode enable
pub type CALON_R = crate::BitReader;
///Field `CALON` writer - Calibration mode enable
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::BitReader;
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USERTRIM` reader - ‘factory’ or ‘user’ offset trimmed values selection This bit is active for normal and low-power modes.
pub type USERTRIM_R = crate::BitReader;
///Field `USERTRIM` writer - ‘factory’ or ‘user’ offset trimmed values selection This bit is active for normal and low-power modes.
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALOUT` reader - OPAMP calibration output During the calibration mode, the offset is trimmed when this signal toggles.
pub type CALOUT_R = crate::BitReader;
///Field `OPAHSM` reader - OPAMP high-speed mode This bit is effective for both normal and low-power modes.
pub type OPAHSM_R = crate::BitReader;
///Field `OPAHSM` writer - OPAMP high-speed mode This bit is effective for both normal and low-power modes.
pub type OPAHSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA_RANGE` reader - OPAMP range setting This bit must be set before enabling the OPAMP and this bit affects all OPAMP instances.
pub type OPA_RANGE_R = crate::BitReader;
///Field `OPA_RANGE` writer - OPAMP range setting This bit must be set before enabling the OPAMP and this bit affects all OPAMP instances.
pub type OPA_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OPAMP enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OPAMP low-power mode The OPAMP must be disabled to change this configuration.
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - OPAMP PGA mode 00 and 01: internal PGA disabled
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - OPAMP programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: inverting input not externally connected
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Non-inverted input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ‘factory’ or ‘user’ offset trimmed values selection This bit is active for normal and low-power modes.
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OPAMP calibration output During the calibration mode, the offset is trimmed when this signal toggles.
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 30 - OPAMP high-speed mode This bit is effective for both normal and low-power modes.
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OPAMP range setting This bit must be set before enabling the OPAMP and this bit affects all OPAMP instances.
    #[inline(always)]
    pub fn opa_range(&self) -> OPA_RANGE_R {
        OPA_RANGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP1_CSR")
            .field("opaen", &self.opaen())
            .field("opalpm", &self.opalpm())
            .field("opamode", &self.opamode())
            .field("pga_gain", &self.pga_gain())
            .field("vm_sel", &self.vm_sel())
            .field("vp_sel", &self.vp_sel())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("usertrim", &self.usertrim())
            .field("calout", &self.calout())
            .field("opahsm", &self.opahsm())
            .field("opa_range", &self.opa_range())
            .finish()
    }
}
impl W {
    ///Bit 0 - OPAMP enable
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP1_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - OPAMP low-power mode The OPAMP must be disabled to change this configuration.
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W<OPAMP1_CSRrs> {
        OPALPM_W::new(self, 1)
    }
    ///Bits 2:3 - OPAMP PGA mode 00 and 01: internal PGA disabled
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W<OPAMP1_CSRrs> {
        OPAMODE_W::new(self, 2)
    }
    ///Bits 4:5 - OPAMP programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP1_CSRrs> {
        PGA_GAIN_W::new(self, 4)
    }
    ///Bits 8:9 - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: inverting input not externally connected
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP1_CSRrs> {
        VM_SEL_W::new(self, 8)
    }
    ///Bit 10 - Non-inverted input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP1_CSRrs> {
        VP_SEL_W::new(self, 10)
    }
    ///Bit 12 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<OPAMP1_CSRrs> {
        CALON_W::new(self, 12)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP1_CSRrs> {
        CALSEL_W::new(self, 13)
    }
    ///Bit 14 - ‘factory’ or ‘user’ offset trimmed values selection This bit is active for normal and low-power modes.
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP1_CSRrs> {
        USERTRIM_W::new(self, 14)
    }
    ///Bit 30 - OPAMP high-speed mode This bit is effective for both normal and low-power modes.
    #[inline(always)]
    pub fn opahsm(&mut self) -> OPAHSM_W<OPAMP1_CSRrs> {
        OPAHSM_W::new(self, 30)
    }
    ///Bit 31 - OPAMP range setting This bit must be set before enabling the OPAMP and this bit affects all OPAMP instances.
    #[inline(always)]
    pub fn opa_range(&mut self) -> OPA_RANGE_W<OPAMP1_CSRrs> {
        OPA_RANGE_W::new(self, 31)
    }
}
/**OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#OPAMP:OPAMP1_CSR)*/
pub struct OPAMP1_CSRrs;
impl crate::RegisterSpec for OPAMP1_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp1_csr::R`](R) reader structure
impl crate::Readable for OPAMP1_CSRrs {}
///`write(|w| ..)` method takes [`opamp1_csr::W`](W) writer structure
impl crate::Writable for OPAMP1_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP1_CSR to value 0
impl crate::Resettable for OPAMP1_CSRrs {}
