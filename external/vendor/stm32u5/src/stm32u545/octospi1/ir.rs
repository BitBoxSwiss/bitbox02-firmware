///Register `IR` reader
pub type R = crate::R<IRrs>;
///Register `IR` writer
pub type W = crate::W<IRrs>;
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
        f.debug_struct("IR")
            .field("instruction", &self.instruction())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W<IRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
/**instruction register

You can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OCTOSPI1:IR)*/
pub struct IRrs;
impl crate::RegisterSpec for IRrs {
    type Ux = u32;
}
///`read()` method returns [`ir::R`](R) reader structure
impl crate::Readable for IRrs {}
///`write(|w| ..)` method takes [`ir::W`](W) writer structure
impl crate::Writable for IRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IRrs {}
