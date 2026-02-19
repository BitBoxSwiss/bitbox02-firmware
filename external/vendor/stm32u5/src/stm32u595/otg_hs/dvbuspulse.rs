///Register `DVBUSPULSE` reader
pub type R = crate::R<DVBUSPULSErs>;
///Register `DVBUSPULSE` writer
pub type W = crate::W<DVBUSPULSErs>;
///Field `DVBUSP` reader - DVBUSP
pub type DVBUSP_R = crate::FieldReader<u16>;
///Field `DVBUSP` writer - DVBUSP
pub type DVBUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DVBUSP
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSPULSE")
            .field("dvbusp", &self.dvbusp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - DVBUSP
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W<DVBUSPULSErs> {
        DVBUSP_W::new(self, 0)
    }
}
/**This register specifies the VBUS pulsing time during SRP.

You can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:DVBUSPULSE)*/
pub struct DVBUSPULSErs;
impl crate::RegisterSpec for DVBUSPULSErs {
    type Ux = u32;
}
///`read()` method returns [`dvbuspulse::R`](R) reader structure
impl crate::Readable for DVBUSPULSErs {}
///`write(|w| ..)` method takes [`dvbuspulse::W`](W) writer structure
impl crate::Writable for DVBUSPULSErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVBUSPULSE to value 0x05b8
impl crate::Resettable for DVBUSPULSErs {
    const RESET_VALUE: u32 = 0x05b8;
}
