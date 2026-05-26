///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
/**end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTCW {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<EOTCW> for bool {
    #[inline(always)]
    fn from(variant: EOTCW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOTC` writer - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register
pub type EOTC_W<'a, REG> = crate::BitWriter1C<'a, REG, EOTCW>;
impl<'a, REG> EOTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOTCW::Clear)
    }
}
///Field `TXTFC` writer - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register
pub use EOTC_W as TXTFC_W;
///Field `UDRC` writer - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register
pub use EOTC_W as UDRC_W;
///Field `OVRC` writer - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register
pub use EOTC_W as OVRC_W;
///Field `CRCEC` writer - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register
pub use EOTC_W as CRCEC_W;
///Field `TIFREC` writer - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register
pub use EOTC_W as TIFREC_W;
///Field `MODFC` writer - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register
pub use EOTC_W as MODFC_W;
///Field `SUSPC` writer - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register
pub use EOTC_W as SUSPC_W;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W<IFCRrs> {
        EOTC_W::new(self, 3)
    }
    ///Bit 4 - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W<IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    ///Bit 5 - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W<IFCRrs> {
        UDRC_W::new(self, 5)
    }
    ///Bit 6 - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W<IFCRrs> {
        OVRC_W::new(self, 6)
    }
    ///Bit 7 - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W<IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    ///Bit 8 - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W<IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    ///Bit 9 - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W<IFCRrs> {
        MODFC_W::new(self, 9)
    }
    ///Bit 11 - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W<IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SPI1:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0bf8;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
