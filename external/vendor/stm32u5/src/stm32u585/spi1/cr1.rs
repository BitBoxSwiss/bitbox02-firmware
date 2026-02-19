///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE {
    ///0: Peripheral disabled
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<SPE> for bool {
    #[inline(always)]
    fn from(variant: SPE) -> Self {
        variant as u8 != 0
    }
}
///Field `SPE` reader - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
pub type SPE_R = crate::BitReader<SPE>;
impl SPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPE {
        match self.bits {
            false => SPE::Disabled,
            true => SPE::Enabled,
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE::Enabled
    }
}
///Field `SPE` writer - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::Enabled)
    }
}
/**master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASRX {
    ///0: Automatic suspend in master receive-only mode disabled
    Disabled = 0,
    ///1: Automatic suspend in master receive-only mode enabled
    Enabled = 1,
}
impl From<MASRX> for bool {
    #[inline(always)]
    fn from(variant: MASRX) -> Self {
        variant as u8 != 0
    }
}
///Field `MASRX` reader - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
pub type MASRX_R = crate::BitReader<MASRX>;
impl MASRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MASRX {
        match self.bits {
            false => MASRX::Disabled,
            true => MASRX::Enabled,
        }
    }
    ///Automatic suspend in master receive-only mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASRX::Disabled
    }
    ///Automatic suspend in master receive-only mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASRX::Enabled
    }
}
///Field `MASRX` writer - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG, MASRX>;
impl<'a, REG> MASRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic suspend in master receive-only mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASRX::Disabled)
    }
    ///Automatic suspend in master receive-only mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASRX::Enabled)
    }
}
/**master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTART {
    ///0: Do not start master transfer
    NotStarted = 0,
    ///1: Start master transfer
    Started = 1,
}
impl From<CSTART> for bool {
    #[inline(always)]
    fn from(variant: CSTART) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTART` reader - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
pub type CSTART_R = crate::BitReader<CSTART>;
impl CSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTART {
        match self.bits {
            false => CSTART::NotStarted,
            true => CSTART::Started,
        }
    }
    ///Do not start master transfer
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == CSTART::NotStarted
    }
    ///Start master transfer
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == CSTART::Started
    }
}
///Field `CSTART` writer - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG, CSTART>;
impl<'a, REG> CSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not start master transfer
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(CSTART::NotStarted)
    }
    ///Start master transfer
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(CSTART::Started)
    }
}
/**master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and SPI communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before disabling the SPI or going to Low-power mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSUSPW {
    ///0: Do not request master suspend
    NotRequested = 0,
    ///1: Request master suspend
    Requested = 1,
}
impl From<CSUSPW> for bool {
    #[inline(always)]
    fn from(variant: CSUSPW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSUSP` writer - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and SPI communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before disabling the SPI or going to Low-power mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts.
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG, CSUSPW>;
impl<'a, REG> CSUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not request master suspend
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut crate::W<REG> {
        self.variant(CSUSPW::NotRequested)
    }
    ///Request master suspend
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(CSUSPW::Requested)
    }
}
/**Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDDIR {
    ///0: Receiver in half duplex mode
    Receiver = 0,
    ///1: Transmitter in half duplex mode
    Transmitter = 1,
}
impl From<HDDIR> for bool {
    #[inline(always)]
    fn from(variant: HDDIR) -> Self {
        variant as u8 != 0
    }
}
///Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
pub type HDDIR_R = crate::BitReader<HDDIR>;
impl HDDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDDIR {
        match self.bits {
            false => HDDIR::Receiver,
            true => HDDIR::Transmitter,
        }
    }
    ///Receiver in half duplex mode
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == HDDIR::Receiver
    }
    ///Transmitter in half duplex mode
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == HDDIR::Transmitter
    }
}
///Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG, HDDIR>;
impl<'a, REG> HDDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver in half duplex mode
    #[inline(always)]
    pub fn receiver(self) -> &'a mut crate::W<REG> {
        self.variant(HDDIR::Receiver)
    }
    ///Transmitter in half duplex mode
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut crate::W<REG> {
        self.variant(HDDIR::Transmitter)
    }
}
/**internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSI {
    ///0: 0 is forced onto the SS signal and the I/O value of the SS pin is ignored
    SlaveSelected = 0,
    ///1: 1 is forced onto the SS signal and the I/O value of the SS pin is ignored
    SlaveNotSelected = 1,
}
impl From<SSI> for bool {
    #[inline(always)]
    fn from(variant: SSI) -> Self {
        variant as u8 != 0
    }
}
///Field `SSI` reader - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
pub type SSI_R = crate::BitReader<SSI>;
impl SSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSI {
        match self.bits {
            false => SSI::SlaveSelected,
            true => SSI::SlaveNotSelected,
        }
    }
    ///0 is forced onto the SS signal and the I/O value of the SS pin is ignored
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI::SlaveSelected
    }
    ///1 is forced onto the SS signal and the I/O value of the SS pin is ignored
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI::SlaveNotSelected
    }
}
///Field `SSI` writer - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG, SSI>;
impl<'a, REG> SSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///0 is forced onto the SS signal and the I/O value of the SS pin is ignored
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SSI::SlaveSelected)
    }
    ///1 is forced onto the SS signal and the I/O value of the SS pin is ignored
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SSI::SlaveNotSelected)
    }
}
/**32-bit CRC polynomial configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC33_17 {
    ///0: Full size (33/17 bit) CRC polynomial is not used
    Disabled = 0,
    ///1: Full size (33/17 bit) CRC polynomial is used
    Enabled = 1,
}
impl From<CRC33_17> for bool {
    #[inline(always)]
    fn from(variant: CRC33_17) -> Self {
        variant as u8 != 0
    }
}
///Field `CRC33_17` reader - 32-bit CRC polynomial configuration
pub type CRC33_17_R = crate::BitReader<CRC33_17>;
impl CRC33_17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRC33_17 {
        match self.bits {
            false => CRC33_17::Disabled,
            true => CRC33_17::Enabled,
        }
    }
    ///Full size (33/17 bit) CRC polynomial is not used
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRC33_17::Disabled
    }
    ///Full size (33/17 bit) CRC polynomial is used
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRC33_17::Enabled
    }
}
///Field `CRC33_17` writer - 32-bit CRC polynomial configuration
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG, CRC33_17>;
impl<'a, REG> CRC33_17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full size (33/17 bit) CRC polynomial is not used
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRC33_17::Disabled)
    }
    ///Full size (33/17 bit) CRC polynomial is used
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRC33_17::Enabled)
    }
}
/**CRC calculation initialization pattern control for receiver

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCRCINI {
    ///0: All zeros RX CRC initialization pattern
    AllZeros = 0,
    ///1: All ones RX CRC initialization pattern
    AllOnes = 1,
}
impl From<RCRCINI> for bool {
    #[inline(always)]
    fn from(variant: RCRCINI) -> Self {
        variant as u8 != 0
    }
}
///Field `RCRCINI` reader - CRC calculation initialization pattern control for receiver
pub type RCRCINI_R = crate::BitReader<RCRCINI>;
impl RCRCINI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCRCINI {
        match self.bits {
            false => RCRCINI::AllZeros,
            true => RCRCINI::AllOnes,
        }
    }
    ///All zeros RX CRC initialization pattern
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == RCRCINI::AllZeros
    }
    ///All ones RX CRC initialization pattern
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == RCRCINI::AllOnes
    }
}
///Field `RCRCINI` writer - CRC calculation initialization pattern control for receiver
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG, RCRCINI>;
impl<'a, REG> RCRCINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All zeros RX CRC initialization pattern
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut crate::W<REG> {
        self.variant(RCRCINI::AllZeros)
    }
    ///All ones RX CRC initialization pattern
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut crate::W<REG> {
        self.variant(RCRCINI::AllOnes)
    }
}
/**CRC calculation initialization pattern control for transmitter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRCINI {
    ///0: All zeros TX CRC initialization pattern
    AllZeros = 0,
    ///1: All ones TX CRC initialization pattern
    AllOnes = 1,
}
impl From<TCRCINI> for bool {
    #[inline(always)]
    fn from(variant: TCRCINI) -> Self {
        variant as u8 != 0
    }
}
///Field `TCRCINI` reader - CRC calculation initialization pattern control for transmitter
pub type TCRCINI_R = crate::BitReader<TCRCINI>;
impl TCRCINI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCRCINI {
        match self.bits {
            false => TCRCINI::AllZeros,
            true => TCRCINI::AllOnes,
        }
    }
    ///All zeros TX CRC initialization pattern
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == TCRCINI::AllZeros
    }
    ///All ones TX CRC initialization pattern
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == TCRCINI::AllOnes
    }
}
///Field `TCRCINI` writer - CRC calculation initialization pattern control for transmitter
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG, TCRCINI>;
impl<'a, REG> TCRCINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All zeros TX CRC initialization pattern
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut crate::W<REG> {
        self.variant(TCRCINI::AllZeros)
    }
    ///All ones TX CRC initialization pattern
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut crate::W<REG> {
        self.variant(TCRCINI::AllOnes)
    }
}
/**locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOLOCK {
    ///0: IO configuration unlocked
    Unlocked = 0,
    ///1: IO configuration locked
    Locked = 1,
}
impl From<IOLOCK> for bool {
    #[inline(always)]
    fn from(variant: IOLOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `IOLOCK` reader - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.
pub type IOLOCK_R = crate::BitReader<IOLOCK>;
impl IOLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOLOCK {
        match self.bits {
            false => IOLOCK::Unlocked,
            true => IOLOCK::Locked,
        }
    }
    ///IO configuration unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == IOLOCK::Unlocked
    }
    ///IO configuration locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == IOLOCK::Locked
    }
}
///Field `IOLOCK` writer - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG, IOLOCK>;
impl<'a, REG> IOLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IO configuration unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(IOLOCK::Unlocked)
    }
    ///IO configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(IOLOCK::Locked)
    }
}
impl R {
    ///Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 32-bit CRC polynomial configuration
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CRC calculation initialization pattern control for receiver
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CRC calculation initialization pattern control for transmitter
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("spe", &self.spe())
            .field("masrx", &self.masrx())
            .field("cstart", &self.cstart())
            .field("hddir", &self.hddir())
            .field("ssi", &self.ssi())
            .field("crc33_17", &self.crc33_17())
            .field("rcrcini", &self.rcrcini())
            .field("tcrcini", &self.tcrcini())
            .field("iolock", &self.iolock())
            .finish()
    }
}
impl W {
    ///Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<CR1rs> {
        SPE_W::new(self, 0)
    }
    ///Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W<CR1rs> {
        MASRX_W::new(self, 8)
    }
    ///Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W<CR1rs> {
        CSTART_W::new(self, 9)
    }
    ///Bit 10 - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and SPI communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before disabling the SPI or going to Low-power mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts.
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W<CR1rs> {
        CSUSP_W::new(self, 10)
    }
    ///Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W<CR1rs> {
        HDDIR_W::new(self, 11)
    }
    ///Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<CR1rs> {
        SSI_W::new(self, 12)
    }
    ///Bit 13 - 32-bit CRC polynomial configuration
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W<CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    ///Bit 14 - CRC calculation initialization pattern control for receiver
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W<CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    ///Bit 15 - CRC calculation initialization pattern control for transmitter
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W<CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    ///Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W<CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SPI1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
