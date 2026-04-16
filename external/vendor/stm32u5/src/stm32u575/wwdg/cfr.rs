///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `W` reader - 7-bit window value
pub type W_R = crate::FieldReader;
///Field `W` writer - 7-bit window value
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Early wakeup interrupt

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIW {
    ///1: interrupt occurs whenever the counter reaches the value 0x40
    Enable = 1,
}
impl From<EWIW> for bool {
    #[inline(always)]
    fn from(variant: EWIW) -> Self {
        variant as u8 != 0
    }
}
///Field `EWI` reader - Early wakeup interrupt
pub type EWI_R = crate::BitReader<EWIW>;
impl EWI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EWIW> {
        match self.bits {
            true => Some(EWIW::Enable),
            _ => None,
        }
    }
    ///interrupt occurs whenever the counter reaches the value 0x40
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIW::Enable
    }
}
///Field `EWI` writer - Early wakeup interrupt
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG, EWIW>;
impl<'a, REG> EWI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///interrupt occurs whenever the counter reaches the value 0x40
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EWIW::Enable)
    }
}
/**Timer base

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB {
    ///0: Counter clock (PCLK1 div 4096) div 1
    Div1 = 0,
    ///1: Counter clock (PCLK1 div 4096) div 2
    Div2 = 1,
    ///2: Counter clock (PCLK1 div 4096) div 4
    Div4 = 2,
    ///3: Counter clock (PCLK1 div 4096) div 8
    Div8 = 3,
    ///4: Counter clock (PCLK1 div 4096) div 16
    Div16 = 4,
    ///5: Counter clock (PCLK1 div 4096) div 32
    Div32 = 5,
    ///6: Counter clock (PCLK1 div 4096) div 64
    Div64 = 6,
    ///7: Counter clock (PCLK1 div 4096) div 128
    Div128 = 7,
}
impl From<WDGTB> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDGTB {
    type Ux = u8;
}
impl crate::IsEnum for WDGTB {}
///Field `WDGTB` reader - Timer base
pub type WDGTB_R = crate::FieldReader<WDGTB>;
impl WDGTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDGTB {
        match self.bits {
            0 => WDGTB::Div1,
            1 => WDGTB::Div2,
            2 => WDGTB::Div4,
            3 => WDGTB::Div8,
            4 => WDGTB::Div16,
            5 => WDGTB::Div32,
            6 => WDGTB::Div64,
            7 => WDGTB::Div128,
            _ => unreachable!(),
        }
    }
    ///Counter clock (PCLK1 div 4096) div 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == WDGTB::Div1
    }
    ///Counter clock (PCLK1 div 4096) div 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WDGTB::Div2
    }
    ///Counter clock (PCLK1 div 4096) div 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WDGTB::Div4
    }
    ///Counter clock (PCLK1 div 4096) div 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WDGTB::Div8
    }
    ///Counter clock (PCLK1 div 4096) div 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WDGTB::Div16
    }
    ///Counter clock (PCLK1 div 4096) div 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == WDGTB::Div32
    }
    ///Counter clock (PCLK1 div 4096) div 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == WDGTB::Div64
    }
    ///Counter clock (PCLK1 div 4096) div 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == WDGTB::Div128
    }
}
///Field `WDGTB` writer - Timer base
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDGTB, crate::Safe>;
impl<'a, REG> WDGTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Counter clock (PCLK1 div 4096) div 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div1)
    }
    ///Counter clock (PCLK1 div 4096) div 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div2)
    }
    ///Counter clock (PCLK1 div 4096) div 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div4)
    }
    ///Counter clock (PCLK1 div 4096) div 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div8)
    }
    ///Counter clock (PCLK1 div 4096) div 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div16)
    }
    ///Counter clock (PCLK1 div 4096) div 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div32)
    }
    ///Counter clock (PCLK1 div 4096) div 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div64)
    }
    ///Counter clock (PCLK1 div 4096) div 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div128)
    }
}
impl R {
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - Early wakeup interrupt
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 11:13 - Timer base
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFR")
            .field("wdgtb", &self.wdgtb())
            .field("ewi", &self.ewi())
            .field("w", &self.w())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&mut self) -> W_W<CFRrs> {
        W_W::new(self, 0)
    }
    ///Bit 9 - Early wakeup interrupt
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<CFRrs> {
        EWI_W::new(self, 9)
    }
    ///Bits 11:13 - Timer base
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W<CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#WWDG:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u16;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u16 = 0x7f;
}
