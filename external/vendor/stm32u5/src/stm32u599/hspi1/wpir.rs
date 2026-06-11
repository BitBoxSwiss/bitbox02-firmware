///Register `WPIR` reader
pub type R = crate::R<WPIRrs>;
///Register `WPIR` writer
pub type W = crate::W<WPIRrs>;
///Field `INSTRUCTION` reader - 31: 0\]: Instruction Instruction to be sent to the external SPI device
pub type INSTRUCTION_R = crate::FieldReader<u32>;
///Field `INSTRUCTION` writer - 31: 0\]: Instruction Instruction to be sent to the external SPI device
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 31: 0\]: Instruction Instruction to be sent to the external SPI device
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPIR")
            .field("instruction", &self.instruction())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - 31: 0\]: Instruction Instruction to be sent to the external SPI device
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W<WPIRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
/**HSPI wrap instruction register

You can [`read`](crate::Reg::read) this register and get [`wpir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WPIR)*/
pub struct WPIRrs;
impl crate::RegisterSpec for WPIRrs {
    type Ux = u32;
}
///`read()` method returns [`wpir::R`](R) reader structure
impl crate::Readable for WPIRrs {}
///`write(|w| ..)` method takes [`wpir::W`](W) writer structure
impl crate::Writable for WPIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPIR to value 0
impl crate::Resettable for WPIRrs {}
