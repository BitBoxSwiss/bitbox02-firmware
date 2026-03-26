///Register `ECR` reader
pub type R = crate::R<ECRrs>;
///Register `ECR` writer
pub type W = crate::W<ECRrs>;
///Field `IE` reader - Index enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Index enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDIR` reader - Index direction
pub type IDIR_R = crate::FieldReader;
///Field `IDIR` writer - Index direction
pub type IDIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FIDX` reader - First index
pub type FIDX_R = crate::BitReader;
///Field `FIDX` writer - First index
pub type FIDX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPOS` reader - Index positioning
pub type IPOS_R = crate::FieldReader;
///Field `IPOS` writer - Index positioning
pub type IPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PW` reader - Pulse width
pub type PW_R = crate::FieldReader;
///Field `PW` writer - Pulse width
pub type PW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PWPRSC` reader - Pulse width prescaler
pub type PWPRSC_R = crate::FieldReader;
///Field `PWPRSC` writer - Pulse width prescaler
pub type PWPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Index enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Index direction
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 5 - First index
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Index positioning
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 16:23 - Pulse width
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - Pulse width prescaler
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("pwprsc", &self.pwprsc())
            .field("pw", &self.pw())
            .field("ipos", &self.ipos())
            .field("fidx", &self.fidx())
            .field("idir", &self.idir())
            .field("ie", &self.ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Index enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<ECRrs> {
        IE_W::new(self, 0)
    }
    ///Bits 1:2 - Index direction
    #[inline(always)]
    pub fn idir(&mut self) -> IDIR_W<ECRrs> {
        IDIR_W::new(self, 1)
    }
    ///Bit 5 - First index
    #[inline(always)]
    pub fn fidx(&mut self) -> FIDX_W<ECRrs> {
        FIDX_W::new(self, 5)
    }
    ///Bits 6:7 - Index positioning
    #[inline(always)]
    pub fn ipos(&mut self) -> IPOS_W<ECRrs> {
        IPOS_W::new(self, 6)
    }
    ///Bits 16:23 - Pulse width
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W<ECRrs> {
        PW_W::new(self, 16)
    }
    ///Bits 24:26 - Pulse width prescaler
    #[inline(always)]
    pub fn pwprsc(&mut self) -> PWPRSC_W<ECRrs> {
        PWPRSC_W::new(self, 24)
    }
}
/**DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#TIM2:ECR)*/
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
///`read()` method returns [`ecr::R`](R) reader structure
impl crate::Readable for ECRrs {}
///`write(|w| ..)` method takes [`ecr::W`](W) writer structure
impl crate::Writable for ECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECRrs {}
