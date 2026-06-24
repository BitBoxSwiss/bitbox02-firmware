///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
///Field `BS(0-15)` writer - BS%s
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR(0-15)` writer - BR%s
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///BS(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BS0` field.</div>
    #[inline(always)]
    pub fn bs(&mut self, n: u8) -> BS_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BS_W::new(self, n)
    }
    ///Bit 0 - BS0
    #[inline(always)]
    pub fn bs0(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 0)
    }
    ///Bit 1 - BS1
    #[inline(always)]
    pub fn bs1(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 1)
    }
    ///Bit 2 - BS2
    #[inline(always)]
    pub fn bs2(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 2)
    }
    ///Bit 3 - BS3
    #[inline(always)]
    pub fn bs3(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 3)
    }
    ///Bit 4 - BS4
    #[inline(always)]
    pub fn bs4(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 4)
    }
    ///Bit 5 - BS5
    #[inline(always)]
    pub fn bs5(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 5)
    }
    ///Bit 6 - BS6
    #[inline(always)]
    pub fn bs6(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 6)
    }
    ///Bit 7 - BS7
    #[inline(always)]
    pub fn bs7(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 7)
    }
    ///Bit 8 - BS8
    #[inline(always)]
    pub fn bs8(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 8)
    }
    ///Bit 9 - BS9
    #[inline(always)]
    pub fn bs9(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 9)
    }
    ///Bit 10 - BS10
    #[inline(always)]
    pub fn bs10(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 10)
    }
    ///Bit 11 - BS11
    #[inline(always)]
    pub fn bs11(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 11)
    }
    ///Bit 12 - BS12
    #[inline(always)]
    pub fn bs12(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 12)
    }
    ///Bit 13 - BS13
    #[inline(always)]
    pub fn bs13(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 13)
    }
    ///Bit 14 - BS14
    #[inline(always)]
    pub fn bs14(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 14)
    }
    ///Bit 15 - BS15
    #[inline(always)]
    pub fn bs15(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 15)
    }
    ///BR(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n + 16)
    }
    ///Bit 16 - BR0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 16)
    }
    ///Bit 17 - BR1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 17)
    }
    ///Bit 18 - BR2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 18)
    }
    ///Bit 19 - BR3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 19)
    }
    ///Bit 20 - BR4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 20)
    }
    ///Bit 21 - BR5
    #[inline(always)]
    pub fn br5(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 21)
    }
    ///Bit 22 - BR6
    #[inline(always)]
    pub fn br6(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 22)
    }
    ///Bit 23 - BR7
    #[inline(always)]
    pub fn br7(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 23)
    }
    ///Bit 24 - BR8
    #[inline(always)]
    pub fn br8(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 24)
    }
    ///Bit 25 - BR9
    #[inline(always)]
    pub fn br9(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 25)
    }
    ///Bit 26 - BR10
    #[inline(always)]
    pub fn br10(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 26)
    }
    ///Bit 27 - BR11
    #[inline(always)]
    pub fn br11(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 27)
    }
    ///Bit 28 - BR12
    #[inline(always)]
    pub fn br12(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 28)
    }
    ///Bit 29 - BR13
    #[inline(always)]
    pub fn br13(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 29)
    }
    ///Bit 30 - BR14
    #[inline(always)]
    pub fn br14(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 30)
    }
    ///Bit 31 - BR15
    #[inline(always)]
    pub fn br15(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 31)
    }
}
/**LPGPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#LPGPIO1:BSRR)*/
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
