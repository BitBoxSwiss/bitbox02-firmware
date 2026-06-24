///Register `ATCR1` reader
pub type R = crate::R<ATCR1rs>;
///Register `ATCR1` writer
pub type W = crate::W<ATCR1rs>;
///Field `TAMP1AM` reader - TAMP1AM
pub type TAMP1AM_R = crate::BitReader;
///Field `TAMP1AM` writer - TAMP1AM
pub type TAMP1AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2AM` reader - TAMP2AM
pub type TAMP2AM_R = crate::BitReader;
///Field `TAMP2AM` writer - TAMP2AM
pub type TAMP2AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3AM` reader - TAMP3AM
pub type TAMP3AM_R = crate::BitReader;
///Field `TAMP3AM` writer - TAMP3AM
pub type TAMP3AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4AM` reader - TAMP4AM
pub type TAMP4AM_R = crate::BitReader;
///Field `TAMP4AM` writer - TAMP4AM
pub type TAMP4AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5AM` reader - TAMP5AM
pub type TAMP5AM_R = crate::BitReader;
///Field `TAMP5AM` writer - TAMP5AM
pub type TAMP5AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6AM` reader - TAMP6AM
pub type TAMP6AM_R = crate::BitReader;
///Field `TAMP6AM` writer - TAMP6AM
pub type TAMP6AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP7AM` reader - TAMP7AM
pub type TAMP7AM_R = crate::BitReader;
///Field `TAMP7AM` writer - TAMP7AM
pub type TAMP7AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP8AM` reader - TAMP8AM
pub type TAMP8AM_R = crate::BitReader;
///Field `TAMP8AM` writer - TAMP8AM
pub type TAMP8AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATOSEL1` reader - ATOSEL1
pub type ATOSEL1_R = crate::FieldReader;
///Field `ATOSEL1` writer - ATOSEL1
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL2` reader - ATOSEL2
pub type ATOSEL2_R = crate::FieldReader;
///Field `ATOSEL2` writer - ATOSEL2
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL3` reader - ATOSEL3
pub type ATOSEL3_R = crate::FieldReader;
///Field `ATOSEL3` writer - ATOSEL3
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL4` reader - ATOSEL4
pub type ATOSEL4_R = crate::FieldReader;
///Field `ATOSEL4` writer - ATOSEL4
pub type ATOSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATCKSEL` reader - ATCKSEL
pub type ATCKSEL_R = crate::FieldReader;
///Field `ATCKSEL` writer - ATCKSEL
pub type ATCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATPER` reader - ATPER
pub type ATPER_R = crate::FieldReader;
///Field `ATPER` writer - ATPER
pub type ATPER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSHARE` reader - ATOSHARE
pub type ATOSHARE_R = crate::BitReader;
///Field `ATOSHARE` writer - ATOSHARE
pub type ATOSHARE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTEN` reader - ATOSHARE
pub type FLTEN_R = crate::BitReader;
///Field `FLTEN` writer - ATOSHARE
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TAMP1AM
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2AM
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3AM
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4AM
    #[inline(always)]
    pub fn tamp4am(&self) -> TAMP4AM_R {
        TAMP4AM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5AM
    #[inline(always)]
    pub fn tamp5am(&self) -> TAMP5AM_R {
        TAMP5AM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6AM
    #[inline(always)]
    pub fn tamp6am(&self) -> TAMP6AM_R {
        TAMP6AM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7AM
    #[inline(always)]
    pub fn tamp7am(&self) -> TAMP7AM_R {
        TAMP7AM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8AM
    #[inline(always)]
    pub fn tamp8am(&self) -> TAMP8AM_R {
        TAMP8AM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - ATOSEL1
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - ATOSEL2
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - ATOSEL3
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - ATOSEL4
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:18 - ATCKSEL
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - ATPER
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 30 - ATOSHARE
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ATOSHARE
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATCR1")
            .field("tamp1am", &self.tamp1am())
            .field("tamp2am", &self.tamp2am())
            .field("tamp3am", &self.tamp3am())
            .field("tamp4am", &self.tamp4am())
            .field("tamp5am", &self.tamp5am())
            .field("tamp6am", &self.tamp6am())
            .field("tamp7am", &self.tamp7am())
            .field("tamp8am", &self.tamp8am())
            .field("atosel1", &self.atosel1())
            .field("atosel2", &self.atosel2())
            .field("atosel3", &self.atosel3())
            .field("atosel4", &self.atosel4())
            .field("atcksel", &self.atcksel())
            .field("atper", &self.atper())
            .field("atoshare", &self.atoshare())
            .field("flten", &self.flten())
            .finish()
    }
}
impl W {
    ///Bit 0 - TAMP1AM
    #[inline(always)]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<ATCR1rs> {
        TAMP1AM_W::new(self, 0)
    }
    ///Bit 1 - TAMP2AM
    #[inline(always)]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<ATCR1rs> {
        TAMP2AM_W::new(self, 1)
    }
    ///Bit 2 - TAMP3AM
    #[inline(always)]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<ATCR1rs> {
        TAMP3AM_W::new(self, 2)
    }
    ///Bit 3 - TAMP4AM
    #[inline(always)]
    pub fn tamp4am(&mut self) -> TAMP4AM_W<ATCR1rs> {
        TAMP4AM_W::new(self, 3)
    }
    ///Bit 4 - TAMP5AM
    #[inline(always)]
    pub fn tamp5am(&mut self) -> TAMP5AM_W<ATCR1rs> {
        TAMP5AM_W::new(self, 4)
    }
    ///Bit 5 - TAMP6AM
    #[inline(always)]
    pub fn tamp6am(&mut self) -> TAMP6AM_W<ATCR1rs> {
        TAMP6AM_W::new(self, 5)
    }
    ///Bit 6 - TAMP7AM
    #[inline(always)]
    pub fn tamp7am(&mut self) -> TAMP7AM_W<ATCR1rs> {
        TAMP7AM_W::new(self, 6)
    }
    ///Bit 7 - TAMP8AM
    #[inline(always)]
    pub fn tamp8am(&mut self) -> TAMP8AM_W<ATCR1rs> {
        TAMP8AM_W::new(self, 7)
    }
    ///Bits 8:9 - ATOSEL1
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W<ATCR1rs> {
        ATOSEL1_W::new(self, 8)
    }
    ///Bits 10:11 - ATOSEL2
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W<ATCR1rs> {
        ATOSEL2_W::new(self, 10)
    }
    ///Bits 12:13 - ATOSEL3
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W<ATCR1rs> {
        ATOSEL3_W::new(self, 12)
    }
    ///Bits 14:15 - ATOSEL4
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W<ATCR1rs> {
        ATOSEL4_W::new(self, 14)
    }
    ///Bits 16:18 - ATCKSEL
    #[inline(always)]
    pub fn atcksel(&mut self) -> ATCKSEL_W<ATCR1rs> {
        ATCKSEL_W::new(self, 16)
    }
    ///Bits 24:26 - ATPER
    #[inline(always)]
    pub fn atper(&mut self) -> ATPER_W<ATCR1rs> {
        ATPER_W::new(self, 24)
    }
    ///Bit 30 - ATOSHARE
    #[inline(always)]
    pub fn atoshare(&mut self) -> ATOSHARE_W<ATCR1rs> {
        ATOSHARE_W::new(self, 30)
    }
    ///Bit 31 - ATOSHARE
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<ATCR1rs> {
        FLTEN_W::new(self, 31)
    }
}
/**TAMP active tamper control register

You can [`read`](crate::Reg::read) this register and get [`atcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TAMP:ATCR1)*/
pub struct ATCR1rs;
impl crate::RegisterSpec for ATCR1rs {
    type Ux = u32;
}
///`read()` method returns [`atcr1::R`](R) reader structure
impl crate::Readable for ATCR1rs {}
///`write(|w| ..)` method takes [`atcr1::W`](W) writer structure
impl crate::Writable for ATCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ATCR1 to value 0x0007_0000
impl crate::Resettable for ATCR1rs {
    const RESET_VALUE: u32 = 0x0007_0000;
}
