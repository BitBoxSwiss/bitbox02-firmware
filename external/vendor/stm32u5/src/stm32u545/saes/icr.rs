///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CCF` writer - Computation complete flag clear
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWEIF` writer - Read or write error interrupt flag clear
pub type RWEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIF` writer - Key error interrupt flag clear
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEIF` writer - RNGEIF
pub type RNGEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - RNGEIF
    #[inline(always)]
    pub fn rngeif(&mut self) -> RNGEIF_W<ICRrs> {
        RNGEIF_W::new(self, 3)
    }
}
/**interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SAES:ICR)*/
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
