///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - SAES enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - SAES enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - DATATYPE
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - DATATYPE
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - MODE
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - MODE
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD` reader - CHMOD
pub type CHMOD_R = crate::FieldReader;
///Field `CHMOD` writer - CHMOD
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMAINEN` reader - DMAINEN
pub type DMAINEN_R = crate::BitReader;
///Field `DMAINEN` writer - DMAINEN
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAOUTEN` reader - DMAOUTEN
pub type DMAOUTEN_R = crate::BitReader;
///Field `DMAOUTEN` writer - DMAOUTEN
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSIZE` reader - KEYSIZE
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - KEYSIZE
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYPROT` reader - KEYPROT
pub type KEYPROT_R = crate::BitReader;
///Field `KEYPROT` writer - KEYPROT
pub type KEYPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KMOD` reader - KMOD
pub type KMOD_R = crate::FieldReader;
///Field `KMOD` writer - KMOD
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KSHAREID` reader - KSHAREID
pub type KSHAREID_R = crate::FieldReader;
///Field `KSHAREID` writer - KSHAREID
pub type KSHAREID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYSEL` reader - KEYSEL
pub type KEYSEL_R = crate::FieldReader;
///Field `KEYSEL` writer - KEYSEL
pub type KEYSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IPRST` reader - IPRST
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - IPRST
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SAES enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATATYPE
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - MODE
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - CHMOD
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - DMAINEN
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMAOUTEN
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - KEYPROT
    #[inline(always)]
    pub fn keyprot(&self) -> KEYPROT_R {
        KEYPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - KSHAREID
    #[inline(always)]
    pub fn kshareid(&self) -> KSHAREID_R {
        KSHAREID_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:30 - KEYSEL
    #[inline(always)]
    pub fn keysel(&self) -> KEYSEL_R {
        KEYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("iprst", &self.iprst())
            .field("keysel", &self.keysel())
            .field("kshareid", &self.kshareid())
            .field("kmod", &self.kmod())
            .field("keyprot", &self.keyprot())
            .field("keysize", &self.keysize())
            .field("dmaouten", &self.dmaouten())
            .field("dmainen", &self.dmainen())
            .field("chmod", &self.chmod())
            .field("mode", &self.mode())
            .field("datatype", &self.datatype())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - SAES enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - DATATYPE
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - MODE
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - CHMOD
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 11 - DMAINEN
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - DMAOUTEN
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bit 19 - KEYPROT
    #[inline(always)]
    pub fn keyprot(&mut self) -> KEYPROT_W<CRrs> {
        KEYPROT_W::new(self, 19)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bits 26:27 - KSHAREID
    #[inline(always)]
    pub fn kshareid(&mut self) -> KSHAREID_W<CRrs> {
        KSHAREID_W::new(self, 26)
    }
    ///Bits 28:30 - KEYSEL
    #[inline(always)]
    pub fn keysel(&mut self) -> KEYSEL_W<CRrs> {
        KEYSEL_W::new(self, 28)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SAES:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
