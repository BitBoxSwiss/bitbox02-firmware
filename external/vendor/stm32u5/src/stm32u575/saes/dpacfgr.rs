///Register `DPACFGR` reader
pub type R = crate::R<DPACFGRrs>;
///Register `DPACFGR` writer
pub type W = crate::W<DPACFGRrs>;
///Field `REDCFG` reader - REDCFG
pub type REDCFG_R = crate::BitReader;
///Field `REDCFG` writer - REDCFG
pub type REDCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESEED` reader - RESEED
pub type RESEED_R = crate::BitReader;
///Field `RESEED` writer - RESEED
pub type RESEED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIMCFG` reader - TRIMCFG
pub type TRIMCFG_R = crate::FieldReader;
///Field `TRIMCFG` writer - TRIMCFG
pub type TRIMCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFIGLOCK` reader - CONFIGLOCK
pub type CONFIGLOCK_R = crate::BitReader;
impl R {
    ///Bit 1 - REDCFG
    #[inline(always)]
    pub fn redcfg(&self) -> REDCFG_R {
        REDCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RESEED
    #[inline(always)]
    pub fn reseed(&self) -> RESEED_R {
        RESEED_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - TRIMCFG
    #[inline(always)]
    pub fn trimcfg(&self) -> TRIMCFG_R {
        TRIMCFG_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 31 - CONFIGLOCK
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPACFGR")
            .field("configlock", &self.configlock())
            .field("trimcfg", &self.trimcfg())
            .field("reseed", &self.reseed())
            .field("redcfg", &self.redcfg())
            .finish()
    }
}
impl W {
    ///Bit 1 - REDCFG
    #[inline(always)]
    pub fn redcfg(&mut self) -> REDCFG_W<DPACFGRrs> {
        REDCFG_W::new(self, 1)
    }
    ///Bit 2 - RESEED
    #[inline(always)]
    pub fn reseed(&mut self) -> RESEED_W<DPACFGRrs> {
        RESEED_W::new(self, 2)
    }
    ///Bits 3:4 - TRIMCFG
    #[inline(always)]
    pub fn trimcfg(&mut self) -> TRIMCFG_W<DPACFGRrs> {
        TRIMCFG_W::new(self, 3)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`dpacfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpacfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SAES:DPACFGR)*/
pub struct DPACFGRrs;
impl crate::RegisterSpec for DPACFGRrs {
    type Ux = u32;
}
///`read()` method returns [`dpacfgr::R`](R) reader structure
impl crate::Readable for DPACFGRrs {}
///`write(|w| ..)` method takes [`dpacfgr::W`](W) writer structure
impl crate::Writable for DPACFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPACFGR to value 0x08
impl crate::Resettable for DPACFGRrs {
    const RESET_VALUE: u32 = 0x08;
}
