///Register `OTGHSPHYCR` reader
pub type R = crate::R<OTGHSPHYCRrs>;
///Register `OTGHSPHYCR` writer
pub type W = crate::W<OTGHSPHYCRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDCTRL` reader - PDCTRL
pub type PDCTRL_R = crate::BitReader;
///Field `PDCTRL` writer - PDCTRL
pub type PDCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKSEL` reader - CLKSEL
pub type CLKSEL_R = crate::FieldReader;
///Field `CLKSEL` writer - CLKSEL
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PDCTRL
    #[inline(always)]
    pub fn pdctrl(&self) -> PDCTRL_R {
        PDCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - CLKSEL
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTGHSPHYCR")
            .field("en", &self.en())
            .field("pdctrl", &self.pdctrl())
            .field("clksel", &self.clksel())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<OTGHSPHYCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - PDCTRL
    #[inline(always)]
    pub fn pdctrl(&mut self) -> PDCTRL_W<OTGHSPHYCRrs> {
        PDCTRL_W::new(self, 1)
    }
    ///Bits 2:5 - CLKSEL
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W<OTGHSPHYCRrs> {
        CLKSEL_W::new(self, 2)
    }
}
/**SYSCFG USB OTG_HS PHY register

You can [`read`](crate::Reg::read) this register and get [`otghsphycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otghsphycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SYSCFG:OTGHSPHYCR)*/
pub struct OTGHSPHYCRrs;
impl crate::RegisterSpec for OTGHSPHYCRrs {
    type Ux = u32;
}
///`read()` method returns [`otghsphycr::R`](R) reader structure
impl crate::Readable for OTGHSPHYCRrs {}
///`write(|w| ..)` method takes [`otghsphycr::W`](W) writer structure
impl crate::Writable for OTGHSPHYCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTGHSPHYCR to value 0
impl crate::Resettable for OTGHSPHYCRrs {}
