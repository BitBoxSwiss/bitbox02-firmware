///Register `R4STARTADDR` reader
pub type R = crate::R<R4STARTADDRrs>;
///Register `R4STARTADDR` writer
pub type W = crate::W<R4STARTADDRrs>;
///Field `REG4_START_ADDR` reader - Region AXI start address
pub type REG4_START_ADDR_R = crate::FieldReader<u32>;
///Field `REG4_START_ADDR` writer - Region AXI start address
pub type REG4_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI start address
    #[inline(always)]
    pub fn reg4_start_addr(&self) -> REG4_START_ADDR_R {
        REG4_START_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R4STARTADDR")
            .field("reg4_start_addr", &self.reg4_start_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI start address
    #[inline(always)]
    pub fn reg4_start_addr(&mut self) -> REG4_START_ADDR_W<R4STARTADDRrs> {
        REG4_START_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region x start address register

You can [`read`](crate::Reg::read) this register and get [`r4startaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4startaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTFDEC1:R4STARTADDR)*/
pub struct R4STARTADDRrs;
impl crate::RegisterSpec for R4STARTADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r4startaddr::R`](R) reader structure
impl crate::Readable for R4STARTADDRrs {}
///`write(|w| ..)` method takes [`r4startaddr::W`](W) writer structure
impl crate::Writable for R4STARTADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R4STARTADDR to value 0
impl crate::Resettable for R4STARTADDRrs {}
