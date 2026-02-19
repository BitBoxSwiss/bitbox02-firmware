///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
/**Port x configuration pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PULL {
    ///0: No pull-up, pull-down
    Floating = 0,
    ///1: Pull-up
    PullUp = 1,
    ///2: Pull-down
    PullDown = 2,
}
impl From<PULL> for u8 {
    #[inline(always)]
    fn from(variant: PULL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PULL {
    type Ux = u8;
}
impl crate::IsEnum for PULL {}
///Field `PUPD(0-15)` reader - Port x configuration pin %s
pub type PUPD_R = crate::FieldReader<PULL>;
impl PUPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PULL> {
        match self.bits {
            0 => Some(PULL::Floating),
            1 => Some(PULL::PullUp),
            2 => Some(PULL::PullDown),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PULL::Floating
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PULL::PullUp
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PULL::PullDown
    }
}
///Field `PUPD(0-15)` writer - Port x configuration pin %s
pub type PUPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PULL>;
impl<'a, REG> PUPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PULL::Floating)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PULL::PullUp)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PULL::PullDown)
    }
}
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PUPD0` field.</div>
    #[inline(always)]
    pub fn pupd(&self, n: u8) -> PUPD_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PUPD_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn pupd_iter(&self) -> impl Iterator<Item = PUPD_R> + '_ {
        (0..16).map(move |n| PUPD_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD_R {
        PUPD_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupd0", &self.pupd0())
            .field("pupd1", &self.pupd1())
            .field("pupd2", &self.pupd2())
            .field("pupd3", &self.pupd3())
            .field("pupd4", &self.pupd4())
            .field("pupd5", &self.pupd5())
            .field("pupd6", &self.pupd6())
            .field("pupd7", &self.pupd7())
            .field("pupd8", &self.pupd8())
            .field("pupd9", &self.pupd9())
            .field("pupd10", &self.pupd10())
            .field("pupd11", &self.pupd11())
            .field("pupd12", &self.pupd12())
            .field("pupd13", &self.pupd13())
            .field("pupd14", &self.pupd14())
            .field("pupd15", &self.pupd15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PUPD0` field.</div>
    #[inline(always)]
    pub fn pupd(&mut self, n: u8) -> PUPD_W<PUPDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PUPD_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn pupd0(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn pupd1(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn pupd2(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn pupd3(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn pupd4(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn pupd5(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn pupd6(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn pupd7(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn pupd8(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn pupd9(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn pupd10(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn pupd11(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn pupd12(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD_W<PUPDRrs> {
        PUPD_W::new(self, 30)
    }
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#GPIOA:PUPDR)*/
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`pupdr::R`](R) reader structure
impl crate::Readable for PUPDRrs {}
///`write(|w| ..)` method takes [`pupdr::W`](W) writer structure
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUPDR to value 0x6400_0000
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0x6400_0000;
}
