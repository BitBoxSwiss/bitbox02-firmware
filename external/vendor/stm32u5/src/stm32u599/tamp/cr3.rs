///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `ITAMP1NOER` reader - ITAMP1NOER
pub type ITAMP1NOER_R = crate::BitReader;
///Field `ITAMP1NOER` writer - ITAMP1NOER
pub type ITAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2NOER` reader - ITAMP2NOER
pub type ITAMP2NOER_R = crate::BitReader;
///Field `ITAMP2NOER` writer - ITAMP2NOER
pub type ITAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3NOER` reader - ITAMP3NOER
pub type ITAMP3NOER_R = crate::BitReader;
///Field `ITAMP3NOER` writer - ITAMP3NOER
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5NOER` reader - TAMP5NOER
pub type TAMP5NOER_R = crate::BitReader;
///Field `TAMP5NOER` writer - TAMP5NOER
pub type TAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6NOER` reader - TAMP6NOER
pub type TAMP6NOER_R = crate::BitReader;
///Field `TAMP6NOER` writer - TAMP6NOER
pub type TAMP6NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP7NOER` reader - TAMP7NOER
pub type TAMP7NOER_R = crate::BitReader;
///Field `TAMP7NOER` writer - TAMP7NOER
pub type TAMP7NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP8NOER` reader - TAMP8NOER
pub type TAMP8NOER_R = crate::BitReader;
///Field `TAMP8NOER` writer - TAMP8NOER
pub type TAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP9NOER` reader - ITAMP9NOER
pub type ITAMP9NOER_R = crate::BitReader;
///Field `ITAMP9NOER` writer - ITAMP9NOER
pub type ITAMP9NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP11NOER` reader - ITAMP11NOER
pub type ITAMP11NOER_R = crate::BitReader;
///Field `ITAMP11NOER` writer - ITAMP11NOER
pub type ITAMP11NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP12NOER` reader - ITAMP12NOER
pub type ITAMP12NOER_R = crate::BitReader;
///Field `ITAMP12NOER` writer - ITAMP12NOER
pub type ITAMP12NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP13NOER` reader - ITAMP13NOER
pub type ITAMP13NOER_R = crate::BitReader;
///Field `ITAMP13NOER` writer - ITAMP13NOER
pub type ITAMP13NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ITAMP1NOER
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ITAMP2NOER
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ITAMP3NOER
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - TAMP5NOER
    #[inline(always)]
    pub fn tamp5noer(&self) -> TAMP5NOER_R {
        TAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6NOER
    #[inline(always)]
    pub fn tamp6noer(&self) -> TAMP6NOER_R {
        TAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7NOER
    #[inline(always)]
    pub fn tamp7noer(&self) -> TAMP7NOER_R {
        TAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8NOER
    #[inline(always)]
    pub fn tamp8noer(&self) -> TAMP8NOER_R {
        TAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ITAMP9NOER
    #[inline(always)]
    pub fn itamp9noer(&self) -> ITAMP9NOER_R {
        ITAMP9NOER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ITAMP11NOER
    #[inline(always)]
    pub fn itamp11noer(&self) -> ITAMP11NOER_R {
        ITAMP11NOER_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ITAMP12NOER
    #[inline(always)]
    pub fn itamp12noer(&self) -> ITAMP12NOER_R {
        ITAMP12NOER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ITAMP13NOER
    #[inline(always)]
    pub fn itamp13noer(&self) -> ITAMP13NOER_R {
        ITAMP13NOER_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("itamp1noer", &self.itamp1noer())
            .field("itamp2noer", &self.itamp2noer())
            .field("itamp3noer", &self.itamp3noer())
            .field("tamp5noer", &self.tamp5noer())
            .field("tamp6noer", &self.tamp6noer())
            .field("tamp7noer", &self.tamp7noer())
            .field("tamp8noer", &self.tamp8noer())
            .field("itamp9noer", &self.itamp9noer())
            .field("itamp11noer", &self.itamp11noer())
            .field("itamp12noer", &self.itamp12noer())
            .field("itamp13noer", &self.itamp13noer())
            .finish()
    }
}
impl W {
    ///Bit 0 - ITAMP1NOER
    #[inline(always)]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<CR3rs> {
        ITAMP1NOER_W::new(self, 0)
    }
    ///Bit 1 - ITAMP2NOER
    #[inline(always)]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<CR3rs> {
        ITAMP2NOER_W::new(self, 1)
    }
    ///Bit 2 - ITAMP3NOER
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    ///Bit 4 - TAMP5NOER
    #[inline(always)]
    pub fn tamp5noer(&mut self) -> TAMP5NOER_W<CR3rs> {
        TAMP5NOER_W::new(self, 4)
    }
    ///Bit 5 - TAMP6NOER
    #[inline(always)]
    pub fn tamp6noer(&mut self) -> TAMP6NOER_W<CR3rs> {
        TAMP6NOER_W::new(self, 5)
    }
    ///Bit 6 - TAMP7NOER
    #[inline(always)]
    pub fn tamp7noer(&mut self) -> TAMP7NOER_W<CR3rs> {
        TAMP7NOER_W::new(self, 6)
    }
    ///Bit 7 - TAMP8NOER
    #[inline(always)]
    pub fn tamp8noer(&mut self) -> TAMP8NOER_W<CR3rs> {
        TAMP8NOER_W::new(self, 7)
    }
    ///Bit 8 - ITAMP9NOER
    #[inline(always)]
    pub fn itamp9noer(&mut self) -> ITAMP9NOER_W<CR3rs> {
        ITAMP9NOER_W::new(self, 8)
    }
    ///Bit 10 - ITAMP11NOER
    #[inline(always)]
    pub fn itamp11noer(&mut self) -> ITAMP11NOER_W<CR3rs> {
        ITAMP11NOER_W::new(self, 10)
    }
    ///Bit 11 - ITAMP12NOER
    #[inline(always)]
    pub fn itamp12noer(&mut self) -> ITAMP12NOER_W<CR3rs> {
        ITAMP12NOER_W::new(self, 11)
    }
    ///Bit 12 - ITAMP13NOER
    #[inline(always)]
    pub fn itamp13noer(&mut self) -> ITAMP13NOER_W<CR3rs> {
        ITAMP13NOER_W::new(self, 12)
    }
}
/**control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TAMP:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
