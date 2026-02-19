///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Computation complete flag clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFW {
    ///1: Setting this bit clears corresponding interrupt status bit
    Clear = 1,
}
impl From<CCFW> for bool {
    #[inline(always)]
    fn from(variant: CCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCF` writer - Computation complete flag clear
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG, CCFW>;
impl<'a, REG> CCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears corresponding interrupt status bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCFW::Clear)
    }
}
///Field `RWEIF` writer - Read or write error interrupt flag clear
pub use CCF_W as RWEIF_W;
///Field `KEIF` writer - Key error interrupt flag clear
pub use CCF_W as KEIF_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Computation complete flag clear
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W<ICRrs> {
        CCF_W::new(self, 0)
    }
    ///Bit 1 - Read or write error interrupt flag clear
    #[inline(always)]
    pub fn rweif(&mut self) -> RWEIF_W<ICRrs> {
        RWEIF_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt flag clear
    #[inline(always)]
    pub fn keif(&mut self) -> KEIF_W<ICRrs> {
        KEIF_W::new(self, 2)
    }
}
/**interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#AES:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
