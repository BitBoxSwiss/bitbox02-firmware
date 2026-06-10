///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `TAMP1NOER` reader - Tamper 1 no erase
pub type TAMP1NOER_R = crate::BitReader;
///Field `TAMP1NOER` writer - Tamper 1 no erase
pub type TAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2NOER` reader - Tamper 2 no erase
pub type TAMP2NOER_R = crate::BitReader;
///Field `TAMP2NOER` writer - Tamper 2 no erase
pub type TAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3NOER` reader - Tamper 3 no erase
pub type TAMP3NOER_R = crate::BitReader;
///Field `TAMP3NOER` writer - Tamper 3 no erase
pub type TAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4NOER` reader - Tamper 4 no erase
pub type TAMP4NOER_R = crate::BitReader;
///Field `TAMP4NOER` writer - Tamper 4 no erase
pub type TAMP4NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5NOER` reader - Tamper 5 no erase
pub type TAMP5NOER_R = crate::BitReader;
///Field `TAMP5NOER` writer - Tamper 5 no erase
pub type TAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6NOER` reader - Tamper 6 no erase
pub type TAMP6NOER_R = crate::BitReader;
///Field `TAMP6NOER` writer - Tamper 6 no erase
pub type TAMP6NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP7NOER` reader - Tamper 7 no erase
pub type TAMP7NOER_R = crate::BitReader;
///Field `TAMP7NOER` writer - Tamper 7 no erase
pub type TAMP7NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP8NOER` reader - Tamper 8 no erase
pub type TAMP8NOER_R = crate::BitReader;
///Field `TAMP8NOER` writer - Tamper 8 no erase
pub type TAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub type TAMP1MSK_R = crate::BitReader;
///Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub type TAMP2MSK_R = crate::BitReader;
///Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub type TAMP2MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3MSK` reader - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
pub type TAMP3MSK_R = crate::BitReader;
///Field `TAMP3MSK` writer - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
pub type TAMP3MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKBLOCK` reader - Backup registers and device secrets access blocked
pub type BKBLOCK_R = crate::BitReader;
///Field `BKBLOCK` writer - Backup registers and device secrets access blocked
pub type BKBLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKERASE` writer - Backup registers and device secrets erase Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0.
pub type BKERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1TRG` reader - Active level for tamper 1 input If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMP1TRG` writer - Active level for tamper 1 input If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2TRG` reader - Active level for tamper 2 input If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
pub type TAMP2TRG_R = crate::BitReader;
///Field `TAMP2TRG` writer - Active level for tamper 2 input If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3TRG` reader - Active level for tamper 3 input If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event.
pub type TAMP3TRG_R = crate::BitReader;
///Field `TAMP3TRG` writer - Active level for tamper 3 input If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event.
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4TRG` reader - Active level for tamper 4 input (active mode disabled) If TAMPFLT = 00 Tamper 4 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 4 input falling edge and low level triggers a tamper detection event.
pub type TAMP4TRG_R = crate::BitReader;
///Field `TAMP4TRG` writer - Active level for tamper 4 input (active mode disabled) If TAMPFLT = 00 Tamper 4 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 4 input falling edge and low level triggers a tamper detection event.
pub type TAMP4TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5TRG` reader - Active level for tamper 5 input (active mode disabled) If TAMPFLT = 00 Tamper 5 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 5 input falling edge and low level triggers a tamper detection event.
pub type TAMP5TRG_R = crate::BitReader;
///Field `TAMP5TRG` writer - Active level for tamper 5 input (active mode disabled) If TAMPFLT = 00 Tamper 5 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 5 input falling edge and low level triggers a tamper detection event.
pub type TAMP5TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6TRG` reader - Active level for tamper 6 input (active mode disabled) If TAMPFLT = 00 Tamper 6 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 6 input falling edge and low level triggers a tamper detection event.
pub type TAMP6TRG_R = crate::BitReader;
///Field `TAMP6TRG` writer - Active level for tamper 6 input (active mode disabled) If TAMPFLT = 00 Tamper 6 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 6 input falling edge and low level triggers a tamper detection event.
pub type TAMP6TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP7TRG` reader - Active level for tamper 7 input (active mode disabled) If TAMPFLT = 00 Tamper 7 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 7 input falling edge and low level triggers a tamper detection event.
pub type TAMP7TRG_R = crate::BitReader;
///Field `TAMP7TRG` writer - Active level for tamper 7 input (active mode disabled) If TAMPFLT = 00 Tamper 7 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 7 input falling edge and low level triggers a tamper detection event.
pub type TAMP7TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP8TRG` reader - Active level for tamper 8 input (active mode disabled) If TAMPFLT = 00 Tamper 8 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 8 input falling edge and low level triggers a tamper detection event.
pub type TAMP8TRG_R = crate::BitReader;
///Field `TAMP8TRG` writer - Active level for tamper 8 input (active mode disabled) If TAMPFLT = 00 Tamper 8 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 8 input falling edge and low level triggers a tamper detection event.
pub type TAMP8TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper 3 no erase
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 4 no erase
    #[inline(always)]
    pub fn tamp4noer(&self) -> TAMP4NOER_R {
        TAMP4NOER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tamper 5 no erase
    #[inline(always)]
    pub fn tamp5noer(&self) -> TAMP5NOER_R {
        TAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Tamper 6 no erase
    #[inline(always)]
    pub fn tamp6noer(&self) -> TAMP6NOER_R {
        TAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Tamper 7 no erase
    #[inline(always)]
    pub fn tamp7noer(&self) -> TAMP7NOER_R {
        TAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tamper 8 no erase
    #[inline(always)]
    pub fn tamp8noer(&self) -> TAMP8NOER_R {
        TAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - Backup registers and device secrets access blocked
    #[inline(always)]
    pub fn bkblock(&self) -> BKBLOCK_R {
        BKBLOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Active level for tamper 1 input If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Active level for tamper 2 input If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Active level for tamper 3 input If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Active level for tamper 4 input (active mode disabled) If TAMPFLT = 00 Tamper 4 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 4 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp4trg(&self) -> TAMP4TRG_R {
        TAMP4TRG_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Active level for tamper 5 input (active mode disabled) If TAMPFLT = 00 Tamper 5 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 5 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp5trg(&self) -> TAMP5TRG_R {
        TAMP5TRG_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Active level for tamper 6 input (active mode disabled) If TAMPFLT = 00 Tamper 6 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 6 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp6trg(&self) -> TAMP6TRG_R {
        TAMP6TRG_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Active level for tamper 7 input (active mode disabled) If TAMPFLT = 00 Tamper 7 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 7 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp7trg(&self) -> TAMP7TRG_R {
        TAMP7TRG_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Active level for tamper 8 input (active mode disabled) If TAMPFLT = 00 Tamper 8 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 8 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp8trg(&self) -> TAMP8TRG_R {
        TAMP8TRG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tamp1noer", &self.tamp1noer())
            .field("tamp2noer", &self.tamp2noer())
            .field("tamp3noer", &self.tamp3noer())
            .field("tamp4noer", &self.tamp4noer())
            .field("tamp5noer", &self.tamp5noer())
            .field("tamp6noer", &self.tamp6noer())
            .field("tamp7noer", &self.tamp7noer())
            .field("tamp8noer", &self.tamp8noer())
            .field("tamp1msk", &self.tamp1msk())
            .field("tamp2msk", &self.tamp2msk())
            .field("tamp3msk", &self.tamp3msk())
            .field("bkblock", &self.bkblock())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp3trg", &self.tamp3trg())
            .field("tamp4trg", &self.tamp4trg())
            .field("tamp5trg", &self.tamp5trg())
            .field("tamp6trg", &self.tamp6trg())
            .field("tamp7trg", &self.tamp7trg())
            .field("tamp8trg", &self.tamp8trg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<CR2rs> {
        TAMP1NOER_W::new(self, 0)
    }
    ///Bit 1 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<CR2rs> {
        TAMP2NOER_W::new(self, 1)
    }
    ///Bit 2 - Tamper 3 no erase
    #[inline(always)]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W<CR2rs> {
        TAMP3NOER_W::new(self, 2)
    }
    ///Bit 3 - Tamper 4 no erase
    #[inline(always)]
    pub fn tamp4noer(&mut self) -> TAMP4NOER_W<CR2rs> {
        TAMP4NOER_W::new(self, 3)
    }
    ///Bit 4 - Tamper 5 no erase
    #[inline(always)]
    pub fn tamp5noer(&mut self) -> TAMP5NOER_W<CR2rs> {
        TAMP5NOER_W::new(self, 4)
    }
    ///Bit 5 - Tamper 6 no erase
    #[inline(always)]
    pub fn tamp6noer(&mut self) -> TAMP6NOER_W<CR2rs> {
        TAMP6NOER_W::new(self, 5)
    }
    ///Bit 6 - Tamper 7 no erase
    #[inline(always)]
    pub fn tamp7noer(&mut self) -> TAMP7NOER_W<CR2rs> {
        TAMP7NOER_W::new(self, 6)
    }
    ///Bit 7 - Tamper 8 no erase
    #[inline(always)]
    pub fn tamp8noer(&mut self) -> TAMP8NOER_W<CR2rs> {
        TAMP8NOER_W::new(self, 7)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<CR2rs> {
        TAMP1MSK_W::new(self, 16)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<CR2rs> {
        TAMP2MSK_W::new(self, 17)
    }
    ///Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
    #[inline(always)]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W<CR2rs> {
        TAMP3MSK_W::new(self, 18)
    }
    ///Bit 22 - Backup registers and device secrets access blocked
    #[inline(always)]
    pub fn bkblock(&mut self) -> BKBLOCK_W<CR2rs> {
        BKBLOCK_W::new(self, 22)
    }
    ///Bit 23 - Backup registers and device secrets erase Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0.
    #[inline(always)]
    pub fn bkerase(&mut self) -> BKERASE_W<CR2rs> {
        BKERASE_W::new(self, 23)
    }
    ///Bit 24 - Active level for tamper 1 input If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<CR2rs> {
        TAMP1TRG_W::new(self, 24)
    }
    ///Bit 25 - Active level for tamper 2 input If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<CR2rs> {
        TAMP2TRG_W::new(self, 25)
    }
    ///Bit 26 - Active level for tamper 3 input If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<CR2rs> {
        TAMP3TRG_W::new(self, 26)
    }
    ///Bit 27 - Active level for tamper 4 input (active mode disabled) If TAMPFLT = 00 Tamper 4 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 4 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp4trg(&mut self) -> TAMP4TRG_W<CR2rs> {
        TAMP4TRG_W::new(self, 27)
    }
    ///Bit 28 - Active level for tamper 5 input (active mode disabled) If TAMPFLT = 00 Tamper 5 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 5 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp5trg(&mut self) -> TAMP5TRG_W<CR2rs> {
        TAMP5TRG_W::new(self, 28)
    }
    ///Bit 29 - Active level for tamper 6 input (active mode disabled) If TAMPFLT = 00 Tamper 6 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 6 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp6trg(&mut self) -> TAMP6TRG_W<CR2rs> {
        TAMP6TRG_W::new(self, 29)
    }
    ///Bit 30 - Active level for tamper 7 input (active mode disabled) If TAMPFLT = 00 Tamper 7 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 7 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp7trg(&mut self) -> TAMP7TRG_W<CR2rs> {
        TAMP7TRG_W::new(self, 30)
    }
    ///Bit 31 - Active level for tamper 8 input (active mode disabled) If TAMPFLT = 00 Tamper 8 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 8 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp8trg(&mut self) -> TAMP8TRG_W<CR2rs> {
        TAMP8TRG_W::new(self, 31)
    }
}
/**TAMP control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
