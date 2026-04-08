///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
/**GPDMA1 clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1EN {
    ///0: Peripheral clock disabled
    Disabled = 0,
    ///1: Peripheral clock enabled
    Enabled = 1,
}
impl From<GPDMA1EN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1EN` reader - GPDMA1 clock enable This bit is set and cleared by software.
pub type GPDMA1EN_R = crate::BitReader<GPDMA1EN>;
impl GPDMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1EN {
        match self.bits {
            false => GPDMA1EN::Disabled,
            true => GPDMA1EN::Enabled,
        }
    }
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1EN::Disabled
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1EN::Enabled
    }
}
///Field `GPDMA1EN` writer - GPDMA1 clock enable This bit is set and cleared by software.
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1EN>;
impl<'a, REG> GPDMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Disabled)
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Enabled)
    }
}
///Field `CORDICEN` reader - CORDIC clock enable This bit is set and cleared by software.
pub use GPDMA1EN_R as CORDICEN_R;
///Field `FMACEN` reader - FMAC clock enable This bit is set and reset by software.
pub use GPDMA1EN_R as FMACEN_R;
///Field `MDF1EN` reader - MDF1 clock enable This bit is set and reset by software.
pub use GPDMA1EN_R as MDF1EN_R;
///Field `FLASHEN` reader - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode.
pub use GPDMA1EN_R as FLASHEN_R;
///Field `CRCEN` reader - CRC clock enable This bit is set and cleared by software.
pub use GPDMA1EN_R as CRCEN_R;
///Field `JPEGEN` reader - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_R as JPEGEN_R;
///Field `TSCEN` reader - Touch sensing controller clock enable This bit is set and cleared by software.
pub use GPDMA1EN_R as TSCEN_R;
///Field `RAMCFGEN` reader - RAMCFG clock enable This bit is set and cleared by software.
pub use GPDMA1EN_R as RAMCFGEN_R;
///Field `DMA2DEN` reader - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_R as DMA2DEN_R;
///Field `GFXMMUEN` reader - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_R as GFXMMUEN_R;
///Field `GPU2DEN` reader - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_R as GPU2DEN_R;
///Field `DCACHE2EN` reader - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_R as DCACHE2EN_R;
///Field `GTZC1EN` reader - GTZC1 clock enable This bit is set and reset by software.
pub use GPDMA1EN_R as GTZC1EN_R;
///Field `BKPSRAMEN` reader - BKPSRAM clock enable This bit is set and reset by software.
pub use GPDMA1EN_R as BKPSRAMEN_R;
///Field `DCACHE1EN` reader - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed.
pub use GPDMA1EN_R as DCACHE1EN_R;
///Field `SRAM1EN` reader - SRAM1 clock enable This bit is set and reset by software.
pub use GPDMA1EN_R as SRAM1EN_R;
///Field `CORDICEN` writer - CORDIC clock enable This bit is set and cleared by software.
pub use GPDMA1EN_W as CORDICEN_W;
///Field `FMACEN` writer - FMAC clock enable This bit is set and reset by software.
pub use GPDMA1EN_W as FMACEN_W;
///Field `MDF1EN` writer - MDF1 clock enable This bit is set and reset by software.
pub use GPDMA1EN_W as MDF1EN_W;
///Field `FLASHEN` writer - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode.
pub use GPDMA1EN_W as FLASHEN_W;
///Field `CRCEN` writer - CRC clock enable This bit is set and cleared by software.
pub use GPDMA1EN_W as CRCEN_W;
///Field `JPEGEN` writer - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_W as JPEGEN_W;
///Field `TSCEN` writer - Touch sensing controller clock enable This bit is set and cleared by software.
pub use GPDMA1EN_W as TSCEN_W;
///Field `RAMCFGEN` writer - RAMCFG clock enable This bit is set and cleared by software.
pub use GPDMA1EN_W as RAMCFGEN_W;
///Field `DMA2DEN` writer - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_W as DMA2DEN_W;
///Field `GFXMMUEN` writer - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_W as GFXMMUEN_W;
///Field `GPU2DEN` writer - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_W as GPU2DEN_W;
///Field `DCACHE2EN` writer - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1EN_W as DCACHE2EN_W;
///Field `GTZC1EN` writer - GTZC1 clock enable This bit is set and reset by software.
pub use GPDMA1EN_W as GTZC1EN_W;
///Field `BKPSRAMEN` writer - BKPSRAM clock enable This bit is set and reset by software.
pub use GPDMA1EN_W as BKPSRAMEN_W;
///Field `DCACHE1EN` writer - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed.
pub use GPDMA1EN_W as DCACHE1EN_W;
///Field `SRAM1EN` writer - SRAM1 clock enable This bit is set and reset by software.
pub use GPDMA1EN_W as SRAM1EN_W;
impl R {
    ///Bit 0 - GPDMA1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CORDIC clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FMAC clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MDF1 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn mdf1en(&self) -> MDF1EN_R {
        MDF1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode.
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn jpegen(&self) -> JPEGEN_R {
        JPEGEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Touch sensing controller clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpu2den(&self) -> GPU2DEN_R {
        GPU2DEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dcache2en(&self) -> DCACHE2EN_R {
        DCACHE2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - GTZC1 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn gtzc1en(&self) -> GTZC1EN_R {
        GTZC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - BKPSRAM clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed.
    #[inline(always)]
    pub fn dcache1en(&self) -> DCACHE1EN_R {
        DCACHE1EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM1 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("gpdma1en", &self.gpdma1en())
            .field("cordicen", &self.cordicen())
            .field("fmacen", &self.fmacen())
            .field("mdf1en", &self.mdf1en())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .field("jpegen", &self.jpegen())
            .field("tscen", &self.tscen())
            .field("ramcfgen", &self.ramcfgen())
            .field("dma2den", &self.dma2den())
            .field("gfxmmuen", &self.gfxmmuen())
            .field("gpu2den", &self.gpu2den())
            .field("dcache2en", &self.dcache2en())
            .field("gtzc1en", &self.gtzc1en())
            .field("bkpsramen", &self.bkpsramen())
            .field("dcache1en", &self.dcache1en())
            .field("sram1en", &self.sram1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<AHB1ENRrs> {
        GPDMA1EN_W::new(self, 0)
    }
    ///Bit 1 - CORDIC clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cordicen(&mut self) -> CORDICEN_W<AHB1ENRrs> {
        CORDICEN_W::new(self, 1)
    }
    ///Bit 2 - FMAC clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn fmacen(&mut self) -> FMACEN_W<AHB1ENRrs> {
        FMACEN_W::new(self, 2)
    }
    ///Bit 3 - MDF1 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn mdf1en(&mut self) -> MDF1EN_W<AHB1ENRrs> {
        MDF1EN_W::new(self, 3)
    }
    ///Bit 8 - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode.
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 15 - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn jpegen(&mut self) -> JPEGEN_W<AHB1ENRrs> {
        JPEGEN_W::new(self, 15)
    }
    ///Bit 16 - Touch sensing controller clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
    ///Bit 17 - RAMCFG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<AHB1ENRrs> {
        RAMCFGEN_W::new(self, 17)
    }
    ///Bit 18 - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W<AHB1ENRrs> {
        DMA2DEN_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<AHB1ENRrs> {
        GFXMMUEN_W::new(self, 19)
    }
    ///Bit 20 - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpu2den(&mut self) -> GPU2DEN_W<AHB1ENRrs> {
        GPU2DEN_W::new(self, 20)
    }
    ///Bit 21 - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dcache2en(&mut self) -> DCACHE2EN_W<AHB1ENRrs> {
        DCACHE2EN_W::new(self, 21)
    }
    ///Bit 24 - GTZC1 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn gtzc1en(&mut self) -> GTZC1EN_W<AHB1ENRrs> {
        GTZC1EN_W::new(self, 24)
    }
    ///Bit 28 - BKPSRAM clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<AHB1ENRrs> {
        BKPSRAMEN_W::new(self, 28)
    }
    ///Bit 30 - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed.
    #[inline(always)]
    pub fn dcache1en(&mut self) -> DCACHE1EN_W<AHB1ENRrs> {
        DCACHE1EN_W::new(self, 30)
    }
    ///Bit 31 - SRAM1 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn sram1en(&mut self) -> SRAM1EN_W<AHB1ENRrs> {
        SRAM1EN_W::new(self, 31)
    }
}
/**RCC AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0xd020_0100
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0xd020_0100;
}
