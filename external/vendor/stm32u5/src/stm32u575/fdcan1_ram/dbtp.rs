///Register `DBTP` reader
pub type R = crate::R<DBTPrs>;
///Register `DBTP` writer
pub type W = crate::W<DBTPrs>;
///Field `DSJW` reader - Synchronization Jump Width
pub type DSJW_R = crate::FieldReader;
///Field `DSJW` writer - Synchronization Jump Width
pub type DSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DTSEG2` reader - Data time segment after sample point
pub type DTSEG2_R = crate::FieldReader;
///Field `DTSEG2` writer - Data time segment after sample point
pub type DTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DTSEG1` reader - Data time segment after sample point
pub type DTSEG1_R = crate::FieldReader;
///Field `DTSEG1` writer - Data time segment after sample point
pub type DTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DBRP` reader - Data BIt Rate Prescaler
pub type DBRP_R = crate::FieldReader;
///Field `DBRP` writer - Data BIt Rate Prescaler
pub type DBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TDC` reader - Transceiver Delay Compensation
pub type TDC_R = crate::BitReader;
///Field `TDC` writer - Transceiver Delay Compensation
pub type TDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Synchronization Jump Width
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:12 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Data BIt Rate Prescaler
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - Transceiver Delay Compensation
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBTP")
            .field("dsjw", &self.dsjw())
            .field("dtseg2", &self.dtseg2())
            .field("dtseg1", &self.dtseg1())
            .field("dbrp", &self.dbrp())
            .field("tdc", &self.tdc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Synchronization Jump Width
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W<DBTPrs> {
        DSJW_W::new(self, 0)
    }
    ///Bits 4:7 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W<DBTPrs> {
        DTSEG2_W::new(self, 4)
    }
    ///Bits 8:12 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W<DBTPrs> {
        DTSEG1_W::new(self, 8)
    }
    ///Bits 16:20 - Data BIt Rate Prescaler
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W<DBTPrs> {
        DBRP_W::new(self, 16)
    }
    ///Bit 23 - Transceiver Delay Compensation
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W<DBTPrs> {
        TDC_W::new(self, 23)
    }
}
/**FDCAN Data Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FDCAN1_RAM:DBTP)*/
pub struct DBTPrs;
impl crate::RegisterSpec for DBTPrs {
    type Ux = u32;
}
///`read()` method returns [`dbtp::R`](R) reader structure
impl crate::Readable for DBTPrs {}
///`write(|w| ..)` method takes [`dbtp::W`](W) writer structure
impl crate::Writable for DBTPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBTP to value 0x0a33
impl crate::Resettable for DBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
