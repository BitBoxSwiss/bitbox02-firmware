///Register `_WIR` reader
pub type R = crate::R<_WIRrs>;
///Register `_WIR` writer
pub type W = crate::W<_WIRrs>;
///Field `INSTRUCTION` reader - Instruction Instruction to be sent to the external SPI device
pub type INSTRUCTION_R = crate::FieldReader<u32>;
///Field `INSTRUCTION` writer - Instruction Instruction to be sent to the external SPI device
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Instruction Instruction to be sent to the external SPI device
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_WIR")
            .field("instruction", &self.instruction())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Instruction Instruction to be sent to the external SPI device
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W<_WIRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
/**HSPI write instruction register

You can [`read`](crate::Reg::read) this register and get [`_wir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WIR)*/
pub struct _WIRrs;
impl crate::RegisterSpec for _WIRrs {
    type Ux = u32;
}
///`read()` method returns [`_wir::R`](R) reader structure
impl crate::Readable for _WIRrs {}
///`write(|w| ..)` method takes [`_wir::W`](W) writer structure
impl crate::Writable for _WIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _WIR to value 0
impl crate::Resettable for _WIRrs {}
