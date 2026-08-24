///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - Slave mode selection
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - Slave mode selection
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TS` reader - Trigger selection
pub type TS_R = crate::FieldReader;
///Field `TS` writer - Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSM` reader - Master/slave mode
pub type MSM_R = crate::BitReader;
///Field `MSM` writer - Master/slave mode
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMS_3` reader - Slave mode selection
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - Slave mode selection
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS2` reader - Trigger selection
pub type TS2_R = crate::FieldReader;
///Field `TS2` writer - Trigger selection
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("ts2", &self.ts2())
            .field("sms_3", &self.sms_3())
            .field("cc1de", &self.cc1de())
            .field("msm", &self.msm())
            .field("ts", &self.ts())
            .field("sms", &self.sms())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<SMCRrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 16 - Slave mode selection
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<SMCRrs> {
        TS2_W::new(self, 20)
    }
}
/**slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#TIM15:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {}
