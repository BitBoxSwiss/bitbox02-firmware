///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `LOCK(0-3)` reader - LOCK%s
pub type LOCK_R = crate::BitReader;
///Field `LOCK(0-3)` writer - LOCK%s
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///LOCK(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&self, n: u8) -> LOCK_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        LOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///LOCK(0-3)
    #[inline(always)]
    pub fn lock_iter(&self) -> impl Iterator<Item = LOCK_R> + '_ {
        (0..4).map(move |n| LOCK_R::new(((self.bits >> n) & 1) != 0))
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCFGLOCKR")
            .field("lock0", &self.lock0())
            .field("lock1", &self.lock1())
            .field("lock2", &self.lock2())
            .field("lock3", &self.lock3())
            .finish()
    }
}
impl W {
    ///LOCK(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&mut self, n: u8) -> LOCK_W<RCFGLOCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
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
}
/**LPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPDMA1:RCFGLOCKR)*/
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
