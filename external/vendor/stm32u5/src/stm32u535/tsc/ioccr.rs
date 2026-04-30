///Register `IOCCR` reader
pub type R = crate::R<IOCCRrs>;
///Register `IOCCR` writer
pub type W = crate::W<IOCCRrs>;
/**G%s_IO1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1_IO1 {
    ///0: Gx_IOy unused
    Disabled = 0,
    ///1: Gx_IOy used as channel
    Enabled = 1,
}
impl From<G1_IO1> for bool {
    #[inline(always)]
    fn from(variant: G1_IO1) -> Self {
        variant as u8 != 0
    }
}
///Field `G_IO1(1-8)` reader - G%s_IO1
pub type G_IO1_R = crate::BitReader<G1_IO1>;
impl G_IO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> G1_IO1 {
        match self.bits {
            false => G1_IO1::Disabled,
            true => G1_IO1::Enabled,
        }
    }
    ///Gx_IOy unused
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == G1_IO1::Disabled
    }
    ///Gx_IOy used as channel
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == G1_IO1::Enabled
    }
}
///Field `G_IO1(1-8)` writer - G%s_IO1
pub type G_IO1_W<'a, REG> = crate::BitWriter<'a, REG, G1_IO1>;
impl<'a, REG> G_IO1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Gx_IOy unused
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(G1_IO1::Disabled)
    }
    ///Gx_IOy used as channel
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(G1_IO1::Enabled)
    }
}
///Field `G_IO2(1-8)` reader - G%s_IO2
pub use G_IO1_R as G_IO2_R;
///Field `G_IO3(1-8)` reader - G%s_IO3
pub use G_IO1_R as G_IO3_R;
///Field `G_IO4(1-8)` reader - G%s_IO4
pub use G_IO1_R as G_IO4_R;
///Field `G_IO2(1-8)` writer - G%s_IO2
pub use G_IO1_W as G_IO2_W;
///Field `G_IO3(1-8)` writer - G%s_IO3
pub use G_IO1_W as G_IO3_W;
///Field `G_IO4(1-8)` writer - G%s_IO4
pub use G_IO1_W as G_IO4_W;
impl R {
    ///G(1-8)_IO1
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO1` field.</div>
    #[inline(always)]
    pub fn g_io1(&self, n: u8) -> G_IO1_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO1_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///G(1-8)_IO1
    #[inline(always)]
    pub fn g_io1_iter(&self) -> impl Iterator<Item = G_IO1_R> + '_ {
        (0..8).map(move |n| G_IO1_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    ///Bit 0 - G1_IO1
    #[inline(always)]
    pub fn g1_io1(&self) -> G_IO1_R {
        G_IO1_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - G2_IO1
    #[inline(always)]
    pub fn g2_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - G3_IO1
    #[inline(always)]
    pub fn g3_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - G4_IO1
    #[inline(always)]
    pub fn g4_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - G5_IO1
    #[inline(always)]
    pub fn g5_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - G6_IO1
    #[inline(always)]
    pub fn g6_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - G7_IO1
    #[inline(always)]
    pub fn g7_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - G8_IO1
    #[inline(always)]
    pub fn g8_io1(&self) -> G_IO1_R {
        G_IO1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///G(1-8)_IO2
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO2` field.</div>
    #[inline(always)]
    pub fn g_io2(&self, n: u8) -> G_IO2_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO2_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///G(1-8)_IO2
    #[inline(always)]
    pub fn g_io2_iter(&self) -> impl Iterator<Item = G_IO2_R> + '_ {
        (0..8).map(move |n| G_IO2_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    ///Bit 1 - G1_IO2
    #[inline(always)]
    pub fn g1_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - G2_IO2
    #[inline(always)]
    pub fn g2_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - G3_IO2
    #[inline(always)]
    pub fn g3_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - G4_IO2
    #[inline(always)]
    pub fn g4_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - G5_IO2
    #[inline(always)]
    pub fn g5_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - G6_IO2
    #[inline(always)]
    pub fn g6_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - G7_IO2
    #[inline(always)]
    pub fn g7_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 29 - G8_IO2
    #[inline(always)]
    pub fn g8_io2(&self) -> G_IO2_R {
        G_IO2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///G(1-8)_IO3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO3` field.</div>
    #[inline(always)]
    pub fn g_io3(&self, n: u8) -> G_IO3_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO3_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///G(1-8)_IO3
    #[inline(always)]
    pub fn g_io3_iter(&self) -> impl Iterator<Item = G_IO3_R> + '_ {
        (0..8).map(move |n| G_IO3_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    ///Bit 2 - G1_IO3
    #[inline(always)]
    pub fn g1_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - G2_IO3
    #[inline(always)]
    pub fn g2_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - G3_IO3
    #[inline(always)]
    pub fn g3_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - G4_IO3
    #[inline(always)]
    pub fn g4_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - G5_IO3
    #[inline(always)]
    pub fn g5_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - G6_IO3
    #[inline(always)]
    pub fn g6_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 26 - G7_IO3
    #[inline(always)]
    pub fn g7_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 30 - G8_IO3
    #[inline(always)]
    pub fn g8_io3(&self) -> G_IO3_R {
        G_IO3_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///G(1-8)_IO4
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO4` field.</div>
    #[inline(always)]
    pub fn g_io4(&self, n: u8) -> G_IO4_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO4_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///G(1-8)_IO4
    #[inline(always)]
    pub fn g_io4_iter(&self) -> impl Iterator<Item = G_IO4_R> + '_ {
        (0..8).map(move |n| G_IO4_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    ///Bit 3 - G1_IO4
    #[inline(always)]
    pub fn g1_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - G2_IO4
    #[inline(always)]
    pub fn g2_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - G3_IO4
    #[inline(always)]
    pub fn g3_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - G4_IO4
    #[inline(always)]
    pub fn g4_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - G5_IO4
    #[inline(always)]
    pub fn g5_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - G6_IO4
    #[inline(always)]
    pub fn g6_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - G7_IO4
    #[inline(always)]
    pub fn g7_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - G8_IO4
    #[inline(always)]
    pub fn g8_io4(&self) -> G_IO4_R {
        G_IO4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCCR")
            .field("g1_io1", &self.g1_io1())
            .field("g2_io1", &self.g2_io1())
            .field("g3_io1", &self.g3_io1())
            .field("g4_io1", &self.g4_io1())
            .field("g5_io1", &self.g5_io1())
            .field("g6_io1", &self.g6_io1())
            .field("g7_io1", &self.g7_io1())
            .field("g8_io1", &self.g8_io1())
            .field("g1_io4", &self.g1_io4())
            .field("g2_io4", &self.g2_io4())
            .field("g3_io4", &self.g3_io4())
            .field("g4_io4", &self.g4_io4())
            .field("g5_io4", &self.g5_io4())
            .field("g6_io4", &self.g6_io4())
            .field("g7_io4", &self.g7_io4())
            .field("g8_io4", &self.g8_io4())
            .field("g1_io3", &self.g1_io3())
            .field("g2_io3", &self.g2_io3())
            .field("g3_io3", &self.g3_io3())
            .field("g4_io3", &self.g4_io3())
            .field("g5_io3", &self.g5_io3())
            .field("g6_io3", &self.g6_io3())
            .field("g7_io3", &self.g7_io3())
            .field("g8_io3", &self.g8_io3())
            .field("g1_io2", &self.g1_io2())
            .field("g2_io2", &self.g2_io2())
            .field("g3_io2", &self.g3_io2())
            .field("g4_io2", &self.g4_io2())
            .field("g5_io2", &self.g5_io2())
            .field("g6_io2", &self.g6_io2())
            .field("g7_io2", &self.g7_io2())
            .field("g8_io2", &self.g8_io2())
            .finish()
    }
}
impl W {
    ///G(1-8)_IO1
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO1` field.</div>
    #[inline(always)]
    pub fn g_io1(&mut self, n: u8) -> G_IO1_W<IOCCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO1_W::new(self, n * 4)
    }
    ///Bit 0 - G1_IO1
    #[inline(always)]
    pub fn g1_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 0)
    }
    ///Bit 4 - G2_IO1
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 4)
    }
    ///Bit 8 - G3_IO1
    #[inline(always)]
    pub fn g3_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 8)
    }
    ///Bit 12 - G4_IO1
    #[inline(always)]
    pub fn g4_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 12)
    }
    ///Bit 16 - G5_IO1
    #[inline(always)]
    pub fn g5_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 16)
    }
    ///Bit 20 - G6_IO1
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 20)
    }
    ///Bit 24 - G7_IO1
    #[inline(always)]
    pub fn g7_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 24)
    }
    ///Bit 28 - G8_IO1
    #[inline(always)]
    pub fn g8_io1(&mut self) -> G_IO1_W<IOCCRrs> {
        G_IO1_W::new(self, 28)
    }
    ///G(1-8)_IO2
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO2` field.</div>
    #[inline(always)]
    pub fn g_io2(&mut self, n: u8) -> G_IO2_W<IOCCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO2_W::new(self, n * 4 + 1)
    }
    ///Bit 1 - G1_IO2
    #[inline(always)]
    pub fn g1_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 1)
    }
    ///Bit 5 - G2_IO2
    #[inline(always)]
    pub fn g2_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 5)
    }
    ///Bit 9 - G3_IO2
    #[inline(always)]
    pub fn g3_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 9)
    }
    ///Bit 13 - G4_IO2
    #[inline(always)]
    pub fn g4_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 13)
    }
    ///Bit 17 - G5_IO2
    #[inline(always)]
    pub fn g5_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 17)
    }
    ///Bit 21 - G6_IO2
    #[inline(always)]
    pub fn g6_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 21)
    }
    ///Bit 25 - G7_IO2
    #[inline(always)]
    pub fn g7_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 25)
    }
    ///Bit 29 - G8_IO2
    #[inline(always)]
    pub fn g8_io2(&mut self) -> G_IO2_W<IOCCRrs> {
        G_IO2_W::new(self, 29)
    }
    ///G(1-8)_IO3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO3` field.</div>
    #[inline(always)]
    pub fn g_io3(&mut self, n: u8) -> G_IO3_W<IOCCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO3_W::new(self, n * 4 + 2)
    }
    ///Bit 2 - G1_IO3
    #[inline(always)]
    pub fn g1_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 2)
    }
    ///Bit 6 - G2_IO3
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 6)
    }
    ///Bit 10 - G3_IO3
    #[inline(always)]
    pub fn g3_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 10)
    }
    ///Bit 14 - G4_IO3
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 14)
    }
    ///Bit 18 - G5_IO3
    #[inline(always)]
    pub fn g5_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 18)
    }
    ///Bit 22 - G6_IO3
    #[inline(always)]
    pub fn g6_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 22)
    }
    ///Bit 26 - G7_IO3
    #[inline(always)]
    pub fn g7_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 26)
    }
    ///Bit 30 - G8_IO3
    #[inline(always)]
    pub fn g8_io3(&mut self) -> G_IO3_W<IOCCRrs> {
        G_IO3_W::new(self, 30)
    }
    ///G(1-8)_IO4
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1_IO4` field.</div>
    #[inline(always)]
    pub fn g_io4(&mut self, n: u8) -> G_IO4_W<IOCCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        G_IO4_W::new(self, n * 4 + 3)
    }
    ///Bit 3 - G1_IO4
    #[inline(always)]
    pub fn g1_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 3)
    }
    ///Bit 7 - G2_IO4
    #[inline(always)]
    pub fn g2_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 7)
    }
    ///Bit 11 - G3_IO4
    #[inline(always)]
    pub fn g3_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 11)
    }
    ///Bit 15 - G4_IO4
    #[inline(always)]
    pub fn g4_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 15)
    }
    ///Bit 19 - G5_IO4
    #[inline(always)]
    pub fn g5_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 19)
    }
    ///Bit 23 - G6_IO4
    #[inline(always)]
    pub fn g6_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 23)
    }
    ///Bit 27 - G7_IO4
    #[inline(always)]
    pub fn g7_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 27)
    }
    ///Bit 31 - G8_IO4
    #[inline(always)]
    pub fn g8_io4(&mut self) -> G_IO4_W<IOCCRrs> {
        G_IO4_W::new(self, 31)
    }
}
/**I/O channel control register

You can [`read`](crate::Reg::read) this register and get [`ioccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:IOCCR)*/
pub struct IOCCRrs;
impl crate::RegisterSpec for IOCCRrs {
    type Ux = u32;
}
///`read()` method returns [`ioccr::R`](R) reader structure
impl crate::Readable for IOCCRrs {}
///`write(|w| ..)` method takes [`ioccr::W`](W) writer structure
impl crate::Writable for IOCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCCR to value 0
impl crate::Resettable for IOCCRrs {}
