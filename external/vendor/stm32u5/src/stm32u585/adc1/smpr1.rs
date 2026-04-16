///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
/**Channel %s sample time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0 {
    ///0: 5 ADC clock cycles
    Cycles5 = 0,
    ///1: 6 ADC clock cycles
    Cycles6 = 1,
    ///2: 12 ADC clock cycles
    Cycles12 = 2,
    ///3: 20 ADC clock cycles
    Cycles20 = 3,
    ///4: 36 ADC clock cycles
    Cycles36 = 4,
    ///5: 68 ADC clock cycles
    Cycles68 = 5,
    ///6: 391 ADC clock cycles
    Cycles391 = 6,
    ///7: 814 ADC clock cycles
    Cycles814 = 7,
}
impl From<SMP0> for u8 {
    #[inline(always)]
    fn from(variant: SMP0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP0 {
    type Ux = u8;
}
impl crate::IsEnum for SMP0 {}
///Field `SMP(0-9)` reader - Channel %s sample time selection
pub type SMP_R = crate::FieldReader<SMP0>;
impl SMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP0 {
        match self.bits {
            0 => SMP0::Cycles5,
            1 => SMP0::Cycles6,
            2 => SMP0::Cycles12,
            3 => SMP0::Cycles20,
            4 => SMP0::Cycles36,
            5 => SMP0::Cycles68,
            6 => SMP0::Cycles391,
            7 => SMP0::Cycles814,
            _ => unreachable!(),
        }
    }
    ///5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles5(&self) -> bool {
        *self == SMP0::Cycles5
    }
    ///6 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles6(&self) -> bool {
        *self == SMP0::Cycles6
    }
    ///12 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles12(&self) -> bool {
        *self == SMP0::Cycles12
    }
    ///20 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles20(&self) -> bool {
        *self == SMP0::Cycles20
    }
    ///36 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles36(&self) -> bool {
        *self == SMP0::Cycles36
    }
    ///68 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles68(&self) -> bool {
        *self == SMP0::Cycles68
    }
    ///391 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles391(&self) -> bool {
        *self == SMP0::Cycles391
    }
    ///814 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles814(&self) -> bool {
        *self == SMP0::Cycles814
    }
}
///Field `SMP(0-9)` writer - Channel %s sample time selection
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP0, crate::Safe>;
impl<'a, REG> SMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///5 ADC clock cycles
    #[inline(always)]
    pub fn cycles5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles5)
    }
    ///6 ADC clock cycles
    #[inline(always)]
    pub fn cycles6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles6)
    }
    ///12 ADC clock cycles
    #[inline(always)]
    pub fn cycles12(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles12)
    }
    ///20 ADC clock cycles
    #[inline(always)]
    pub fn cycles20(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles20)
    }
    ///36 ADC clock cycles
    #[inline(always)]
    pub fn cycles36(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles36)
    }
    ///68 ADC clock cycles
    #[inline(always)]
    pub fn cycles68(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles68)
    }
    ///391 ADC clock cycles
    #[inline(always)]
    pub fn cycles391(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles391)
    }
    ///814 ADC clock cycles
    #[inline(always)]
    pub fn cycles814(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles814)
    }
}
impl R {
    ///Channel (0-9) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP0` field.</div>
    #[inline(always)]
    pub fn smp(&self, n: u8) -> SMP_R {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Channel (0-9) sample time selection
    #[inline(always)]
    pub fn smp_iter(&self) -> impl Iterator<Item = SMP_R> + '_ {
        (0..10).map(move |n| SMP_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    ///Bits 0:2 - Channel 0 sample time selection
    #[inline(always)]
    pub fn smp0(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 1 sample time selection
    #[inline(always)]
    pub fn smp1(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 2 sample time selection
    #[inline(always)]
    pub fn smp2(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 3 sample time selection
    #[inline(always)]
    pub fn smp3(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 4 sample time selection
    #[inline(always)]
    pub fn smp4(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 5 sample time selection
    #[inline(always)]
    pub fn smp5(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 6 sample time selection
    #[inline(always)]
    pub fn smp6(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 7 sample time selection
    #[inline(always)]
    pub fn smp7(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 8 sample time selection
    #[inline(always)]
    pub fn smp8(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 9 sample time selection
    #[inline(always)]
    pub fn smp9(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp0", &self.smp0())
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smp3", &self.smp3())
            .field("smp4", &self.smp4())
            .field("smp5", &self.smp5())
            .field("smp6", &self.smp6())
            .field("smp7", &self.smp7())
            .field("smp8", &self.smp8())
            .field("smp9", &self.smp9())
            .finish()
    }
}
impl W {
    ///Channel (0-9) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP0` field.</div>
    #[inline(always)]
    pub fn smp(&mut self, n: u8) -> SMP_W<SMPR1rs> {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_W::new(self, n * 3)
    }
    ///Bits 0:2 - Channel 0 sample time selection
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 1 sample time selection
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 2 sample time selection
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 3 sample time selection
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 4 sample time selection
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 5 sample time selection
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 6 sample time selection
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 7 sample time selection
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 8 sample time selection
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 9 sample time selection
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP_W<SMPR1rs> {
        SMP_W::new(self, 27)
    }
}
/**ADC sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC1:SMPR1)*/
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
///`read()` method returns [`smpr1::R`](R) reader structure
impl crate::Readable for SMPR1rs {}
///`write(|w| ..)` method takes [`smpr1::W`](W) writer structure
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1rs {}
