///Register `OTYPER` reader
pub type R = crate::R<OTYPERrs>;
///Register `OTYPER` writer
pub type W = crate::W<OTYPERrs>;
/**Port x configuration pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_TYPE {
    ///0: Output push-pull (reset state)
    PushPull = 0,
    ///1: Output open-drain
    OpenDrain = 1,
}
impl From<OUTPUT_TYPE> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_TYPE) -> Self {
        variant as u8 != 0
    }
}
///Field `OT(0-15)` reader - Port x configuration pin %s
pub type OT_R = crate::BitReader<OUTPUT_TYPE>;
impl OT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTPUT_TYPE {
        match self.bits {
            false => OUTPUT_TYPE::PushPull,
            true => OUTPUT_TYPE::OpenDrain,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OUTPUT_TYPE::PushPull
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OUTPUT_TYPE::OpenDrain
    }
}
///Field `OT(0-15)` writer - Port x configuration pin %s
pub type OT_W<'a, REG> = crate::BitWriter<'a, REG, OUTPUT_TYPE>;
impl<'a, REG> OT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_TYPE::PushPull)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_TYPE::OpenDrain)
    }
}
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OT0` field.</div>
    #[inline(always)]
    pub fn ot(&self, n: u8) -> OT_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OT_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn ot_iter(&self) -> impl Iterator<Item = OT_R> + '_ {
        (0..16).map(move |n| OT_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port x configuration pin 0
    #[inline(always)]
    pub fn ot0(&self) -> OT_R {
        OT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x configuration pin 1
    #[inline(always)]
    pub fn ot1(&self) -> OT_R {
        OT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x configuration pin 2
    #[inline(always)]
    pub fn ot2(&self) -> OT_R {
        OT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x configuration pin 3
    #[inline(always)]
    pub fn ot3(&self) -> OT_R {
        OT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x configuration pin 4
    #[inline(always)]
    pub fn ot4(&self) -> OT_R {
        OT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x configuration pin 5
    #[inline(always)]
    pub fn ot5(&self) -> OT_R {
        OT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x configuration pin 6
    #[inline(always)]
    pub fn ot6(&self) -> OT_R {
        OT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x configuration pin 7
    #[inline(always)]
    pub fn ot7(&self) -> OT_R {
        OT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x configuration pin 8
    #[inline(always)]
    pub fn ot8(&self) -> OT_R {
        OT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x configuration pin 9
    #[inline(always)]
    pub fn ot9(&self) -> OT_R {
        OT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x configuration pin 10
    #[inline(always)]
    pub fn ot10(&self) -> OT_R {
        OT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x configuration pin 11
    #[inline(always)]
    pub fn ot11(&self) -> OT_R {
        OT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x configuration pin 12
    #[inline(always)]
    pub fn ot12(&self) -> OT_R {
        OT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x configuration pin 13
    #[inline(always)]
    pub fn ot13(&self) -> OT_R {
        OT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x configuration pin 14
    #[inline(always)]
    pub fn ot14(&self) -> OT_R {
        OT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x configuration pin 15
    #[inline(always)]
    pub fn ot15(&self) -> OT_R {
        OT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTYPER")
            .field("ot0", &self.ot0())
            .field("ot1", &self.ot1())
            .field("ot2", &self.ot2())
            .field("ot3", &self.ot3())
            .field("ot4", &self.ot4())
            .field("ot5", &self.ot5())
            .field("ot6", &self.ot6())
            .field("ot7", &self.ot7())
            .field("ot8", &self.ot8())
            .field("ot9", &self.ot9())
            .field("ot10", &self.ot10())
            .field("ot11", &self.ot11())
            .field("ot12", &self.ot12())
            .field("ot13", &self.ot13())
            .field("ot14", &self.ot14())
            .field("ot15", &self.ot15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OT0` field.</div>
    #[inline(always)]
    pub fn ot(&mut self, n: u8) -> OT_W<OTYPERrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OT_W::new(self, n)
    }
    ///Bit 0 - Port x configuration pin 0
    #[inline(always)]
    pub fn ot0(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 0)
    }
    ///Bit 1 - Port x configuration pin 1
    #[inline(always)]
    pub fn ot1(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 1)
    }
    ///Bit 2 - Port x configuration pin 2
    #[inline(always)]
    pub fn ot2(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 2)
    }
    ///Bit 3 - Port x configuration pin 3
    #[inline(always)]
    pub fn ot3(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 3)
    }
    ///Bit 4 - Port x configuration pin 4
    #[inline(always)]
    pub fn ot4(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 4)
    }
    ///Bit 5 - Port x configuration pin 5
    #[inline(always)]
    pub fn ot5(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 5)
    }
    ///Bit 6 - Port x configuration pin 6
    #[inline(always)]
    pub fn ot6(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 6)
    }
    ///Bit 7 - Port x configuration pin 7
    #[inline(always)]
    pub fn ot7(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 7)
    }
    ///Bit 8 - Port x configuration pin 8
    #[inline(always)]
    pub fn ot8(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 8)
    }
    ///Bit 9 - Port x configuration pin 9
    #[inline(always)]
    pub fn ot9(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 9)
    }
    ///Bit 10 - Port x configuration pin 10
    #[inline(always)]
    pub fn ot10(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 10)
    }
    ///Bit 11 - Port x configuration pin 11
    #[inline(always)]
    pub fn ot11(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 11)
    }
    ///Bit 12 - Port x configuration pin 12
    #[inline(always)]
    pub fn ot12(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 12)
    }
    ///Bit 13 - Port x configuration pin 13
    #[inline(always)]
    pub fn ot13(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 13)
    }
    ///Bit 14 - Port x configuration pin 14
    #[inline(always)]
    pub fn ot14(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 14)
    }
    ///Bit 15 - Port x configuration pin 15
    #[inline(always)]
    pub fn ot15(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 15)
    }
}
/**GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GPIOA:OTYPER)*/
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
///`read()` method returns [`otyper::R`](R) reader structure
impl crate::Readable for OTYPERrs {}
///`write(|w| ..)` method takes [`otyper::W`](W) writer structure
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPERrs {}
