///Register `RAM1ISR` reader
pub type R = crate::R<RAM1ISRrs>;
///Field `SEDC` reader - SEDC
pub type SEDC_R = crate::BitReader;
///Field `DED` reader - DED
pub type DED_R = crate::BitReader;
///Field `SRAMBUSY` reader - SRAMBUSY
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - SEDC
    #[inline(always)]
    pub fn sedc(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DED
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SRAMBUSY
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM1ISR")
            .field("sedc", &self.sedc())
            .field("ded", &self.ded())
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG RAMx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ram1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM1ISR)*/
pub struct RAM1ISRrs;
impl crate::RegisterSpec for RAM1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ram1isr::R`](R) reader structure
impl crate::Readable for RAM1ISRrs {}
///`reset()` method sets RAM1ISR to value 0
impl crate::Resettable for RAM1ISRrs {}
