///Register `R3ENDADDR` reader
pub type R = crate::R<R3ENDADDRrs>;
///Register `R3ENDADDR` writer
pub type W = crate::W<R3ENDADDRrs>;
///Field `REG3_END_ADDR` reader - Region AXI end address
pub type REG3_END_ADDR_R = crate::FieldReader<u32>;
///Field `REG3_END_ADDR` writer - Region AXI end address
pub type REG3_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI end address
    #[inline(always)]
    pub fn reg3_end_addr(&self) -> REG3_END_ADDR_R {
        REG3_END_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R3ENDADDR")
            .field("reg3_end_addr", &self.reg3_end_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI end address
    #[inline(always)]
    pub fn reg3_end_addr(&mut self) -> REG3_END_ADDR_W<R3ENDADDRrs> {
        REG3_END_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region x end address register

You can [`read`](crate::Reg::read) this register and get [`r3endaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3endaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OTFDEC1:R3ENDADDR)*/
pub struct R3ENDADDRrs;
impl crate::RegisterSpec for R3ENDADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r3endaddr::R`](R) reader structure
impl crate::Readable for R3ENDADDRrs {}
///`write(|w| ..)` method takes [`r3endaddr::W`](W) writer structure
impl crate::Writable for R3ENDADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3ENDADDR to value 0x0fff
impl crate::Resettable for R3ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
