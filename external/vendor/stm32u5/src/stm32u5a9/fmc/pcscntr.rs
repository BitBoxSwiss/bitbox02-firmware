///Register `PCSCNTR` reader
pub type R = crate::R<PCSCNTRrs>;
///Register `PCSCNTR` writer
pub type W = crate::W<PCSCNTRrs>;
///Field `CSCOUNT` reader - Chip select counter
pub type CSCOUNT_R = crate::FieldReader<u16>;
///Field `CSCOUNT` writer - Chip select counter
pub type CSCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CNTBEN(1-4)` reader - Counter Bank %s enable
pub type CNTBEN_R = crate::BitReader;
///Field `CNTBEN(1-4)` writer - Counter Bank %s enable
pub type CNTBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Chip select counter
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Counter Bank (1-4) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CNTB1EN` field.</div>
    #[inline(always)]
    pub fn cntben(&self, n: u8) -> CNTBEN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNTBEN_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Counter Bank (1-4) enable
    #[inline(always)]
    pub fn cntben_iter(&self) -> impl Iterator<Item = CNTBEN_R> + '_ {
        (0..4).map(move |n| CNTBEN_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - Counter Bank 1 enable
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTBEN_R {
        CNTBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Counter Bank 2 enable
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTBEN_R {
        CNTBEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Counter Bank 3 enable
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTBEN_R {
        CNTBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Counter Bank 4 enable
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTBEN_R {
        CNTBEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSCNTR")
            .field("cscount", &self.cscount())
            .field("cntb1en", &self.cntb1en())
            .field("cntb2en", &self.cntb2en())
            .field("cntb3en", &self.cntb3en())
            .field("cntb4en", &self.cntb4en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Chip select counter
    #[inline(always)]
    pub fn cscount(&mut self) -> CSCOUNT_W<PCSCNTRrs> {
        CSCOUNT_W::new(self, 0)
    }
    ///Counter Bank (1-4) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CNTB1EN` field.</div>
    #[inline(always)]
    pub fn cntben(&mut self, n: u8) -> CNTBEN_W<PCSCNTRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNTBEN_W::new(self, n + 16)
    }
    ///Bit 16 - Counter Bank 1 enable
    #[inline(always)]
    pub fn cntb1en(&mut self) -> CNTBEN_W<PCSCNTRrs> {
        CNTBEN_W::new(self, 16)
    }
    ///Bit 17 - Counter Bank 2 enable
    #[inline(always)]
    pub fn cntb2en(&mut self) -> CNTBEN_W<PCSCNTRrs> {
        CNTBEN_W::new(self, 17)
    }
    ///Bit 18 - Counter Bank 3 enable
    #[inline(always)]
    pub fn cntb3en(&mut self) -> CNTBEN_W<PCSCNTRrs> {
        CNTBEN_W::new(self, 18)
    }
    ///Bit 19 - Counter Bank 4 enable
    #[inline(always)]
    pub fn cntb4en(&mut self) -> CNTBEN_W<PCSCNTRrs> {
        CNTBEN_W::new(self, 19)
    }
}
/**PSRAM chip select counter register

You can [`read`](crate::Reg::read) this register and get [`pcscntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcscntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#FMC:PCSCNTR)*/
pub struct PCSCNTRrs;
impl crate::RegisterSpec for PCSCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`pcscntr::R`](R) reader structure
impl crate::Readable for PCSCNTRrs {}
///`write(|w| ..)` method takes [`pcscntr::W`](W) writer structure
impl crate::Writable for PCSCNTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCSCNTR to value 0
impl crate::Resettable for PCSCNTRrs {}
