///Register `POWER` reader
pub type R = crate::R<POWERrs>;
///Register `POWER` writer
pub type W = crate::W<POWERrs>;
///Field `PWRCTRL` reader - SDMMC state control bits
pub type PWRCTRL_R = crate::FieldReader;
///Field `PWRCTRL` writer - SDMMC state control bits
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VSWITCH` reader - Voltage switch sequence start
pub type VSWITCH_R = crate::BitReader;
///Field `VSWITCH` writer - Voltage switch sequence start
pub type VSWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSWITCHEN` reader - Voltage switch procedure enable
pub type VSWITCHEN_R = crate::BitReader;
///Field `VSWITCHEN` writer - Voltage switch procedure enable
pub type VSWITCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRPOL` reader - Data and command direction signals polarity selection
pub type DIRPOL_R = crate::BitReader;
///Field `DIRPOL` writer - Data and command direction signals polarity selection
pub type DIRPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - SDMMC state control bits
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Voltage switch sequence start
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage switch procedure enable
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data and command direction signals polarity selection
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER")
            .field("pwrctrl", &self.pwrctrl())
            .field("vswitch", &self.vswitch())
            .field("vswitchen", &self.vswitchen())
            .field("dirpol", &self.dirpol())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SDMMC state control bits
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
    ///Bit 2 - Voltage switch sequence start
    #[inline(always)]
    pub fn vswitch(&mut self) -> VSWITCH_W<POWERrs> {
        VSWITCH_W::new(self, 2)
    }
    ///Bit 3 - Voltage switch procedure enable
    #[inline(always)]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W<POWERrs> {
        VSWITCHEN_W::new(self, 3)
    }
    ///Bit 4 - Data and command direction signals polarity selection
    #[inline(always)]
    pub fn dirpol(&mut self) -> DIRPOL_W<POWERrs> {
        DIRPOL_W::new(self, 4)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SDMMC1:POWER)*/
pub struct POWERrs;
impl crate::RegisterSpec for POWERrs {
    type Ux = u32;
}
///`read()` method returns [`power::R`](R) reader structure
impl crate::Readable for POWERrs {}
///`write(|w| ..)` method takes [`power::W`](W) writer structure
impl crate::Writable for POWERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POWER to value 0
impl crate::Resettable for POWERrs {}
