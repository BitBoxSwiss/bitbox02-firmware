///Register `DR16` reader
pub type R = crate::R<DR16rs>;
///Register `DR16` writer
pub type W = crate::W<DR16rs>;
///Field `DR16` reader - Data register bits
pub type DR16_R = crate::FieldReader<u16>;
///Field `DR16` writer - Data register bits
pub type DR16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Data register bits
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR16").field("dr16", &self.dr16()).finish()
    }
}
impl W {
    ///Bits 0:15 - Data register bits
    #[inline(always)]
    pub fn dr16(&mut self) -> DR16_W<DR16rs> {
        DR16_W::new(self, 0)
    }
}
/**Data register - half-word sized

You can [`read`](crate::Reg::read) this register and get [`dr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#CRC:DR16)*/
pub struct DR16rs;
impl crate::RegisterSpec for DR16rs {
    type Ux = u16;
}
///`read()` method returns [`dr16::R`](R) reader structure
impl crate::Readable for DR16rs {}
///`write(|w| ..)` method takes [`dr16::W`](W) writer structure
impl crate::Writable for DR16rs {
    type Safety = crate::Safe;
}
///`reset()` method sets DR16 to value 0xffff
impl crate::Resettable for DR16rs {
    const RESET_VALUE: u16 = 0xffff;
}
