///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
///Field `TRIGSEL` reader - TRIGSEL
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - TRIGSEL
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**TRIGPOL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGPOL {
    ///0: trigger is active on raising edge
    RaisingEdge = 0,
    ///1: trigger is active on falling edge
    FallingEdge = 1,
}
impl From<TRIGPOL> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `TRIGPOL` reader - TRIGPOL
pub type TRIGPOL_R = crate::BitReader<TRIGPOL>;
impl TRIGPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRIGPOL {
        match self.bits {
            false => TRIGPOL::RaisingEdge,
            true => TRIGPOL::FallingEdge,
        }
    }
    ///trigger is active on raising edge
    #[inline(always)]
    pub fn is_raising_edge(&self) -> bool {
        *self == TRIGPOL::RaisingEdge
    }
    ///trigger is active on falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGPOL::FallingEdge
    }
}
///Field `TRIGPOL` writer - TRIGPOL
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG, TRIGPOL>;
impl<'a, REG> TRIGPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///trigger is active on raising edge
    #[inline(always)]
    pub fn raising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::RaisingEdge)
    }
    ///trigger is active on falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::FallingEdge)
    }
}
/**TRIGEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGEN {
    ///0: Hardware control disabled
    Disabled = 0,
    ///1: Hardware control enabled
    Enabled = 1,
}
impl From<TRIGEN> for bool {
    #[inline(always)]
    fn from(variant: TRIGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TRIGEN` reader - TRIGEN
pub type TRIGEN_R = crate::BitReader<TRIGEN>;
impl TRIGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRIGEN {
        match self.bits {
            false => TRIGEN::Disabled,
            true => TRIGEN::Enabled,
        }
    }
    ///Hardware control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGEN::Disabled
    }
    ///Hardware control enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGEN::Enabled
    }
}
///Field `TRIGEN` writer - TRIGEN
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG, TRIGEN>;
impl<'a, REG> TRIGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::Disabled)
    }
    ///Hardware control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::Enabled)
    }
}
impl R {
    ///Bits 16:19 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - TRIGPOL
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TRIGEN
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("trigen", &self.trigen())
            .field("trigpol", &self.trigpol())
            .field("trigsel", &self.trigsel())
            .finish()
    }
}
impl W {
    ///Bits 16:19 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<AUTOCRrs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bit 20 - TRIGPOL
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<AUTOCRrs> {
        TRIGPOL_W::new(self, 20)
    }
    ///Bit 21 - TRIGEN
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<AUTOCRrs> {
        TRIGEN_W::new(self, 21)
    }
}
/**SPI autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SPI1:AUTOCR)*/
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
///`read()` method returns [`autocr::R`](R) reader structure
impl crate::Readable for AUTOCRrs {}
///`write(|w| ..)` method takes [`autocr::W`](W) writer structure
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AUTOCR to value 0
impl crate::Resettable for AUTOCRrs {}
