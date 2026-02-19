///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
/**GPDMA1 reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<GPDMA1RST> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1RST` reader - GPDMA1 reset This bit is set and cleared by software.
pub type GPDMA1RST_R = crate::BitReader<GPDMA1RST>;
impl GPDMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1RST {
        match self.bits {
            false => GPDMA1RST::NoEffect,
            true => GPDMA1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GPDMA1RST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPDMA1RST::Reset
    }
}
///Field `GPDMA1RST` writer - GPDMA1 reset This bit is set and cleared by software.
pub type GPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1RST>;
impl<'a, REG> GPDMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1RST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1RST::Reset)
    }
}
///Field `CORDICRST` reader - CORDIC reset This bit is set and cleared by software.
pub use GPDMA1RST_R as CORDICRST_R;
///Field `FMACRST` reader - FMAC reset This bit is set and cleared by software.
pub use GPDMA1RST_R as FMACRST_R;
///Field `MDF1RST` reader - MDF1 reset This bit is set and cleared by software.
pub use GPDMA1RST_R as MDF1RST_R;
///Field `CRCRST` reader - CRC reset This bit is set and cleared by software.
pub use GPDMA1RST_R as CRCRST_R;
///Field `JPEGRST` reader - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_R as JPEGRST_R;
///Field `TSCRST` reader - TSC reset This bit is set and cleared by software.
pub use GPDMA1RST_R as TSCRST_R;
///Field `RAMCFGRST` reader - RAMCFG reset This bit is set and cleared by software.
pub use GPDMA1RST_R as RAMCFGRST_R;
///Field `DMA2DRST` reader - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_R as DMA2DRST_R;
///Field `GFXMMURST` reader - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_R as GFXMMURST_R;
///Field `GPU2DRST` reader - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_R as GPU2DRST_R;
///Field `CORDICRST` writer - CORDIC reset This bit is set and cleared by software.
pub use GPDMA1RST_W as CORDICRST_W;
///Field `FMACRST` writer - FMAC reset This bit is set and cleared by software.
pub use GPDMA1RST_W as FMACRST_W;
///Field `MDF1RST` writer - MDF1 reset This bit is set and cleared by software.
pub use GPDMA1RST_W as MDF1RST_W;
///Field `CRCRST` writer - CRC reset This bit is set and cleared by software.
pub use GPDMA1RST_W as CRCRST_W;
///Field `JPEGRST` writer - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_W as JPEGRST_W;
///Field `TSCRST` writer - TSC reset This bit is set and cleared by software.
pub use GPDMA1RST_W as TSCRST_W;
///Field `RAMCFGRST` writer - RAMCFG reset This bit is set and cleared by software.
pub use GPDMA1RST_W as RAMCFGRST_W;
///Field `DMA2DRST` writer - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_W as DMA2DRST_W;
///Field `GFXMMURST` writer - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_W as GFXMMURST_W;
///Field `GPU2DRST` writer - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPDMA1RST_W as GPU2DRST_W;
impl R {
    ///Bit 0 - GPDMA1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CORDIC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FMAC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MDF1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn mdf1rst(&self) -> MDF1RST_R {
        MDF1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 12 - CRC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn jpegrst(&self) -> JPEGRST_R {
        JPEGRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TSC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpu2drst(&self) -> GPU2DRST_R {
        GPU2DRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("gpdma1rst", &self.gpdma1rst())
            .field("cordicrst", &self.cordicrst())
            .field("fmacrst", &self.fmacrst())
            .field("mdf1rst", &self.mdf1rst())
            .field("crcrst", &self.crcrst())
            .field("jpegrst", &self.jpegrst())
            .field("tscrst", &self.tscrst())
            .field("ramcfgrst", &self.ramcfgrst())
            .field("dma2drst", &self.dma2drst())
            .field("gfxmmurst", &self.gfxmmurst())
            .field("gpu2drst", &self.gpu2drst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<AHB1RSTRrs> {
        GPDMA1RST_W::new(self, 0)
    }
    ///Bit 1 - CORDIC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn cordicrst(&mut self) -> CORDICRST_W<AHB1RSTRrs> {
        CORDICRST_W::new(self, 1)
    }
    ///Bit 2 - FMAC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn fmacrst(&mut self) -> FMACRST_W<AHB1RSTRrs> {
        FMACRST_W::new(self, 2)
    }
    ///Bit 3 - MDF1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn mdf1rst(&mut self) -> MDF1RST_W<AHB1RSTRrs> {
        MDF1RST_W::new(self, 3)
    }
    ///Bit 12 - CRC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 15 - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn jpegrst(&mut self) -> JPEGRST_W<AHB1RSTRrs> {
        JPEGRST_W::new(self, 15)
    }
    ///Bit 16 - TSC reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W<AHB1RSTRrs> {
        TSCRST_W::new(self, 16)
    }
    ///Bit 17 - RAMCFG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<AHB1RSTRrs> {
        RAMCFGRST_W::new(self, 17)
    }
    ///Bit 18 - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<AHB1RSTRrs> {
        DMA2DRST_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<AHB1RSTRrs> {
        GFXMMURST_W::new(self, 19)
    }
    ///Bit 20 - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpu2drst(&mut self) -> GPU2DRST_W<AHB1RSTRrs> {
        GPU2DRST_W::new(self, 20)
    }
}
/**RCC AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB1RSTR)*/
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1rstr::R`](R) reader structure
impl crate::Readable for AHB1RSTRrs {}
///`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTRrs {}
