///Register `CMDRSADDRR` reader
pub type R = crate::R<CMDRSADDRRrs>;
///Register `CMDRSADDRR` writer
pub type W = crate::W<CMDRSADDRRrs>;
///Field `CMDSTARTADDR` reader - CMDSTARTADDR
pub type CMDSTARTADDR_R = crate::FieldReader<u32>;
///Field `CMDSTARTADDR` writer - CMDSTARTADDR
pub type CMDSTARTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 4:31 - CMDSTARTADDR
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDRSADDRR")
            .field("cmdstartaddr", &self.cmdstartaddr())
            .finish()
    }
}
impl W {
    ///Bits 4:31 - CMDSTARTADDR
    #[inline(always)]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<CMDRSADDRRrs> {
        CMDSTARTADDR_W::new(self, 4)
    }
}
/**command range start address register

You can [`read`](crate::Reg::read) this register and get [`cmdrsaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdrsaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DCACHE1:CMDRSADDRR)*/
pub struct CMDRSADDRRrs;
impl crate::RegisterSpec for CMDRSADDRRrs {
    type Ux = u32;
}
///`read()` method returns [`cmdrsaddrr::R`](R) reader structure
impl crate::Readable for CMDRSADDRRrs {}
///`write(|w| ..)` method takes [`cmdrsaddrr::W`](W) writer structure
impl crate::Writable for CMDRSADDRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMDRSADDRR to value 0
impl crate::Resettable for CMDRSADDRRrs {}
