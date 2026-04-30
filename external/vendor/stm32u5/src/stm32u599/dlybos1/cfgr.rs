///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SEL` reader - SEL
pub type SEL_R = crate::FieldReader;
///Field `SEL` writer - SEL
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `UNIT` reader - UNIT
pub type UNIT_R = crate::FieldReader;
///Field `UNIT` writer - UNIT
pub type UNIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `LNG` reader - LNG
pub type LNG_R = crate::FieldReader<u16>;
///Field `LNGF` reader - LNGF
pub type LNGF_R = crate::BitReader;
impl R {
    ///Bits 0:3 - SEL
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:14 - UNIT
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:27 - LNG
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - LNGF
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("sel", &self.sel())
            .field("unit", &self.unit())
            .field("lng", &self.lng())
            .field("lngf", &self.lngf())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SEL
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<CFGRrs> {
        SEL_W::new(self, 0)
    }
    ///Bits 8:14 - UNIT
    #[inline(always)]
    pub fn unit(&mut self) -> UNIT_W<CFGRrs> {
        UNIT_W::new(self, 8)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DLYBOS1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
