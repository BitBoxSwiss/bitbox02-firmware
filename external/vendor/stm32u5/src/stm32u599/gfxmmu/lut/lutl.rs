///Register `LUTL` reader
pub type R = crate::R<LUTLrs>;
///Register `LUTL` writer
pub type W = crate::W<LUTLrs>;
///Field `EN` reader - Enable Line enable.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enable Line enable.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FVB` reader - First Valid Block Number of the first valid block of line number x.
pub type FVB_R = crate::FieldReader;
///Field `FVB` writer - First Valid Block Number of the first valid block of line number x.
pub type FVB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LVB` reader - Last Valid Block Number of the last valid block of line number X.
pub type LVB_R = crate::FieldReader;
///Field `LVB` writer - Last Valid Block Number of the last valid block of line number X.
pub type LVB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Enable Line enable.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - First Valid Block Number of the first valid block of line number x.
    #[inline(always)]
    pub fn fvb(&self) -> FVB_R {
        FVB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Last Valid Block Number of the last valid block of line number X.
    #[inline(always)]
    pub fn lvb(&self) -> LVB_R {
        LVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUTL")
            .field("en", &self.en())
            .field("fvb", &self.fvb())
            .field("lvb", &self.lvb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Line enable.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<LUTLrs> {
        EN_W::new(self, 0)
    }
    ///Bits 8:15 - First Valid Block Number of the first valid block of line number x.
    #[inline(always)]
    pub fn fvb(&mut self) -> FVB_W<LUTLrs> {
        FVB_W::new(self, 8)
    }
    ///Bits 16:23 - Last Valid Block Number of the last valid block of line number X.
    #[inline(always)]
    pub fn lvb(&mut self) -> LVB_W<LUTLrs> {
        LVB_W::new(self, 16)
    }
}
/**Graphic MMU LUT entry x low

You can [`read`](crate::Reg::read) this register and get [`lutl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUTLrs;
impl crate::RegisterSpec for LUTLrs {
    type Ux = u32;
}
///`read()` method returns [`lutl::R`](R) reader structure
impl crate::Readable for LUTLrs {}
///`write(|w| ..)` method takes [`lutl::W`](W) writer structure
impl crate::Writable for LUTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LUTL to value 0
impl crate::Resettable for LUTLrs {}
