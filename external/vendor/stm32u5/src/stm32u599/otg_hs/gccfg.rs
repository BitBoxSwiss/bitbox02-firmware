///Register `GCCFG` reader
pub type R = crate::R<GCCFGrs>;
///Register `GCCFG` writer
pub type W = crate::W<GCCFGrs>;
///Field `DCDET` reader - DCDET
pub type DCDET_R = crate::BitReader;
///Field `PDET` reader - PDET
pub type PDET_R = crate::BitReader;
///Field `SDET` reader - SDET
pub type SDET_R = crate::BitReader;
///Field `PS2DET` reader - PS2DET
pub type PS2DET_R = crate::BitReader;
///Field `PWRDWN` reader - PWRDWN
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - PWRDWN
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCDEN` reader - BCDEN
pub type BCDEN_R = crate::BitReader;
///Field `BCDEN` writer - BCDEN
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDEN` reader - DCDEN
pub type DCDEN_R = crate::BitReader;
///Field `DCDEN` writer - DCDEN
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDEN` reader - PDEN
pub type PDEN_R = crate::BitReader;
///Field `PDEN` writer - PDEN
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - SDEN
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - SDEN
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBDEN` reader - VBDEN
pub type VBDEN_R = crate::BitReader;
///Field `VBDEN` writer - VBDEN
pub type VBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DCDET
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PDET
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SDET
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PS2DET
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - PWRDWN
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - BCDEN
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DCDEN
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PDEN
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SDEN
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - VBDEN
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("dcdet", &self.dcdet())
            .field("pdet", &self.pdet())
            .field("sdet", &self.sdet())
            .field("ps2det", &self.ps2det())
            .field("pwrdwn", &self.pwrdwn())
            .field("bcden", &self.bcden())
            .field("dcden", &self.dcden())
            .field("pden", &self.pden())
            .field("sden", &self.sden())
            .field("vbden", &self.vbden())
            .finish()
    }
}
impl W {
    ///Bit 16 - PWRDWN
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    ///Bit 17 - BCDEN
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W<GCCFGrs> {
        BCDEN_W::new(self, 17)
    }
    ///Bit 18 - DCDEN
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W<GCCFGrs> {
        DCDEN_W::new(self, 18)
    }
    ///Bit 19 - PDEN
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<GCCFGrs> {
        PDEN_W::new(self, 19)
    }
    ///Bit 20 - SDEN
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<GCCFGrs> {
        SDEN_W::new(self, 20)
    }
    ///Bit 21 - VBDEN
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W<GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
}
/**OTG general core configuration register

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:GCCFG)*/
pub struct GCCFGrs;
impl crate::RegisterSpec for GCCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gccfg::R`](R) reader structure
impl crate::Readable for GCCFGrs {}
///`write(|w| ..)` method takes [`gccfg::W`](W) writer structure
impl crate::Writable for GCCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCCFG to value 0
impl crate::Resettable for GCCFGrs {}
