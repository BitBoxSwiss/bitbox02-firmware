///Register `AFRCR` reader
pub type R = crate::R<AFRCRrs>;
///Register `AFRCR` writer
pub type W = crate::W<AFRCRrs>;
///Field `FRL` reader - Frame length
pub type FRL_R = crate::FieldReader;
///Field `FRL` writer - Frame length
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FSALL` reader - Frame synchronization active level length
pub type FSALL_R = crate::FieldReader;
///Field `FSALL` writer - Frame synchronization active level length
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `FSDEF` reader - Frame synchronization definition
pub type FSDEF_R = crate::BitReader;
///Field `FSDEF` writer - Frame synchronization definition
pub type FSDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSPOL` reader - Frame synchronization polarity
pub type FSPOL_R = crate::BitReader;
///Field `FSPOL` writer - Frame synchronization polarity
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSOFF` reader - Frame synchronization offset
pub type FSOFF_R = crate::BitReader;
///Field `FSOFF` writer - Frame synchronization offset
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRCR")
            .field("fsoff", &self.fsoff())
            .field("fspol", &self.fspol())
            .field("fsdef", &self.fsdef())
            .field("fsall", &self.fsall())
            .field("frl", &self.frl())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W<AFRCRrs> {
        FRL_W::new(self, 0)
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W<AFRCRrs> {
        FSALL_W::new(self, 8)
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W<AFRCRrs> {
        FSDEF_W::new(self, 16)
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W<AFRCRrs> {
        FSPOL_W::new(self, 17)
    }
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W<AFRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
/**A frame configuration register

You can [`read`](crate::Reg::read) this register and get [`afrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#SAI1:AFRCR)*/
pub struct AFRCRrs;
impl crate::RegisterSpec for AFRCRrs {
    type Ux = u32;
}
///`read()` method returns [`afrcr::R`](R) reader structure
impl crate::Readable for AFRCRrs {}
///`write(|w| ..)` method takes [`afrcr::W`](W) writer structure
impl crate::Writable for AFRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRCR to value 0x07
impl crate::Resettable for AFRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
