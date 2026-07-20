///Register `AWD2CR` reader
pub type R = crate::R<AWD2CRrs>;
///Register `AWD2CR` writer
pub type W = crate::W<AWD2CRrs>;
/**AWD2CH

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH {
    ///0: ADC analog input channel x is not monitored by AWDy
    Disabled = 0,
    ///1: ADC analog input channel x is monitored by AWDy
    Enabled = 1,
}
impl From<AWD2CH> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH(0-19)` reader - AWD2CH
pub type AWD2CH_R = crate::BitReader<AWD2CH>;
impl AWD2CH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH {
        match self.bits {
            false => AWD2CH::Disabled,
            true => AWD2CH::Enabled,
        }
    }
    ///ADC analog input channel x is not monitored by AWDy
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD2CH::Disabled
    }
    ///ADC analog input channel x is monitored by AWDy
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD2CH::Enabled
    }
}
///Field `AWD2CH(0-19)` writer - AWD2CH
pub type AWD2CH_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH>;
impl<'a, REG> AWD2CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog input channel x is not monitored by AWDy
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH::Disabled)
    }
    ///ADC analog input channel x is monitored by AWDy
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH::Enabled)
    }
}
impl R {
    ///AWD2CH
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD2CH0` field.</div>
    #[inline(always)]
    pub fn awd2ch(&self, n: u8) -> AWD2CH_R {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        AWD2CH_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///AWD2CH
    #[inline(always)]
    pub fn awd2ch_iter(&self) -> impl Iterator<Item = AWD2CH_R> + '_ {
        (0..20).map(move |n| AWD2CH_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - AWD2CH
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH_R {
        AWD2CH_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AWD2CH
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AWD2CH
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AWD2CH
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AWD2CH
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AWD2CH
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AWD2CH
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AWD2CH
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD2CH
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD2CH
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AWD2CH
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AWD2CH
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AWD2CH
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AWD2CH
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AWD2CH
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AWD2CH
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AWD2CH
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AWD2CH
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AWD2CH
    #[inline(always)]
    pub fn awd2ch18(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AWD2CH
    #[inline(always)]
    pub fn awd2ch19(&self) -> AWD2CH_R {
        AWD2CH_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2CR")
            .field("awd2ch0", &self.awd2ch0())
            .field("awd2ch1", &self.awd2ch1())
            .field("awd2ch2", &self.awd2ch2())
            .field("awd2ch3", &self.awd2ch3())
            .field("awd2ch4", &self.awd2ch4())
            .field("awd2ch5", &self.awd2ch5())
            .field("awd2ch6", &self.awd2ch6())
            .field("awd2ch7", &self.awd2ch7())
            .field("awd2ch8", &self.awd2ch8())
            .field("awd2ch9", &self.awd2ch9())
            .field("awd2ch10", &self.awd2ch10())
            .field("awd2ch11", &self.awd2ch11())
            .field("awd2ch12", &self.awd2ch12())
            .field("awd2ch13", &self.awd2ch13())
            .field("awd2ch14", &self.awd2ch14())
            .field("awd2ch15", &self.awd2ch15())
            .field("awd2ch16", &self.awd2ch16())
            .field("awd2ch17", &self.awd2ch17())
            .field("awd2ch18", &self.awd2ch18())
            .field("awd2ch19", &self.awd2ch19())
            .finish()
    }
}
impl W {
    ///AWD2CH
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD2CH0` field.</div>
    #[inline(always)]
    pub fn awd2ch(&mut self, n: u8) -> AWD2CH_W<AWD2CRrs> {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        AWD2CH_W::new(self, n)
    }
    ///Bit 0 - AWD2CH
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 0)
    }
    ///Bit 1 - AWD2CH
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 1)
    }
    ///Bit 2 - AWD2CH
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 2)
    }
    ///Bit 3 - AWD2CH
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 3)
    }
    ///Bit 4 - AWD2CH
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 4)
    }
    ///Bit 5 - AWD2CH
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 5)
    }
    ///Bit 6 - AWD2CH
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 6)
    }
    ///Bit 7 - AWD2CH
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 7)
    }
    ///Bit 8 - AWD2CH
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 8)
    }
    ///Bit 9 - AWD2CH
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 9)
    }
    ///Bit 10 - AWD2CH
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 10)
    }
    ///Bit 11 - AWD2CH
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 11)
    }
    ///Bit 12 - AWD2CH
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 12)
    }
    ///Bit 13 - AWD2CH
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 13)
    }
    ///Bit 14 - AWD2CH
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 14)
    }
    ///Bit 15 - AWD2CH
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 15)
    }
    ///Bit 16 - AWD2CH
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 16)
    }
    ///Bit 17 - AWD2CH
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 17)
    }
    ///Bit 18 - AWD2CH
    #[inline(always)]
    pub fn awd2ch18(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 18)
    }
    ///Bit 19 - AWD2CH
    #[inline(always)]
    pub fn awd2ch19(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 19)
    }
}
/**ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC1:AWD2CR)*/
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2cr::R`](R) reader structure
impl crate::Readable for AWD2CRrs {}
///`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD2CR to value 0
impl crate::Resettable for AWD2CRrs {}
