///Register `GOTGINT` reader
pub type R = crate::R<GOTGINTrs>;
///Register `GOTGINT` writer
pub type W = crate::W<GOTGINTrs>;
///Field `SEDET` reader - SEDET
pub type SEDET_R = crate::BitReader;
///Field `SEDET` writer - SEDET
pub type SEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRSSCHG` reader - SRSSCHG
pub type SRSSCHG_R = crate::BitReader;
///Field `SRSSCHG` writer - SRSSCHG
pub type SRSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNSSCHG` reader - HNSSCHG
pub type HNSSCHG_R = crate::BitReader;
///Field `HNSSCHG` writer - HNSSCHG
pub type HNSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNGDET` reader - HNGDET
pub type HNGDET_R = crate::BitReader;
///Field `HNGDET` writer - HNGDET
pub type HNGDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADTOCHG` reader - ADTOCHG
pub type ADTOCHG_R = crate::BitReader;
///Field `ADTOCHG` writer - ADTOCHG
pub type ADTOCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBCDNE` reader - DBCDNE
pub type DBCDNE_R = crate::BitReader;
///Field `DBCDNE` writer - DBCDNE
pub type DBCDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - SEDET
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - SRSSCHG
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNSSCHG
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - HNGDET
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ADTOCHG
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DBCDNE
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sedet", &self.sedet())
            .field("srsschg", &self.srsschg())
            .field("hnsschg", &self.hnsschg())
            .field("hngdet", &self.hngdet())
            .field("adtochg", &self.adtochg())
            .field("dbcdne", &self.dbcdne())
            .finish()
    }
}
impl W {
    ///Bit 2 - SEDET
    #[inline(always)]
    pub fn sedet(&mut self) -> SEDET_W<GOTGINTrs> {
        SEDET_W::new(self, 2)
    }
    ///Bit 8 - SRSSCHG
    #[inline(always)]
    pub fn srsschg(&mut self) -> SRSSCHG_W<GOTGINTrs> {
        SRSSCHG_W::new(self, 8)
    }
    ///Bit 9 - HNSSCHG
    #[inline(always)]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<GOTGINTrs> {
        HNSSCHG_W::new(self, 9)
    }
    ///Bit 17 - HNGDET
    #[inline(always)]
    pub fn hngdet(&mut self) -> HNGDET_W<GOTGINTrs> {
        HNGDET_W::new(self, 17)
    }
    ///Bit 18 - ADTOCHG
    #[inline(always)]
    pub fn adtochg(&mut self) -> ADTOCHG_W<GOTGINTrs> {
        ADTOCHG_W::new(self, 18)
    }
    ///Bit 19 - DBCDNE
    #[inline(always)]
    pub fn dbcdne(&mut self) -> DBCDNE_W<GOTGINTrs> {
        DBCDNE_W::new(self, 19)
    }
}
/**The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.

You can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:GOTGINT)*/
pub struct GOTGINTrs;
impl crate::RegisterSpec for GOTGINTrs {
    type Ux = u32;
}
///`read()` method returns [`gotgint::R`](R) reader structure
impl crate::Readable for GOTGINTrs {}
///`write(|w| ..)` method takes [`gotgint::W`](W) writer structure
impl crate::Writable for GOTGINTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GOTGINT to value 0
impl crate::Resettable for GOTGINTrs {}
