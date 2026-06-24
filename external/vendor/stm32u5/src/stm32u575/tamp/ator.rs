///Register `ATOR` reader
pub type R = crate::R<ATORrs>;
///Field `PRNG` reader - Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode.
pub type PRNG_R = crate::FieldReader;
///Field `SEEDF` reader - Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set.
pub type SEEDF_R = crate::BitReader;
///Field `INITS` reader - Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled.
pub type INITS_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode.
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 14 - Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set.
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled.
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATOR")
            .field("prng", &self.prng())
            .field("seedf", &self.seedf())
            .field("inits", &self.inits())
            .finish()
    }
}
/**TAMP active tamper output register

You can [`read`](crate::Reg::read) this register and get [`ator::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ATOR)*/
pub struct ATORrs;
impl crate::RegisterSpec for ATORrs {
    type Ux = u32;
}
///`read()` method returns [`ator::R`](R) reader structure
impl crate::Readable for ATORrs {}
///`reset()` method sets ATOR to value 0
impl crate::Resettable for ATORrs {}
