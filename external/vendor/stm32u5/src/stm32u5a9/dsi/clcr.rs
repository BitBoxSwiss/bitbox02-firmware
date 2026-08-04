///Register `CLCR` reader
pub type R = crate::R<CLCRrs>;
///Register `CLCR` writer
pub type W = crate::W<CLCRrs>;
///Field `DPCC` reader - D-PHY clock control This bit controls the D-PHY clock state:
pub type DPCC_R = crate::BitReader;
///Field `DPCC` writer - D-PHY clock control This bit controls the D-PHY clock state:
pub type DPCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACR` reader - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
pub type ACR_R = crate::BitReader;
///Field `ACR` writer - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
pub type ACR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLCR")
            .field("dpcc", &self.dpcc())
            .field("acr", &self.acr())
            .finish()
    }
}
impl W {
    ///Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:
    #[inline(always)]
    pub fn dpcc(&mut self) -> DPCC_W<CLCRrs> {
        DPCC_W::new(self, 0)
    }
    ///Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
    #[inline(always)]
    pub fn acr(&mut self) -> ACR_W<CLCRrs> {
        ACR_W::new(self, 1)
    }
}
/**DSI Host clock lane configuration register

You can [`read`](crate::Reg::read) this register and get [`clcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:CLCR)*/
pub struct CLCRrs;
impl crate::RegisterSpec for CLCRrs {
    type Ux = u32;
}
///`read()` method returns [`clcr::R`](R) reader structure
impl crate::Readable for CLCRrs {}
///`write(|w| ..)` method takes [`clcr::W`](W) writer structure
impl crate::Writable for CLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLCR to value 0
impl crate::Resettable for CLCRrs {}
