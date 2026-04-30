///Register `PIDR0` reader
pub type R = crate::R<PIDR0rs>;
///Field `PARTNUM` reader - part number bits \[7:0\]
pub type PARTNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - part number bits \[7:0\]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR0")
            .field("partnum", &self.partnum())
            .finish()
    }
}
/**Debug MCU CoreSight peripheral identity register 0

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DBGMCU:PIDR0)*/
pub struct PIDR0rs;
impl crate::RegisterSpec for PIDR0rs {
    type Ux = u32;
}
///`read()` method returns [`pidr0::R`](R) reader structure
impl crate::Readable for PIDR0rs {}
///`reset()` method sets PIDR0 to value 0
impl crate::Resettable for PIDR0rs {}
