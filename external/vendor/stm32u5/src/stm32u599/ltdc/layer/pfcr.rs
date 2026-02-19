///Register `PFCR` reader
pub type R = crate::R<PFCRrs>;
///Register `PFCR` writer
pub type W = crate::W<PFCRrs>;
/**pixel format These bits configure the pixel format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF {
    ///0: ARGB8888
    Argb8888 = 0,
    ///1: RGB888
    Rgb888 = 1,
    ///2: RGB565
    Rgb565 = 2,
    ///3: ARGB1555
    Argb1555 = 3,
    ///4: ARGB4444
    Argb4444 = 4,
    ///5: L8 (8-bit luminance)
    L8 = 5,
    ///6: AL44 (4-bit alpha, 4-bit luminance)
    Al44 = 6,
    ///7: AL88 (8-bit alpha, 8-bit luminance)
    Al88 = 7,
}
impl From<PF> for u8 {
    #[inline(always)]
    fn from(variant: PF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PF {
    type Ux = u8;
}
impl crate::IsEnum for PF {}
///Field `PF` reader - pixel format These bits configure the pixel format
pub type PF_R = crate::FieldReader<PF>;
impl PF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PF {
        match self.bits {
            0 => PF::Argb8888,
            1 => PF::Rgb888,
            2 => PF::Rgb565,
            3 => PF::Argb1555,
            4 => PF::Argb4444,
            5 => PF::L8,
            6 => PF::Al44,
            7 => PF::Al88,
            _ => unreachable!(),
        }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PF::Argb8888
    }
    ///RGB888
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PF::Rgb888
    }
    ///RGB565
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PF::Rgb565
    }
    ///ARGB1555
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == PF::Argb1555
    }
    ///ARGB4444
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == PF::Argb4444
    }
    ///L8 (8-bit luminance)
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == PF::L8
    }
    ///AL44 (4-bit alpha, 4-bit luminance)
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == PF::Al44
    }
    ///AL88 (8-bit alpha, 8-bit luminance)
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == PF::Al88
    }
}
///Field `PF` writer - pixel format These bits configure the pixel format
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PF, crate::Safe>;
impl<'a, REG> PF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Argb8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Rgb888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Rgb565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Argb1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Argb4444)
    }
    ///L8 (8-bit luminance)
    #[inline(always)]
    pub fn l8(self) -> &'a mut crate::W<REG> {
        self.variant(PF::L8)
    }
    ///AL44 (4-bit alpha, 4-bit luminance)
    #[inline(always)]
    pub fn al44(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Al44)
    }
    ///AL88 (8-bit alpha, 8-bit luminance)
    #[inline(always)]
    pub fn al88(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Al88)
    }
}
impl R {
    ///Bits 0:2 - pixel format These bits configure the pixel format
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFCR").field("pf", &self.pf()).finish()
    }
}
impl W {
    ///Bits 0:2 - pixel format These bits configure the pixel format
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W<PFCRrs> {
        PF_W::new(self, 0)
    }
}
/**LTDC layer 1 pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`pfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PFCRrs;
impl crate::RegisterSpec for PFCRrs {
    type Ux = u32;
}
///`read()` method returns [`pfcr::R`](R) reader structure
impl crate::Readable for PFCRrs {}
///`write(|w| ..)` method takes [`pfcr::W`](W) writer structure
impl crate::Writable for PFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PFCR to value 0
impl crate::Resettable for PFCRrs {}
