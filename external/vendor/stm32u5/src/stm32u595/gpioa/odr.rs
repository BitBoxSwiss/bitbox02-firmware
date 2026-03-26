///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
/**Port output data pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_DATA {
    ///0: Set output to logic low
    Low = 0,
    ///1: Set output to logic high
    High = 1,
}
impl From<OUTPUT_DATA> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_DATA) -> Self {
        variant as u8 != 0
    }
}
///Field `OD(0-15)` reader - Port output data pin %s
pub type OD_R = crate::BitReader<OUTPUT_DATA>;
impl OD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTPUT_DATA {
        match self.bits {
            false => OUTPUT_DATA::Low,
            true => OUTPUT_DATA::High,
        }
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTPUT_DATA::Low
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTPUT_DATA::High
    }
}
///Field `OD(0-15)` writer - Port output data pin %s
pub type OD_W<'a, REG> = crate::BitWriter<'a, REG, OUTPUT_DATA>;
impl<'a, REG> OD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_DATA::Low)
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_DATA::High)
    }
}
impl R {
    ///Port output data pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OD0` field.</div>
    #[inline(always)]
    pub fn od(&self, n: u8) -> OD_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OD_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port output data pin (0-15)
    #[inline(always)]
    pub fn od_iter(&self) -> impl Iterator<Item = OD_R> + '_ {
        (0..16).map(move |n| OD_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port output data pin 0
    #[inline(always)]
    pub fn od0(&self) -> OD_R {
        OD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port output data pin 1
    #[inline(always)]
    pub fn od1(&self) -> OD_R {
        OD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port output data pin 2
    #[inline(always)]
    pub fn od2(&self) -> OD_R {
        OD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port output data pin 3
    #[inline(always)]
    pub fn od3(&self) -> OD_R {
        OD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port output data pin 4
    #[inline(always)]
    pub fn od4(&self) -> OD_R {
        OD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port output data pin 5
    #[inline(always)]
    pub fn od5(&self) -> OD_R {
        OD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port output data pin 6
    #[inline(always)]
    pub fn od6(&self) -> OD_R {
        OD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port output data pin 7
    #[inline(always)]
    pub fn od7(&self) -> OD_R {
        OD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port output data pin 8
    #[inline(always)]
    pub fn od8(&self) -> OD_R {
        OD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port output data pin 9
    #[inline(always)]
    pub fn od9(&self) -> OD_R {
        OD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port output data pin 10
    #[inline(always)]
    pub fn od10(&self) -> OD_R {
        OD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port output data pin 11
    #[inline(always)]
    pub fn od11(&self) -> OD_R {
        OD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port output data pin 12
    #[inline(always)]
    pub fn od12(&self) -> OD_R {
        OD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port output data pin 13
    #[inline(always)]
    pub fn od13(&self) -> OD_R {
        OD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port output data pin 14
    #[inline(always)]
    pub fn od14(&self) -> OD_R {
        OD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port output data pin 15
    #[inline(always)]
    pub fn od15(&self) -> OD_R {
        OD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("od0", &self.od0())
            .field("od1", &self.od1())
            .field("od2", &self.od2())
            .field("od3", &self.od3())
            .field("od4", &self.od4())
            .field("od5", &self.od5())
            .field("od6", &self.od6())
            .field("od7", &self.od7())
            .field("od8", &self.od8())
            .field("od9", &self.od9())
            .field("od10", &self.od10())
            .field("od11", &self.od11())
            .field("od12", &self.od12())
            .field("od13", &self.od13())
            .field("od14", &self.od14())
            .field("od15", &self.od15())
            .finish()
    }
}
impl W {
    ///Port output data pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OD0` field.</div>
    #[inline(always)]
    pub fn od(&mut self, n: u8) -> OD_W<ODRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OD_W::new(self, n)
    }
    ///Bit 0 - Port output data pin 0
    #[inline(always)]
    pub fn od0(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 0)
    }
    ///Bit 1 - Port output data pin 1
    #[inline(always)]
    pub fn od1(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 1)
    }
    ///Bit 2 - Port output data pin 2
    #[inline(always)]
    pub fn od2(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 2)
    }
    ///Bit 3 - Port output data pin 3
    #[inline(always)]
    pub fn od3(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 3)
    }
    ///Bit 4 - Port output data pin 4
    #[inline(always)]
    pub fn od4(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 4)
    }
    ///Bit 5 - Port output data pin 5
    #[inline(always)]
    pub fn od5(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 5)
    }
    ///Bit 6 - Port output data pin 6
    #[inline(always)]
    pub fn od6(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 6)
    }
    ///Bit 7 - Port output data pin 7
    #[inline(always)]
    pub fn od7(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 7)
    }
    ///Bit 8 - Port output data pin 8
    #[inline(always)]
    pub fn od8(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 8)
    }
    ///Bit 9 - Port output data pin 9
    #[inline(always)]
    pub fn od9(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 9)
    }
    ///Bit 10 - Port output data pin 10
    #[inline(always)]
    pub fn od10(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 10)
    }
    ///Bit 11 - Port output data pin 11
    #[inline(always)]
    pub fn od11(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 11)
    }
    ///Bit 12 - Port output data pin 12
    #[inline(always)]
    pub fn od12(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 12)
    }
    ///Bit 13 - Port output data pin 13
    #[inline(always)]
    pub fn od13(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 13)
    }
    ///Bit 14 - Port output data pin 14
    #[inline(always)]
    pub fn od14(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 14)
    }
    ///Bit 15 - Port output data pin 15
    #[inline(always)]
    pub fn od15(&mut self) -> OD_W<ODRrs> {
        OD_W::new(self, 15)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPIOA:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {}
