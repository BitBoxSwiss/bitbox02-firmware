///Register `IFCR` reader
pub type R = crate::R<IFCRrs>;
///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
/**Clear Transfer error interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF {
    ///1: Clear the TEIF flag in the ISR register
    Clear = 1,
}
impl From<CTEIF> for bool {
    #[inline(always)]
    fn from(variant: CTEIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEIF` reader - Clear Transfer error interrupt flag
pub type CTEIF_R = crate::BitReader<CTEIF>;
impl CTEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTEIF> {
        match self.bits {
            true => Some(CTEIF::Clear),
            _ => None,
        }
    }
    ///Clear the TEIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF::Clear
    }
}
///Field `CTEIF` writer - Clear Transfer error interrupt flag
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF>;
impl<'a, REG> CTEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF::Clear)
    }
}
/**Clear transfer complete interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF {
    ///1: Clear the TCIF flag in the ISR register
    Clear = 1,
}
impl From<CTCIF> for bool {
    #[inline(always)]
    fn from(variant: CTCIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIF` reader - Clear transfer complete interrupt flag
pub type CTCIF_R = crate::BitReader<CTCIF>;
impl CTCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTCIF> {
        match self.bits {
            true => Some(CTCIF::Clear),
            _ => None,
        }
    }
    ///Clear the TCIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF::Clear
    }
}
///Field `CTCIF` writer - Clear transfer complete interrupt flag
pub type CTCIF_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF>;
impl<'a, REG> CTCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF::Clear)
    }
}
/**Clear transfer watermark interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTWIF {
    ///1: Clear the TWIF flag in the ISR register
    Clear = 1,
}
impl From<CTWIF> for bool {
    #[inline(always)]
    fn from(variant: CTWIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTWIF` reader - Clear transfer watermark interrupt flag
pub type CTWIF_R = crate::BitReader<CTWIF>;
impl CTWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTWIF> {
        match self.bits {
            true => Some(CTWIF::Clear),
            _ => None,
        }
    }
    ///Clear the TWIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTWIF::Clear
    }
}
///Field `CTWIF` writer - Clear transfer watermark interrupt flag
pub type CTWIF_W<'a, REG> = crate::BitWriter<'a, REG, CTWIF>;
impl<'a, REG> CTWIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the TWIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTWIF::Clear)
    }
}
/**Clear CLUT access error interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAECIF {
    ///1: Clear the CAEIF flag in the ISR register
    Clear = 1,
}
impl From<CAECIF> for bool {
    #[inline(always)]
    fn from(variant: CAECIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CAECIF` reader - Clear CLUT access error interrupt flag
pub type CAECIF_R = crate::BitReader<CAECIF>;
impl CAECIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAECIF> {
        match self.bits {
            true => Some(CAECIF::Clear),
            _ => None,
        }
    }
    ///Clear the CAEIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAECIF::Clear
    }
}
///Field `CAECIF` writer - Clear CLUT access error interrupt flag
pub type CAECIF_W<'a, REG> = crate::BitWriter<'a, REG, CAECIF>;
impl<'a, REG> CAECIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the CAEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CAECIF::Clear)
    }
}
/**Clear CLUT transfer complete interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTCIF {
    ///1: Clear the CTCIF flag in the ISR register
    Clear = 1,
}
impl From<CCTCIF> for bool {
    #[inline(always)]
    fn from(variant: CCTCIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag
pub type CCTCIF_R = crate::BitReader<CCTCIF>;
impl CCTCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCTCIF> {
        match self.bits {
            true => Some(CCTCIF::Clear),
            _ => None,
        }
    }
    ///Clear the CTCIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCTCIF::Clear
    }
}
///Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG, CCTCIF>;
impl<'a, REG> CCTCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the CTCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCTCIF::Clear)
    }
}
/**Clear configuration error interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIF {
    ///1: Clear the CEIF flag in the ISR register
    Clear = 1,
}
impl From<CCEIF> for bool {
    #[inline(always)]
    fn from(variant: CCEIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CCEIF` reader - Clear configuration error interrupt flag
pub type CCEIF_R = crate::BitReader<CCEIF>;
impl CCEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCEIF> {
        match self.bits {
            true => Some(CCEIF::Clear),
            _ => None,
        }
    }
    ///Clear the CEIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCEIF::Clear
    }
}
///Field `CCEIF` writer - Clear configuration error interrupt flag
pub type CCEIF_W<'a, REG> = crate::BitWriter<'a, REG, CCEIF>;
impl<'a, REG> CCEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the CEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCEIF::Clear)
    }
}
impl R {
    ///Bit 0 - Clear Transfer error interrupt flag
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear transfer complete interrupt flag
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear configuration error interrupt flag
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFCR")
            .field("cceif", &self.cceif())
            .field("cctcif", &self.cctcif())
            .field("caecif", &self.caecif())
            .field("ctwif", &self.ctwif())
            .field("ctcif", &self.ctcif())
            .field("cteif", &self.cteif())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Transfer error interrupt flag
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - Clear transfer complete interrupt flag
    #[inline(always)]
    pub fn ctcif(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 1)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag
    #[inline(always)]
    pub fn ctwif(&mut self) -> CTWIF_W<IFCRrs> {
        CTWIF_W::new(self, 2)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag
    #[inline(always)]
    pub fn caecif(&mut self) -> CAECIF_W<IFCRrs> {
        CAECIF_W::new(self, 3)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W<IFCRrs> {
        CCTCIF_W::new(self, 4)
    }
    ///Bit 5 - Clear configuration error interrupt flag
    #[inline(always)]
    pub fn cceif(&mut self) -> CCEIF_W<IFCRrs> {
        CCEIF_W::new(self, 5)
    }
}
/**interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ifcr::R`](R) reader structure
impl crate::Readable for IFCRrs {}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
