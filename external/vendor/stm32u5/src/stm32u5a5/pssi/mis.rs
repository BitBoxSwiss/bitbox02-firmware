///Register `MIS` reader
pub type R = crate::R<MISrs>;
/**OVR_MIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MIS {
    ///0: No interrupt is generated when an overrun/underrun error occurs
    Disabled = 0,
    ///1: An interrupt is generated if there is either an overrun or an underrun error and the OVR_IE bit is set in PSSI_IER
    Enabled = 1,
}
impl From<OVR_MIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_MIS) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_MIS` reader - OVR_MIS
pub type OVR_MIS_R = crate::BitReader<OVR_MIS>;
impl OVR_MIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR_MIS {
        match self.bits {
            false => OVR_MIS::Disabled,
            true => OVR_MIS::Enabled,
        }
    }
    ///No interrupt is generated when an overrun/underrun error occurs
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_MIS::Disabled
    }
    ///An interrupt is generated if there is either an overrun or an underrun error and the OVR_IE bit is set in PSSI_IER
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_MIS::Enabled
    }
}
impl R {
    ///Bit 1 - OVR_MIS
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("ovr_mis", &self.ovr_mis())
            .finish()
    }
}
/**PSSI masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#PSSI:MIS)*/
pub struct MISrs;
impl crate::RegisterSpec for MISrs {
    type Ux = u32;
}
///`read()` method returns [`mis::R`](R) reader structure
impl crate::Readable for MISrs {}
///`reset()` method sets MIS to value 0
impl crate::Resettable for MISrs {}
