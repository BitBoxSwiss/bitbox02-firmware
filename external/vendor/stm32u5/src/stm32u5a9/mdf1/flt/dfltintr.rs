///Register `DFLTINTR` reader
pub type R = crate::R<DFLTINTRrs>;
///Register `DFLTINTR` writer
pub type W = crate::W<DFLTINTRrs>;
///Field `INTDIV` reader - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTDIV_R = crate::FieldReader;
///Field `INTDIV` writer - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INTVAL` reader - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTVAL_R = crate::FieldReader;
///Field `INTVAL` writer - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:1 - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn intdiv(&self) -> INTDIV_R {
        INTDIV_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:10 - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn intval(&self) -> INTVAL_R {
        INTVAL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTINTR")
            .field("intdiv", &self.intdiv())
            .field("intval", &self.intval())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn intdiv(&mut self) -> INTDIV_W<DFLTINTRrs> {
        INTDIV_W::new(self, 0)
    }
    ///Bits 4:10 - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn intval(&mut self) -> INTVAL_W<DFLTINTRrs> {
        INTVAL_W::new(self, 4)
    }
}
/**This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`dfltintr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltintr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTINTRrs;
impl crate::RegisterSpec for DFLTINTRrs {
    type Ux = u32;
}
///`read()` method returns [`dfltintr::R`](R) reader structure
impl crate::Readable for DFLTINTRrs {}
///`write(|w| ..)` method takes [`dfltintr::W`](W) writer structure
impl crate::Writable for DFLTINTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLTINTR to value 0
impl crate::Resettable for DFLTINTRrs {}
