///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `TAMP1E` reader - TAMP1E
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1E` writer - TAMP1E
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2E` reader - TAMP2E
pub type TAMP2E_R = crate::BitReader;
///Field `TAMP2E` writer - TAMP2E
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3E` reader - TAMP3E
pub type TAMP3E_R = crate::BitReader;
///Field `TAMP3E` writer - TAMP3E
pub type TAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4E` reader - TAMP4E
pub type TAMP4E_R = crate::BitReader;
///Field `TAMP4E` writer - TAMP4E
pub type TAMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5E` reader - TAMP5E
pub type TAMP5E_R = crate::BitReader;
///Field `TAMP5E` writer - TAMP5E
pub type TAMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6E` reader - TAMP6E
pub type TAMP6E_R = crate::BitReader;
///Field `TAMP6E` writer - TAMP6E
pub type TAMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP7E` reader - TAMP7E
pub type TAMP7E_R = crate::BitReader;
///Field `TAMP7E` writer - TAMP7E
pub type TAMP7E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP8E` reader - TAMP8E
pub type TAMP8E_R = crate::BitReader;
///Field `TAMP8E` writer - TAMP8E
pub type TAMP8E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP1E` reader - ITAMP1E
pub type ITAMP1E_R = crate::BitReader;
///Field `ITAMP1E` writer - ITAMP1E
pub type ITAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2E` reader - ITAMP2E
pub type ITAMP2E_R = crate::BitReader;
///Field `ITAMP2E` writer - ITAMP2E
pub type ITAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3E` reader - ITAMP3E
pub type ITAMP3E_R = crate::BitReader;
///Field `ITAMP3E` writer - ITAMP3E
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5E` reader - ITAMP5E
pub type ITAMP5E_R = crate::BitReader;
///Field `ITAMP5E` writer - ITAMP5E
pub type ITAMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP6E` reader - ITAMP6E
pub type ITAMP6E_R = crate::BitReader;
///Field `ITAMP6E` writer - ITAMP6E
pub type ITAMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP7E` reader - ITAMP7E
pub type ITAMP7E_R = crate::BitReader;
///Field `ITAMP7E` writer - ITAMP7E
pub type ITAMP7E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP8E` reader - ITAMP8E
pub type ITAMP8E_R = crate::BitReader;
///Field `ITAMP8E` writer - ITAMP8E
pub type ITAMP8E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP9E` reader - ITAMP9E
pub type ITAMP9E_R = crate::BitReader;
///Field `ITAMP9E` writer - ITAMP9E
pub type ITAMP9E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP11E` reader - TAMP1E
pub type ITAMP11E_R = crate::BitReader;
///Field `ITAMP11E` writer - TAMP1E
pub type ITAMP11E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP12E` reader - ITAMP12E
pub type ITAMP12E_R = crate::BitReader;
///Field `ITAMP12E` writer - ITAMP12E
pub type ITAMP12E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP13E` reader - ITAMP13E
pub type ITAMP13E_R = crate::BitReader;
///Field `ITAMP13E` writer - ITAMP13E
pub type ITAMP13E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TAMP1E
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2E
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3E
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4E
    #[inline(always)]
    pub fn tamp4e(&self) -> TAMP4E_R {
        TAMP4E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5E
    #[inline(always)]
    pub fn tamp5e(&self) -> TAMP5E_R {
        TAMP5E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6E
    #[inline(always)]
    pub fn tamp6e(&self) -> TAMP6E_R {
        TAMP6E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7E
    #[inline(always)]
    pub fn tamp7e(&self) -> TAMP7E_R {
        TAMP7E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8E
    #[inline(always)]
    pub fn tamp8e(&self) -> TAMP8E_R {
        TAMP8E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - ITAMP1E
    #[inline(always)]
    pub fn itamp1e(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ITAMP2E
    #[inline(always)]
    pub fn itamp2e(&self) -> ITAMP2E_R {
        ITAMP2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ITAMP3E
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5E
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6E
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ITAMP7E
    #[inline(always)]
    pub fn itamp7e(&self) -> ITAMP7E_R {
        ITAMP7E_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ITAMP8E
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ITAMP9E
    #[inline(always)]
    pub fn itamp9e(&self) -> ITAMP9E_R {
        ITAMP9E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - TAMP1E
    #[inline(always)]
    pub fn itamp11e(&self) -> ITAMP11E_R {
        ITAMP11E_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ITAMP12E
    #[inline(always)]
    pub fn itamp12e(&self) -> ITAMP12E_R {
        ITAMP12E_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ITAMP13E
    #[inline(always)]
    pub fn itamp13e(&self) -> ITAMP13E_R {
        ITAMP13E_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("itamp13e", &self.itamp13e())
            .field("itamp12e", &self.itamp12e())
            .field("itamp11e", &self.itamp11e())
            .field("itamp9e", &self.itamp9e())
            .field("itamp8e", &self.itamp8e())
            .field("itamp7e", &self.itamp7e())
            .field("itamp6e", &self.itamp6e())
            .field("itamp5e", &self.itamp5e())
            .field("itamp3e", &self.itamp3e())
            .field("itamp2e", &self.itamp2e())
            .field("itamp1e", &self.itamp1e())
            .field("tamp8e", &self.tamp8e())
            .field("tamp7e", &self.tamp7e())
            .field("tamp6e", &self.tamp6e())
            .field("tamp5e", &self.tamp5e())
            .field("tamp4e", &self.tamp4e())
            .field("tamp3e", &self.tamp3e())
            .field("tamp2e", &self.tamp2e())
            .field("tamp1e", &self.tamp1e())
            .finish()
    }
}
impl W {
    ///Bit 0 - TAMP1E
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<CR1rs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - TAMP2E
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<CR1rs> {
        TAMP2E_W::new(self, 1)
    }
    ///Bit 2 - TAMP3E
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<CR1rs> {
        TAMP3E_W::new(self, 2)
    }
    ///Bit 3 - TAMP4E
    #[inline(always)]
    pub fn tamp4e(&mut self) -> TAMP4E_W<CR1rs> {
        TAMP4E_W::new(self, 3)
    }
    ///Bit 4 - TAMP5E
    #[inline(always)]
    pub fn tamp5e(&mut self) -> TAMP5E_W<CR1rs> {
        TAMP5E_W::new(self, 4)
    }
    ///Bit 5 - TAMP6E
    #[inline(always)]
    pub fn tamp6e(&mut self) -> TAMP6E_W<CR1rs> {
        TAMP6E_W::new(self, 5)
    }
    ///Bit 6 - TAMP7E
    #[inline(always)]
    pub fn tamp7e(&mut self) -> TAMP7E_W<CR1rs> {
        TAMP7E_W::new(self, 6)
    }
    ///Bit 7 - TAMP8E
    #[inline(always)]
    pub fn tamp8e(&mut self) -> TAMP8E_W<CR1rs> {
        TAMP8E_W::new(self, 7)
    }
    ///Bit 16 - ITAMP1E
    #[inline(always)]
    pub fn itamp1e(&mut self) -> ITAMP1E_W<CR1rs> {
        ITAMP1E_W::new(self, 16)
    }
    ///Bit 17 - ITAMP2E
    #[inline(always)]
    pub fn itamp2e(&mut self) -> ITAMP2E_W<CR1rs> {
        ITAMP2E_W::new(self, 17)
    }
    ///Bit 18 - ITAMP3E
    #[inline(always)]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<CR1rs> {
        ITAMP3E_W::new(self, 18)
    }
    ///Bit 20 - ITAMP5E
    #[inline(always)]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<CR1rs> {
        ITAMP5E_W::new(self, 20)
    }
    ///Bit 21 - ITAMP6E
    #[inline(always)]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<CR1rs> {
        ITAMP6E_W::new(self, 21)
    }
    ///Bit 22 - ITAMP7E
    #[inline(always)]
    pub fn itamp7e(&mut self) -> ITAMP7E_W<CR1rs> {
        ITAMP7E_W::new(self, 22)
    }
    ///Bit 23 - ITAMP8E
    #[inline(always)]
    pub fn itamp8e(&mut self) -> ITAMP8E_W<CR1rs> {
        ITAMP8E_W::new(self, 23)
    }
    ///Bit 24 - ITAMP9E
    #[inline(always)]
    pub fn itamp9e(&mut self) -> ITAMP9E_W<CR1rs> {
        ITAMP9E_W::new(self, 24)
    }
    ///Bit 26 - TAMP1E
    #[inline(always)]
    pub fn itamp11e(&mut self) -> ITAMP11E_W<CR1rs> {
        ITAMP11E_W::new(self, 26)
    }
    ///Bit 27 - ITAMP12E
    #[inline(always)]
    pub fn itamp12e(&mut self) -> ITAMP12E_W<CR1rs> {
        ITAMP12E_W::new(self, 27)
    }
    ///Bit 28 - ITAMP13E
    #[inline(always)]
    pub fn itamp13e(&mut self) -> ITAMP13E_W<CR1rs> {
        ITAMP13E_W::new(self, 28)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TAMP:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
