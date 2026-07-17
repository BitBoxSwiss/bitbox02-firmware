///Register `ATCR2` reader
pub type R = crate::R<ATCR2rs>;
///Register `ATCR2` writer
pub type W = crate::W<ATCR2rs>;
///Field `ATOSEL1` reader - ATOSEL1
pub type ATOSEL1_R = crate::FieldReader;
///Field `ATOSEL1` writer - ATOSEL1
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSEL2` reader - ATOSEL2
pub type ATOSEL2_R = crate::FieldReader;
///Field `ATOSEL2` writer - ATOSEL2
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSEL3` reader - ATOSEL3
pub type ATOSEL3_R = crate::FieldReader;
///Field `ATOSEL3` writer - ATOSEL3
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSEL4` reader - ATOSEL4
pub type ATOSEL4_R = crate::FieldReader;
///Field `ATOSEL4` writer - ATOSEL4
pub type ATOSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL5` reader - ATOSEL5
pub type ATOSEL5_R = crate::FieldReader;
///Field `ATOSEL5` writer - ATOSEL5
pub type ATOSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSEL6` reader - ATOSEL6
pub type ATOSEL6_R = crate::FieldReader;
///Field `ATOSEL6` writer - ATOSEL6
pub type ATOSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSEL7` reader - ATOSEL7
pub type ATOSEL7_R = crate::FieldReader;
///Field `ATOSEL7` writer - ATOSEL7
pub type ATOSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSEL8` reader - ATOSEL8
pub type ATOSEL8_R = crate::FieldReader;
///Field `ATOSEL8` writer - ATOSEL8
pub type ATOSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:10 - ATOSEL1
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - ATOSEL2
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:16 - ATOSEL3
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 17:18 - ATOSEL4
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 20:22 - ATOSEL5
    #[inline(always)]
    pub fn atosel5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - ATOSEL6
    #[inline(always)]
    pub fn atosel6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:28 - ATOSEL7
    #[inline(always)]
    pub fn atosel7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - ATOSEL8
    #[inline(always)]
    pub fn atosel8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATCR2")
            .field("atosel1", &self.atosel1())
            .field("atosel2", &self.atosel2())
            .field("atosel3", &self.atosel3())
            .field("atosel4", &self.atosel4())
            .field("atosel5", &self.atosel5())
            .field("atosel6", &self.atosel6())
            .field("atosel7", &self.atosel7())
            .field("atosel8", &self.atosel8())
            .finish()
    }
}
impl W {
    ///Bits 8:10 - ATOSEL1
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W<ATCR2rs> {
        ATOSEL1_W::new(self, 8)
    }
    ///Bits 11:13 - ATOSEL2
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W<ATCR2rs> {
        ATOSEL2_W::new(self, 11)
    }
    ///Bits 14:16 - ATOSEL3
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W<ATCR2rs> {
        ATOSEL3_W::new(self, 14)
    }
    ///Bits 17:18 - ATOSEL4
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W<ATCR2rs> {
        ATOSEL4_W::new(self, 17)
    }
    ///Bits 20:22 - ATOSEL5
    #[inline(always)]
    pub fn atosel5(&mut self) -> ATOSEL5_W<ATCR2rs> {
        ATOSEL5_W::new(self, 20)
    }
    ///Bits 23:25 - ATOSEL6
    #[inline(always)]
    pub fn atosel6(&mut self) -> ATOSEL6_W<ATCR2rs> {
        ATOSEL6_W::new(self, 23)
    }
    ///Bits 26:28 - ATOSEL7
    #[inline(always)]
    pub fn atosel7(&mut self) -> ATOSEL7_W<ATCR2rs> {
        ATOSEL7_W::new(self, 26)
    }
    ///Bits 29:31 - ATOSEL8
    #[inline(always)]
    pub fn atosel8(&mut self) -> ATOSEL8_W<ATCR2rs> {
        ATOSEL8_W::new(self, 29)
    }
}
/**TAMP active tamper control register 2

You can [`read`](crate::Reg::read) this register and get [`atcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TAMP:ATCR2)*/
pub struct ATCR2rs;
impl crate::RegisterSpec for ATCR2rs {
    type Ux = u32;
}
///`read()` method returns [`atcr2::R`](R) reader structure
impl crate::Readable for ATCR2rs {}
///`write(|w| ..)` method takes [`atcr2::W`](W) writer structure
impl crate::Writable for ATCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ATCR2 to value 0
impl crate::Resettable for ATCR2rs {}
