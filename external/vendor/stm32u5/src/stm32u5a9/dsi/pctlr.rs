///Register `PCTLR` reader
pub type R = crate::R<PCTLRrs>;
///Register `PCTLR` writer
pub type W = crate::W<PCTLRrs>;
///Field `DEN` reader - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
pub type DEN_R = crate::BitReader;
///Field `DEN` writer - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKE` reader - Clock enable This bit enables the D-PHY clock lane module:
pub type CKE_R = crate::BitReader;
///Field `CKE` writer - Clock enable This bit enables the D-PHY clock lane module:
pub type CKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clock enable This bit enables the D-PHY clock lane module:
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCTLR")
            .field("den", &self.den())
            .field("cke", &self.cke())
            .finish()
    }
}
impl W {
    ///Bit 1 - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W<PCTLRrs> {
        DEN_W::new(self, 1)
    }
    ///Bit 2 - Clock enable This bit enables the D-PHY clock lane module:
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W<PCTLRrs> {
        CKE_W::new(self, 2)
    }
}
/**DSI Host PHY control register

You can [`read`](crate::Reg::read) this register and get [`pctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:PCTLR)*/
pub struct PCTLRrs;
impl crate::RegisterSpec for PCTLRrs {
    type Ux = u32;
}
///`read()` method returns [`pctlr::R`](R) reader structure
impl crate::Readable for PCTLRrs {}
///`write(|w| ..)` method takes [`pctlr::W`](W) writer structure
impl crate::Writable for PCTLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCTLR to value 0
impl crate::Resettable for PCTLRrs {}
