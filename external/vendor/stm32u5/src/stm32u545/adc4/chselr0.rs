///Register `CHSELR0` reader
pub type R = crate::R<CHSELR0rs>;
///Register `CHSELR0` writer
pub type W = crate::W<CHSELR0rs>;
/**Channel-%s selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0 {
    ///0: Input channel x is not selected for conversion
    Disabled = 0,
    ///1: Input channel x is selected for conversion
    Enabled = 1,
}
impl From<CHSEL0> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL(0-23)` reader - Channel-%s selection
pub type CHSEL_R = crate::BitReader<CHSEL0>;
impl CHSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL0 {
        match self.bits {
            false => CHSEL0::Disabled,
            true => CHSEL0::Enabled,
        }
    }
    ///Input channel x is not selected for conversion
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHSEL0::Disabled
    }
    ///Input channel x is selected for conversion
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHSEL0::Enabled
    }
}
///Field `CHSEL(0-23)` writer - Channel-%s selection
pub type CHSEL_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL0>;
impl<'a, REG> CHSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel x is not selected for conversion
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::Disabled)
    }
    ///Input channel x is selected for conversion
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::Enabled)
    }
}
impl R {
    ///Channel-(0-23) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CHSEL0` field.</div>
    #[inline(always)]
    pub fn chsel(&self, n: u8) -> CHSEL_R {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        CHSEL_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel-(0-23) selection
    #[inline(always)]
    pub fn chsel_iter(&self) -> impl Iterator<Item = CHSEL_R> + '_ {
        (0..24).map(move |n| CHSEL_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Channel-0 selection
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel-1 selection
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel-2 selection
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel-3 selection
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel-4 selection
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel-5 selection
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel-6 selection
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel-7 selection
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel-8 selection
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-9 selection
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-10 selection
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-11 selection
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-12 selection
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-13 selection
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-14 selection
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-15 selection
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-16 selection
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-17 selection
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-18 selection
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-19 selection
    #[inline(always)]
    pub fn chsel19(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel-20 selection
    #[inline(always)]
    pub fn chsel20(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel-21 selection
    #[inline(always)]
    pub fn chsel21(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel-22 selection
    #[inline(always)]
    pub fn chsel22(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel-23 selection
    #[inline(always)]
    pub fn chsel23(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR0")
            .field("chsel0", &self.chsel0())
            .field("chsel1", &self.chsel1())
            .field("chsel2", &self.chsel2())
            .field("chsel3", &self.chsel3())
            .field("chsel4", &self.chsel4())
            .field("chsel5", &self.chsel5())
            .field("chsel6", &self.chsel6())
            .field("chsel7", &self.chsel7())
            .field("chsel8", &self.chsel8())
            .field("chsel9", &self.chsel9())
            .field("chsel10", &self.chsel10())
            .field("chsel11", &self.chsel11())
            .field("chsel12", &self.chsel12())
            .field("chsel13", &self.chsel13())
            .field("chsel14", &self.chsel14())
            .field("chsel15", &self.chsel15())
            .field("chsel16", &self.chsel16())
            .field("chsel17", &self.chsel17())
            .field("chsel18", &self.chsel18())
            .field("chsel19", &self.chsel19())
            .field("chsel20", &self.chsel20())
            .field("chsel21", &self.chsel21())
            .field("chsel22", &self.chsel22())
            .field("chsel23", &self.chsel23())
            .finish()
    }
}
impl W {
    ///Channel-(0-23) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CHSEL0` field.</div>
    #[inline(always)]
    pub fn chsel(&mut self, n: u8) -> CHSEL_W<CHSELR0rs> {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        CHSEL_W::new(self, n)
    }
    ///Bit 0 - Channel-0 selection
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 0)
    }
    ///Bit 1 - Channel-1 selection
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 1)
    }
    ///Bit 2 - Channel-2 selection
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 2)
    }
    ///Bit 3 - Channel-3 selection
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 3)
    }
    ///Bit 4 - Channel-4 selection
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 4)
    }
    ///Bit 5 - Channel-5 selection
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 5)
    }
    ///Bit 6 - Channel-6 selection
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 6)
    }
    ///Bit 7 - Channel-7 selection
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 7)
    }
    ///Bit 8 - Channel-8 selection
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 8)
    }
    ///Bit 9 - Channel-9 selection
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 9)
    }
    ///Bit 10 - Channel-10 selection
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 10)
    }
    ///Bit 11 - Channel-11 selection
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 11)
    }
    ///Bit 12 - Channel-12 selection
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 12)
    }
    ///Bit 13 - Channel-13 selection
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 13)
    }
    ///Bit 14 - Channel-14 selection
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 14)
    }
    ///Bit 15 - Channel-15 selection
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 15)
    }
    ///Bit 16 - Channel-16 selection
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 16)
    }
    ///Bit 17 - Channel-17 selection
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 17)
    }
    ///Bit 18 - Channel-18 selection
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 18)
    }
    ///Bit 19 - Channel-19 selection
    #[inline(always)]
    pub fn chsel19(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 19)
    }
    ///Bit 20 - Channel-20 selection
    #[inline(always)]
    pub fn chsel20(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 20)
    }
    ///Bit 21 - Channel-21 selection
    #[inline(always)]
    pub fn chsel21(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 21)
    }
    ///Bit 22 - Channel-22 selection
    #[inline(always)]
    pub fn chsel22(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 22)
    }
    ///Bit 23 - Channel-23 selection
    #[inline(always)]
    pub fn chsel23(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 23)
    }
}
/**ADC channel selection register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`chselr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC4:CHSELR0)*/
pub struct CHSELR0rs;
impl crate::RegisterSpec for CHSELR0rs {
    type Ux = u32;
}
///`read()` method returns [`chselr0::R`](R) reader structure
impl crate::Readable for CHSELR0rs {}
///`write(|w| ..)` method takes [`chselr0::W`](W) writer structure
impl crate::Writable for CHSELR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CHSELR0 to value 0
impl crate::Resettable for CHSELR0rs {}
