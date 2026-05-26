///Register `LCKR` reader
pub type R = crate::R<LCKRrs>;
///Register `LCKR` writer
pub type W = crate::W<LCKRrs>;
/**Port x lock pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    ///0: Port configuration not locked
    Unlocked = 0,
    ///1: Port configuration locked
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK(0-15)` reader - Port x lock pin %s
pub type LCK_R = crate::BitReader<LOCK>;
impl LCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::Unlocked,
            true => LOCK::Locked,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK::Unlocked
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK::Locked
    }
}
///Field `LCK(0-15)` writer - Port x lock pin %s
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
/**Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the lock key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_KEY {
    ///0: Port configuration lock key not active
    NotActive = 0,
    ///1: Port configuration lock key active
    Active = 1,
}
impl From<LOCK_KEY> for bool {
    #[inline(always)]
    fn from(variant: LOCK_KEY) -> Self {
        variant as u8 != 0
    }
}
///Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the lock key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
pub type LCKK_R = crate::BitReader<LOCK_KEY>;
impl LCKK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_KEY {
        match self.bits {
            false => LOCK_KEY::NotActive,
            true => LOCK_KEY::Active,
        }
    }
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LOCK_KEY::NotActive
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LOCK_KEY::Active
    }
}
///Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the lock key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_KEY>;
impl<'a, REG> LCKK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_KEY::NotActive)
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_KEY::Active)
    }
}
impl R {
    ///Port x lock pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LCK0` field.</div>
    #[inline(always)]
    pub fn lck(&self, n: u8) -> LCK_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port x lock pin (0-15)
    #[inline(always)]
    pub fn lck_iter(&self) -> impl Iterator<Item = LCK_R> + '_ {
        (0..16).map(move |n| LCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port x lock pin 0
    #[inline(always)]
    pub fn lck0(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock pin 1
    #[inline(always)]
    pub fn lck1(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x lock pin 2
    #[inline(always)]
    pub fn lck2(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x lock pin 3
    #[inline(always)]
    pub fn lck3(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x lock pin 4
    #[inline(always)]
    pub fn lck4(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x lock pin 5
    #[inline(always)]
    pub fn lck5(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x lock pin 6
    #[inline(always)]
    pub fn lck6(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x lock pin 7
    #[inline(always)]
    pub fn lck7(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x lock pin 8
    #[inline(always)]
    pub fn lck8(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x lock pin 9
    #[inline(always)]
    pub fn lck9(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x lock pin 10
    #[inline(always)]
    pub fn lck10(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x lock pin 11
    #[inline(always)]
    pub fn lck11(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x lock pin 12
    #[inline(always)]
    pub fn lck12(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x lock pin 13
    #[inline(always)]
    pub fn lck13(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x lock pin 14
    #[inline(always)]
    pub fn lck14(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x lock pin 15
    #[inline(always)]
    pub fn lck15(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the lock key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCKR")
            .field("lck0", &self.lck0())
            .field("lck1", &self.lck1())
            .field("lck2", &self.lck2())
            .field("lck3", &self.lck3())
            .field("lck4", &self.lck4())
            .field("lck5", &self.lck5())
            .field("lck6", &self.lck6())
            .field("lck7", &self.lck7())
            .field("lck8", &self.lck8())
            .field("lck9", &self.lck9())
            .field("lck10", &self.lck10())
            .field("lck11", &self.lck11())
            .field("lck12", &self.lck12())
            .field("lck13", &self.lck13())
            .field("lck14", &self.lck14())
            .field("lck15", &self.lck15())
            .field("lckk", &self.lckk())
            .finish()
    }
}
impl W {
    ///Port x lock pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LCK0` field.</div>
    #[inline(always)]
    pub fn lck(&mut self, n: u8) -> LCK_W<LCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LCK_W::new(self, n)
    }
    ///Bit 0 - Port x lock pin 0
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 0)
    }
    ///Bit 1 - Port x lock pin 1
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 1)
    }
    ///Bit 2 - Port x lock pin 2
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 2)
    }
    ///Bit 3 - Port x lock pin 3
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 3)
    }
    ///Bit 4 - Port x lock pin 4
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 4)
    }
    ///Bit 5 - Port x lock pin 5
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 5)
    }
    ///Bit 6 - Port x lock pin 6
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 6)
    }
    ///Bit 7 - Port x lock pin 7
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 7)
    }
    ///Bit 8 - Port x lock pin 8
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 8)
    }
    ///Bit 9 - Port x lock pin 9
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 9)
    }
    ///Bit 10 - Port x lock pin 10
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 10)
    }
    ///Bit 11 - Port x lock pin 11
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 11)
    }
    ///Bit 12 - Port x lock pin 12
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 12)
    }
    ///Bit 13 - Port x lock pin 13
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 13)
    }
    ///Bit 14 - Port x lock pin 14
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 14)
    }
    ///Bit 15 - Port x lock pin 15
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 15)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the lock key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GPIOA:LCKR)*/
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
///`read()` method returns [`lckr::R`](R) reader structure
impl crate::Readable for LCKRrs {}
///`write(|w| ..)` method takes [`lckr::W`](W) writer structure
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKRrs {}
