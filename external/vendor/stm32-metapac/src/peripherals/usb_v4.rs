#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal serial bus full-speed host/device interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "endpoint/channel"]
    #[inline(always)]
    pub const fn epr(self, n: usize) -> crate::common::Reg<regs::Epr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "interrupt status register"]
    #[inline(always)]
    pub const fn istr(self) -> crate::common::Reg<regs::Istr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "frame number register"]
    #[inline(always)]
    pub const fn fnr(self) -> crate::common::Reg<regs::Fnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "device address"]
    #[inline(always)]
    pub const fn daddr(self) -> crate::common::Reg<regs::Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "LPM control and status register"]
    #[inline(always)]
    pub const fn lpmcsr(self) -> crate::common::Reg<regs::Lpmcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Battery charging detector"]
    #[inline(always)]
    pub const fn bcdr(self) -> crate::common::Reg<regs::Bcdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
}
pub mod regs {
    #[doc = "Battery charging detector"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcdr(pub u32);
    impl Bcdr {
        #[doc = "Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to '0 in order to allow the normal USB operation."]
        #[inline(always)]
        pub const fn bcden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to '0 in order to allow the normal USB operation."]
        #[inline(always)]
        pub fn set_bcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
        #[inline(always)]
        pub const fn dcden(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
        #[inline(always)]
        pub fn set_dcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
        #[inline(always)]
        pub const fn pden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
        #[inline(always)]
        pub fn set_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
        #[inline(always)]
        pub const fn sden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
        #[inline(always)]
        pub fn set_sden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data contact detection (DCD) status Device mode This bit gives the result of DCD."]
        #[inline(always)]
        pub const fn dcdet(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data contact detection (DCD) status Device mode This bit gives the result of DCD."]
        #[inline(always)]
        pub fn set_dcdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Primary detection (PD) status Device mode This bit gives the result of PD."]
        #[inline(always)]
        pub const fn pdet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Primary detection (PD) status Device mode This bit gives the result of PD."]
        #[inline(always)]
        pub fn set_pdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Secondary detection (SD) status Device mode This bit gives the result of SD."]
        #[inline(always)]
        pub const fn sdet(&self) -> super::vals::Sdet {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Sdet::from_bits(val as u8)
        }
        #[doc = "Secondary detection (SD) status Device mode This bit gives the result of SD."]
        #[inline(always)]
        pub fn set_sdet(&mut self, val: super::vals::Sdet) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification."]
        #[inline(always)]
        pub const fn ps2det(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification."]
        #[inline(always)]
        pub fn set_ps2det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to '0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
        #[inline(always)]
        pub const fn dppu(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to '0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
        #[inline(always)]
        pub fn set_dppu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Bcdr {
        #[inline(always)]
        fn default() -> Bcdr {
            Bcdr(0)
        }
    }
    impl core::fmt::Debug for Bcdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcdr")
                .field("bcden", &self.bcden())
                .field("dcden", &self.dcden())
                .field("pden", &self.pden())
                .field("sden", &self.sden())
                .field("dcdet", &self.dcdet())
                .field("pdet", &self.pdet())
                .field("sdet", &self.sdet())
                .field("ps2det", &self.ps2det())
                .field("dppu", &self.dppu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcdr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bcdr {{ bcden: {=bool:?}, dcden: {=bool:?}, pden: {=bool:?}, sden: {=bool:?}, dcdet: {=bool:?}, pdet: {=bool:?}, sdet: {:?}, ps2det: {=bool:?}, dppu: {=bool:?} }}" , self . bcden () , self . dcden () , self . pden () , self . sden () , self . dcdet () , self . pdet () , self . sdet () , self . ps2det () , self . dppu ())
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
        #[inline(always)]
        pub const fn fres(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
        #[inline(always)]
        pub fn set_fres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
        #[inline(always)]
        pub const fn pdwn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
        #[inline(always)]
        pub fn set_pdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
        #[inline(always)]
        pub const fn lpmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
        #[inline(always)]
        pub fn set_lpmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Suspend state enable Device mode Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to purse more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Host mode Software can set this bit when Host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to purse more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
        #[inline(always)]
        pub const fn fsusp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend state enable Device mode Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to purse more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Host mode Software can set this bit when Host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to purse more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
        #[inline(always)]
        pub fn set_fsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "L2 Remote Wakeup / Resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the Host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the Host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "L2 Remote Wakeup / Resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the Host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the Host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
        #[inline(always)]
        pub fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "L1 Remote Wakeup / Resume driver Device mode Software sets this bit to send a LPM L1 50us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware. Host mode Software sets this bit to send L1 resume signaling to device. Resume duration and next SOF generation is automatically driven to set the restart of USB activity timely aligned with the programmed BESL value. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt. This bit is cleared by hardware at the end of resume."]
        #[inline(always)]
        pub const fn l1resume(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "L1 Remote Wakeup / Resume driver Device mode Software sets this bit to send a LPM L1 50us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware. Host mode Software sets this bit to send L1 resume signaling to device. Resume duration and next SOF generation is automatically driven to set the restart of USB activity timely aligned with the programmed BESL value. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt. This bit is cleared by hardware at the end of resume."]
        #[inline(always)]
        pub fn set_l1resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPM L1 state request interrupt mask"]
        #[inline(always)]
        pub const fn l1reqm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM L1 state request interrupt mask"]
        #[inline(always)]
        pub fn set_l1reqm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Expected start of frame interrupt mask"]
        #[inline(always)]
        pub const fn esofm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Expected start of frame interrupt mask"]
        #[inline(always)]
        pub fn set_esofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Start of frame interrupt mask"]
        #[inline(always)]
        pub const fn sofm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame interrupt mask"]
        #[inline(always)]
        pub fn set_sofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "reset interrupt mask"]
        #[inline(always)]
        pub const fn resetm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "reset interrupt mask"]
        #[inline(always)]
        pub fn set_resetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Suspend mode interrupt mask"]
        #[inline(always)]
        pub const fn suspm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend mode interrupt mask"]
        #[inline(always)]
        pub fn set_suspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Wakeup interrupt mask"]
        #[inline(always)]
        pub const fn wkupm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt mask"]
        #[inline(always)]
        pub fn set_wkupm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Error interrupt mask"]
        #[inline(always)]
        pub const fn errm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt mask"]
        #[inline(always)]
        pub fn set_errm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Packet memory area over / underrun interrupt mask"]
        #[inline(always)]
        pub const fn pmaovrm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Packet memory area over / underrun interrupt mask"]
        #[inline(always)]
        pub fn set_pmaovrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn ctrm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_ctrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "512 byte threshold interrupt mask"]
        #[inline(always)]
        pub const fn thr512m(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "512 byte threshold interrupt mask"]
        #[inline(always)]
        pub fn set_thr512m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HOST mode HOST bit selects betweens Host or Device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
        #[inline(always)]
        pub const fn host(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "HOST mode HOST bit selects betweens Host or Device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
        #[inline(always)]
        pub fn set_host(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr")
                .field("fres", &self.fres())
                .field("pdwn", &self.pdwn())
                .field("lpmode", &self.lpmode())
                .field("fsusp", &self.fsusp())
                .field("resume", &self.resume())
                .field("l1resume", &self.l1resume())
                .field("l1reqm", &self.l1reqm())
                .field("esofm", &self.esofm())
                .field("sofm", &self.sofm())
                .field("resetm", &self.resetm())
                .field("suspm", &self.suspm())
                .field("wkupm", &self.wkupm())
                .field("errm", &self.errm())
                .field("pmaovrm", &self.pmaovrm())
                .field("ctrm", &self.ctrm())
                .field("thr512m", &self.thr512m())
                .field("host", &self.host())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cntr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cntr {{ fres: {=bool:?}, pdwn: {=bool:?}, lpmode: {=bool:?}, fsusp: {=bool:?}, resume: {=bool:?}, l1resume: {=bool:?}, l1reqm: {=bool:?}, esofm: {=bool:?}, sofm: {=bool:?}, resetm: {=bool:?}, suspm: {=bool:?}, wkupm: {=bool:?}, errm: {=bool:?}, pmaovrm: {=bool:?}, ctrm: {=bool:?}, thr512m: {=bool:?}, host: {=bool:?} }}" , self . fres () , self . pdwn () , self . lpmode () , self . fsusp () , self . resume () , self . l1resume () , self . l1reqm () , self . esofm () , self . sofm () , self . resetm () , self . suspm () , self . wkupm () , self . errm () , self . pmaovrm () , self . ctrm () , self . thr512m () , self . host ())
        }
    }
    #[doc = "device address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daddr(pub u32);
    impl Daddr {
        #[doc = "Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel Address (EA) field in the associated USB_EPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel Address (EA) field in the associated USB_EPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Enable function This bit is set by the software to enable the USB device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at '0 no transactions are handled, irrespective of the settings of USB_EPnR registers."]
        #[inline(always)]
        pub const fn ef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable function This bit is set by the software to enable the USB device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at '0 no transactions are handled, irrespective of the settings of USB_EPnR registers."]
        #[inline(always)]
        pub fn set_ef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Daddr {
        #[inline(always)]
        fn default() -> Daddr {
            Daddr(0)
        }
    }
    impl core::fmt::Debug for Daddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daddr")
                .field("add", &self.add())
                .field("ef", &self.ef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Daddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Daddr {{ add: {=u8:?}, ef: {=bool:?} }}", self.add(), self.ef())
        }
    }
    #[doc = "endpoint/channel 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epr(pub u32);
    impl Epr {
        #[doc = "endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
        #[inline(always)]
        pub const fn ea(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
        #[inline(always)]
        pub fn set_ea(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be VALID or DISABLED. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to STALL or NAK for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)"]
        #[inline(always)]
        pub const fn stat_tx(&self) -> super::vals::Stat {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Stat::from_bits(val as u8)
        }
        #[doc = "Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be VALID or DISABLED. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to STALL or NAK for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)"]
        #[inline(always)]
        pub fn set_stat_tx(&mut self, val: super::vals::Stat) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Data Toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to ) If the endpoint/channel is Isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (Refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for Isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGTX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can only be toggled by writing 1."]
        #[inline(always)]
        pub const fn dtog_tx(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data Toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to ) If the endpoint/channel is Isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (Refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for Isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGTX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can only be toggled by writing 1."]
        #[inline(always)]
        pub fn set_dtog_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions."]
        #[inline(always)]
        pub const fn ctr_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions."]
        #[inline(always)]
        pub fn set_ctr_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the EP_TYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered STALL instead of ACK. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
        #[inline(always)]
        pub const fn ep_kind(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the EP_TYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered STALL instead of ACK. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
        #[inline(always)]
        pub fn set_ep_kind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page 2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers"]
        #[inline(always)]
        pub const fn ep_type(&self) -> super::vals::EpType {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::EpType::from_bits(val as u8)
        }
        #[doc = "USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page 2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers"]
        #[inline(always)]
        pub fn set_ep_type(&mut self, val: super::vals::EpType) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated."]
        #[inline(always)]
        pub const fn setup(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated."]
        #[inline(always)]
        pub fn set_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page 2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only VALID or DISABLED, so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL or 'NAK for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate."]
        #[inline(always)]
        pub const fn stat_rx(&self) -> super::vals::Stat {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stat::from_bits(val as u8)
        }
        #[doc = "Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page 2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only VALID or DISABLED, so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL or 'NAK for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate."]
        #[inline(always)]
        pub fn set_stat_rx(&mut self, val: super::vals::Stat) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Data Toggle, for reception transfers If the endpoint/channel is not Isochronous, this bit contains the expected value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to ). If the endpoint/channel is Isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGRX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1."]
        #[inline(always)]
        pub const fn dtog_rx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Data Toggle, for reception transfers If the endpoint/channel is not Isochronous, this bit contains the expected value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to ). If the endpoint/channel is Isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGRX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1."]
        #[inline(always)]
        pub fn set_dtog_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect."]
        #[inline(always)]
        pub const fn ctr_rx(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect."]
        #[inline(always)]
        pub fn set_ctr_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Host mode Device address assigned to the endpoint during the enumeration process."]
        #[inline(always)]
        pub const fn devaddr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Host mode Device address assigned to the endpoint during the enumeration process."]
        #[inline(always)]
        pub fn set_devaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device."]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device."]
        #[inline(always)]
        pub fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Low speed endpoint Host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint."]
        #[inline(always)]
        pub const fn ls_ep(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low speed endpoint Host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint."]
        #[inline(always)]
        pub fn set_ls_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated."]
        #[inline(always)]
        pub const fn err_tx(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated."]
        #[inline(always)]
        pub fn set_err_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated."]
        #[inline(always)]
        pub const fn err_rx(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated."]
        #[inline(always)]
        pub fn set_err_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Epr {
        #[inline(always)]
        fn default() -> Epr {
            Epr(0)
        }
    }
    impl core::fmt::Debug for Epr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epr")
                .field("ea", &self.ea())
                .field("stat_tx", &self.stat_tx())
                .field("dtog_tx", &self.dtog_tx())
                .field("ctr_tx", &self.ctr_tx())
                .field("ep_kind", &self.ep_kind())
                .field("ep_type", &self.ep_type())
                .field("setup", &self.setup())
                .field("stat_rx", &self.stat_rx())
                .field("dtog_rx", &self.dtog_rx())
                .field("ctr_rx", &self.ctr_rx())
                .field("devaddr", &self.devaddr())
                .field("nak", &self.nak())
                .field("ls_ep", &self.ls_ep())
                .field("err_tx", &self.err_tx())
                .field("err_rx", &self.err_rx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Epr {{ ea: {=u8:?}, stat_tx: {:?}, dtog_tx: {=bool:?}, ctr_tx: {=bool:?}, ep_kind: {=bool:?}, ep_type: {:?}, setup: {=bool:?}, stat_rx: {:?}, dtog_rx: {=bool:?}, ctr_rx: {=bool:?}, devaddr: {=u8:?}, nak: {=bool:?}, ls_ep: {=bool:?}, err_tx: {=bool:?}, err_rx: {=bool:?} }}" , self . ea () , self . stat_tx () , self . dtog_tx () , self . ctr_tx () , self . ep_kind () , self . ep_type () , self . setup () , self . stat_rx () , self . dtog_rx () , self . ctr_rx () , self . devaddr () , self . nak () , self . ls_ep () , self . err_tx () , self . err_rx ())
        }
    }
    #[doc = "frame number register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fnr(pub u32);
    impl Fnr {
        #[doc = "Frame number This bit field contains the 11-bits frame number contained in the last received SOF packet. The frame number is incremented for every frame sent by the host and it is useful for Isochronous transfers. This bit field is updated on the generation of an SOF interrupt."]
        #[inline(always)]
        pub const fn fn_(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Frame number This bit field contains the 11-bits frame number contained in the last received SOF packet. The frame number is incremented for every frame sent by the host and it is useful for Isochronous transfers. This bit field is updated on the generation of an SOF interrupt."]
        #[inline(always)]
        pub fn set_fn_(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Lost SOF Device mode These bits are written by the hardware when an ESOF interrupt is generated, counting the number of consecutive SOF packets lost. At the reception of an SOF packet, these bits are cleared."]
        #[inline(always)]
        pub const fn lsof(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Lost SOF Device mode These bits are written by the hardware when an ESOF interrupt is generated, counting the number of consecutive SOF packets lost. At the reception of an SOF packet, these bits are cleared."]
        #[inline(always)]
        pub fn set_lsof(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Locked Device mode This bit is set by the hardware when at least two consecutive SOF packets have been received after the end of an USB reset condition or after the end of an USB resume sequence. Once locked, the frame timer remains in this state until an USB reset or USB suspend event occurs."]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Locked Device mode This bit is set by the hardware when at least two consecutive SOF packets have been received after the end of an USB reset condition or after the end of an USB resume sequence. Once locked, the frame timer remains in this state until an USB reset or USB suspend event occurs."]
        #[inline(always)]
        pub fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Receive data - line status This bit can be used to observe the status of received data minus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
        #[inline(always)]
        pub const fn rxdm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data - line status This bit can be used to observe the status of received data minus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
        #[inline(always)]
        pub fn set_rxdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive data + line status This bit can be used to observe the status of received data plus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
        #[inline(always)]
        pub const fn rxdp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data + line status This bit can be used to observe the status of received data plus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
        #[inline(always)]
        pub fn set_rxdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Fnr {
        #[inline(always)]
        fn default() -> Fnr {
            Fnr(0)
        }
    }
    impl core::fmt::Debug for Fnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fnr")
                .field("fn_", &self.fn_())
                .field("lsof", &self.lsof())
                .field("lck", &self.lck())
                .field("rxdm", &self.rxdm())
                .field("rxdp", &self.rxdp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fnr {{ fn_: {=u16:?}, lsof: {=u8:?}, lck: {=bool:?}, rxdm: {=bool:?}, rxdp: {=bool:?} }}",
                self.fn_(),
                self.lsof(),
                self.lck(),
                self.rxdm(),
                self.rxdp()
            )
        }
    }
    #[doc = "interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Istr(pub u32);
    impl Istr {
        #[doc = "Device Endpoint / Host channel identification number These bits are written by the hardware according to the host channel or device endpoint number, which generated the interrupt request. If several endpoint/channel transactions are pending, the hardware writes the identification number related to the endpoint/channel having the highest priority defined in the following way: Two levels are defined, in order of priority: Isochronous and double-buffered bulk channels/endpoints are considered first and then the others are examined. If more than one endpoint/channel from the same set is requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so on. The application software can assign a register to each endpoint/channel according to this priority scheme, so as to order the concurring endpoint/channel requests in a suitable way. These bits are read only."]
        #[inline(always)]
        pub const fn ep_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Device Endpoint / Host channel identification number These bits are written by the hardware according to the host channel or device endpoint number, which generated the interrupt request. If several endpoint/channel transactions are pending, the hardware writes the identification number related to the endpoint/channel having the highest priority defined in the following way: Two levels are defined, in order of priority: Isochronous and double-buffered bulk channels/endpoints are considered first and then the others are examined. If more than one endpoint/channel from the same set is requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so on. The application software can assign a register to each endpoint/channel according to this priority scheme, so as to order the concurring endpoint/channel requests in a suitable way. These bits are read only."]
        #[inline(always)]
        pub fn set_ep_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit=0, VTTX bit is set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit=1, VTRX bit or both VTTX/VTRX are set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_EPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only."]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit=0, VTTX bit is set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit=1, VTRX bit or both VTTX/VTRX are set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_EPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn l1req(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_l1req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1 ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn esof(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1 ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_esof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1 ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1 ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state."]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state."]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Suspend mode request This bit is set by the hardware when no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend mode request This bit is set by the hardware when no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn wkup(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub const fn pmaovr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect."]
        #[inline(always)]
        pub fn set_pmaovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Correct transfer This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and EP_ID bits software can determine which endpoint/channel requested the interrupt. This bit is read-only."]
        #[inline(always)]
        pub const fn ctr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Correct transfer This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and EP_ID bits software can determine which endpoint/channel requested the interrupt. This bit is read-only."]
        #[inline(always)]
        pub fn set_ctr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel."]
        #[inline(always)]
        pub const fn thr512(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel."]
        #[inline(always)]
        pub fn set_thr512(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected."]
        #[inline(always)]
        pub const fn dcon_stat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected."]
        #[inline(always)]
        pub fn set_dcon_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Low Speed device connected Host mode: This bit is set by hardware when an LS device connection is detected. Device connection is signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz) from the unconnected state."]
        #[inline(always)]
        pub const fn ls_dcon(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Low Speed device connected Host mode: This bit is set by hardware when an LS device connection is detected. Device connection is signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz) from the unconnected state."]
        #[inline(always)]
        pub fn set_ls_dcon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Istr {
        #[inline(always)]
        fn default() -> Istr {
            Istr(0)
        }
    }
    impl core::fmt::Debug for Istr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Istr")
                .field("ep_id", &self.ep_id())
                .field("dir", &self.dir())
                .field("l1req", &self.l1req())
                .field("esof", &self.esof())
                .field("sof", &self.sof())
                .field("reset", &self.reset())
                .field("susp", &self.susp())
                .field("wkup", &self.wkup())
                .field("err", &self.err())
                .field("pmaovr", &self.pmaovr())
                .field("ctr", &self.ctr())
                .field("thr512", &self.thr512())
                .field("dcon_stat", &self.dcon_stat())
                .field("ls_dcon", &self.ls_dcon())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Istr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Istr {{ ep_id: {=u8:?}, dir: {:?}, l1req: {=bool:?}, esof: {=bool:?}, sof: {=bool:?}, reset: {=bool:?}, susp: {=bool:?}, wkup: {=bool:?}, err: {=bool:?}, pmaovr: {=bool:?}, ctr: {=bool:?}, thr512: {=bool:?}, dcon_stat: {=bool:?}, ls_dcon: {=bool:?} }}" , self . ep_id () , self . dir () , self . l1req () , self . esof () , self . sof () , self . reset () , self . susp () , self . wkup () , self . err () , self . pmaovr () , self . ctr () , self . thr512 () , self . dcon_stat () , self . ls_dcon ())
        }
    }
    #[doc = "LPM control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmcsr(pub u32);
    impl Lpmcsr {
        #[doc = "LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB device. If this bit is at '0 no LPM transactions are handled. Host mode Software sets this bit to transmit an LPM transaction to device. This bit is cleared by hardware, simultaneous with L1REQ flag set, when device answer is received"]
        #[inline(always)]
        pub const fn lpmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB device. If this bit is at '0 no LPM transactions are handled. Host mode Software sets this bit to transmit an LPM transaction to device. This bit is cleared by hardware, simultaneous with L1REQ flag set, when device answer is received"]
        #[inline(always)]
        pub fn set_lpmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPM Token acknowledge enable The NYET/ACK will be returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL) This bit contains the device answer to the LPM transaction. It mast be evaluated following the L1REQ interrupt."]
        #[inline(always)]
        pub const fn lpmack(&self) -> super::vals::Lpmack {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lpmack::from_bits(val as u8)
        }
        #[doc = "LPM Token acknowledge enable The NYET/ACK will be returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL) This bit contains the device answer to the LPM transaction. It mast be evaluated following the L1REQ interrupt."]
        #[inline(always)]
        pub fn set_lpmack(&mut self, val: super::vals::Lpmack) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token Host mode This bit contains the bRemoteWake value transmitted with the LPM transaction"]
        #[inline(always)]
        pub const fn remwake(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token Host mode This bit contains the bRemoteWake value transmitted with the LPM transaction"]
        #[inline(always)]
        pub fn set_remwake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token Host mode These bits contain the BESL value transmitted with the LPM transaction"]
        #[inline(always)]
        pub const fn besl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token Host mode These bits contain the BESL value transmitted with the LPM transaction"]
        #[inline(always)]
        pub fn set_besl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Lpmcsr {
        #[inline(always)]
        fn default() -> Lpmcsr {
            Lpmcsr(0)
        }
    }
    impl core::fmt::Debug for Lpmcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpmcsr")
                .field("lpmen", &self.lpmen())
                .field("lpmack", &self.lpmack())
                .field("remwake", &self.remwake())
                .field("besl", &self.besl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpmcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lpmcsr {{ lpmen: {=bool:?}, lpmack: {:?}, remwake: {=bool:?}, besl: {=u8:?} }}",
                self.lpmen(),
                self.lpmack(),
                self.remwake(),
                self.besl()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "data transmitted by the USB peripheral to the host PC"]
        TO = 0x0,
        #[doc = "data received by the USB peripheral from the host PC"]
        FROM = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EpType {
        #[doc = "Bulk endpoint"]
        BULK = 0x0,
        #[doc = "Control endpoint"]
        CONTROL = 0x01,
        #[doc = "Iso endpoint"]
        ISO = 0x02,
        #[doc = "Interrupt endpoint"]
        INTERRUPT = 0x03,
    }
    impl EpType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpType {
        #[inline(always)]
        fn from(val: u8) -> EpType {
            EpType::from_bits(val)
        }
    }
    impl From<EpType> for u8 {
        #[inline(always)]
        fn from(val: EpType) -> u8 {
            EpType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpmack {
        #[doc = "The valid LPM Token will be NYET / NYET answer"]
        NYET = 0x0,
        #[doc = "The valid LPM Token will be ACK / ACK answer"]
        ACK = 0x01,
    }
    impl Lpmack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpmack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpmack {
        #[inline(always)]
        fn from(val: u8) -> Lpmack {
            Lpmack::from_bits(val)
        }
    }
    impl From<Lpmack> for u8 {
        #[inline(always)]
        fn from(val: Lpmack) -> u8 {
            Lpmack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdet {
        #[doc = "CDP detected"]
        CDP = 0x0,
        #[doc = "DCP detected"]
        DCP = 0x01,
    }
    impl Sdet {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdet {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdet {
        #[inline(always)]
        fn from(val: u8) -> Sdet {
            Sdet::from_bits(val)
        }
    }
    impl From<Sdet> for u8 {
        #[inline(always)]
        fn from(val: Sdet) -> u8 {
            Sdet::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stat {
        #[doc = "all requests addressed to this endpoint are ignored"]
        DISABLED = 0x0,
        #[doc = "the endpoint is stalled and all requests result in a STALL handshake"]
        STALL = 0x01,
        #[doc = "the endpoint is naked and all requests result in a NAK handshake"]
        NAK = 0x02,
        #[doc = "this endpoint is enabled, requests are ACKed"]
        VALID = 0x03,
    }
    impl Stat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stat {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stat {
        #[inline(always)]
        fn from(val: u8) -> Stat {
            Stat::from_bits(val)
        }
    }
    impl From<Stat> for u8 {
        #[inline(always)]
        fn from(val: Stat) -> u8 {
            Stat::to_bits(val)
        }
    }
}
