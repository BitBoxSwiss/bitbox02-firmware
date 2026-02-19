///Register `ISR_input` reader
pub type R = crate::R<ISR_INPUTrs>;
///Field `CC1IF` reader - Compare 1 interrupt flag
pub type CC1IF_R = crate::BitReader;
///Field `ARRM` reader - Autoreload match
pub type ARRM_R = crate::BitReader;
///Field `EXTTRIG` reader - External trigger edge event
pub type EXTTRIG_R = crate::BitReader;
///Field `ARROK` reader - Autoreload register update OK
pub type ARROK_R = crate::BitReader;
///Field `UP` reader - Counter direction change down to up
pub type UP_R = crate::BitReader;
///Field `DOWN` reader - Counter direction change up to down
pub type DOWN_R = crate::BitReader;
///Field `UE` reader - LPTIM update event occurred
pub type UE_R = crate::BitReader;
///Field `REPOK` reader - Repetition register update Ok
pub type REPOK_R = crate::BitReader;
///Field `CC2IF` reader - Capture 2 interrupt flag
pub type CC2IF_R = crate::BitReader;
///Field `CC1OF` reader - Capture 1 over-capture flag
pub type CC1OF_R = crate::BitReader;
///Field `CC2OF` reader - Capture 2 over-capture flag
pub type CC2OF_R = crate::BitReader;
///Field `DIEROK` reader - Interrupt enable register update OK
pub type DIEROK_R = crate::BitReader;
impl R {
    ///Bit 0 - Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger edge event
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Counter direction change down to up
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter direction change up to down
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LPTIM update event occurred
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update Ok
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Capture 1 over-capture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture 2 over-capture flag
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 24 - Interrupt enable register update OK
    #[inline(always)]
    pub fn dierok(&self) -> DIEROK_R {
        DIEROK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR_input")
            .field("dierok", &self.dierok())
            .field("cc2of", &self.cc2of())
            .field("cc1of", &self.cc1of())
            .field("cc2if", &self.cc2if())
            .field("repok", &self.repok())
            .field("ue", &self.ue())
            .field("down", &self.down())
            .field("up", &self.up())
            .field("arrok", &self.arrok())
            .field("exttrig", &self.exttrig())
            .field("arrm", &self.arrm())
            .field("cc1if", &self.cc1if())
            .finish()
    }
}
/**Interrupt and Status Register (intput mode)

You can [`read`](crate::Reg::read) this register and get [`isr_input::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPTIM1:ISR_input)*/
pub struct ISR_INPUTrs;
impl crate::RegisterSpec for ISR_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`isr_input::R`](R) reader structure
impl crate::Readable for ISR_INPUTrs {}
///`reset()` method sets ISR_input to value 0
impl crate::Resettable for ISR_INPUTrs {}
