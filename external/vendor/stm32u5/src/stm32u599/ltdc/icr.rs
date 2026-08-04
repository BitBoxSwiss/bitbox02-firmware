///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**clears the line interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLIFW {
    ///1: Clears the LIF flag in the ISR register
    Clear = 1,
}
impl From<CLIFW> for bool {
    #[inline(always)]
    fn from(variant: CLIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLIF` writer - clears the line interrupt flag
pub type CLIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CLIFW>;
impl<'a, REG> CLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the LIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLIFW::Clear)
    }
}
/**clears the FIFO underrun interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFUIFW {
    ///1: Clears the FUIF flag in the ISR register
    Clear = 1,
}
impl From<CFUIFW> for bool {
    #[inline(always)]
    fn from(variant: CFUIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CFUIF` writer - clears the FIFO underrun interrupt flag
pub type CFUIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CFUIFW>;
impl<'a, REG> CFUIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the FUIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFUIFW::Clear)
    }
}
/**clears the transfer error interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTERRIFW {
    ///1: Clears the TERRIF flag in the ISR register
    Clear = 1,
}
impl From<CTERRIFW> for bool {
    #[inline(always)]
    fn from(variant: CTERRIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTERRIF` writer - clears the transfer error interrupt flag
pub type CTERRIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CTERRIFW>;
impl<'a, REG> CTERRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the TERRIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTERRIFW::Clear)
    }
}
/**clears register reload interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRRIFW {
    ///1: Clears the RRIF flag in the ISR register
    Clear = 1,
}
impl From<CRRIFW> for bool {
    #[inline(always)]
    fn from(variant: CRRIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CRRIF` writer - clears register reload interrupt flag
pub type CRRIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CRRIFW>;
impl<'a, REG> CRRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the RRIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRRIFW::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clears the line interrupt flag
    #[inline(always)]
    pub fn clif(&mut self) -> CLIF_W<ICRrs> {
        CLIF_W::new(self, 0)
    }
    ///Bit 1 - clears the FIFO underrun interrupt flag
    #[inline(always)]
    pub fn cfuif(&mut self) -> CFUIF_W<ICRrs> {
        CFUIF_W::new(self, 1)
    }
    ///Bit 2 - clears the transfer error interrupt flag
    #[inline(always)]
    pub fn cterrif(&mut self) -> CTERRIF_W<ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    ///Bit 3 - clears register reload interrupt flag
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
