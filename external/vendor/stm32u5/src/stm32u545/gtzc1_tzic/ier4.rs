///Register `IER4` reader
pub type R = crate::R<IER4rs>;
///Register `IER4` writer
pub type W = crate::W<IER4rs>;
///Field `GPDMA1IE` reader - illegal access interrupt enable for GPDMA1
pub type GPDMA1IE_R = crate::BitReader;
///Field `GPDMA1IE` writer - illegal access interrupt enable for GPDMA1
pub type GPDMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_REGIE` reader - illegal access interrupt enable for FLASH registers
pub type FLASH_REGIE_R = crate::BitReader;
///Field `FLASH_REGIE` writer - illegal access interrupt enable for FLASH registers
pub type FLASH_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHIE` reader - illegal access interrupt enable for FLASH memory
pub type FLASHIE_R = crate::BitReader;
///Field `FLASHIE` writer - illegal access interrupt enable for FLASH memory
pub type FLASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTFDEC1IE` reader - illegal access interrupt enable for OTFDEC1
pub type OTFDEC1IE_R = crate::BitReader;
///Field `OTFDEC1IE` writer - illegal access interrupt enable for OTFDEC1
pub type OTFDEC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZSC1IE` reader - illegal access interrupt enable for GTZC1 TZSC registers
pub type TZSC1IE_R = crate::BitReader;
///Field `TZSC1IE` writer - illegal access interrupt enable for GTZC1 TZSC registers
pub type TZSC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZIC1IE` reader - illegal access interrupt enable for GTZC1 TZIC registers
pub type TZIC1IE_R = crate::BitReader;
///Field `TZIC1IE` writer - illegal access interrupt enable for GTZC1 TZIC registers
pub type TZIC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1_MEMIE` reader - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMIE_R = crate::BitReader;
///Field `OCTOSPI1_MEMIE` writer - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMIE` reader - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
pub type BKPSRAMIE_R = crate::BitReader;
///Field `BKPSRAMIE` writer - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
pub type BKPSRAMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPI1_MEMIE` reader - illegal access interrupt enable for HSPI1 memory bank
pub type HSPI1_MEMIE_R = crate::BitReader;
///Field `HSPI1_MEMIE` writer - illegal access interrupt enable for HSPI1 memory bank
pub type HSPI1_MEMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1IE` reader - illegal access interrupt enable for SRAM1
pub type SRAM1IE_R = crate::BitReader;
///Field `SRAM1IE` writer - illegal access interrupt enable for SRAM1
pub type SRAM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB1_REGIE` reader - illegal access interrupt enable for MPCBB1 registers
pub type MPCBB1_REGIE_R = crate::BitReader;
///Field `MPCBB1_REGIE` writer - illegal access interrupt enable for MPCBB1 registers
pub type MPCBB1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2IE` reader - illegal access interrupt enable for SRAM2
pub type SRAM2IE_R = crate::BitReader;
///Field `SRAM2IE` writer - illegal access interrupt enable for SRAM2
pub type SRAM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB2_REGIE` reader - illegal access interrupt enable for MPCBB2 registers
pub type MPCBB2_REGIE_R = crate::BitReader;
///Field `MPCBB2_REGIE` writer - illegal access interrupt enable for MPCBB2 registers
pub type MPCBB2_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM5IE` reader - illegal access interrupt enable for SRAM5
pub type SRAM5IE_R = crate::BitReader;
///Field `SRAM5IE` writer - illegal access interrupt enable for SRAM5
pub type SRAM5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for GPDMA1
    #[inline(always)]
    pub fn gpdma1ie(&self) -> GPDMA1IE_R {
        GPDMA1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for FLASH registers
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for FLASH memory
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for OTFDEC1
    #[inline(always)]
    pub fn otfdec1ie(&self) -> OTFDEC1IE_R {
        OTFDEC1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC1 TZSC registers
    #[inline(always)]
    pub fn tzsc1ie(&self) -> TZSC1IE_R {
        TZSC1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC1 TZIC registers
    #[inline(always)]
    pub fn tzic1ie(&self) -> TZIC1IE_R {
        TZIC1IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn octospi1_memie(&self) -> OCTOSPI1_MEMIE_R {
        OCTOSPI1_MEMIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn bkpsramie(&self) -> BKPSRAMIE_R {
        BKPSRAMIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for HSPI1 memory bank
    #[inline(always)]
    pub fn hspi1_memie(&self) -> HSPI1_MEMIE_R {
        HSPI1_MEMIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM1
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB1 registers
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for SRAM2
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for MPCBB2 registers
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for SRAM5
    #[inline(always)]
    pub fn sram5ie(&self) -> SRAM5IE_R {
        SRAM5IE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER4")
            .field("gpdma1ie", &self.gpdma1ie())
            .field("flash_regie", &self.flash_regie())
            .field("flashie", &self.flashie())
            .field("otfdec1ie", &self.otfdec1ie())
            .field("tzsc1ie", &self.tzsc1ie())
            .field("tzic1ie", &self.tzic1ie())
            .field("octospi1_memie", &self.octospi1_memie())
            .field("bkpsramie", &self.bkpsramie())
            .field("hspi1_memie", &self.hspi1_memie())
            .field("sram1ie", &self.sram1ie())
            .field("mpcbb1_regie", &self.mpcbb1_regie())
            .field("sram2ie", &self.sram2ie())
            .field("mpcbb2_regie", &self.mpcbb2_regie())
            .field("sram5ie", &self.sram5ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for GPDMA1
    #[inline(always)]
    pub fn gpdma1ie(&mut self) -> GPDMA1IE_W<IER4rs> {
        GPDMA1IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for FLASH registers
    #[inline(always)]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W<IER4rs> {
        FLASH_REGIE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for FLASH memory
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W<IER4rs> {
        FLASHIE_W::new(self, 2)
    }
    ///Bit 3 - illegal access interrupt enable for OTFDEC1
    #[inline(always)]
    pub fn otfdec1ie(&mut self) -> OTFDEC1IE_W<IER4rs> {
        OTFDEC1IE_W::new(self, 3)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC1 TZSC registers
    #[inline(always)]
    pub fn tzsc1ie(&mut self) -> TZSC1IE_W<IER4rs> {
        TZSC1IE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC1 TZIC registers
    #[inline(always)]
    pub fn tzic1ie(&mut self) -> TZIC1IE_W<IER4rs> {
        TZIC1IE_W::new(self, 15)
    }
    ///Bit 16 - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn octospi1_memie(&mut self) -> OCTOSPI1_MEMIE_W<IER4rs> {
        OCTOSPI1_MEMIE_W::new(self, 16)
    }
    ///Bit 18 - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn bkpsramie(&mut self) -> BKPSRAMIE_W<IER4rs> {
        BKPSRAMIE_W::new(self, 18)
    }
    ///Bit 20 - illegal access interrupt enable for HSPI1 memory bank
    #[inline(always)]
    pub fn hspi1_memie(&mut self) -> HSPI1_MEMIE_W<IER4rs> {
        HSPI1_MEMIE_W::new(self, 20)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM1
    #[inline(always)]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<IER4rs> {
        SRAM1IE_W::new(self, 24)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB1 registers
    #[inline(always)]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W<IER4rs> {
        MPCBB1_REGIE_W::new(self, 25)
    }
    ///Bit 26 - illegal access interrupt enable for SRAM2
    #[inline(always)]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<IER4rs> {
        SRAM2IE_W::new(self, 26)
    }
    ///Bit 27 - illegal access interrupt enable for MPCBB2 registers
    #[inline(always)]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W<IER4rs> {
        MPCBB2_REGIE_W::new(self, 27)
    }
    ///Bit 30 - illegal access interrupt enable for SRAM5
    #[inline(always)]
    pub fn sram5ie(&mut self) -> SRAM5IE_W<IER4rs> {
        SRAM5IE_W::new(self, 30)
    }
}
/**TZIC interrupt enable register 4

You can [`read`](crate::Reg::read) this register and get [`ier4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#GTZC1_TZIC:IER4)*/
pub struct IER4rs;
impl crate::RegisterSpec for IER4rs {
    type Ux = u32;
}
///`read()` method returns [`ier4::R`](R) reader structure
impl crate::Readable for IER4rs {}
///`write(|w| ..)` method takes [`ier4::W`](W) writer structure
impl crate::Writable for IER4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER4 to value 0
impl crate::Resettable for IER4rs {}
