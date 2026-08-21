///Register `PIDR1` reader
pub type R = crate::R<PIDR1rs>;
///Field `PARTNUM` reader - part number bits \[11:8\]
pub type PARTNUM_R = crate::FieldReader;
///Field `JEP106ID` reader - JEP106 identity code bits \[3:0\]
pub type JEP106ID_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - part number bits \[11:8\]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - JEP106 identity code bits \[3:0\]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR1")
            .field("partnum", &self.partnum())
            .field("jep106id", &self.jep106id())
            .finish()
    }
}
/**Debug MCU CoreSight peripheral identity register 1

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DBGMCU:PIDR1)*/
pub struct PIDR1rs;
impl crate::RegisterSpec for PIDR1rs {
    type Ux = u32;
}
///`read()` method returns [`pidr1::R`](R) reader structure
impl crate::Readable for PIDR1rs {}
///`reset()` method sets PIDR1 to value 0
impl crate::Resettable for PIDR1rs {}
