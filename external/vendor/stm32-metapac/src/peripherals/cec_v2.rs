#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CEC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cec {
    ptr: *mut u8,
}
unsafe impl Send for Cec {}
unsafe impl Sync for Cec {}
impl Cec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CEC control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "CEC Tx data register."]
    #[inline(always)]
    pub const fn txdr(self) -> crate::common::Reg<regs::Txdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CEC Rx Data Register."]
    #[inline(always)]
    pub const fn rxdr(self) -> crate::common::Reg<regs::Rxdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CEC Interrupt and Status Register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CEC interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Signal Free Time SFT bits are set by software. In the SFT=0x0 configuration the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. * 0x0 ** 2.5 Data-Bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST=1, TXERR=1, TXUDR=1 or TXACKE= 1) ** 4 Data-Bit periods if CEC is the new bus initiator ** 6 Data-Bit periods if CEC is the last bus initiator with successful transmission (TXEOM=1) * 0x1: 0.5 nominal data bit periods * 0x2: 1.5 nominal data bit periods * 0x3: 2.5 nominal data bit periods * 0x4: 3.5 nominal data bit periods * 0x5: 4.5 nominal data bit periods * 0x6: 5.5 nominal data bit periods * 0x7: 6.5 nominal data bit periods."]
        #[inline(always)]
        pub const fn sft(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Signal Free Time SFT bits are set by software. In the SFT=0x0 configuration the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. * 0x0 ** 2.5 Data-Bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST=1, TXERR=1, TXUDR=1 or TXACKE= 1) ** 4 Data-Bit periods if CEC is the new bus initiator ** 6 Data-Bit periods if CEC is the last bus initiator with successful transmission (TXEOM=1) * 0x1: 0.5 nominal data bit periods * 0x2: 1.5 nominal data bit periods * 0x3: 2.5 nominal data bit periods * 0x4: 3.5 nominal data bit periods * 0x5: 4.5 nominal data bit periods * 0x6: 5.5 nominal data bit periods * 0x7: 6.5 nominal data bit periods."]
        #[inline(always)]
        pub fn set_sft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Rx-Tolerance The RXTOL bit is set and cleared by software. ** Start-Bit, +/- 200 s rise, +/- 200 s fall. ** Data-Bit: +/- 200 s rise. +/- 350 s fall. ** Start-Bit: +/- 400 s rise, +/- 400 s fall ** Data-Bit: +/-300 s rise, +/- 500 s fall."]
        #[inline(always)]
        pub const fn rxtol(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Tolerance The RXTOL bit is set and cleared by software. ** Start-Bit, +/- 200 s rise, +/- 200 s fall. ** Data-Bit: +/- 200 s rise. +/- 350 s fall. ** Start-Bit: +/- 400 s rise, +/- 400 s fall ** Data-Bit: +/-300 s rise, +/- 500 s fall."]
        #[inline(always)]
        pub fn set_rxtol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rx-Stop on Bit Rising Error The BRESTP bit is set and cleared by software."]
        #[inline(always)]
        pub const fn brestp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Stop on Bit Rising Error The BRESTP bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_brestp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Generate Error-Bit on Bit Rising Error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon BRE detection with BRESTP=1 in broadcast even if BREGEN=0."]
        #[inline(always)]
        pub const fn bregen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Generate Error-Bit on Bit Rising Error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon BRE detection with BRESTP=1 in broadcast even if BREGEN=0."]
        #[inline(always)]
        pub fn set_bregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Generate Error-Bit on Long Bit Period Error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon LBPE detection in broadcast even if LBPEGEN=0."]
        #[inline(always)]
        pub const fn lbpegen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Generate Error-Bit on Long Bit Period Error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon LBPE detection in broadcast even if LBPEGEN=0."]
        #[inline(always)]
        pub fn set_lbpegen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Avoid Error-Bit Generation in Broadcast The BRDNOGEN bit is set and cleared by software."]
        #[inline(always)]
        pub const fn brdnogen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Avoid Error-Bit Generation in Broadcast The BRDNOGEN bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_brdnogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SFT Option Bit The SFTOPT bit is set and cleared by software."]
        #[inline(always)]
        pub const fn sftopt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SFT Option Bit The SFTOPT bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_sftopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN=1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
        #[inline(always)]
        pub const fn oar(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN=1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
        #[inline(always)]
        pub fn set_oar(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
        #[doc = "Listen mode LSTN bit is set and cleared by software."]
        #[inline(always)]
        pub const fn lstn(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Listen mode LSTN bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_lstn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("sft", &self.sft())
                .field("rxtol", &self.rxtol())
                .field("brestp", &self.brestp())
                .field("bregen", &self.bregen())
                .field("lbpegen", &self.lbpegen())
                .field("brdnogen", &self.brdnogen())
                .field("sftopt", &self.sftopt())
                .field("oar", &self.oar())
                .field("lstn", &self.lstn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ sft: {=u8:?}, rxtol: {=bool:?}, brestp: {=bool:?}, bregen: {=bool:?}, lbpegen: {=bool:?}, brdnogen: {=bool:?}, sftopt: {=bool:?}, oar: {=u16:?}, lstn: {=bool:?} }}" , self . sft () , self . rxtol () , self . brestp () , self . bregen () , self . lbpegen () , self . brdnogen () , self . sftopt () , self . oar () , self . lstn ())
        }
    }
    #[doc = "CEC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "CEC Enable The CECEN bit is set and cleared by software. CECEN=1 starts message reception and enables the TXSOM control. CECEN=0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CEC Enable The CECEN bit is set and cleared by software. CECEN=1 starts message reception and enables the TXSOM control. CECEN=0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
        #[inline(always)]
        pub fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tx Start Of Message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-Bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission will start after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND=1), in case of transmission underrun (TXUDR=1), negative acknowledge (TXACKE=1), and transmission error (TXERR=1). It is also cleared by CECEN=0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST=1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN=1 TXSOM must be set when transmission data is available into TXDR HEADERs first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR which is used only for reception."]
        #[inline(always)]
        pub const fn txsom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Start Of Message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-Bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission will start after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND=1), in case of transmission underrun (TXUDR=1), negative acknowledge (TXACKE=1), and transmission error (TXERR=1). It is also cleared by CECEN=0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST=1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN=1 TXSOM must be set when transmission data is available into TXDR HEADERs first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR which is used only for reception."]
        #[inline(always)]
        pub fn set_txsom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Tx End Of Message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN=1 TXEOM must be set before writing transmission data to TXDR If TXEOM is set when TXSOM=0, transmitted message will consist of 1 byte (HEADER) only (PING message)."]
        #[inline(always)]
        pub const fn txeom(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Tx End Of Message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN=1 TXEOM must be set before writing transmission data to TXDR If TXEOM is set when TXSOM=0, transmitted message will consist of 1 byte (HEADER) only (PING message)."]
        #[inline(always)]
        pub fn set_txeom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("cecen", &self.cecen())
                .field("txsom", &self.txsom())
                .field("txeom", &self.txeom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ cecen: {=bool:?}, txsom: {=bool:?}, txeom: {=bool:?} }}",
                self.cecen(),
                self.txsom(),
                self.txeom()
            )
        }
    }
    #[doc = "CEC interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn rxbrie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_rxbrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn rxendie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_rxendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn rxovrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_rxovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn breie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_breie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn sbpeie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_sbpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn lbpeie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_lbpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn rxackie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_rxackie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn arblstie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_arblstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn txbrie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_txbrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn txendie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_txendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn txudrie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_txudrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn txerrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_txerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn txackie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_txackie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("rxbrie", &self.rxbrie())
                .field("rxendie", &self.rxendie())
                .field("rxovrie", &self.rxovrie())
                .field("breie", &self.breie())
                .field("sbpeie", &self.sbpeie())
                .field("lbpeie", &self.lbpeie())
                .field("rxackie", &self.rxackie())
                .field("arblstie", &self.arblstie())
                .field("txbrie", &self.txbrie())
                .field("txendie", &self.txendie())
                .field("txudrie", &self.txudrie())
                .field("txerrie", &self.txerrie())
                .field("txackie", &self.txackie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ rxbrie: {=bool:?}, rxendie: {=bool:?}, rxovrie: {=bool:?}, breie: {=bool:?}, sbpeie: {=bool:?}, lbpeie: {=bool:?}, rxackie: {=bool:?}, arblstie: {=bool:?}, txbrie: {=bool:?}, txendie: {=bool:?}, txudrie: {=bool:?}, txerrie: {=bool:?}, txackie: {=bool:?} }}" , self . rxbrie () , self . rxendie () , self . rxovrie () , self . breie () , self . sbpeie () , self . lbpeie () , self . rxackie () , self . arblstie () , self . txbrie () , self . txendie () , self . txudrie () , self . txerrie () , self . txackie ())
        }
    }
    #[doc = "CEC Interrupt and Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Rx-Byte Received The RXBR bit is set by hardware to inform application that a new byte has been received from the CEC line and stored into the RXD buffer. RXBR is cleared by software write at 1."]
        #[inline(always)]
        pub const fn rxbr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Byte Received The RXBR bit is set by hardware to inform application that a new byte has been received from the CEC line and stored into the RXD buffer. RXBR is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_rxbr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End Of Reception RXEND is set by hardware to inform application that the last byte of a CEC message is received from the CEC line and stored into the RXD buffer. RXEND is set at the same time of RXBR. RXEND is cleared by software write at 1."]
        #[inline(always)]
        pub const fn rxend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End Of Reception RXEND is set by hardware to inform application that the last byte of a CEC message is received from the CEC line and stored into the RXD buffer. RXEND is set at the same time of RXBR. RXEND is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_rxend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rx-Overrun RXOVR is set by hardware if RXBR is not yet cleared at the time a new byte is received on the CEC line and stored into RXD. RXOVR assertion stops message reception so that no acknowledge is sent. In case of broadcast, a negative acknowledge is sent. RXOVR is cleared by software write at 1."]
        #[inline(always)]
        pub const fn rxovr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Overrun RXOVR is set by hardware if RXBR is not yet cleared at the time a new byte is received on the CEC line and stored into RXD. RXOVR assertion stops message reception so that no acknowledge is sent. In case of broadcast, a negative acknowledge is sent. RXOVR is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_rxovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Rx-Bit Rising Error BRE is set by hardware in case a Data-Bit waveform is detected with Bit Rising Error. BRE is set either at the time the misplaced rising edge occurs, or at the end of the maximum BRE tolerance allowed by RXTOL, in case rising edge is still longing. BRE stops message reception if BRESTP=1. BRE generates an Error-Bit on the CEC line if BREGEN=1. BRE is cleared by software write at 1."]
        #[inline(always)]
        pub const fn bre(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Bit Rising Error BRE is set by hardware in case a Data-Bit waveform is detected with Bit Rising Error. BRE is set either at the time the misplaced rising edge occurs, or at the end of the maximum BRE tolerance allowed by RXTOL, in case rising edge is still longing. BRE stops message reception if BRESTP=1. BRE generates an Error-Bit on the CEC line if BREGEN=1. BRE is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_bre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rx-Short Bit Period Error SBPE is set by hardware in case a Data-Bit waveform is detected with Short Bit Period Error. SBPE is set at the time the anticipated falling edge occurs. SBPE generates an Error-Bit on the CEC line. SBPE is cleared by software write at 1."]
        #[inline(always)]
        pub const fn sbpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Short Bit Period Error SBPE is set by hardware in case a Data-Bit waveform is detected with Short Bit Period Error. SBPE is set at the time the anticipated falling edge occurs. SBPE generates an Error-Bit on the CEC line. SBPE is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_sbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx-Long Bit Period Error LBPE is set by hardware in case a Data-Bit waveform is detected with Long Bit Period Error. LBPE is set at the end of the maximum bit-extension tolerance allowed by RXTOL, in case falling edge is still longing. LBPE always stops reception of the CEC message. LBPE generates an Error-Bit on the CEC line if LBPEGEN=1. In case of broadcast, Error-Bit is generated even in case of LBPEGEN=0. LBPE is cleared by software write at 1."]
        #[inline(always)]
        pub const fn lbpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Long Bit Period Error LBPE is set by hardware in case a Data-Bit waveform is detected with Long Bit Period Error. LBPE is set at the end of the maximum bit-extension tolerance allowed by RXTOL, in case falling edge is still longing. LBPE always stops reception of the CEC message. LBPE generates an Error-Bit on the CEC line if LBPEGEN=1. In case of broadcast, Error-Bit is generated even in case of LBPEGEN=0. LBPE is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_lbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rx-Missing Acknowledge In receive mode, RXACKE is set by hardware to inform application that no acknowledge was seen on the CEC line. RXACKE applies only for broadcast messages and in listen mode also for not directly addressed messages (destination address not enabled in OAR). RXACKE aborts message reception. RXACKE is cleared by software write at 1."]
        #[inline(always)]
        pub const fn rxacke(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rx-Missing Acknowledge In receive mode, RXACKE is set by hardware to inform application that no acknowledge was seen on the CEC line. RXACKE applies only for broadcast messages and in listen mode also for not directly addressed messages (destination address not enabled in OAR). RXACKE aborts message reception. RXACKE is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_rxacke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Arbitration Lost ARBLST is set by hardware to inform application that CEC device is switching to reception due to arbitration lost event following the TXSOM command. ARBLST can be due either to a contending CEC device starting earlier or starting at the same time but with higher HEADER priority. After ARBLST assertion TXSOM bit keeps pending for next transmission attempt. ARBLST is cleared by software write at 1."]
        #[inline(always)]
        pub const fn arblst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration Lost ARBLST is set by hardware to inform application that CEC device is switching to reception due to arbitration lost event following the TXSOM command. ARBLST can be due either to a contending CEC device starting earlier or starting at the same time but with higher HEADER priority. After ARBLST assertion TXSOM bit keeps pending for next transmission attempt. ARBLST is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_arblst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Tx-Byte Request TXBR is set by hardware to inform application that the next transmission data has to be written to TXDR. TXBR is set when the 4th bit of currently transmitted byte is sent. Application must write the next byte to TXDR within 6 nominal data-bit periods before transmission underrun error occurs (TXUDR). TXBR is cleared by software write at 1."]
        #[inline(always)]
        pub const fn txbr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Byte Request TXBR is set by hardware to inform application that the next transmission data has to be written to TXDR. TXBR is set when the 4th bit of currently transmitted byte is sent. Application must write the next byte to TXDR within 6 nominal data-bit periods before transmission underrun error occurs (TXUDR). TXBR is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_txbr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "End of Transmission TXEND is set by hardware to inform application that the last byte of the CEC message has been successfully transmitted. TXEND clears the TXSOM and TXEOM control bits. TXEND is cleared by software write at 1."]
        #[inline(always)]
        pub const fn txend(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "End of Transmission TXEND is set by hardware to inform application that the last byte of the CEC message has been successfully transmitted. TXEND clears the TXSOM and TXEOM control bits. TXEND is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_txend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx-Buffer Underrun In transmission mode, TXUDR is set by hardware if application was not in time to load TXDR before of next byte transmission. TXUDR aborts message transmission and clears TXSOM and TXEOM control bits. TXUDR is cleared by software write at 1."]
        #[inline(always)]
        pub const fn txudr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Buffer Underrun In transmission mode, TXUDR is set by hardware if application was not in time to load TXDR before of next byte transmission. TXUDR aborts message transmission and clears TXSOM and TXEOM control bits. TXUDR is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_txudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx-Error In transmission mode, TXERR is set by hardware if the CEC initiator detects low impedance on the CEC line while it is released. TXERR aborts message transmission and clears TXSOM and TXEOM controls. TXERR is cleared by software write at 1."]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Error In transmission mode, TXERR is set by hardware if the CEC initiator detects low impedance on the CEC line while it is released. TXERR aborts message transmission and clears TXSOM and TXEOM controls. TXERR is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx-Missing Acknowledge Error In transmission mode, TXACKE is set by hardware to inform application that no acknowledge was received. In case of broadcast transmission, TXACKE informs application that a negative acknowledge was received. TXACKE aborts message transmission and clears TXSOM and TXEOM controls. TXACKE is cleared by software write at 1."]
        #[inline(always)]
        pub const fn txacke(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx-Missing Acknowledge Error In transmission mode, TXACKE is set by hardware to inform application that no acknowledge was received. In case of broadcast transmission, TXACKE informs application that a negative acknowledge was received. TXACKE aborts message transmission and clears TXSOM and TXEOM controls. TXACKE is cleared by software write at 1."]
        #[inline(always)]
        pub fn set_txacke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("rxbr", &self.rxbr())
                .field("rxend", &self.rxend())
                .field("rxovr", &self.rxovr())
                .field("bre", &self.bre())
                .field("sbpe", &self.sbpe())
                .field("lbpe", &self.lbpe())
                .field("rxacke", &self.rxacke())
                .field("arblst", &self.arblst())
                .field("txbr", &self.txbr())
                .field("txend", &self.txend())
                .field("txudr", &self.txudr())
                .field("txerr", &self.txerr())
                .field("txacke", &self.txacke())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ rxbr: {=bool:?}, rxend: {=bool:?}, rxovr: {=bool:?}, bre: {=bool:?}, sbpe: {=bool:?}, lbpe: {=bool:?}, rxacke: {=bool:?}, arblst: {=bool:?}, txbr: {=bool:?}, txend: {=bool:?}, txudr: {=bool:?}, txerr: {=bool:?}, txacke: {=bool:?} }}" , self . rxbr () , self . rxend () , self . rxovr () , self . bre () , self . sbpe () , self . lbpe () , self . rxacke () , self . arblst () , self . txbr () , self . txend () , self . txudr () , self . txerr () , self . txacke ())
        }
    }
    #[doc = "CEC Rx Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdr(pub u32);
    impl Rxdr {
        #[doc = "Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line."]
        #[inline(always)]
        pub const fn rxd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line."]
        #[inline(always)]
        pub fn set_rxd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rxdr {
        #[inline(always)]
        fn default() -> Rxdr {
            Rxdr(0)
        }
    }
    impl core::fmt::Debug for Rxdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdr").field("rxd", &self.rxd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxdr {{ rxd: {=u8:?} }}", self.rxd())
        }
    }
    #[doc = "CEC Tx data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdr(pub u32);
    impl Txdr {
        #[doc = "Tx Data register. TXD is a write-only register containing the data byte to be transmitted. Note: TXD must be written when TXSTART=1."]
        #[inline(always)]
        pub const fn txd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Tx Data register. TXD is a write-only register containing the data byte to be transmitted. Note: TXD must be written when TXSTART=1."]
        #[inline(always)]
        pub fn set_txd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Txdr {
        #[inline(always)]
        fn default() -> Txdr {
            Txdr(0)
        }
    }
    impl core::fmt::Debug for Txdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdr").field("txd", &self.txd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txdr {{ txd: {=u8:?} }}", self.txd())
        }
    }
}
