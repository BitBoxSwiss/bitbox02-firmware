///Register `SCR` reader
pub type R = crate::R<SCRrs>;
///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CTAMP1F` reader - CTAMP1F
pub type CTAMP1F_R = crate::BitReader;
///Field `CTAMP1F` writer - CTAMP1F
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP2F` reader - CTAMP2F
pub type CTAMP2F_R = crate::BitReader;
///Field `CTAMP2F` writer - CTAMP2F
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP3F` reader - CTAMP3F
pub type CTAMP3F_R = crate::BitReader;
///Field `CTAMP3F` writer - CTAMP3F
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP4F` reader - CTAMP4F
pub type CTAMP4F_R = crate::BitReader;
///Field `CTAMP4F` writer - CTAMP4F
pub type CTAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP5F` reader - CTAMP5F
pub type CTAMP5F_R = crate::BitReader;
///Field `CTAMP5F` writer - CTAMP5F
pub type CTAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP6F` reader - CTAMP6F
pub type CTAMP6F_R = crate::BitReader;
///Field `CTAMP6F` writer - CTAMP6F
pub type CTAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP7F` reader - CITAMP3F
pub type CITAMP7F_R = crate::BitReader;
///Field `CITAMP7F` writer - CITAMP3F
pub type CITAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP8F` reader - CITAMP3F
pub type CITAMP8F_R = crate::BitReader;
///Field `CITAMP8F` writer - CITAMP3F
pub type CITAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP1F` reader - CITAMP1F
pub type CITAMP1F_R = crate::BitReader;
///Field `CITAMP1F` writer - CITAMP1F
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP2F` reader - CITAMP2F
pub type CITAMP2F_R = crate::BitReader;
///Field `CITAMP2F` writer - CITAMP2F
pub type CITAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP3F` reader - CITAMP3F
pub type CITAMP3F_R = crate::BitReader;
///Field `CITAMP3F` writer - CITAMP3F
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP5F` reader - CITAMP5F
pub type CITAMP5F_R = crate::BitReader;
///Field `CITAMP5F` writer - CITAMP5F
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP6F_bit21` reader - CITAMP6F_bit21
pub type CITAMP6F_BIT21_R = crate::BitReader;
///Field `CITAMP6F_bit21` writer - CITAMP6F_bit21
pub type CITAMP6F_BIT21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP7F_bit22` reader - CITAMP7F_bit22
pub type CITAMP7F_BIT22_R = crate::BitReader;
///Field `CITAMP7F_bit22` writer - CITAMP7F_bit22
pub type CITAMP7F_BIT22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP8F_bit23` reader - CITAMP8F_bit23
pub type CITAMP8F_BIT23_R = crate::BitReader;
///Field `CITAMP8F_bit23` writer - CITAMP8F_bit23
pub type CITAMP8F_BIT23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP9F` reader - CITAMP9F
pub type CITAMP9F_R = crate::BitReader;
///Field `CITAMP9F` writer - CITAMP9F
pub type CITAMP9F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP11F` reader - CITAMP11F
pub type CITAMP11F_R = crate::BitReader;
///Field `CITAMP11F` writer - CITAMP11F
pub type CITAMP11F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP12F` reader - CITAMP12F
pub type CITAMP12F_R = crate::BitReader;
///Field `CITAMP12F` writer - CITAMP12F
pub type CITAMP12F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP13F` reader - CITAMP13F
pub type CITAMP13F_R = crate::BitReader;
///Field `CITAMP13F` writer - CITAMP13F
pub type CITAMP13F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CTAMP1F
    #[inline(always)]
    pub fn ctamp1f(&self) -> CTAMP1F_R {
        CTAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTAMP2F
    #[inline(always)]
    pub fn ctamp2f(&self) -> CTAMP2F_R {
        CTAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTAMP3F
    #[inline(always)]
    pub fn ctamp3f(&self) -> CTAMP3F_R {
        CTAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTAMP4F
    #[inline(always)]
    pub fn ctamp4f(&self) -> CTAMP4F_R {
        CTAMP4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CTAMP5F
    #[inline(always)]
    pub fn ctamp5f(&self) -> CTAMP5F_R {
        CTAMP5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CTAMP6F
    #[inline(always)]
    pub fn ctamp6f(&self) -> CTAMP6F_R {
        CTAMP6F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CITAMP3F
    #[inline(always)]
    pub fn citamp7f(&self) -> CITAMP7F_R {
        CITAMP7F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CITAMP3F
    #[inline(always)]
    pub fn citamp8f(&self) -> CITAMP8F_R {
        CITAMP8F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - CITAMP1F
    #[inline(always)]
    pub fn citamp1f(&self) -> CITAMP1F_R {
        CITAMP1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CITAMP2F
    #[inline(always)]
    pub fn citamp2f(&self) -> CITAMP2F_R {
        CITAMP2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CITAMP3F
    #[inline(always)]
    pub fn citamp3f(&self) -> CITAMP3F_R {
        CITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CITAMP5F
    #[inline(always)]
    pub fn citamp5f(&self) -> CITAMP5F_R {
        CITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CITAMP6F_bit21
    #[inline(always)]
    pub fn citamp6f_bit21(&self) -> CITAMP6F_BIT21_R {
        CITAMP6F_BIT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CITAMP7F_bit22
    #[inline(always)]
    pub fn citamp7f_bit22(&self) -> CITAMP7F_BIT22_R {
        CITAMP7F_BIT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CITAMP8F_bit23
    #[inline(always)]
    pub fn citamp8f_bit23(&self) -> CITAMP8F_BIT23_R {
        CITAMP8F_BIT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CITAMP9F
    #[inline(always)]
    pub fn citamp9f(&self) -> CITAMP9F_R {
        CITAMP9F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - CITAMP11F
    #[inline(always)]
    pub fn citamp11f(&self) -> CITAMP11F_R {
        CITAMP11F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CITAMP12F
    #[inline(always)]
    pub fn citamp12f(&self) -> CITAMP12F_R {
        CITAMP12F_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CITAMP13F
    #[inline(always)]
    pub fn citamp13f(&self) -> CITAMP13F_R {
        CITAMP13F_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("ctamp1f", &self.ctamp1f())
            .field("ctamp2f", &self.ctamp2f())
            .field("ctamp3f", &self.ctamp3f())
            .field("ctamp4f", &self.ctamp4f())
            .field("ctamp5f", &self.ctamp5f())
            .field("ctamp6f", &self.ctamp6f())
            .field("citamp7f", &self.citamp7f())
            .field("citamp8f", &self.citamp8f())
            .field("citamp1f", &self.citamp1f())
            .field("citamp2f", &self.citamp2f())
            .field("citamp3f", &self.citamp3f())
            .field("citamp5f", &self.citamp5f())
            .field("citamp6f_bit21", &self.citamp6f_bit21())
            .field("citamp7f_bit22", &self.citamp7f_bit22())
            .field("citamp8f_bit23", &self.citamp8f_bit23())
            .field("citamp9f", &self.citamp9f())
            .field("citamp11f", &self.citamp11f())
            .field("citamp12f", &self.citamp12f())
            .field("citamp13f", &self.citamp13f())
            .finish()
    }
}
impl W {
    ///Bit 0 - CTAMP1F
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    ///Bit 1 - CTAMP2F
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    ///Bit 2 - CTAMP3F
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    ///Bit 3 - CTAMP4F
    #[inline(always)]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<SCRrs> {
        CTAMP4F_W::new(self, 3)
    }
    ///Bit 4 - CTAMP5F
    #[inline(always)]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<SCRrs> {
        CTAMP5F_W::new(self, 4)
    }
    ///Bit 5 - CTAMP6F
    #[inline(always)]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W<SCRrs> {
        CTAMP6F_W::new(self, 5)
    }
    ///Bit 6 - CITAMP3F
    #[inline(always)]
    pub fn citamp7f(&mut self) -> CITAMP7F_W<SCRrs> {
        CITAMP7F_W::new(self, 6)
    }
    ///Bit 7 - CITAMP3F
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<SCRrs> {
        CITAMP8F_W::new(self, 7)
    }
    ///Bit 16 - CITAMP1F
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    ///Bit 17 - CITAMP2F
    #[inline(always)]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<SCRrs> {
        CITAMP2F_W::new(self, 17)
    }
    ///Bit 18 - CITAMP3F
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    ///Bit 20 - CITAMP5F
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    ///Bit 21 - CITAMP6F_bit21
    #[inline(always)]
    pub fn citamp6f_bit21(&mut self) -> CITAMP6F_BIT21_W<SCRrs> {
        CITAMP6F_BIT21_W::new(self, 21)
    }
    ///Bit 22 - CITAMP7F_bit22
    #[inline(always)]
    pub fn citamp7f_bit22(&mut self) -> CITAMP7F_BIT22_W<SCRrs> {
        CITAMP7F_BIT22_W::new(self, 22)
    }
    ///Bit 23 - CITAMP8F_bit23
    #[inline(always)]
    pub fn citamp8f_bit23(&mut self) -> CITAMP8F_BIT23_W<SCRrs> {
        CITAMP8F_BIT23_W::new(self, 23)
    }
    ///Bit 24 - CITAMP9F
    #[inline(always)]
    pub fn citamp9f(&mut self) -> CITAMP9F_W<SCRrs> {
        CITAMP9F_W::new(self, 24)
    }
    ///Bit 26 - CITAMP11F
    #[inline(always)]
    pub fn citamp11f(&mut self) -> CITAMP11F_W<SCRrs> {
        CITAMP11F_W::new(self, 26)
    }
    ///Bit 27 - CITAMP12F
    #[inline(always)]
    pub fn citamp12f(&mut self) -> CITAMP12F_W<SCRrs> {
        CITAMP12F_W::new(self, 27)
    }
    ///Bit 28 - CITAMP13F
    #[inline(always)]
    pub fn citamp13f(&mut self) -> CITAMP13F_W<SCRrs> {
        CITAMP13F_W::new(self, 28)
    }
}
/**TAMP status clear register

You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#TAMP:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCRrs {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
