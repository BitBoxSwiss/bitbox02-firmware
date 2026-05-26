///Register `FCR` writer
pub type W = crate::W<FCRrs>;
/**Clear Transfer error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEF {
    ///1: Writing 1 clears the TEF flag in the OCTOSPI_SR register
    Clear = 1,
}
impl From<CTEF> for bool {
    #[inline(always)]
    fn from(variant: CTEF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEF` writer - Clear Transfer error flag
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG, CTEF>;
impl<'a, REG> CTEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing 1 clears the TEF flag in the OCTOSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEF::Clear)
    }
}
/**Clear transfer complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCF {
    ///1: Writing 1 clears the TCF flag in the OCTOSPI_SR register
    Clear = 1,
}
impl From<CTCF> for bool {
    #[inline(always)]
    fn from(variant: CTCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCF` writer - Clear transfer complete flag
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG, CTCF>;
impl<'a, REG> CTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing 1 clears the TCF flag in the OCTOSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCF::Clear)
    }
}
/**Clear status match flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSMF {
    ///1: Writing 1 clears the SMF flag in the OCTOSPI_SR register
    Clear = 1,
}
impl From<CSMF> for bool {
    #[inline(always)]
    fn from(variant: CSMF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSMF` writer - Clear status match flag
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG, CSMF>;
impl<'a, REG> CSMF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing 1 clears the SMF flag in the OCTOSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSMF::Clear)
    }
}
/**Clear timeout flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOF {
    ///1: Writing 1 clears the TOF flag in the OCTOSPI_SR register
    Clear = 1,
}
impl From<CTOF> for bool {
    #[inline(always)]
    fn from(variant: CTOF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTOF` writer - Clear timeout flag
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG, CTOF>;
impl<'a, REG> CTOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing 1 clears the TOF flag in the OCTOSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTOF::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear Transfer error flag
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W<FCRrs> {
        CTEF_W::new(self, 0)
    }
    ///Bit 1 - Clear transfer complete flag
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W<FCRrs> {
        CTCF_W::new(self, 1)
    }
    ///Bit 3 - Clear status match flag
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W<FCRrs> {
        CSMF_W::new(self, 3)
    }
    ///Bit 4 - Clear timeout flag
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W<FCRrs> {
        CTOF_W::new(self, 4)
    }
}
/**flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#OCTOSPI1:FCR)*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
