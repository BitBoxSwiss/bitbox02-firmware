///Register `GAHBCFG` reader
pub type R = crate::R<GAHBCFGrs>;
///Register `GAHBCFG` writer
pub type W = crate::W<GAHBCFGrs>;
///Field `GINTMSK` reader - GINTMSK
pub type GINTMSK_R = crate::BitReader;
///Field `GINTMSK` writer - GINTMSK
pub type GINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFELVL` reader - TXFELVL
pub type TXFELVL_R = crate::BitReader;
///Field `TXFELVL` writer - TXFELVL
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTXFELVL` reader - PTXFELVL
pub type PTXFELVL_R = crate::BitReader;
///Field `PTXFELVL` writer - PTXFELVL
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GINTMSK
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - TXFELVL
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PTXFELVL
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("gintmsk", &self.gintmsk())
            .field("txfelvl", &self.txfelvl())
            .field("ptxfelvl", &self.ptxfelvl())
            .finish()
    }
}
impl W {
    ///Bit 0 - GINTMSK
    #[inline(always)]
    pub fn gintmsk(&mut self) -> GINTMSK_W<GAHBCFGrs> {
        GINTMSK_W::new(self, 0)
    }
    ///Bit 7 - TXFELVL
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W<GAHBCFGrs> {
        TXFELVL_W::new(self, 7)
    }
    ///Bit 8 - PTXFELVL
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<GAHBCFGrs> {
        PTXFELVL_W::new(self, 8)
    }
}
/**This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.

You can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OTG_FS:GAHBCFG)*/
pub struct GAHBCFGrs;
impl crate::RegisterSpec for GAHBCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gahbcfg::R`](R) reader structure
impl crate::Readable for GAHBCFGrs {}
///`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure
impl crate::Writable for GAHBCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAHBCFG to value 0
impl crate::Resettable for GAHBCFGrs {}
