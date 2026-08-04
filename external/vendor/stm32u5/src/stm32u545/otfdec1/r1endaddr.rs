///Register `R1ENDADDR` reader
pub type R = crate::R<R1ENDADDRrs>;
///Register `R1ENDADDR` writer
pub type W = crate::W<R1ENDADDRrs>;
///Field `REG1_END_ADDR` reader - Region AXI end address
pub type REG1_END_ADDR_R = crate::FieldReader<u32>;
///Field `REG1_END_ADDR` writer - Region AXI end address
pub type REG1_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI end address
    #[inline(always)]
    pub fn reg1_end_addr(&self) -> REG1_END_ADDR_R {
        REG1_END_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R1ENDADDR")
            .field("reg1_end_addr", &self.reg1_end_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI end address
    #[inline(always)]
    pub fn reg1_end_addr(&mut self) -> REG1_END_ADDR_W<R1ENDADDRrs> {
        REG1_END_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region x end address register

You can [`read`](crate::Reg::read) this register and get [`r1endaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r1endaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OTFDEC1:R1ENDADDR)*/
pub struct R1ENDADDRrs;
impl crate::RegisterSpec for R1ENDADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r1endaddr::R`](R) reader structure
impl crate::Readable for R1ENDADDRrs {}
///`write(|w| ..)` method takes [`r1endaddr::W`](W) writer structure
impl crate::Writable for R1ENDADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R1ENDADDR to value 0x0fff
impl crate::Resettable for R1ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
