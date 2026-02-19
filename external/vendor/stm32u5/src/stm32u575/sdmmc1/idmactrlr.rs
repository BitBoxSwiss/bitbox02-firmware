///Register `IDMACTRLR` reader
pub type R = crate::R<IDMACTRLRrs>;
///Register `IDMACTRLR` writer
pub type W = crate::W<IDMACTRLRrs>;
///Field `IDMAEN` reader - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMAEN_R = crate::BitReader;
///Field `IDMAEN` writer - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDMABMODE` reader - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABMODE_R = crate::BitReader;
///Field `IDMABMODE` writer - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMACTRLR")
            .field("idmaen", &self.idmaen())
            .field("idmabmode", &self.idmabmode())
            .finish()
    }
}
impl W {
    ///Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmaen(&mut self) -> IDMAEN_W<IDMACTRLRrs> {
        IDMAEN_W::new(self, 0)
    }
    ///Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabmode(&mut self) -> IDMABMODE_W<IDMACTRLRrs> {
        IDMABMODE_W::new(self, 1)
    }
}
/**DMA control register

You can [`read`](crate::Reg::read) this register and get [`idmactrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmactrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SDMMC1:IDMACTRLR)*/
pub struct IDMACTRLRrs;
impl crate::RegisterSpec for IDMACTRLRrs {
    type Ux = u32;
}
///`read()` method returns [`idmactrlr::R`](R) reader structure
impl crate::Readable for IDMACTRLRrs {}
///`write(|w| ..)` method takes [`idmactrlr::W`](W) writer structure
impl crate::Writable for IDMACTRLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMACTRLR to value 0
impl crate::Resettable for IDMACTRLRrs {}
