///Register `CLRFR` writer
pub type W = crate::W<CLRFRrs>;
///Field `PROCENDFC` writer - Clear PKA End of Operation flag
pub type PROCENDFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMERRFC` writer - RAMERRFC
pub type RAMERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRERRFC` writer - ADDRERRFC
pub type ADDRERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERRFC` writer - OPERRFC
pub type OPERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CLRFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 17 - Clear PKA End of Operation flag
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W<CLRFRrs> {
        PROCENDFC_W::new(self, 17)
    }
    ///Bit 19 - RAMERRFC
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<CLRFRrs> {
        RAMERRFC_W::new(self, 19)
    }
    ///Bit 20 - ADDRERRFC
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<CLRFRrs> {
        ADDRERRFC_W::new(self, 20)
    }
    ///Bit 21 - OPERRFC
    #[inline(always)]
    pub fn operrfc(&mut self) -> OPERRFC_W<CLRFRrs> {
        OPERRFC_W::new(self, 21)
    }
}
/**PKA clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PKA:CLRFR)*/
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`clrfr::W`](W) writer structure
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLRFR to value 0
impl crate::Resettable for CLRFRrs {}
