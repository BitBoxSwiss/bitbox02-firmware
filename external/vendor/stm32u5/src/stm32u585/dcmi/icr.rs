///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Capture complete interrupt status clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_ISC {
    ///1: Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register
    Clear = 1,
}
impl From<FRAME_ISC> for bool {
    #[inline(always)]
    fn from(variant: FRAME_ISC) -> Self {
        variant as u8 != 0
    }
}
///Field `FRAME_ISC` writer - Capture complete interrupt status clear
pub type FRAME_ISC_W<'a, REG> = crate::BitWriter<'a, REG, FRAME_ISC>;
impl<'a, REG> FRAME_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_ISC::Clear)
    }
}
/**Overrun interrupt status clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_ISC {
    ///1: Setting this bit clears the OVR_RIS flag in the DCMI_RIS register
    Clear = 1,
}
impl From<OVR_ISC> for bool {
    #[inline(always)]
    fn from(variant: OVR_ISC) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_ISC` writer - Overrun interrupt status clear
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG, OVR_ISC>;
impl<'a, REG> OVR_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the OVR_RIS flag in the DCMI_RIS register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_ISC::Clear)
    }
}
/**Synchronization error interrupt status clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_ISC {
    ///1: Setting this bit clears the ERR_RIS flag in the DCMI_RIS register
    Clear = 1,
}
impl From<ERR_ISC> for bool {
    #[inline(always)]
    fn from(variant: ERR_ISC) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR_ISC` writer - Synchronization error interrupt status clear
pub type ERR_ISC_W<'a, REG> = crate::BitWriter<'a, REG, ERR_ISC>;
impl<'a, REG> ERR_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the ERR_RIS flag in the DCMI_RIS register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_ISC::Clear)
    }
}
/**Vertical Synchronization interrupt status clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_ISC {
    ///1: Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register
    Clear = 1,
}
impl From<VSYNC_ISC> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_ISC) -> Self {
        variant as u8 != 0
    }
}
///Field `VSYNC_ISC` writer - Vertical Synchronization interrupt status clear
pub type VSYNC_ISC_W<'a, REG> = crate::BitWriter<'a, REG, VSYNC_ISC>;
impl<'a, REG> VSYNC_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VSYNC_ISC::Clear)
    }
}
/**line interrupt status clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_ISC {
    ///1: Setting this bit clears the LINE_RIS flag in the DCMI_RIS register
    Clear = 1,
}
impl From<LINE_ISC> for bool {
    #[inline(always)]
    fn from(variant: LINE_ISC) -> Self {
        variant as u8 != 0
    }
}
///Field `LINE_ISC` writer - line interrupt status clear
pub type LINE_ISC_W<'a, REG> = crate::BitWriter<'a, REG, LINE_ISC>;
impl<'a, REG> LINE_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the LINE_RIS flag in the DCMI_RIS register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_ISC::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Capture complete interrupt status clear
    #[inline(always)]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<ICRrs> {
        FRAME_ISC_W::new(self, 0)
    }
    ///Bit 1 - Overrun interrupt status clear
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
    ///Bit 2 - Synchronization error interrupt status clear
    #[inline(always)]
    pub fn err_isc(&mut self) -> ERR_ISC_W<ICRrs> {
        ERR_ISC_W::new(self, 2)
    }
    ///Bit 3 - Vertical Synchronization interrupt status clear
    #[inline(always)]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<ICRrs> {
        VSYNC_ISC_W::new(self, 3)
    }
    ///Bit 4 - line interrupt status clear
    #[inline(always)]
    pub fn line_isc(&mut self) -> LINE_ISC_W<ICRrs> {
        LINE_ISC_W::new(self, 4)
    }
}
/**interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCMI:ICR)*/
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
