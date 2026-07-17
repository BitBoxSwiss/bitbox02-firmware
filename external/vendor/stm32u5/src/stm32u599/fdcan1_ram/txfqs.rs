///Register `TXFQS` reader
pub type R = crate::R<TXFQSrs>;
///Field `TFFL` reader - Tx FIFO Free Level
pub type TFFL_R = crate::FieldReader;
///Field `TFGI` reader - TFGI
pub type TFGI_R = crate::FieldReader;
///Field `TFQPI` reader - Tx FIFO/Queue Put Index
pub type TFQPI_R = crate::FieldReader;
///Field `TFQF` reader - Tx FIFO/Queue Full
pub type TFQF_R = crate::BitReader;
impl R {
    ///Bits 0:2 - Tx FIFO Free Level
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - TFGI
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Tx FIFO/Queue Put Index
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 21 - Tx FIFO/Queue Full
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFQS")
            .field("tffl", &self.tffl())
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
/**FDCAN Tx FIFO/Queue Status Register

You can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FDCAN1_RAM:TXFQS)*/
pub struct TXFQSrs;
impl crate::RegisterSpec for TXFQSrs {
    type Ux = u32;
}
///`read()` method returns [`txfqs::R`](R) reader structure
impl crate::Readable for TXFQSrs {}
///`reset()` method sets TXFQS to value 0x03
impl crate::Resettable for TXFQSrs {
    const RESET_VALUE: u32 = 0x03;
}
