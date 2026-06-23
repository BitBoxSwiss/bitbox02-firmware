///Register `CIDR2` reader
pub type R = crate::R<CIDR2rs>;
///Field `PREAMBLE` reader - component identification bits \[23:16\]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - component identification bits \[23:16\]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIDR2")
            .field("preamble", &self.preamble())
            .finish()
    }
}
/**Debug MCU CoreSight component identity register 2

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DBGMCU:CIDR2)*/
pub struct CIDR2rs;
impl crate::RegisterSpec for CIDR2rs {
    type Ux = u32;
}
///`read()` method returns [`cidr2::R`](R) reader structure
impl crate::Readable for CIDR2rs {}
///`reset()` method sets CIDR2 to value 0x05
impl crate::Resettable for CIDR2rs {
    const RESET_VALUE: u32 = 0x05;
}
