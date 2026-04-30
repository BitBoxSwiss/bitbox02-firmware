///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port x set pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_SET {
    ///1: Sets the corresponding ODx bit
    Set = 1,
}
impl From<BIT_SET> for bool {
    #[inline(always)]
    fn from(variant: BIT_SET) -> Self {
        variant as u8 != 0
    }
}
///Field `BS(0-15)` writer - Port x set pin %s
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG, BIT_SET>;
impl<'a, REG> BS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_SET::Set)
    }
}
/**Port x reset pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///1: Resets the corresponding ODx bit
    Reset = 1,
}
impl From<BIT_RESET> for bool {
    #[inline(always)]
    fn from(variant: BIT_RESET) -> Self {
        variant as u8 != 0
    }
}
///Field `BR(0-15)` writer - Port x reset pin %s
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Port x set pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BS0` field.</div>
    #[inline(always)]
    pub fn bs(&mut self, n: u8) -> BS_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BS_W::new(self, n)
    }
    ///Bit 0 - Port x set pin 0
    #[inline(always)]
    pub fn bs0(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 0)
    }
    ///Bit 1 - Port x set pin 1
    #[inline(always)]
    pub fn bs1(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 1)
    }
    ///Bit 2 - Port x set pin 2
    #[inline(always)]
    pub fn bs2(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 2)
    }
    ///Bit 3 - Port x set pin 3
    #[inline(always)]
    pub fn bs3(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 3)
    }
    ///Bit 4 - Port x set pin 4
    #[inline(always)]
    pub fn bs4(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 4)
    }
    ///Bit 5 - Port x set pin 5
    #[inline(always)]
    pub fn bs5(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 5)
    }
    ///Bit 6 - Port x set pin 6
    #[inline(always)]
    pub fn bs6(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 6)
    }
    ///Bit 7 - Port x set pin 7
    #[inline(always)]
    pub fn bs7(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 7)
    }
    ///Bit 8 - Port x set pin 8
    #[inline(always)]
    pub fn bs8(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 8)
    }
    ///Bit 9 - Port x set pin 9
    #[inline(always)]
    pub fn bs9(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 9)
    }
    ///Bit 10 - Port x set pin 10
    #[inline(always)]
    pub fn bs10(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 10)
    }
    ///Bit 11 - Port x set pin 11
    #[inline(always)]
    pub fn bs11(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 11)
    }
    ///Bit 12 - Port x set pin 12
    #[inline(always)]
    pub fn bs12(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 12)
    }
    ///Bit 13 - Port x set pin 13
    #[inline(always)]
    pub fn bs13(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 13)
    }
    ///Bit 14 - Port x set pin 14
    #[inline(always)]
    pub fn bs14(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 14)
    }
    ///Bit 15 - Port x set pin 15
    #[inline(always)]
    pub fn bs15(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 15)
    }
    ///Port x reset pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n + 16)
    }
    ///Bit 16 - Port x reset pin 0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 16)
    }
    ///Bit 17 - Port x reset pin 1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 17)
    }
    ///Bit 18 - Port x reset pin 2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 18)
    }
    ///Bit 19 - Port x reset pin 3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 19)
    }
    ///Bit 20 - Port x reset pin 4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 20)
    }
    ///Bit 21 - Port x reset pin 5
    #[inline(always)]
    pub fn br5(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 21)
    }
    ///Bit 22 - Port x reset pin 6
    #[inline(always)]
    pub fn br6(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 22)
    }
    ///Bit 23 - Port x reset pin 7
    #[inline(always)]
    pub fn br7(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 23)
    }
    ///Bit 24 - Port x reset pin 8
    #[inline(always)]
    pub fn br8(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 24)
    }
    ///Bit 25 - Port x reset pin 9
    #[inline(always)]
    pub fn br9(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 25)
    }
    ///Bit 26 - Port x reset pin 10
    #[inline(always)]
    pub fn br10(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 26)
    }
    ///Bit 27 - Port x reset pin 11
    #[inline(always)]
    pub fn br11(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 27)
    }
    ///Bit 28 - Port x reset pin 12
    #[inline(always)]
    pub fn br12(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 28)
    }
    ///Bit 29 - Port x reset pin 13
    #[inline(always)]
    pub fn br13(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 29)
    }
    ///Bit 30 - Port x reset pin 14
    #[inline(always)]
    pub fn br14(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 30)
    }
    ///Bit 31 - Port x reset pin 15
    #[inline(always)]
    pub fn br15(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 31)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPIOA:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {}
