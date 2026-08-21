///Register `WIR` reader
pub type R = crate::R<WIRrs>;
///Register `WIR` writer
pub type W = crate::W<WIRrs>;
///Field `INSTRUCTION` reader - INSTRUCTION
pub type INSTRUCTION_R = crate::FieldReader<u32>;
///Field `INSTRUCTION` writer - INSTRUCTION
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIR")
            .field("instruction", &self.instruction())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W<WIRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
/**write instruction register

You can [`read`](crate::Reg::read) this register and get [`wir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OCTOSPI1:WIR)*/
pub struct WIRrs;
impl crate::RegisterSpec for WIRrs {
    type Ux = u32;
}
///`read()` method returns [`wir::R`](R) reader structure
impl crate::Readable for WIRrs {}
///`write(|w| ..)` method takes [`wir::W`](W) writer structure
impl crate::Writable for WIRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets WIR to value 0
impl crate::Resettable for WIRrs {}
