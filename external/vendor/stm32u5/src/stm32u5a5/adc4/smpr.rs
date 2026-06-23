///Register `SMPR` reader
pub type R = crate::R<SMPRrs>;
///Register `SMPR` writer
pub type W = crate::W<SMPRrs>;
/**Sampling time selection %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1 {
    ///0: 1.5 ADC clock cycles
    Cycles1_5 = 0,
    ///1: 3.5 ADC clock cycles
    Cycles3_5 = 1,
    ///2: 7.5 ADC clock cycles
    Cycles7_5 = 2,
    ///3: 12.5 ADC clock cycles
    Cycles12_5 = 3,
    ///4: 19.5 ADC clock cycles
    Cycles19_5 = 4,
    ///5: 39.5 ADC clock cycles
    Cycles39_5 = 5,
    ///6: 79.5 ADC clock cycles
    Cycles79_5 = 6,
    ///7: 814.5 ADC clock cycles
    Cycles814_5 = 7,
}
impl From<SMP1> for u8 {
    #[inline(always)]
    fn from(variant: SMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP1 {
    type Ux = u8;
}
impl crate::IsEnum for SMP1 {}
///Field `SMP(1-2)` reader - Sampling time selection %s
pub type SMP_R = crate::FieldReader<SMP1>;
impl SMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP1 {
        match self.bits {
            0 => SMP1::Cycles1_5,
            1 => SMP1::Cycles3_5,
            2 => SMP1::Cycles7_5,
            3 => SMP1::Cycles12_5,
            4 => SMP1::Cycles19_5,
            5 => SMP1::Cycles39_5,
            6 => SMP1::Cycles79_5,
            7 => SMP1::Cycles814_5,
            _ => unreachable!(),
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP1::Cycles1_5
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        *self == SMP1::Cycles3_5
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP1::Cycles7_5
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP1::Cycles12_5
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP1::Cycles19_5
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        *self == SMP1::Cycles39_5
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        *self == SMP1::Cycles79_5
    }
    ///814.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles814_5(&self) -> bool {
        *self == SMP1::Cycles814_5
    }
}
///Field `SMP(1-2)` writer - Sampling time selection %s
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP1, crate::Safe>;
impl<'a, REG> SMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles1_5)
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles3_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles7_5)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles12_5)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles19_5)
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles39_5)
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles79_5)
    }
    ///814.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles814_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles814_5)
    }
}
/**Channel-%s sampling time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0 {
    ///0: Sampling time of channel x uses the setting of SMP1 register.
    Smp1 = 0,
    ///1: Sampling time of channel x uses the setting of SMP2 register.
    Smp2 = 1,
}
impl From<SMPSEL0> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL(0-23)` reader - Channel-%s sampling time selection
pub type SMPSEL_R = crate::BitReader<SMPSEL0>;
impl SMPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL0 {
        match self.bits {
            false => SMPSEL0::Smp1,
            true => SMPSEL0::Smp2,
        }
    }
    ///Sampling time of channel x uses the setting of SMP1 register.
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        *self == SMPSEL0::Smp1
    }
    ///Sampling time of channel x uses the setting of SMP2 register.
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        *self == SMPSEL0::Smp2
    }
}
///Field `SMPSEL(0-23)` writer - Channel-%s sampling time selection
pub type SMPSEL_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL0>;
impl<'a, REG> SMPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of channel x uses the setting of SMP1 register.
    #[inline(always)]
    pub fn smp1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp1)
    }
    ///Sampling time of channel x uses the setting of SMP2 register.
    #[inline(always)]
    pub fn smp2(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp2)
    }
}
impl R {
    ///Sampling time selection (1-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP1` field.</div>
    #[inline(always)]
    pub fn smp(&self, n: u8) -> SMP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SMP_R::new(((self.bits >> (n * 4)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Sampling time selection (1-2)
    #[inline(always)]
    pub fn smp_iter(&self) -> impl Iterator<Item = SMP_R> + '_ {
        (0..2).map(move |n| SMP_R::new(((self.bits >> (n * 4)) & 7) as u8))
    }
    ///Bits 0:2 - Sampling time selection 1
    #[inline(always)]
    pub fn smp1(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Sampling time selection 2
    #[inline(always)]
    pub fn smp2(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Channel-(0-23) sampling time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMPSEL0` field.</div>
    #[inline(always)]
    pub fn smpsel(&self, n: u8) -> SMPSEL_R {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        SMPSEL_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel-(0-23) sampling time selection
    #[inline(always)]
    pub fn smpsel_iter(&self) -> impl Iterator<Item = SMPSEL_R> + '_ {
        (0..24).map(move |n| SMPSEL_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Channel-0 sampling time selection
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-1 sampling time selection
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-2 sampling time selection
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-3 sampling time selection
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-4 sampling time selection
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-5 sampling time selection
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-6 sampling time selection
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-7 sampling time selection
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-8 sampling time selection
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-9 sampling time selection
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-10 sampling time selection
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-11 sampling time selection
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel-12 sampling time selection
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel-13 sampling time selection
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel-14 sampling time selection
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel-15 sampling time selection
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel-16 sampling time selection
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Channel-17 sampling time selection
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Channel-18 sampling time selection
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Channel-19 sampling time selection
    #[inline(always)]
    pub fn smpsel19(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Channel-20 sampling time selection
    #[inline(always)]
    pub fn smpsel20(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Channel-21 sampling time selection
    #[inline(always)]
    pub fn smpsel21(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Channel-22 sampling time selection
    #[inline(always)]
    pub fn smpsel22(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Channel-23 sampling time selection
    #[inline(always)]
    pub fn smpsel23(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR")
            .field("smpsel0", &self.smpsel0())
            .field("smpsel1", &self.smpsel1())
            .field("smpsel2", &self.smpsel2())
            .field("smpsel3", &self.smpsel3())
            .field("smpsel4", &self.smpsel4())
            .field("smpsel5", &self.smpsel5())
            .field("smpsel6", &self.smpsel6())
            .field("smpsel7", &self.smpsel7())
            .field("smpsel8", &self.smpsel8())
            .field("smpsel9", &self.smpsel9())
            .field("smpsel10", &self.smpsel10())
            .field("smpsel11", &self.smpsel11())
            .field("smpsel12", &self.smpsel12())
            .field("smpsel13", &self.smpsel13())
            .field("smpsel14", &self.smpsel14())
            .field("smpsel15", &self.smpsel15())
            .field("smpsel16", &self.smpsel16())
            .field("smpsel17", &self.smpsel17())
            .field("smpsel18", &self.smpsel18())
            .field("smpsel19", &self.smpsel19())
            .field("smpsel20", &self.smpsel20())
            .field("smpsel21", &self.smpsel21())
            .field("smpsel22", &self.smpsel22())
            .field("smpsel23", &self.smpsel23())
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .finish()
    }
}
impl W {
    ///Sampling time selection (1-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP1` field.</div>
    #[inline(always)]
    pub fn smp(&mut self, n: u8) -> SMP_W<SMPRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SMP_W::new(self, n * 4)
    }
    ///Bits 0:2 - Sampling time selection 1
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP_W<SMPRrs> {
        SMP_W::new(self, 0)
    }
    ///Bits 4:6 - Sampling time selection 2
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP_W<SMPRrs> {
        SMP_W::new(self, 4)
    }
    ///Channel-(0-23) sampling time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMPSEL0` field.</div>
    #[inline(always)]
    pub fn smpsel(&mut self, n: u8) -> SMPSEL_W<SMPRrs> {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        SMPSEL_W::new(self, n + 8)
    }
    ///Bit 8 - Channel-0 sampling time selection
    #[inline(always)]
    pub fn smpsel0(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 8)
    }
    ///Bit 9 - Channel-1 sampling time selection
    #[inline(always)]
    pub fn smpsel1(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 9)
    }
    ///Bit 10 - Channel-2 sampling time selection
    #[inline(always)]
    pub fn smpsel2(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 10)
    }
    ///Bit 11 - Channel-3 sampling time selection
    #[inline(always)]
    pub fn smpsel3(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 11)
    }
    ///Bit 12 - Channel-4 sampling time selection
    #[inline(always)]
    pub fn smpsel4(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 12)
    }
    ///Bit 13 - Channel-5 sampling time selection
    #[inline(always)]
    pub fn smpsel5(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 13)
    }
    ///Bit 14 - Channel-6 sampling time selection
    #[inline(always)]
    pub fn smpsel6(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 14)
    }
    ///Bit 15 - Channel-7 sampling time selection
    #[inline(always)]
    pub fn smpsel7(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 15)
    }
    ///Bit 16 - Channel-8 sampling time selection
    #[inline(always)]
    pub fn smpsel8(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 16)
    }
    ///Bit 17 - Channel-9 sampling time selection
    #[inline(always)]
    pub fn smpsel9(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 17)
    }
    ///Bit 18 - Channel-10 sampling time selection
    #[inline(always)]
    pub fn smpsel10(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 18)
    }
    ///Bit 19 - Channel-11 sampling time selection
    #[inline(always)]
    pub fn smpsel11(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 19)
    }
    ///Bit 20 - Channel-12 sampling time selection
    #[inline(always)]
    pub fn smpsel12(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 20)
    }
    ///Bit 21 - Channel-13 sampling time selection
    #[inline(always)]
    pub fn smpsel13(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 21)
    }
    ///Bit 22 - Channel-14 sampling time selection
    #[inline(always)]
    pub fn smpsel14(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 22)
    }
    ///Bit 23 - Channel-15 sampling time selection
    #[inline(always)]
    pub fn smpsel15(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 23)
    }
    ///Bit 24 - Channel-16 sampling time selection
    #[inline(always)]
    pub fn smpsel16(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 24)
    }
    ///Bit 25 - Channel-17 sampling time selection
    #[inline(always)]
    pub fn smpsel17(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 25)
    }
    ///Bit 26 - Channel-18 sampling time selection
    #[inline(always)]
    pub fn smpsel18(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 26)
    }
    ///Bit 27 - Channel-19 sampling time selection
    #[inline(always)]
    pub fn smpsel19(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 27)
    }
    ///Bit 28 - Channel-20 sampling time selection
    #[inline(always)]
    pub fn smpsel20(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 28)
    }
    ///Bit 29 - Channel-21 sampling time selection
    #[inline(always)]
    pub fn smpsel21(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 29)
    }
    ///Bit 30 - Channel-22 sampling time selection
    #[inline(always)]
    pub fn smpsel22(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 30)
    }
    ///Bit 31 - Channel-23 sampling time selection
    #[inline(always)]
    pub fn smpsel23(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 31)
    }
}
/**ADC sample time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC4:SMPR)*/
pub struct SMPRrs;
impl crate::RegisterSpec for SMPRrs {
    type Ux = u32;
}
///`read()` method returns [`smpr::R`](R) reader structure
impl crate::Readable for SMPRrs {}
///`write(|w| ..)` method takes [`smpr::W`](W) writer structure
impl crate::Writable for SMPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR to value 0
impl crate::Resettable for SMPRrs {}
