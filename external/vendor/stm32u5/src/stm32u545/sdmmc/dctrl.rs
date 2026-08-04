///Register `DCTRL` reader
pub type R = crate::R<DCTRLrs>;
///Register `DCTRL` writer
pub type W = crate::W<DCTRLrs>;
///Field `DTEN` reader - DTEN
pub type DTEN_R = crate::BitReader;
///Field `DTEN` writer - DTEN
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDIR` reader - Data transfer direction selection
pub type DTDIR_R = crate::BitReader;
///Field `DTDIR` writer - Data transfer direction selection
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTMODE` reader - Data transfer mode selection
pub type DTMODE_R = crate::FieldReader;
///Field `DTMODE` writer - Data transfer mode selection
pub type DTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DBLOCKSIZE` reader - Data block size
pub type DBLOCKSIZE_R = crate::FieldReader;
///Field `DBLOCKSIZE` writer - Data block size
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RWSTART` reader - Read wait start
pub type RWSTART_R = crate::BitReader;
///Field `RWSTART` writer - Read wait start
pub type RWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWSTOP` reader - Read wait stop
pub type RWSTOP_R = crate::BitReader;
///Field `RWSTOP` writer - Read wait stop
pub type RWSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWMOD` reader - Read wait mode
pub type RWMOD_R = crate::BitReader;
///Field `RWMOD` writer - Read wait mode
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOEN` reader - SD I/O enable functions
pub type SDIOEN_R = crate::BitReader;
///Field `SDIOEN` writer - SD I/O enable functions
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTACKEN` reader - Enable the reception of the boot acknowledgment
pub type BOOTACKEN_R = crate::BitReader;
///Field `BOOTACKEN` writer - Enable the reception of the boot acknowledgment
pub type BOOTACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFORST` reader - FIFO reset, will flush any remaining data
pub type FIFORST_R = crate::BitReader;
///Field `FIFORST` writer - FIFO reset, will flush any remaining data
pub type FIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Data transfer mode selection
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable the reception of the boot acknowledgment
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - FIFO reset, will flush any remaining data
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTRL")
            .field("fiforst", &self.fiforst())
            .field("bootacken", &self.bootacken())
            .field("sdioen", &self.sdioen())
            .field("rwmod", &self.rwmod())
            .field("rwstop", &self.rwstop())
            .field("rwstart", &self.rwstart())
            .field("dblocksize", &self.dblocksize())
            .field("dtmode", &self.dtmode())
            .field("dtdir", &self.dtdir())
            .field("dten", &self.dten())
            .finish()
    }
}
impl W {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W<DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    ///Bits 2:3 - Data transfer mode selection
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W<DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    pub fn rwstart(&mut self) -> RWSTART_W<DCTRLrs> {
        RWSTART_W::new(self, 8)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W<DCTRLrs> {
        RWSTOP_W::new(self, 9)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W<DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
    ///Bit 12 - Enable the reception of the boot acknowledgment
    #[inline(always)]
    pub fn bootacken(&mut self) -> BOOTACKEN_W<DCTRLrs> {
        BOOTACKEN_W::new(self, 12)
    }
    ///Bit 13 - FIFO reset, will flush any remaining data
    #[inline(always)]
    pub fn fiforst(&mut self) -> FIFORST_W<DCTRLrs> {
        FIFORST_W::new(self, 13)
    }
}
/**data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DCTRL)*/
pub struct DCTRLrs;
impl crate::RegisterSpec for DCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dctrl::R`](R) reader structure
impl crate::Readable for DCTRLrs {}
///`write(|w| ..)` method takes [`dctrl::W`](W) writer structure
impl crate::Writable for DCTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCTRL to value 0
impl crate::Resettable for DCTRLrs {}
