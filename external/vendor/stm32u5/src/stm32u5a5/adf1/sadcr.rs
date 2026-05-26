///Register `SADCR` reader
pub type R = crate::R<SADCRrs>;
///Register `SADCR` writer
pub type W = crate::W<SADCRrs>;
///Field `SADEN` reader - Sound activity detector enable
pub type SADEN_R = crate::BitReader;
///Field `SADEN` writer - Sound activity detector enable
pub type SADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATCAP` reader - Data capture mode
pub type DATCAP_R = crate::FieldReader;
///Field `DATCAP` writer - Data capture mode
pub type DATCAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DETCFG` reader - Sound trigger event configuration
pub type DETCFG_R = crate::BitReader;
///Field `DETCFG` writer - Sound trigger event configuration
pub type DETCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SADST` reader - SAD state
pub type SADST_R = crate::FieldReader;
///Field `HYSTEN` reader - Hysteresis enable
pub type HYSTEN_R = crate::BitReader;
///Field `HYSTEN` writer - Hysteresis enable
pub type HYSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSIZE` reader - Frame size
pub type FRSIZE_R = crate::FieldReader;
///Field `FRSIZE` writer - Frame size
pub type FRSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SADMOD` reader - SAD working mode
pub type SADMOD_R = crate::FieldReader;
///Field `SADMOD` writer - SAD working mode
pub type SADMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SADACTIVE` reader - SAD Active flag
pub type SADACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Sound activity detector enable
    #[inline(always)]
    pub fn saden(&self) -> SADEN_R {
        SADEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data capture mode
    #[inline(always)]
    pub fn datcap(&self) -> DATCAP_R {
        DATCAP_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Sound trigger event configuration
    #[inline(always)]
    pub fn detcfg(&self) -> DETCFG_R {
        DETCFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SAD state
    #[inline(always)]
    pub fn sadst(&self) -> SADST_R {
        SADST_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Hysteresis enable
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Frame size
    #[inline(always)]
    pub fn frsize(&self) -> FRSIZE_R {
        FRSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:13 - SAD working mode
    #[inline(always)]
    pub fn sadmod(&self) -> SADMOD_R {
        SADMOD_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 31 - SAD Active flag
    #[inline(always)]
    pub fn sadactive(&self) -> SADACTIVE_R {
        SADACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADCR")
            .field("sadactive", &self.sadactive())
            .field("sadmod", &self.sadmod())
            .field("frsize", &self.frsize())
            .field("hysten", &self.hysten())
            .field("sadst", &self.sadst())
            .field("detcfg", &self.detcfg())
            .field("datcap", &self.datcap())
            .field("saden", &self.saden())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sound activity detector enable
    #[inline(always)]
    pub fn saden(&mut self) -> SADEN_W<SADCRrs> {
        SADEN_W::new(self, 0)
    }
    ///Bits 1:2 - Data capture mode
    #[inline(always)]
    pub fn datcap(&mut self) -> DATCAP_W<SADCRrs> {
        DATCAP_W::new(self, 1)
    }
    ///Bit 3 - Sound trigger event configuration
    #[inline(always)]
    pub fn detcfg(&mut self) -> DETCFG_W<SADCRrs> {
        DETCFG_W::new(self, 3)
    }
    ///Bit 7 - Hysteresis enable
    #[inline(always)]
    pub fn hysten(&mut self) -> HYSTEN_W<SADCRrs> {
        HYSTEN_W::new(self, 7)
    }
    ///Bits 8:10 - Frame size
    #[inline(always)]
    pub fn frsize(&mut self) -> FRSIZE_W<SADCRrs> {
        FRSIZE_W::new(self, 8)
    }
    ///Bits 12:13 - SAD working mode
    #[inline(always)]
    pub fn sadmod(&mut self) -> SADMOD_W<SADCRrs> {
        SADMOD_W::new(self, 12)
    }
}
/**ADF SAD control register

You can [`read`](crate::Reg::read) this register and get [`sadcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SADCR)*/
pub struct SADCRrs;
impl crate::RegisterSpec for SADCRrs {
    type Ux = u32;
}
///`read()` method returns [`sadcr::R`](R) reader structure
impl crate::Readable for SADCRrs {}
///`write(|w| ..)` method takes [`sadcr::W`](W) writer structure
impl crate::Writable for SADCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADCR to value 0
impl crate::Resettable for SADCRrs {}
