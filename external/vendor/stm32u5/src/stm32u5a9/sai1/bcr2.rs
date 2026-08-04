///Register `BCR2` reader
pub type R = crate::R<BCR2rs>;
///Register `BCR2` writer
pub type W = crate::W<BCR2rs>;
///Field `FTH` reader - FIFO threshold
pub type FTH_R = crate::FieldReader;
///Field `FTH` writer - FIFO threshold
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FFLUSH` reader - FIFO flush
pub type FFLUSH_R = crate::BitReader;
///Field `FFLUSH` writer - FIFO flush
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIS` reader - Tristate management on data line
pub type TRIS_R = crate::BitReader;
///Field `TRIS` writer - Tristate management on data line
pub type TRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTE` reader - Mute
pub type MUTE_R = crate::BitReader;
///Field `MUTE` writer - Mute
pub type MUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEVAL` reader - Mute value
pub type MUTEVAL_R = crate::BitReader;
///Field `MUTEVAL` writer - Mute value
pub type MUTEVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTECN` reader - Mute counter
pub type MUTECN_R = crate::FieldReader;
///Field `MUTECN` writer - Mute counter
pub type MUTECN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CPL` reader - Complement bit
pub type CPL_R = crate::BitReader;
///Field `CPL` writer - Complement bit
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP` reader - Companding mode
pub type COMP_R = crate::FieldReader;
///Field `COMP` writer - Companding mode
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecn(&self) -> MUTECN_R {
        MUTECN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR2")
            .field("comp", &self.comp())
            .field("cpl", &self.cpl())
            .field("mutecn", &self.mutecn())
            .field("muteval", &self.muteval())
            .field("mute", &self.mute())
            .field("tris", &self.tris())
            .field("fflush", &self.fflush())
            .field("fth", &self.fth())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<BCR2rs> {
        FTH_W::new(self, 0)
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<BCR2rs> {
        FFLUSH_W::new(self, 3)
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W<BCR2rs> {
        TRIS_W::new(self, 4)
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W<BCR2rs> {
        MUTE_W::new(self, 5)
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W<BCR2rs> {
        MUTEVAL_W::new(self, 6)
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecn(&mut self) -> MUTECN_W<BCR2rs> {
        MUTECN_W::new(self, 7)
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W<BCR2rs> {
        CPL_W::new(self, 13)
    }
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<BCR2rs> {
        COMP_W::new(self, 14)
    }
}
/**B Configuration register 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SAI1:BCR2)*/
pub struct BCR2rs;
impl crate::RegisterSpec for BCR2rs {
    type Ux = u32;
}
///`read()` method returns [`bcr2::R`](R) reader structure
impl crate::Readable for BCR2rs {}
///`write(|w| ..)` method takes [`bcr2::W`](W) writer structure
impl crate::Writable for BCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR2 to value 0
impl crate::Resettable for BCR2rs {}
