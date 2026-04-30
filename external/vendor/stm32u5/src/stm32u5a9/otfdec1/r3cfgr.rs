///Register `R3CFGR` reader
pub type R = crate::R<R3CFGRrs>;
///Register `R3CFGR` writer
pub type W = crate::W<R3CFGRrs>;
///Field `REG_EN` reader - region on-the-fly decryption enable
pub type REG_EN_R = crate::BitReader;
///Field `REG_EN` writer - region on-the-fly decryption enable
pub type REG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONFIGLOCK` reader - region config lock
pub type CONFIGLOCK_R = crate::BitReader;
///Field `CONFIGLOCK` writer - region config lock
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYLOCK` reader - region key lock
pub type KEYLOCK_R = crate::BitReader;
///Field `KEYLOCK` writer - region key lock
pub type KEYLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - operating mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - operating mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYCRC` reader - region key 8-bit CRC
pub type KEYCRC_R = crate::FieldReader;
///Field `REG3_VERSION` reader - region firmware version
pub type REG3_VERSION_R = crate::FieldReader<u16>;
///Field `REG3_VERSION` writer - region firmware version
pub type REG3_VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - region on-the-fly decryption enable
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - region config lock
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - region key lock
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:15 - region key 8-bit CRC
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - region firmware version
    #[inline(always)]
    pub fn reg3_version(&self) -> REG3_VERSION_R {
        REG3_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R3CFGR")
            .field("reg_en", &self.reg_en())
            .field("configlock", &self.configlock())
            .field("keylock", &self.keylock())
            .field("mode", &self.mode())
            .field("keycrc", &self.keycrc())
            .field("reg3_version", &self.reg3_version())
            .finish()
    }
}
impl W {
    ///Bit 0 - region on-the-fly decryption enable
    #[inline(always)]
    pub fn reg_en(&mut self) -> REG_EN_W<R3CFGRrs> {
        REG_EN_W::new(self, 0)
    }
    ///Bit 1 - region config lock
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<R3CFGRrs> {
        CONFIGLOCK_W::new(self, 1)
    }
    ///Bit 2 - region key lock
    #[inline(always)]
    pub fn keylock(&mut self) -> KEYLOCK_W<R3CFGRrs> {
        KEYLOCK_W::new(self, 2)
    }
    ///Bits 4:5 - operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<R3CFGRrs> {
        MODE_W::new(self, 4)
    }
    ///Bits 16:31 - region firmware version
    #[inline(always)]
    pub fn reg3_version(&mut self) -> REG3_VERSION_W<R3CFGRrs> {
        REG3_VERSION_W::new(self, 16)
    }
}
/**OTFDEC region x configuration register

You can [`read`](crate::Reg::read) this register and get [`r3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTFDEC1:R3CFGR)*/
pub struct R3CFGRrs;
impl crate::RegisterSpec for R3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`r3cfgr::R`](R) reader structure
impl crate::Readable for R3CFGRrs {}
///`write(|w| ..)` method takes [`r3cfgr::W`](W) writer structure
impl crate::Writable for R3CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3CFGR to value 0
impl crate::Resettable for R3CFGRrs {}
