///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
///Field `DBA` reader - DMA base address
pub type DBA_R = crate::FieldReader;
///Field `DBA` writer - DMA base address
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `DBL` reader - DMA burst length
pub type DBL_R = crate::FieldReader;
///Field `DBL` writer - DMA burst length
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DBSS` reader - DMA burst source selection
pub type DBSS_R = crate::FieldReader;
///Field `DBSS` writer - DMA burst source selection
pub type DBSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:4 - DMA base address
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DMA burst length
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:19 - DMA burst source selection
    #[inline(always)]
    pub fn dbss(&self) -> DBSS_R {
        DBSS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("dbss", &self.dbss())
            .field("dbl", &self.dbl())
            .field("dba", &self.dba())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMA base address
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DMA burst length
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<DCRrs> {
        DBL_W::new(self, 8)
    }
    ///Bits 16:19 - DMA burst source selection
    #[inline(always)]
    pub fn dbss(&mut self) -> DBSS_W<DCRrs> {
        DBSS_W::new(self, 16)
    }
}
/**DMA control register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TIM2:DCR)*/
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DCRrs {}
