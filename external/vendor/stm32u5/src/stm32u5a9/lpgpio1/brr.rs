///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `BR(0-15)` writer - BR%s
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///BR(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<BRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n)
    }
    ///Bit 0 - BR0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 0)
    }
    ///Bit 1 - BR1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 1)
    }
    ///Bit 2 - BR2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 2)
    }
    ///Bit 3 - BR3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 3)
    }
    ///Bit 4 - BR4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 4)
    }
    ///Bit 5 - BR5
    #[inline(always)]
    pub fn br5(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 5)
    }
    ///Bit 6 - BR6
    #[inline(always)]
    pub fn br6(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 6)
    }
    ///Bit 7 - BR7
    #[inline(always)]
    pub fn br7(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 7)
    }
    ///Bit 8 - BR8
    #[inline(always)]
    pub fn br8(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 8)
    }
    ///Bit 9 - BR9
    #[inline(always)]
    pub fn br9(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 9)
    }
    ///Bit 10 - BR10
    #[inline(always)]
    pub fn br10(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 10)
    }
    ///Bit 11 - BR11
    #[inline(always)]
    pub fn br11(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 11)
    }
    ///Bit 12 - BR12
    #[inline(always)]
    pub fn br12(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 12)
    }
    ///Bit 13 - BR13
    #[inline(always)]
    pub fn br13(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 13)
    }
    ///Bit 14 - BR14
    #[inline(always)]
    pub fn br14(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 14)
    }
    ///Bit 15 - BR15
    #[inline(always)]
    pub fn br15(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 15)
    }
}
/**LPGPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPGPIO1:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
