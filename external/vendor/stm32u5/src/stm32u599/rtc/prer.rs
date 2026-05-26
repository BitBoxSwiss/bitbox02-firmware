///Register `PRER` reader
pub type R = crate::R<PRERrs>;
///Register `PRER` writer
pub type W = crate::W<PRERrs>;
///Field `PREDIV_S` reader - Synchronous prescaler factor
pub type PREDIV_S_R = crate::FieldReader<u16>;
///Field `PREDIV_S` writer - Synchronous prescaler factor
pub type PREDIV_S_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16, crate::Safe>;
///Field `PREDIV_A` reader - Asynchronous prescaler factor
pub type PREDIV_A_R = crate::FieldReader;
///Field `PREDIV_A` writer - Asynchronous prescaler factor
pub type PREDIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:14 - Synchronous prescaler factor
    #[inline(always)]
    pub fn prediv_s(&self) -> PREDIV_S_R {
        PREDIV_S_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 16:22 - Asynchronous prescaler factor
    #[inline(always)]
    pub fn prediv_a(&self) -> PREDIV_A_R {
        PREDIV_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRER")
            .field("prediv_a", &self.prediv_a())
            .field("prediv_s", &self.prediv_s())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Synchronous prescaler factor
    #[inline(always)]
    pub fn prediv_s(&mut self) -> PREDIV_S_W<PRERrs> {
        PREDIV_S_W::new(self, 0)
    }
    ///Bits 16:22 - Asynchronous prescaler factor
    #[inline(always)]
    pub fn prediv_a(&mut self) -> PREDIV_A_W<PRERrs> {
        PREDIV_A_W::new(self, 16)
    }
}
/**prescaler register

You can [`read`](crate::Reg::read) this register and get [`prer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RTC:PRER)*/
pub struct PRERrs;
impl crate::RegisterSpec for PRERrs {
    type Ux = u32;
}
///`read()` method returns [`prer::R`](R) reader structure
impl crate::Readable for PRERrs {}
///`write(|w| ..)` method takes [`prer::W`](W) writer structure
impl crate::Writable for PRERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRER to value 0x007f_00ff
impl crate::Resettable for PRERrs {
    const RESET_VALUE: u32 = 0x007f_00ff;
}
