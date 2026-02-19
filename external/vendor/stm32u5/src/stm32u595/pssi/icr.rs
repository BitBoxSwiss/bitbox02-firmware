///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**OVR_ISC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_ISC {
    ///1: Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS
    Clear = 1,
}
impl From<OVR_ISC> for bool {
    #[inline(always)]
    fn from(variant: OVR_ISC) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_ISC` writer - OVR_ISC
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG, OVR_ISC>;
impl<'a, REG> OVR_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_ISC::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - OVR_ISC
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
}
/**PSSI interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#PSSI:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
