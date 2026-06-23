///Register `DVBUSDIS` reader
pub type R = crate::R<DVBUSDISrs>;
///Register `DVBUSDIS` writer
pub type W = crate::W<DVBUSDISrs>;
///Field `VBUSDT` reader - VBUSDT
pub type VBUSDT_R = crate::FieldReader<u16>;
///Field `VBUSDT` writer - VBUSDT
pub type VBUSDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - VBUSDT
    #[inline(always)]
    pub fn vbusdt(&self) -> VBUSDT_R {
        VBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSDIS")
            .field("vbusdt", &self.vbusdt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VBUSDT
    #[inline(always)]
    pub fn vbusdt(&mut self) -> VBUSDT_W<DVBUSDISrs> {
        VBUSDT_W::new(self, 0)
    }
}
/**This register specifies the VBUS discharge time after VBUS pulsing during SRP.

You can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DVBUSDIS)*/
pub struct DVBUSDISrs;
impl crate::RegisterSpec for DVBUSDISrs {
    type Ux = u32;
}
///`read()` method returns [`dvbusdis::R`](R) reader structure
impl crate::Readable for DVBUSDISrs {}
///`write(|w| ..)` method takes [`dvbusdis::W`](W) writer structure
impl crate::Writable for DVBUSDISrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVBUSDIS to value 0x17d7
impl crate::Resettable for DVBUSDISrs {
    const RESET_VALUE: u32 = 0x17d7;
}
