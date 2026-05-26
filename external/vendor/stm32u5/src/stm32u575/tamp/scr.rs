///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CTAMP1F` writer - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register.
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP2F` writer - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register.
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP3F` writer - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register.
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP4F` writer - Clear TAMP4 detection flag Writing 1 in this bit clears the TAMP4F bit in the TAMP_SR register.
pub type CTAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP5F` writer - Clear TAMP5 detection flag Writing 1 in this bit clears the TAMP5F bit in the TAMP_SR register.
pub type CTAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP6F` writer - Clear TAMP6 detection flag Writing 1 in this bit clears the TAMP6F bit in the TAMP_SR register.
pub type CTAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP7F` writer - Clear TAMP7 detection flag Writing 1 in this bit clears the TAMP7F bit in the TAMP_SR register.
pub type CTAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP8F` writer - Clear TAMP8 detection flag Writing 1 in this bit clears the TAMP8F bit in the TAMP_SR register.
pub type CTAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP1F` writer - Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register.
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP2F` writer - Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register.
pub type CITAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP3F` writer - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register.
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP5F` writer - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register.
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP6F` writer - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register.
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP7F` writer - Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register.
pub type CITAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP8F` writer - Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register.
pub type CITAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP9F` writer - Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register.
pub type CITAMP9F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP11F` writer - Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register.
pub type CITAMP11F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP12F` writer - Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register.
pub type CITAMP12F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP13F` writer - Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register.
pub type CITAMP13F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    ///Bit 1 - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    ///Bit 2 - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    ///Bit 3 - Clear TAMP4 detection flag Writing 1 in this bit clears the TAMP4F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<SCRrs> {
        CTAMP4F_W::new(self, 3)
    }
    ///Bit 4 - Clear TAMP5 detection flag Writing 1 in this bit clears the TAMP5F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<SCRrs> {
        CTAMP5F_W::new(self, 4)
    }
    ///Bit 5 - Clear TAMP6 detection flag Writing 1 in this bit clears the TAMP6F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W<SCRrs> {
        CTAMP6F_W::new(self, 5)
    }
    ///Bit 6 - Clear TAMP7 detection flag Writing 1 in this bit clears the TAMP7F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp7f(&mut self) -> CTAMP7F_W<SCRrs> {
        CTAMP7F_W::new(self, 6)
    }
    ///Bit 7 - Clear TAMP8 detection flag Writing 1 in this bit clears the TAMP8F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp8f(&mut self) -> CTAMP8F_W<SCRrs> {
        CTAMP8F_W::new(self, 7)
    }
    ///Bit 16 - Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    ///Bit 17 - Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<SCRrs> {
        CITAMP2F_W::new(self, 17)
    }
    ///Bit 18 - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    ///Bit 20 - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    ///Bit 21 - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<SCRrs> {
        CITAMP6F_W::new(self, 21)
    }
    ///Bit 22 - Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp7f(&mut self) -> CITAMP7F_W<SCRrs> {
        CITAMP7F_W::new(self, 22)
    }
    ///Bit 23 - Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<SCRrs> {
        CITAMP8F_W::new(self, 23)
    }
    ///Bit 24 - Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp9f(&mut self) -> CITAMP9F_W<SCRrs> {
        CITAMP9F_W::new(self, 24)
    }
    ///Bit 26 - Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp11f(&mut self) -> CITAMP11F_W<SCRrs> {
        CITAMP11F_W::new(self, 26)
    }
    ///Bit 27 - Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp12f(&mut self) -> CITAMP12F_W<SCRrs> {
        CITAMP12F_W::new(self, 27)
    }
    ///Bit 28 - Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp13f(&mut self) -> CITAMP13F_W<SCRrs> {
        CITAMP13F_W::new(self, 28)
    }
}
/**TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
