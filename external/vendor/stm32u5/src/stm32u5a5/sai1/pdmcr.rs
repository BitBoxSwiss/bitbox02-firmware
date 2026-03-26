///Register `PDMCR` reader
pub type R = crate::R<PDMCRrs>;
///Register `PDMCR` writer
pub type W = crate::W<PDMCRrs>;
///Field `PDMEN` reader - PDM enable
pub type PDMEN_R = crate::BitReader;
///Field `PDMEN` writer - PDM enable
pub type PDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MICNBR` reader - MICNBR
pub type MICNBR_R = crate::FieldReader;
///Field `MICNBR` writer - MICNBR
pub type MICNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKEN1` reader - Clock enable of bitstream clock number 1
pub type CKEN1_R = crate::BitReader;
///Field `CKEN1` writer - Clock enable of bitstream clock number 1
pub type CKEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN2` reader - CKEN2
pub type CKEN2_R = crate::BitReader;
///Field `CKEN2` writer - CKEN2
pub type CKEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN3` reader - CKEN3
pub type CKEN3_R = crate::BitReader;
///Field `CKEN3` writer - CKEN3
pub type CKEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN4` reader - CKEN4
pub type CKEN4_R = crate::BitReader;
///Field `CKEN4` writer - CKEN4
pub type CKEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PDM enable
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - MICNBR
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - Clock enable of bitstream clock number 1
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CKEN2
    #[inline(always)]
    pub fn cken2(&self) -> CKEN2_R {
        CKEN2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CKEN3
    #[inline(always)]
    pub fn cken3(&self) -> CKEN3_R {
        CKEN3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CKEN4
    #[inline(always)]
    pub fn cken4(&self) -> CKEN4_R {
        CKEN4_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMCR")
            .field("pdmen", &self.pdmen())
            .field("micnbr", &self.micnbr())
            .field("cken1", &self.cken1())
            .field("cken2", &self.cken2())
            .field("cken3", &self.cken3())
            .field("cken4", &self.cken4())
            .finish()
    }
}
impl W {
    ///Bit 0 - PDM enable
    #[inline(always)]
    pub fn pdmen(&mut self) -> PDMEN_W<PDMCRrs> {
        PDMEN_W::new(self, 0)
    }
    ///Bits 4:5 - MICNBR
    #[inline(always)]
    pub fn micnbr(&mut self) -> MICNBR_W<PDMCRrs> {
        MICNBR_W::new(self, 4)
    }
    ///Bit 8 - Clock enable of bitstream clock number 1
    #[inline(always)]
    pub fn cken1(&mut self) -> CKEN1_W<PDMCRrs> {
        CKEN1_W::new(self, 8)
    }
    ///Bit 9 - CKEN2
    #[inline(always)]
    pub fn cken2(&mut self) -> CKEN2_W<PDMCRrs> {
        CKEN2_W::new(self, 9)
    }
    ///Bit 10 - CKEN3
    #[inline(always)]
    pub fn cken3(&mut self) -> CKEN3_W<PDMCRrs> {
        CKEN3_W::new(self, 10)
    }
    ///Bit 11 - CKEN4
    #[inline(always)]
    pub fn cken4(&mut self) -> CKEN4_W<PDMCRrs> {
        CKEN4_W::new(self, 11)
    }
}
/**PDM control register

You can [`read`](crate::Reg::read) this register and get [`pdmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SAI1:PDMCR)*/
pub struct PDMCRrs;
impl crate::RegisterSpec for PDMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pdmcr::R`](R) reader structure
impl crate::Readable for PDMCRrs {}
///`write(|w| ..)` method takes [`pdmcr::W`](W) writer structure
impl crate::Writable for PDMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDMCR to value 0
impl crate::Resettable for PDMCRrs {}
