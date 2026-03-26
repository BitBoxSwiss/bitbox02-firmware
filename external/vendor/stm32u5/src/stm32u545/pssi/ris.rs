///Register `RIS` reader
pub type R = crate::R<RISrs>;
/**OVR_RIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_RIS {
    ///0: No overrun/underrun occurred
    Cleared = 0,
    ///1: An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode. This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR
    Occurred = 1,
}
impl From<OVR_RIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_RIS) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_RIS` reader - OVR_RIS
pub type OVR_RIS_R = crate::BitReader<OVR_RIS>;
impl OVR_RIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR_RIS {
        match self.bits {
            false => OVR_RIS::Cleared,
            true => OVR_RIS::Occurred,
        }
    }
    ///No overrun/underrun occurred
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == OVR_RIS::Cleared
    }
    ///An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode. This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == OVR_RIS::Occurred
    }
}
impl R {
    ///Bit 1 - OVR_RIS
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("ovr_ris", &self.ovr_ris())
            .finish()
    }
}
/**PSSI raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PSSI:RIS)*/
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RISrs {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RISrs {}
