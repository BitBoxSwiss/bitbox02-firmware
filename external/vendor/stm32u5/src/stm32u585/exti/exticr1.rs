///Register `EXTICR1` reader
pub type R = crate::R<EXTICR1rs>;
///Register `EXTICR1` writer
pub type W = crate::W<EXTICR1rs>;
/**EXTIm GPIO port selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0 {
    ///0: Select PAx as the source input for the EXTIx external interrupt
    Pa = 0,
    ///1: Select PBx as the source input for the EXTIx external interrupt
    Pb = 1,
    ///2: Select PCx as the source input for the EXTIx external interrupt
    Pc = 2,
    ///3: Select PDx as the source input for the EXTIx external interrupt
    Pd = 3,
    ///4: Select PEx as the source input for the EXTIx external interrupt
    Pe = 4,
    ///5: Select PFx as the source input for the EXTIx external interrupt
    Pf = 5,
    ///6: Select PGx as the source input for the EXTIx external interrupt
    Pg = 6,
    ///7: Select PHx as the source input for the EXTIx external interrupt
    Ph = 7,
    ///8: Select PIx as the source input for the EXTIx external interrupt
    Pi = 8,
    ///9: Select PJx as the source input for the EXTIx external interrupt
    Pj = 9,
}
impl From<EXTI0> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI0 {}
///Field `EXTI0` reader - EXTIm GPIO port selection
pub type EXTI0_R = crate::FieldReader<EXTI0>;
impl EXTI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0> {
        match self.bits {
            0 => Some(EXTI0::Pa),
            1 => Some(EXTI0::Pb),
            2 => Some(EXTI0::Pc),
            3 => Some(EXTI0::Pd),
            4 => Some(EXTI0::Pe),
            5 => Some(EXTI0::Pf),
            6 => Some(EXTI0::Pg),
            7 => Some(EXTI0::Ph),
            8 => Some(EXTI0::Pi),
            9 => Some(EXTI0::Pj),
            _ => None,
        }
    }
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI0::Pa
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI0::Pb
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI0::Pc
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI0::Pd
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI0::Pe
    }
    ///Select PFx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI0::Pf
    }
    ///Select PGx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == EXTI0::Pg
    }
    ///Select PHx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_ph(&self) -> bool {
        *self == EXTI0::Ph
    }
    ///Select PIx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pi(&self) -> bool {
        *self == EXTI0::Pi
    }
    ///Select PJx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pj(&self) -> bool {
        *self == EXTI0::Pj
    }
}
///Field `EXTI0` writer - EXTIm GPIO port selection
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI0>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pa)
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pb)
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pc)
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pd)
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pe)
    }
    ///Select PFx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pf)
    }
    ///Select PGx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pg(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pg)
    }
    ///Select PHx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn ph(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Ph)
    }
    ///Select PIx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pi(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pi)
    }
    ///Select PJx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pj(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pj)
    }
}
///Field `EXTI1` reader - EXTIm+1 GPIO port selection
pub use EXTI0_R as EXTI1_R;
///Field `EXTI2` reader - EXTIm+2 GPIO port selection
pub use EXTI0_R as EXTI2_R;
///Field `EXTI3` reader - EXTIm+3 GPIO port selection
pub use EXTI0_R as EXTI3_R;
///Field `EXTI1` writer - EXTIm+1 GPIO port selection
pub use EXTI0_W as EXTI1_W;
///Field `EXTI2` writer - EXTIm+2 GPIO port selection
pub use EXTI0_W as EXTI2_W;
///Field `EXTI3` writer - EXTIm+3 GPIO port selection
pub use EXTI0_W as EXTI3_W;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR1")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1rs> {
        EXTI1_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1rs> {
        EXTI2_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1rs> {
        EXTI3_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#EXTI:EXTICR1)*/
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exticr1::R`](R) reader structure
impl crate::Readable for EXTICR1rs {}
///`write(|w| ..)` method takes [`exticr1::W`](W) writer structure
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1rs {}
