///Register `WRPCR` reader
pub type R = crate::R<WRPCRrs>;
///Register `WRPCR` writer
pub type W = crate::W<WRPCRrs>;
///Field `PLLEN` reader - PLL enable This bit enables the D-PHY PLL.
pub type PLLEN_R = crate::BitReader;
///Field `PLLEN` writer - PLL enable This bit enables the D-PHY PLL.
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NDIV` reader - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2
pub type NDIV_R = crate::FieldReader<u16>;
///Field `NDIV` writer - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2
pub type NDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `IDF` reader - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511
pub type IDF_R = crate::FieldReader<u16>;
///Field `IDF` writer - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511
pub type IDF_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `ODF` reader - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511
pub type ODF_R = crate::FieldReader<u16>;
///Field `ODF` writer - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511
pub type ODF_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bit 0 - PLL enable This bit enables the D-PHY PLL.
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:10 - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bits 11:19 - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    ///Bits 20:28 - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPCR")
            .field("pllen", &self.pllen())
            .field("ndiv", &self.ndiv())
            .field("idf", &self.idf())
            .field("odf", &self.odf())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL enable This bit enables the D-PHY PLL.
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<WRPCRrs> {
        PLLEN_W::new(self, 0)
    }
    ///Bits 2:10 - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W<WRPCRrs> {
        NDIV_W::new(self, 2)
    }
    ///Bits 11:19 - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511
    #[inline(always)]
    pub fn idf(&mut self) -> IDF_W<WRPCRrs> {
        IDF_W::new(self, 11)
    }
    ///Bits 20:28 - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511
    #[inline(always)]
    pub fn odf(&mut self) -> ODF_W<WRPCRrs> {
        ODF_W::new(self, 20)
    }
}
/**DSI Wrapper regulator and PLL control register

You can [`read`](crate::Reg::read) this register and get [`wrpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:WRPCR)*/
pub struct WRPCRrs;
impl crate::RegisterSpec for WRPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wrpcr::R`](R) reader structure
impl crate::Readable for WRPCRrs {}
///`write(|w| ..)` method takes [`wrpcr::W`](W) writer structure
impl crate::Writable for WRPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPCR to value 0
impl crate::Resettable for WRPCRrs {}
