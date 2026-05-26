///Register `CCCR` reader
pub type R = crate::R<CCCRrs>;
///Register `CCCR` writer
pub type W = crate::W<CCCRrs>;
///Field `NCC1` reader - NCC1
pub type NCC1_R = crate::FieldReader;
///Field `NCC1` writer - NCC1
pub type NCC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PCC1` reader - PCC1
pub type PCC1_R = crate::FieldReader;
///Field `PCC1` writer - PCC1
pub type PCC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NCC2` reader - NCC2
pub type NCC2_R = crate::FieldReader;
///Field `NCC2` writer - NCC2
pub type NCC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PCC2` reader - PCC2
pub type PCC2_R = crate::FieldReader;
///Field `PCC2` writer - PCC2
pub type PCC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NCC3` reader - NCC3
pub type NCC3_R = crate::FieldReader;
///Field `NCC3` writer - NCC3
pub type NCC3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PCC3` reader - PCC3
pub type PCC3_R = crate::FieldReader;
///Field `PCC3` writer - PCC3
pub type PCC3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NCC1
    #[inline(always)]
    pub fn ncc1(&self) -> NCC1_R {
        NCC1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PCC1
    #[inline(always)]
    pub fn pcc1(&self) -> PCC1_R {
        PCC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NCC2
    #[inline(always)]
    pub fn ncc2(&self) -> NCC2_R {
        NCC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PCC2
    #[inline(always)]
    pub fn pcc2(&self) -> PCC2_R {
        PCC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - NCC3
    #[inline(always)]
    pub fn ncc3(&self) -> NCC3_R {
        NCC3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - PCC3
    #[inline(always)]
    pub fn pcc3(&self) -> PCC3_R {
        PCC3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("ncc1", &self.ncc1())
            .field("pcc1", &self.pcc1())
            .field("ncc2", &self.ncc2())
            .field("pcc2", &self.pcc2())
            .field("ncc3", &self.ncc3())
            .field("pcc3", &self.pcc3())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NCC1
    #[inline(always)]
    pub fn ncc1(&mut self) -> NCC1_W<CCCRrs> {
        NCC1_W::new(self, 0)
    }
    ///Bits 4:7 - PCC1
    #[inline(always)]
    pub fn pcc1(&mut self) -> PCC1_W<CCCRrs> {
        PCC1_W::new(self, 4)
    }
    ///Bits 8:11 - NCC2
    #[inline(always)]
    pub fn ncc2(&mut self) -> NCC2_W<CCCRrs> {
        NCC2_W::new(self, 8)
    }
    ///Bits 12:15 - PCC2
    #[inline(always)]
    pub fn pcc2(&mut self) -> PCC2_W<CCCRrs> {
        PCC2_W::new(self, 12)
    }
    ///Bits 16:19 - NCC3
    #[inline(always)]
    pub fn ncc3(&mut self) -> NCC3_W<CCCRrs> {
        NCC3_W::new(self, 16)
    }
    ///Bits 20:23 - PCC3
    #[inline(always)]
    pub fn pcc3(&mut self) -> PCC3_W<CCCRrs> {
        PCC3_W::new(self, 20)
    }
}
/**compensation cell code register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SYSCFG:CCCR)*/
pub struct CCCRrs;
impl crate::RegisterSpec for CCCRrs {
    type Ux = u32;
}
///`read()` method returns [`cccr::R`](R) reader structure
impl crate::Readable for CCCRrs {}
///`write(|w| ..)` method takes [`cccr::W`](W) writer structure
impl crate::Writable for CCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCR to value 0x7878
impl crate::Resettable for CCCRrs {
    const RESET_VALUE: u32 = 0x7878;
}
