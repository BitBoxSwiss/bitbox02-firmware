///Register `BFCR` reader
pub type R = crate::R<BFCRrs>;
///Register `BFCR` writer
pub type W = crate::W<BFCRrs>;
/**blending factor 2 These bits select the blending factor F2

Value on reset: 7*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BF2 {
    ///5: BF2 = 1 - constant alpha
    Constant = 5,
    ///7: BF2 = 1 - pixel alpha * constant alpha
    Pixel = 7,
}
impl From<BF2> for u8 {
    #[inline(always)]
    fn from(variant: BF2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BF2 {
    type Ux = u8;
}
impl crate::IsEnum for BF2 {}
///Field `BF2` reader - blending factor 2 These bits select the blending factor F2
pub type BF2_R = crate::FieldReader<BF2>;
impl BF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BF2> {
        match self.bits {
            5 => Some(BF2::Constant),
            7 => Some(BF2::Pixel),
            _ => None,
        }
    }
    ///BF2 = 1 - constant alpha
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF2::Constant
    }
    ///BF2 = 1 - pixel alpha * constant alpha
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF2::Pixel
    }
}
///Field `BF2` writer - blending factor 2 These bits select the blending factor F2
pub type BF2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BF2>;
impl<'a, REG> BF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///BF2 = 1 - constant alpha
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(BF2::Constant)
    }
    ///BF2 = 1 - pixel alpha * constant alpha
    #[inline(always)]
    pub fn pixel(self) -> &'a mut crate::W<REG> {
        self.variant(BF2::Pixel)
    }
}
/**blending factor 1 These bits select the blending factor F1.

Value on reset: 6*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BF1 {
    ///4: BF1 = constant alpha
    Constant = 4,
    ///6: BF1 = pixel alpha * constant alpha
    Pixel = 6,
}
impl From<BF1> for u8 {
    #[inline(always)]
    fn from(variant: BF1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BF1 {
    type Ux = u8;
}
impl crate::IsEnum for BF1 {}
///Field `BF1` reader - blending factor 1 These bits select the blending factor F1.
pub type BF1_R = crate::FieldReader<BF1>;
impl BF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BF1> {
        match self.bits {
            4 => Some(BF1::Constant),
            6 => Some(BF1::Pixel),
            _ => None,
        }
    }
    ///BF1 = constant alpha
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF1::Constant
    }
    ///BF1 = pixel alpha * constant alpha
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF1::Pixel
    }
}
///Field `BF1` writer - blending factor 1 These bits select the blending factor F1.
pub type BF1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BF1>;
impl<'a, REG> BF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///BF1 = constant alpha
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(BF1::Constant)
    }
    ///BF1 = pixel alpha * constant alpha
    #[inline(always)]
    pub fn pixel(self) -> &'a mut crate::W<REG> {
        self.variant(BF1::Pixel)
    }
}
impl R {
    ///Bits 0:2 - blending factor 2 These bits select the blending factor F2
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - blending factor 1 These bits select the blending factor F1.
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BFCR")
            .field("bf2", &self.bf2())
            .field("bf1", &self.bf1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - blending factor 2 These bits select the blending factor F2
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W<BFCRrs> {
        BF2_W::new(self, 0)
    }
    ///Bits 8:10 - blending factor 1 These bits select the blending factor F1.
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W<BFCRrs> {
        BF1_W::new(self, 8)
    }
}
/**LTDC layer 1 blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`bfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BFCRrs;
impl crate::RegisterSpec for BFCRrs {
    type Ux = u32;
}
///`read()` method returns [`bfcr::R`](R) reader structure
impl crate::Readable for BFCRrs {}
///`write(|w| ..)` method takes [`bfcr::W`](W) writer structure
impl crate::Writable for BFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BFCR to value 0x0607
impl crate::Resettable for BFCRrs {
    const RESET_VALUE: u32 = 0x0607;
}
