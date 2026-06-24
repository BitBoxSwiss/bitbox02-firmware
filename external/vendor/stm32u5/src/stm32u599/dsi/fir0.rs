///Register `FIR0` writer
pub type W = crate::W<FIR0rs>;
///Field `FAE0` writer - Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0.
pub type FAE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE1` writer - Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1.
pub type FAE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE2` writer - Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2.
pub type FAE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE3` writer - Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3.
pub type FAE3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE4` writer - Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4.
pub type FAE4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE5` writer - Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5.
pub type FAE5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE6` writer - Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6.
pub type FAE6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE7` writer - Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7.
pub type FAE7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE8` writer - Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8.
pub type FAE8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE9` writer - Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9.
pub type FAE9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE10` writer - Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10.
pub type FAE10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE11` writer - Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11.
pub type FAE11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE12` writer - Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12.
pub type FAE12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE13` writer - Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13.
pub type FAE13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE14` writer - Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14.
pub type FAE14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAE15` writer - Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15.
pub type FAE15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE0` writer - Force PHY error 0 Writing one to this bit forces a PHY error 0.
pub type FPE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE1` writer - Force PHY error 1 Writing one to this bit forces a PHY error 1.
pub type FPE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE2` writer - Force PHY error 2 Writing one to this bit forces a PHY error 2.
pub type FPE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE3` writer - Force PHY error 3 Writing one to this bit forces a PHY error 3.
pub type FPE3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE4` writer - Force PHY error 4 Writing one to this bit forces a PHY error 4.
pub type FPE4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FIR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0.
    #[inline(always)]
    pub fn fae0(&mut self) -> FAE0_W<FIR0rs> {
        FAE0_W::new(self, 0)
    }
    ///Bit 1 - Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1.
    #[inline(always)]
    pub fn fae1(&mut self) -> FAE1_W<FIR0rs> {
        FAE1_W::new(self, 1)
    }
    ///Bit 2 - Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2.
    #[inline(always)]
    pub fn fae2(&mut self) -> FAE2_W<FIR0rs> {
        FAE2_W::new(self, 2)
    }
    ///Bit 3 - Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3.
    #[inline(always)]
    pub fn fae3(&mut self) -> FAE3_W<FIR0rs> {
        FAE3_W::new(self, 3)
    }
    ///Bit 4 - Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4.
    #[inline(always)]
    pub fn fae4(&mut self) -> FAE4_W<FIR0rs> {
        FAE4_W::new(self, 4)
    }
    ///Bit 5 - Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5.
    #[inline(always)]
    pub fn fae5(&mut self) -> FAE5_W<FIR0rs> {
        FAE5_W::new(self, 5)
    }
    ///Bit 6 - Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6.
    #[inline(always)]
    pub fn fae6(&mut self) -> FAE6_W<FIR0rs> {
        FAE6_W::new(self, 6)
    }
    ///Bit 7 - Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7.
    #[inline(always)]
    pub fn fae7(&mut self) -> FAE7_W<FIR0rs> {
        FAE7_W::new(self, 7)
    }
    ///Bit 8 - Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8.
    #[inline(always)]
    pub fn fae8(&mut self) -> FAE8_W<FIR0rs> {
        FAE8_W::new(self, 8)
    }
    ///Bit 9 - Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9.
    #[inline(always)]
    pub fn fae9(&mut self) -> FAE9_W<FIR0rs> {
        FAE9_W::new(self, 9)
    }
    ///Bit 10 - Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10.
    #[inline(always)]
    pub fn fae10(&mut self) -> FAE10_W<FIR0rs> {
        FAE10_W::new(self, 10)
    }
    ///Bit 11 - Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11.
    #[inline(always)]
    pub fn fae11(&mut self) -> FAE11_W<FIR0rs> {
        FAE11_W::new(self, 11)
    }
    ///Bit 12 - Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12.
    #[inline(always)]
    pub fn fae12(&mut self) -> FAE12_W<FIR0rs> {
        FAE12_W::new(self, 12)
    }
    ///Bit 13 - Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13.
    #[inline(always)]
    pub fn fae13(&mut self) -> FAE13_W<FIR0rs> {
        FAE13_W::new(self, 13)
    }
    ///Bit 14 - Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14.
    #[inline(always)]
    pub fn fae14(&mut self) -> FAE14_W<FIR0rs> {
        FAE14_W::new(self, 14)
    }
    ///Bit 15 - Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15.
    #[inline(always)]
    pub fn fae15(&mut self) -> FAE15_W<FIR0rs> {
        FAE15_W::new(self, 15)
    }
    ///Bit 16 - Force PHY error 0 Writing one to this bit forces a PHY error 0.
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE0_W<FIR0rs> {
        FPE0_W::new(self, 16)
    }
    ///Bit 17 - Force PHY error 1 Writing one to this bit forces a PHY error 1.
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE1_W<FIR0rs> {
        FPE1_W::new(self, 17)
    }
    ///Bit 18 - Force PHY error 2 Writing one to this bit forces a PHY error 2.
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE2_W<FIR0rs> {
        FPE2_W::new(self, 18)
    }
    ///Bit 19 - Force PHY error 3 Writing one to this bit forces a PHY error 3.
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE3_W<FIR0rs> {
        FPE3_W::new(self, 19)
    }
    ///Bit 20 - Force PHY error 4 Writing one to this bit forces a PHY error 4.
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE4_W<FIR0rs> {
        FPE4_W::new(self, 20)
    }
}
/**DSI Host force interrupt register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:FIR0)*/
pub struct FIR0rs;
impl crate::RegisterSpec for FIR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fir0::W`](W) writer structure
impl crate::Writable for FIR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIR0 to value 0
impl crate::Resettable for FIR0rs {}
