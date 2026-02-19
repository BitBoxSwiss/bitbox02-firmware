///Register `HAINT` reader
pub type R = crate::R<HAINTrs>;
///Field `HAINT` reader - HAINT
pub type HAINT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - HAINT
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINT")
            .field("haint", &self.haint())
            .finish()
    }
}
/**When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.

You can [`read`](crate::Reg::read) this register and get [`haint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HAINT)*/
pub struct HAINTrs;
impl crate::RegisterSpec for HAINTrs {
    type Ux = u32;
}
///`read()` method returns [`haint::R`](R) reader structure
impl crate::Readable for HAINTrs {}
///`reset()` method sets HAINT to value 0
impl crate::Resettable for HAINTrs {}
