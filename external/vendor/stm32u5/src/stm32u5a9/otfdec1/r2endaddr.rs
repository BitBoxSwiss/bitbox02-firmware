///Register `R2ENDADDR` reader
pub type R = crate::R<R2ENDADDRrs>;
///Register `R2ENDADDR` writer
pub type W = crate::W<R2ENDADDRrs>;
///Field `REG2_END_ADDR` reader - Region AXI end address
pub type REG2_END_ADDR_R = crate::FieldReader<u32>;
///Field `REG2_END_ADDR` writer - Region AXI end address
pub type REG2_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI end address
    #[inline(always)]
    pub fn reg2_end_addr(&self) -> REG2_END_ADDR_R {
        REG2_END_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R2ENDADDR")
            .field("reg2_end_addr", &self.reg2_end_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI end address
    #[inline(always)]
    pub fn reg2_end_addr(&mut self) -> REG2_END_ADDR_W<R2ENDADDRrs> {
        REG2_END_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region x end address register

You can [`read`](crate::Reg::read) this register and get [`r2endaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2endaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTFDEC1:R2ENDADDR)*/
pub struct R2ENDADDRrs;
impl crate::RegisterSpec for R2ENDADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r2endaddr::R`](R) reader structure
impl crate::Readable for R2ENDADDRrs {}
///`write(|w| ..)` method takes [`r2endaddr::W`](W) writer structure
impl crate::Writable for R2ENDADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R2ENDADDR to value 0x0fff
impl crate::Resettable for R2ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
