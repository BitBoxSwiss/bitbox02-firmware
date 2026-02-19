///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
/**Differential mode for channel %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL0 {
    ///0: ADC analog input channel x is configured in single-ended mode
    SingleEnded = 0,
    ///1: ADC analog input channel x is configured in differential mode
    Differential = 1,
}
impl From<DIFSEL0> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `DIFSEL(0-19)` reader - Differential mode for channel %s
pub type DIFSEL_R = crate::BitReader<DIFSEL0>;
impl DIFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIFSEL0 {
        match self.bits {
            false => DIFSEL0::SingleEnded,
            true => DIFSEL0::Differential,
        }
    }
    ///ADC analog input channel x is configured in single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL0::SingleEnded
    }
    ///ADC analog input channel x is configured in differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL0::Differential
    }
}
///Field `DIFSEL(0-19)` writer - Differential mode for channel %s
pub type DIFSEL_W<'a, REG> = crate::BitWriter<'a, REG, DIFSEL0>;
impl<'a, REG> DIFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog input channel x is configured in single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL0::SingleEnded)
    }
    ///ADC analog input channel x is configured in differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL0::Differential)
    }
}
impl R {
    ///Differential mode for channel (0-19)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DIFSEL0` field.</div>
    #[inline(always)]
    pub fn difsel(&self, n: u8) -> DIFSEL_R {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        DIFSEL_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Differential mode for channel (0-19)
    #[inline(always)]
    pub fn difsel_iter(&self) -> impl Iterator<Item = DIFSEL_R> + '_ {
        (0..20).map(move |n| DIFSEL_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Differential mode for channel 0
    #[inline(always)]
    pub fn difsel0(&self) -> DIFSEL_R {
        DIFSEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Differential mode for channel 1
    #[inline(always)]
    pub fn difsel1(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Differential mode for channel 2
    #[inline(always)]
    pub fn difsel2(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Differential mode for channel 3
    #[inline(always)]
    pub fn difsel3(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Differential mode for channel 4
    #[inline(always)]
    pub fn difsel4(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Differential mode for channel 5
    #[inline(always)]
    pub fn difsel5(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Differential mode for channel 6
    #[inline(always)]
    pub fn difsel6(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Differential mode for channel 7
    #[inline(always)]
    pub fn difsel7(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Differential mode for channel 8
    #[inline(always)]
    pub fn difsel8(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Differential mode for channel 9
    #[inline(always)]
    pub fn difsel9(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Differential mode for channel 10
    #[inline(always)]
    pub fn difsel10(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Differential mode for channel 11
    #[inline(always)]
    pub fn difsel11(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Differential mode for channel 12
    #[inline(always)]
    pub fn difsel12(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Differential mode for channel 13
    #[inline(always)]
    pub fn difsel13(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Differential mode for channel 14
    #[inline(always)]
    pub fn difsel14(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Differential mode for channel 15
    #[inline(always)]
    pub fn difsel15(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Differential mode for channel 16
    #[inline(always)]
    pub fn difsel16(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Differential mode for channel 17
    #[inline(always)]
    pub fn difsel17(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Differential mode for channel 18
    #[inline(always)]
    pub fn difsel18(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Differential mode for channel 19
    #[inline(always)]
    pub fn difsel19(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel0", &self.difsel0())
            .field("difsel1", &self.difsel1())
            .field("difsel2", &self.difsel2())
            .field("difsel3", &self.difsel3())
            .field("difsel4", &self.difsel4())
            .field("difsel5", &self.difsel5())
            .field("difsel6", &self.difsel6())
            .field("difsel7", &self.difsel7())
            .field("difsel8", &self.difsel8())
            .field("difsel9", &self.difsel9())
            .field("difsel10", &self.difsel10())
            .field("difsel11", &self.difsel11())
            .field("difsel12", &self.difsel12())
            .field("difsel13", &self.difsel13())
            .field("difsel14", &self.difsel14())
            .field("difsel15", &self.difsel15())
            .field("difsel16", &self.difsel16())
            .field("difsel17", &self.difsel17())
            .field("difsel18", &self.difsel18())
            .field("difsel19", &self.difsel19())
            .finish()
    }
}
impl W {
    ///Differential mode for channel (0-19)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DIFSEL0` field.</div>
    #[inline(always)]
    pub fn difsel(&mut self, n: u8) -> DIFSEL_W<DIFSELrs> {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        DIFSEL_W::new(self, n)
    }
    ///Bit 0 - Differential mode for channel 0
    #[inline(always)]
    pub fn difsel0(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 0)
    }
    ///Bit 1 - Differential mode for channel 1
    #[inline(always)]
    pub fn difsel1(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 1)
    }
    ///Bit 2 - Differential mode for channel 2
    #[inline(always)]
    pub fn difsel2(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 2)
    }
    ///Bit 3 - Differential mode for channel 3
    #[inline(always)]
    pub fn difsel3(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 3)
    }
    ///Bit 4 - Differential mode for channel 4
    #[inline(always)]
    pub fn difsel4(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 4)
    }
    ///Bit 5 - Differential mode for channel 5
    #[inline(always)]
    pub fn difsel5(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 5)
    }
    ///Bit 6 - Differential mode for channel 6
    #[inline(always)]
    pub fn difsel6(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 6)
    }
    ///Bit 7 - Differential mode for channel 7
    #[inline(always)]
    pub fn difsel7(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 7)
    }
    ///Bit 8 - Differential mode for channel 8
    #[inline(always)]
    pub fn difsel8(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 8)
    }
    ///Bit 9 - Differential mode for channel 9
    #[inline(always)]
    pub fn difsel9(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 9)
    }
    ///Bit 10 - Differential mode for channel 10
    #[inline(always)]
    pub fn difsel10(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 10)
    }
    ///Bit 11 - Differential mode for channel 11
    #[inline(always)]
    pub fn difsel11(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 11)
    }
    ///Bit 12 - Differential mode for channel 12
    #[inline(always)]
    pub fn difsel12(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 12)
    }
    ///Bit 13 - Differential mode for channel 13
    #[inline(always)]
    pub fn difsel13(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 13)
    }
    ///Bit 14 - Differential mode for channel 14
    #[inline(always)]
    pub fn difsel14(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 14)
    }
    ///Bit 15 - Differential mode for channel 15
    #[inline(always)]
    pub fn difsel15(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 15)
    }
    ///Bit 16 - Differential mode for channel 16
    #[inline(always)]
    pub fn difsel16(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 16)
    }
    ///Bit 17 - Differential mode for channel 17
    #[inline(always)]
    pub fn difsel17(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 17)
    }
    ///Bit 18 - Differential mode for channel 18
    #[inline(always)]
    pub fn difsel18(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 18)
    }
    ///Bit 19 - Differential mode for channel 19
    #[inline(always)]
    pub fn difsel19(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 19)
    }
}
/**ADC differential mode selection register

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:DIFSEL)*/
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
///`read()` method returns [`difsel::R`](R) reader structure
impl crate::Readable for DIFSELrs {}
///`write(|w| ..)` method takes [`difsel::W`](W) writer structure
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIFSEL to value 0
impl crate::Resettable for DIFSELrs {}
