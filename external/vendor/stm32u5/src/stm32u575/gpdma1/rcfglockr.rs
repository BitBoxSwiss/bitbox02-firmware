///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `LOCK(0-15)` reader - LOCK%s
pub type LOCK_R = crate::BitReader;
///Field `LOCK(0-15)` writer - LOCK%s
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///LOCK(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&self, n: u8) -> LOCK_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///LOCK(0-15)
    #[inline(always)]
    pub fn lock_iter(&self) -> impl Iterator<Item = LOCK_R> + '_ {
        (0..16).map(move |n| LOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - LOCK0
    #[inline(always)]
    pub fn lock0(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LOCK1
    #[inline(always)]
    pub fn lock1(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LOCK2
    #[inline(always)]
    pub fn lock2(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LOCK3
    #[inline(always)]
    pub fn lock3(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LOCK4
    #[inline(always)]
    pub fn lock4(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LOCK5
    #[inline(always)]
    pub fn lock5(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LOCK6
    #[inline(always)]
    pub fn lock6(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LOCK7
    #[inline(always)]
    pub fn lock7(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LOCK8
    #[inline(always)]
    pub fn lock8(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LOCK9
    #[inline(always)]
    pub fn lock9(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LOCK10
    #[inline(always)]
    pub fn lock10(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LOCK11
    #[inline(always)]
    pub fn lock11(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LOCK12
    #[inline(always)]
    pub fn lock12(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LOCK13
    #[inline(always)]
    pub fn lock13(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - LOCK14
    #[inline(always)]
    pub fn lock14(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LOCK15
    #[inline(always)]
    pub fn lock15(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCFGLOCKR")
            .field("lock0", &self.lock0())
            .field("lock1", &self.lock1())
            .field("lock2", &self.lock2())
            .field("lock3", &self.lock3())
            .field("lock4", &self.lock4())
            .field("lock5", &self.lock5())
            .field("lock6", &self.lock6())
            .field("lock7", &self.lock7())
            .field("lock8", &self.lock8())
            .field("lock9", &self.lock9())
            .field("lock10", &self.lock10())
            .field("lock11", &self.lock11())
            .field("lock12", &self.lock12())
            .field("lock13", &self.lock13())
            .field("lock14", &self.lock14())
            .field("lock15", &self.lock15())
            .finish()
    }
}
impl W {
    ///LOCK(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&mut self, n: u8) -> LOCK_W<RCFGLOCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LOCK_W::new(self, n)
    }
    ///Bit 0 - LOCK0
    #[inline(always)]
    pub fn lock0(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - LOCK1
    #[inline(always)]
    pub fn lock1(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 1)
    }
    ///Bit 2 - LOCK2
    #[inline(always)]
    pub fn lock2(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 2)
    }
    ///Bit 3 - LOCK3
    #[inline(always)]
    pub fn lock3(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 3)
    }
    ///Bit 4 - LOCK4
    #[inline(always)]
    pub fn lock4(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 4)
    }
    ///Bit 5 - LOCK5
    #[inline(always)]
    pub fn lock5(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 5)
    }
    ///Bit 6 - LOCK6
    #[inline(always)]
    pub fn lock6(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 6)
    }
    ///Bit 7 - LOCK7
    #[inline(always)]
    pub fn lock7(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 7)
    }
    ///Bit 8 - LOCK8
    #[inline(always)]
    pub fn lock8(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 9 - LOCK9
    #[inline(always)]
    pub fn lock9(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 9)
    }
    ///Bit 10 - LOCK10
    #[inline(always)]
    pub fn lock10(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 10)
    }
    ///Bit 11 - LOCK11
    #[inline(always)]
    pub fn lock11(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 11)
    }
    ///Bit 12 - LOCK12
    #[inline(always)]
    pub fn lock12(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 12)
    }
    ///Bit 13 - LOCK13
    #[inline(always)]
    pub fn lock13(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 13)
    }
    ///Bit 14 - LOCK14
    #[inline(always)]
    pub fn lock14(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 14)
    }
    ///Bit 15 - LOCK15
    #[inline(always)]
    pub fn lock15(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 15)
    }
}
/**GPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GPDMA1:RCFGLOCKR)*/
pub struct RCFGLOCKRrs;
impl crate::RegisterSpec for RCFGLOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`rcfglockr::R`](R) reader structure
impl crate::Readable for RCFGLOCKRrs {}
///`write(|w| ..)` method takes [`rcfglockr::W`](W) writer structure
impl crate::Writable for RCFGLOCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCFGLOCKR to value 0
impl crate::Resettable for RCFGLOCKRrs {}
