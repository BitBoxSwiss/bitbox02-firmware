///Register `DCR2` reader
pub type R = crate::R<DCR2rs>;
///Register `DCR2` writer
pub type W = crate::W<DCR2rs>;
///Field `PRESCALER` reader - Clock prescaler
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - Clock prescaler
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Wrap size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRAPSIZE {
    ///0: Wrapped reads are not supported by the memory
    NoWrappingSupport = 0,
    ///2: External memory supports wrap size of 16 bytes
    WrappingSize16 = 2,
    ///3: External memory supports wrap size of 32 bytes
    WrappingSize32 = 3,
    ///4: External memory supports wrap size of 64 bytes
    WrappingSize64 = 4,
    ///5: External memory supports wrap size of 128 bytes
    WrappingSize128 = 5,
}
impl From<WRAPSIZE> for u8 {
    #[inline(always)]
    fn from(variant: WRAPSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRAPSIZE {
    type Ux = u8;
}
impl crate::IsEnum for WRAPSIZE {}
///Field `WRAPSIZE` reader - Wrap size
pub type WRAPSIZE_R = crate::FieldReader<WRAPSIZE>;
impl WRAPSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WRAPSIZE> {
        match self.bits {
            0 => Some(WRAPSIZE::NoWrappingSupport),
            2 => Some(WRAPSIZE::WrappingSize16),
            3 => Some(WRAPSIZE::WrappingSize32),
            4 => Some(WRAPSIZE::WrappingSize64),
            5 => Some(WRAPSIZE::WrappingSize128),
            _ => None,
        }
    }
    ///Wrapped reads are not supported by the memory
    #[inline(always)]
    pub fn is_no_wrapping_support(&self) -> bool {
        *self == WRAPSIZE::NoWrappingSupport
    }
    ///External memory supports wrap size of 16 bytes
    #[inline(always)]
    pub fn is_wrapping_size16(&self) -> bool {
        *self == WRAPSIZE::WrappingSize16
    }
    ///External memory supports wrap size of 32 bytes
    #[inline(always)]
    pub fn is_wrapping_size32(&self) -> bool {
        *self == WRAPSIZE::WrappingSize32
    }
    ///External memory supports wrap size of 64 bytes
    #[inline(always)]
    pub fn is_wrapping_size64(&self) -> bool {
        *self == WRAPSIZE::WrappingSize64
    }
    ///External memory supports wrap size of 128 bytes
    #[inline(always)]
    pub fn is_wrapping_size128(&self) -> bool {
        *self == WRAPSIZE::WrappingSize128
    }
}
///Field `WRAPSIZE` writer - Wrap size
pub type WRAPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WRAPSIZE>;
impl<'a, REG> WRAPSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Wrapped reads are not supported by the memory
    #[inline(always)]
    pub fn no_wrapping_support(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::NoWrappingSupport)
    }
    ///External memory supports wrap size of 16 bytes
    #[inline(always)]
    pub fn wrapping_size16(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize16)
    }
    ///External memory supports wrap size of 32 bytes
    #[inline(always)]
    pub fn wrapping_size32(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize32)
    }
    ///External memory supports wrap size of 64 bytes
    #[inline(always)]
    pub fn wrapping_size64(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize64)
    }
    ///External memory supports wrap size of 128 bytes
    #[inline(always)]
    pub fn wrapping_size128(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize128)
    }
}
impl R {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR2")
            .field("prescaler", &self.prescaler())
            .field("wrapsize", &self.wrapsize())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<DCR2rs> {
        PRESCALER_W::new(self, 0)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<DCR2rs> {
        WRAPSIZE_W::new(self, 16)
    }
}
/**device configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OCTOSPI1:DCR2)*/
pub struct DCR2rs;
impl crate::RegisterSpec for DCR2rs {
    type Ux = u32;
}
///`read()` method returns [`dcr2::R`](R) reader structure
impl crate::Readable for DCR2rs {}
///`write(|w| ..)` method takes [`dcr2::W`](W) writer structure
impl crate::Writable for DCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR2 to value 0
impl crate::Resettable for DCR2rs {}
