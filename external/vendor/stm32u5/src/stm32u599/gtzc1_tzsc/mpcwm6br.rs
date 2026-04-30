///Register `MPCWM6BR` reader
pub type R = crate::R<MPCWM6BRrs>;
///Register `MPCWM6BR` writer
pub type W = crate::W<MPCWM6BRrs>;
///Field `SUBB_START` reader - Start of sub-region A
pub type SUBB_START_R = crate::FieldReader<u16>;
///Field `SUBB_START` writer - Start of sub-region A
pub type SUBB_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `SUBB_LENGTH` reader - Length of sub-region A
pub type SUBB_LENGTH_R = crate::FieldReader<u16>;
///Field `SUBB_LENGTH` writer - Length of sub-region A
pub type SUBB_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    pub fn subb_start(&self) -> SUBB_START_R {
        SUBB_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    pub fn subb_length(&self) -> SUBB_LENGTH_R {
        SUBB_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM6BR")
            .field("subb_start", &self.subb_start())
            .field("subb_length", &self.subb_length())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    pub fn subb_start(&mut self) -> SUBB_START_W<MPCWM6BRrs> {
        SUBB_START_W::new(self, 0)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    pub fn subb_length(&mut self) -> SUBB_LENGTH_W<MPCWM6BRrs> {
        SUBB_LENGTH_W::new(self, 16)
    }
}
/**TZSC memory 6 sub-region B watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm6br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm6br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZSC:MPCWM6BR)*/
pub struct MPCWM6BRrs;
impl crate::RegisterSpec for MPCWM6BRrs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm6br::R`](R) reader structure
impl crate::Readable for MPCWM6BRrs {}
///`write(|w| ..)` method takes [`mpcwm6br::W`](W) writer structure
impl crate::Writable for MPCWM6BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM6BR to value 0
impl crate::Resettable for MPCWM6BRrs {}
