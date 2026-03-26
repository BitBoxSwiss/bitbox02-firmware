///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**layer enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEN {
    ///0: Layer disabled
    Disabled = 0,
    ///1: Layer enabled
    Enabled = 1,
}
impl From<LEN> for bool {
    #[inline(always)]
    fn from(variant: LEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LEN` reader - layer enable This bit is set and cleared by software.
pub type LEN_R = crate::BitReader<LEN>;
impl LEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LEN {
        match self.bits {
            false => LEN::Disabled,
            true => LEN::Enabled,
        }
    }
    ///Layer disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LEN::Disabled
    }
    ///Layer enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LEN::Enabled
    }
}
///Field `LEN` writer - layer enable This bit is set and cleared by software.
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG, LEN>;
impl<'a, REG> LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Layer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LEN::Disabled)
    }
    ///Layer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LEN::Enabled)
    }
}
/**color keying enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLKEN {
    ///0: Color keying disabled
    Disabled = 0,
    ///1: Color keying enabled
    Enabled = 1,
}
impl From<COLKEN> for bool {
    #[inline(always)]
    fn from(variant: COLKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `COLKEN` reader - color keying enable This bit is set and cleared by software.
pub type COLKEN_R = crate::BitReader<COLKEN>;
impl COLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COLKEN {
        match self.bits {
            false => COLKEN::Disabled,
            true => COLKEN::Enabled,
        }
    }
    ///Color keying disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLKEN::Disabled
    }
    ///Color keying enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLKEN::Enabled
    }
}
///Field `COLKEN` writer - color keying enable This bit is set and cleared by software.
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG, COLKEN>;
impl<'a, REG> COLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Color keying disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COLKEN::Disabled)
    }
    ///Color keying enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COLKEN::Enabled)
    }
}
/**color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTEN {
    ///0: Color look-up table disabled
    Disabled = 0,
    ///1: Color look-up table enabled
    Enabled = 1,
}
impl From<CLUTEN> for bool {
    #[inline(always)]
    fn from(variant: CLUTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CLUTEN` reader - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
pub type CLUTEN_R = crate::BitReader<CLUTEN>;
impl CLUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLUTEN {
        match self.bits {
            false => CLUTEN::Disabled,
            true => CLUTEN::Enabled,
        }
    }
    ///Color look-up table disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLUTEN::Disabled
    }
    ///Color look-up table enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLUTEN::Enabled
    }
}
///Field `CLUTEN` writer - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG, CLUTEN>;
impl<'a, REG> CLUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Color look-up table disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTEN::Disabled)
    }
    ///Color look-up table enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTEN::Enabled)
    }
}
impl R {
    ///Bit 0 - layer enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - color keying enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("len", &self.len())
            .field("colken", &self.colken())
            .field("cluten", &self.cluten())
            .finish()
    }
}
impl W {
    ///Bit 0 - layer enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<CRrs> {
        LEN_W::new(self, 0)
    }
    ///Bit 1 - color keying enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W<CRrs> {
        COLKEN_W::new(self, 1)
    }
    ///Bit 4 - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W<CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
