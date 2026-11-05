#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Embedded Flash memory."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Access control register."]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FLASH control key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FLASH control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "FLASH status register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH CRC control register."]
    #[inline(always)]
    pub const fn crccr(self) -> crate::common::Reg<regs::Crccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH CRC start address register."]
    #[inline(always)]
    pub const fn crcsaddr(self) -> crate::common::Reg<regs::Crcsaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "FLASH CRC end address register."]
    #[inline(always)]
    pub const fn crceaddr(self) -> crate::common::Reg<regs::Crceaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "FLASH CRC data register."]
    #[inline(always)]
    pub const fn crcdatar(self) -> crate::common::Reg<regs::Crcdatar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "FLASH ECC single error fail address."]
    #[inline(always)]
    pub const fn eccsfaddr(self) -> crate::common::Reg<regs::Eccsfaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FLASH ECC double error fail address."]
    #[inline(always)]
    pub const fn eccdfaddr(self) -> crate::common::Reg<regs::Eccdfaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FLASH options key register."]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<regs::Optkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "FLASH options control register."]
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "FLASH options interrupt status register."]
    #[inline(always)]
    pub const fn optisr(self) -> crate::common::Reg<regs::Optisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "FLASH options interrupt clear register."]
    #[inline(always)]
    pub const fn opticr(self) -> crate::common::Reg<regs::Opticr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "FLASH option byte key control register."]
    #[inline(always)]
    pub const fn obkcr(self) -> crate::common::Reg<regs::Obkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "FLASH option bytes key data register 0."]
    #[inline(always)]
    pub const fn obkdr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize + n * 4usize) as _) }
    }
    #[doc = "FLASH non-volatile status register."]
    #[inline(always)]
    pub const fn nvsr(self) -> crate::common::Reg<regs::Nvsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "FLASH security status register programming."]
    #[inline(always)]
    pub const fn nvsrp(self) -> crate::common::Reg<regs::Nvsrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "FLASH RoT status register."]
    #[inline(always)]
    pub const fn rotsr(self) -> crate::common::Reg<regs::Rotsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "FLASH RoT status register programming."]
    #[inline(always)]
    pub const fn rotsrp(self) -> crate::common::Reg<regs::Rotsrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "FLASH OTP lock status register."]
    #[inline(always)]
    pub const fn otplsr(self) -> crate::common::Reg<regs::Otplsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "FLASH OTP lock status register programming."]
    #[inline(always)]
    pub const fn otplsrp(self) -> crate::common::Reg<regs::Otplsrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "FLASH write protection status register."]
    #[inline(always)]
    pub const fn wrpsr(self) -> crate::common::Reg<regs::Wrpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "FLASH write protection status register programming."]
    #[inline(always)]
    pub const fn wrpsrp(self) -> crate::common::Reg<regs::Wrpsrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[doc = "FLASH hide protection status register."]
    #[inline(always)]
    pub const fn hdpsr(self) -> crate::common::Reg<regs::Hdpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "FLASH hide protection status register programming."]
    #[inline(always)]
    pub const fn hdpsrp(self) -> crate::common::Reg<regs::Hdpsrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[doc = "FLASH epoch status register."]
    #[inline(always)]
    pub const fn epochsr(self) -> crate::common::Reg<regs::Epochsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[doc = "FLASH RoT status register programming."]
    #[inline(always)]
    pub const fn epochsrp(self) -> crate::common::Reg<regs::Epochsrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[doc = "FLASH option byte word 1 status register."]
    #[inline(always)]
    pub const fn obw1sr(self) -> crate::common::Reg<regs::Obw1sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[doc = "FLASH option byte word 1 status register programming."]
    #[inline(always)]
    pub const fn obw1srp(self) -> crate::common::Reg<regs::Obw1srp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[doc = "FLASH option byte word 2 status register."]
    #[inline(always)]
    pub const fn obw2sr(self) -> crate::common::Reg<regs::Obw2sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[doc = "FLASH option byte word 2 status register programming."]
    #[inline(always)]
    pub const fn obw2srp(self) -> crate::common::Reg<regs::Obw2srp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. Please refer to Table 27 for details. ... Note: Embedded Flash does not verify that the configuration is correct."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. Please refer to Table 27 for details. ... Note: Embedded Flash does not verify that the configuration is correct."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to Table 27 for details. Note: Embedded Flash does not verify that the configuration is correct."]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to Table 27 for details. Note: Embedded Flash does not verify that the configuration is correct."]
        #[inline(always)]
        pub fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("wrhighfreq", &self.wrhighfreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {=u8:?}, wrhighfreq: {=u8:?} }}",
                self.latency(),
                self.wrhighfreq()
            )
        }
    }
    #[doc = "FLASH control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase."]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase."]
        #[inline(always)]
        pub fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase."]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase."]
        #[inline(always)]
        pub fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively)."]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively)."]
        #[inline(always)]
        pub fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ..."]
        #[inline(always)]
        pub const fn ssn(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ..."]
        #[inline(always)]
        pub fn set_ssn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "Program Enable for OTP Area Set this bit to enable write operations to OTP area."]
        #[inline(always)]
        pub const fn pg_otp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Program Enable for OTP Area Set this bit to enable write operations to OTP area."]
        #[inline(always)]
        pub fn set_pg_otp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register."]
        #[inline(always)]
        pub const fn crc_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register."]
        #[inline(always)]
        pub fn set_crc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored."]
        #[inline(always)]
        pub const fn all_banks(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored."]
        #[inline(always)]
        pub fn set_all_banks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("ser", &self.ser())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("start", &self.start())
                .field("ssn", &self.ssn())
                .field("pg_otp", &self.pg_otp())
                .field("crc_en", &self.crc_en())
                .field("all_banks", &self.all_banks())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ lock: {=bool:?}, pg: {=bool:?}, ser: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, start: {=bool:?}, ssn: {=u8:?}, pg_otp: {=bool:?}, crc_en: {=bool:?}, all_banks: {=bool:?} }}" , self . lock () , self . pg () , self . ser () , self . ber () , self . fw () , self . start () , self . ssn () , self . pg_otp () , self . crc_en () , self . all_banks ())
        }
    }
    #[doc = "FLASH CRC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crccr(pub u32);
    impl Crccr {
        #[doc = "CRC sector number CRC_SECT is used to select one user Flash sectors to be added to the list of sectors on which the CRC is calculated. The CRC can be computed either between two addresses (using registers FLASH_CRCSADDR and FLASH_CRCEADDR) or on a list of sectors using this register. If this latter option is selected, it is possible to add a sector to the list of sectors by programming the sector number in CRC_SECT and then setting ADD_SECT bit. The list of sectors can be erased either by setting CLEAN_SECT bit or by disabling the CRC computation. ..."]
        #[inline(always)]
        pub const fn crc_sect(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CRC sector number CRC_SECT is used to select one user Flash sectors to be added to the list of sectors on which the CRC is calculated. The CRC can be computed either between two addresses (using registers FLASH_CRCSADDR and FLASH_CRCEADDR) or on a list of sectors using this register. If this latter option is selected, it is possible to add a sector to the list of sectors by programming the sector number in CRC_SECT and then setting ADD_SECT bit. The list of sectors can be erased either by setting CLEAN_SECT bit or by disabling the CRC computation. ..."]
        #[inline(always)]
        pub fn set_crc_sect(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CRC sector mode select bit When this bit is set the CRC calculation is performed at sector level, on the sectors present in the list of sectors. To add a sector to this list, use ADD_SECT and CRC_SECT bits. To clean the list, use CLEAN_SECT bit. When CRC_BY_SECT is cleared the CRC calculation is performed on all addresses defined between start and end addresses defined in FLASH_CRCSADDR and FLASH_CRCEADDR registers."]
        #[inline(always)]
        pub const fn crc_by_sect(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CRC sector mode select bit When this bit is set the CRC calculation is performed at sector level, on the sectors present in the list of sectors. To add a sector to this list, use ADD_SECT and CRC_SECT bits. To clean the list, use CLEAN_SECT bit. When CRC_BY_SECT is cleared the CRC calculation is performed on all addresses defined between start and end addresses defined in FLASH_CRCSADDR and FLASH_CRCEADDR registers."]
        #[inline(always)]
        pub fn set_crc_by_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CRC sector select bit When this bit is set the sector whose number is written in CRC_SECT is added to the list of sectors on which the CRC is calculated."]
        #[inline(always)]
        pub const fn add_sect(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CRC sector select bit When this bit is set the sector whose number is written in CRC_SECT is added to the list of sectors on which the CRC is calculated."]
        #[inline(always)]
        pub fn set_add_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC sector list clear bit When this bit is set the list of sectors on which the CRC is calculated is cleared."]
        #[inline(always)]
        pub const fn clean_sect(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CRC sector list clear bit When this bit is set the list of sectors on which the CRC is calculated is cleared."]
        #[inline(always)]
        pub fn set_clean_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CRC start bit START_CRC bit triggers a CRC calculation using the current configuration. No CRC calculation can launched when an option byte change operation is ongoing because all read accesses to embedded Flash memory registers are put on hold until the option byte change operation has completed. This bit is cleared when CRC computation starts."]
        #[inline(always)]
        pub const fn start_crc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CRC start bit START_CRC bit triggers a CRC calculation using the current configuration. No CRC calculation can launched when an option byte change operation is ongoing because all read accesses to embedded Flash memory registers are put on hold until the option byte change operation has completed. This bit is cleared when CRC computation starts."]
        #[inline(always)]
        pub fn set_start_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CRC clear bit Setting CLEAN_CRC to 1 clears the current CRC result stored in the FLASH_CRCDATAR register."]
        #[inline(always)]
        pub const fn clean_crc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clear bit Setting CLEAN_CRC to 1 clears the current CRC result stored in the FLASH_CRCDATAR register."]
        #[inline(always)]
        pub fn set_clean_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "CRC burst size CRC_BURST bits set the size of the bursts that are generated by the CRC calculation unit. A Flash word is 128-bit."]
        #[inline(always)]
        pub const fn crc_burst(&self) -> super::vals::CrcBurst {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::CrcBurst::from_bits(val as u8)
        }
        #[doc = "CRC burst size CRC_BURST bits set the size of the bursts that are generated by the CRC calculation unit. A Flash word is 128-bit."]
        #[inline(always)]
        pub fn set_crc_burst(&mut self, val: super::vals::CrcBurst) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "All sectors selection When this bit is set all the sectors in user Flash are added to list of sectors on which the CRC shall be calculated. This bit is cleared when CRC computation starts."]
        #[inline(always)]
        pub const fn all_sect(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "All sectors selection When this bit is set all the sectors in user Flash are added to list of sectors on which the CRC shall be calculated. This bit is cleared when CRC computation starts."]
        #[inline(always)]
        pub fn set_all_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Crccr {
        #[inline(always)]
        fn default() -> Crccr {
            Crccr(0)
        }
    }
    impl core::fmt::Debug for Crccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crccr")
                .field("crc_sect", &self.crc_sect())
                .field("crc_by_sect", &self.crc_by_sect())
                .field("add_sect", &self.add_sect())
                .field("clean_sect", &self.clean_sect())
                .field("start_crc", &self.start_crc())
                .field("clean_crc", &self.clean_crc())
                .field("crc_burst", &self.crc_burst())
                .field("all_sect", &self.all_sect())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Crccr {{ crc_sect: {=u8:?}, crc_by_sect: {=bool:?}, add_sect: {=bool:?}, clean_sect: {=bool:?}, start_crc: {=bool:?}, clean_crc: {=bool:?}, crc_burst: {:?}, all_sect: {=bool:?} }}" , self . crc_sect () , self . crc_by_sect () , self . add_sect () , self . clean_sect () , self . start_crc () , self . clean_crc () , self . crc_burst () , self . all_sect ())
        }
    }
    #[doc = "FLASH CRC data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcdatar(pub u32);
    impl Crcdatar {
        #[doc = "CRC result This bitfield contains the result of the last CRC calculation. The value is valid only when CRC calculation completed (CRCENDF is set in FLASH_ISR register). CRC_DATA is cleared when CRC_EN is cleared in FLASH_CR register (CRC disabled), or when CLEAN_CRC bit is set in FLASH_CRCCR register."]
        #[inline(always)]
        pub const fn crc_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC result This bitfield contains the result of the last CRC calculation. The value is valid only when CRC calculation completed (CRCENDF is set in FLASH_ISR register). CRC_DATA is cleared when CRC_EN is cleared in FLASH_CR register (CRC disabled), or when CLEAN_CRC bit is set in FLASH_CRCCR register."]
        #[inline(always)]
        pub fn set_crc_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crcdatar {
        #[inline(always)]
        fn default() -> Crcdatar {
            Crcdatar(0)
        }
    }
    impl core::fmt::Debug for Crcdatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcdatar").field("crc_data", &self.crc_data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcdatar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crcdatar {{ crc_data: {=u32:?} }}", self.crc_data())
        }
    }
    #[doc = "FLASH CRC end address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crceaddr(pub u32);
    impl Crceaddr {
        #[doc = "CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits \\[5:0\\]
of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank."]
        #[inline(always)]
        pub const fn crc_end_addr(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x07ff;
            val as u16
        }
        #[doc = "CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits \\[5:0\\]
of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank."]
        #[inline(always)]
        pub fn set_crc_end_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 6usize)) | (((val as u32) & 0x07ff) << 6usize);
        }
    }
    impl Default for Crceaddr {
        #[inline(always)]
        fn default() -> Crceaddr {
            Crceaddr(0)
        }
    }
    impl core::fmt::Debug for Crceaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crceaddr")
                .field("crc_end_addr", &self.crc_end_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crceaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crceaddr {{ crc_end_addr: {=u16:?} }}", self.crc_end_addr())
        }
    }
    #[doc = "FLASH CRC start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcsaddr(pub u32);
    impl Crcsaddr {
        #[doc = "CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits \\[5:0\\]
of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank."]
        #[inline(always)]
        pub const fn crc_start_addr(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x07ff;
            val as u16
        }
        #[doc = "CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits \\[5:0\\]
of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank."]
        #[inline(always)]
        pub fn set_crc_start_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 6usize)) | (((val as u32) & 0x07ff) << 6usize);
        }
    }
    impl Default for Crcsaddr {
        #[inline(always)]
        fn default() -> Crcsaddr {
            Crcsaddr(0)
        }
    }
    impl core::fmt::Debug for Crcsaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcsaddr")
                .field("crc_start_addr", &self.crc_start_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcsaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crcsaddr {{ crc_start_addr: {=u16:?} }}", self.crc_start_addr())
        }
    }
    #[doc = "FLASH ECC double error fail address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdfaddr(pub u32);
    impl Eccdfaddr {
        #[doc = "ECC double error detection fail address When a double ECC detection occurs during a read operation, the DED_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when the DBECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC double error detection error is saved in this register."]
        #[inline(always)]
        pub const fn ded_fadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error detection fail address When a double ECC detection occurs during a read operation, the DED_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when the DBECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC double error detection error is saved in this register."]
        #[inline(always)]
        pub fn set_ded_fadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eccdfaddr {
        #[inline(always)]
        fn default() -> Eccdfaddr {
            Eccdfaddr(0)
        }
    }
    impl core::fmt::Debug for Eccdfaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdfaddr").field("ded_fadd", &self.ded_fadd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdfaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccdfaddr {{ ded_fadd: {=u32:?} }}", self.ded_fadd())
        }
    }
    #[doc = "FLASH ECC single error fail address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccsfaddr(pub u32);
    impl Eccsfaddr {
        #[doc = "ECC single error correction fail address When a single ECC error correction occurs during a read operation, the SEC_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when SNECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC single error correction error is saved in this register."]
        #[inline(always)]
        pub const fn sec_fadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error correction fail address When a single ECC error correction occurs during a read operation, the SEC_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when SNECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC single error correction error is saved in this register."]
        #[inline(always)]
        pub fn set_sec_fadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eccsfaddr {
        #[inline(always)]
        fn default() -> Eccsfaddr {
            Eccsfaddr(0)
        }
    }
    impl core::fmt::Debug for Eccsfaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccsfaddr").field("sec_fadd", &self.sec_fadd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccsfaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccsfaddr {{ sec_fadd: {=u32:?} }}", self.sec_fadd())
        }
    }
    #[doc = "FLASH epoch status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epochsr(pub u32);
    impl Epochsr {
        #[doc = "Epoch This value is distributed by hardware to the SAES peripheral."]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Epoch This value is distributed by hardware to the SAES peripheral."]
        #[inline(always)]
        pub fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Epochsr {
        #[inline(always)]
        fn default() -> Epochsr {
            Epochsr(0)
        }
    }
    impl core::fmt::Debug for Epochsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epochsr").field("epoch", &self.epoch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epochsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Epochsr {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[doc = "FLASH RoT status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epochsrp(pub u32);
    impl Epochsrp {
        #[doc = "Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register."]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register."]
        #[inline(always)]
        pub fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Epochsrp {
        #[inline(always)]
        fn default() -> Epochsrp {
            Epochsrp(0)
        }
    }
    impl core::fmt::Debug for Epochsrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epochsrp").field("epoch", &self.epoch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epochsrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Epochsrp {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[doc = "FLASH status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Root code check flag clear Set this bit to clear RCHECKF bit in FLASH_SR."]
        #[inline(always)]
        pub const fn rcheckf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Root code check flag clear Set this bit to clear RCHECKF bit in FLASH_SR."]
        #[inline(always)]
        pub fn set_rcheckf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr").field("rcheckf", &self.rcheckf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fcr {{ rcheckf: {=bool:?} }}", self.rcheckf())
        }
    }
    #[doc = "FLASH hide protection status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpsr(pub u32);
    impl Hdpsr {
        #[doc = "Hide protection user Flash area start This option sets the start address that contains the first 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub const fn hdp_area_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Hide protection user Flash area start This option sets the start address that contains the first 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub fn set_hdp_area_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Hide protection user Flash area end This option sets the end address that contains the last 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub const fn hdp_area_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Hide protection user Flash area end This option sets the end address that contains the last 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub fn set_hdp_area_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Hdpsr {
        #[inline(always)]
        fn default() -> Hdpsr {
            Hdpsr(0)
        }
    }
    impl core::fmt::Debug for Hdpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdpsr")
                .field("hdp_area_start", &self.hdp_area_start())
                .field("hdp_area_end", &self.hdp_area_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdpsr {{ hdp_area_start: {=u16:?}, hdp_area_end: {=u16:?} }}",
                self.hdp_area_start(),
                self.hdp_area_end()
            )
        }
    }
    #[doc = "FLASH hide protection status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpsrp(pub u32);
    impl Hdpsrp {
        #[doc = "Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub const fn hdp_area_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub fn set_hdp_area_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub const fn hdp_area_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected."]
        #[inline(always)]
        pub fn set_hdp_area_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Hdpsrp {
        #[inline(always)]
        fn default() -> Hdpsrp {
            Hdpsrp(0)
        }
    }
    impl core::fmt::Debug for Hdpsrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdpsrp")
                .field("hdp_area_start", &self.hdp_area_start())
                .field("hdp_area_end", &self.hdp_area_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpsrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdpsrp {{ hdp_area_start: {=u16:?}, hdp_area_end: {=u16:?} }}",
                self.hdp_area_start(),
                self.hdp_area_end()
            )
        }
    }
    #[doc = "FLASH interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "End-of-program flag clear Setting this bit clears EOPF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn eopf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End-of-program flag clear Setting this bit clears EOPF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_eopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error flag clear Setting this bit clears WRPERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn wrperrf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error flag clear Setting this bit clears WRPERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_wrperrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error flag clear Setting this bit clears PGSERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn pgserrf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error flag clear Setting this bit clears PGSERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_pgserrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error flag clear Setting this bit clears STRBERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn strberrf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error flag clear Setting this bit clears STRBERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_strberrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Option byte loading error flag clear Setting this bit clears OBLERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn oblerrf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loading error flag clear Setting this bit clears OBLERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_oblerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Inconsistency error flag clear Setting this bit clears INCERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn incerrf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error flag clear Setting this bit clears INCERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_incerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Read security error flag clear Setting this bit clears RDSERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn rdserrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Read security error flag clear Setting this bit clears RDSERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_rdserrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC single error flag clear Setting this bit clears SNECCERRF flag in FLASH_ISR register. If the DBECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well."]
        #[inline(always)]
        pub const fn sneccerrf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error flag clear Setting this bit clears SNECCERRF flag in FLASH_ISR register. If the DBECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well."]
        #[inline(always)]
        pub fn set_sneccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC double error flag clear Setting this bit clears DBECCERRF flag in FLASH_ISR register. If the SNECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well."]
        #[inline(always)]
        pub const fn dbeccerrf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error flag clear Setting this bit clears DBECCERRF flag in FLASH_ISR register. If the SNECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well."]
        #[inline(always)]
        pub fn set_dbeccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "CRC end flag clear Setting this bit clears CRCENDF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn crcendf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "CRC end flag clear Setting this bit clears CRCENDF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_crcendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "CRC error flag clear Setting this bit clears CRCRDERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub const fn crcrderrf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error flag clear Setting this bit clears CRCRDERRF flag in FLASH_ISR register."]
        #[inline(always)]
        pub fn set_crcrderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("eopf", &self.eopf())
                .field("wrperrf", &self.wrperrf())
                .field("pgserrf", &self.pgserrf())
                .field("strberrf", &self.strberrf())
                .field("oblerrf", &self.oblerrf())
                .field("incerrf", &self.incerrf())
                .field("rdserrf", &self.rdserrf())
                .field("sneccerrf", &self.sneccerrf())
                .field("dbeccerrf", &self.dbeccerrf())
                .field("crcendf", &self.crcendf())
                .field("crcrderrf", &self.crcrderrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ eopf: {=bool:?}, wrperrf: {=bool:?}, pgserrf: {=bool:?}, strberrf: {=bool:?}, oblerrf: {=bool:?}, incerrf: {=bool:?}, rdserrf: {=bool:?}, sneccerrf: {=bool:?}, dbeccerrf: {=bool:?}, crcendf: {=bool:?}, crcrderrf: {=bool:?} }}" , self . eopf () , self . wrperrf () , self . pgserrf () , self . strberrf () , self . oblerrf () , self . incerrf () , self . rdserrf () , self . sneccerrf () , self . dbeccerrf () , self . crcendf () , self . crcrderrf ())
        }
    }
    #[doc = "FLASH interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "End-of-program interrupt control bit."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End-of-program interrupt control bit."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error interrupt enable bit."]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error interrupt enable bit."]
        #[inline(always)]
        pub fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error interrupt enable bit."]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error interrupt enable bit."]
        #[inline(always)]
        pub fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error interrupt enable bit."]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error interrupt enable bit."]
        #[inline(always)]
        pub fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Option byte loading error interrupt enable bit."]
        #[inline(always)]
        pub const fn oblerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loading error interrupt enable bit."]
        #[inline(always)]
        pub fn set_oblerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Inconsistency error interrupt enable bit."]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error interrupt enable bit."]
        #[inline(always)]
        pub fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Read security error interrupt enable bit."]
        #[inline(always)]
        pub const fn rdserrie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Read security error interrupt enable bit."]
        #[inline(always)]
        pub fn set_rdserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC single correction error interrupt enable bit."]
        #[inline(always)]
        pub const fn sneccerrie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single correction error interrupt enable bit."]
        #[inline(always)]
        pub fn set_sneccerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC double detection error interrupt enable bit."]
        #[inline(always)]
        pub const fn dbeccerrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double detection error interrupt enable bit."]
        #[inline(always)]
        pub fn set_dbeccerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "CRC end of calculation interrupt enable bit."]
        #[inline(always)]
        pub const fn crcendie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "CRC end of calculation interrupt enable bit."]
        #[inline(always)]
        pub fn set_crcendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "CRC read error interrupt enable bit."]
        #[inline(always)]
        pub const fn crcrderrie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "CRC read error interrupt enable bit."]
        #[inline(always)]
        pub fn set_crcrderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("oblerrie", &self.oblerrie())
                .field("incerrie", &self.incerrie())
                .field("rdserrie", &self.rdserrie())
                .field("sneccerrie", &self.sneccerrie())
                .field("dbeccerrie", &self.dbeccerrie())
                .field("crcendie", &self.crcendie())
                .field("crcrderrie", &self.crcrderrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, oblerrie: {=bool:?}, incerrie: {=bool:?}, rdserrie: {=bool:?}, sneccerrie: {=bool:?}, dbeccerrie: {=bool:?}, crcendie: {=bool:?}, crcrderrie: {=bool:?} }}" , self . eopie () , self . wrperrie () , self . pgserrie () , self . strberrie () , self . oblerrie () , self . incerrie () , self . rdserrie () , self . sneccerrie () , self . dbeccerrie () , self . crcendie () , self . crcrderrie ())
        }
    }
    #[doc = "FLASH interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "End-of-program flag This bit is set when a programming operation completes. An interrupt is generated if the EOPIE is set. It is not necessary to reset EOPF before starting a new operation. Setting EOPF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn eopf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End-of-program flag This bit is set when a programming operation completes. An interrupt is generated if the EOPIE is set. It is not necessary to reset EOPF before starting a new operation. Setting EOPF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_eopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error flag This bit is set when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set. Setting WRPERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn wrperrf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error flag This bit is set when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set. Setting WRPERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_wrperrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error flag This bit is set when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set. Setting PGSERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn pgserrf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error flag This bit is set when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set. Setting PGSERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_pgserrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error flag This bit is set when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set. Setting STRBERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn strberrf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error flag This bit is set when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set. Setting STRBERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_strberrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Option byte loading error flag This bit is set when an error is found during the option byte loading sequence. An interrupt is generated if OBLERRIE is set. Setting OBLERRF bit in the FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn oblerrf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loading error flag This bit is set when an error is found during the option byte loading sequence. An interrupt is generated if OBLERRIE is set. Setting OBLERRF bit in the FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_oblerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Inconsistency error flag This bit is set when a inconsistency error occurs. An interrupt is generated if INCERRIE is set. Setting INCERRF bit in the FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn incerrf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error flag This bit is set when a inconsistency error occurs. An interrupt is generated if INCERRIE is set. Setting INCERRF bit in the FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_incerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Read security error flag This bit is set when a read security error occurs (read access to hide protected area with incorrect hide protection level). An interrupt is generated if RDSERRIE is set. Setting RDSERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn rdserrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Read security error flag This bit is set when a read security error occurs (read access to hide protected area with incorrect hide protection level). An interrupt is generated if RDSERRIE is set. Setting RDSERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_rdserrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC single error flag This bit is set when an ECC single correction error occurs during a read operation. An interrupt is generated if SNECCERRIE is set. Setting SNECCERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn sneccerrf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error flag This bit is set when an ECC single correction error occurs during a read operation. An interrupt is generated if SNECCERRIE is set. Setting SNECCERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_sneccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC double error flag This bit is set when an ECC double detection error occurs during a read operation. An interrupt is generated if DBECCERRIE is set. Setting DBECCERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn dbeccerrf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error flag This bit is set when an ECC double detection error occurs during a read operation. An interrupt is generated if DBECCERRIE is set. Setting DBECCERRF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_dbeccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "CRC end flag This bit is set when the CRC computation has completed. An interrupt is generated if CRCENDIE is set. It is not necessary to reset CRCEND before restarting CRC computation. Setting CRCENDF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub const fn crcendf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "CRC end flag This bit is set when the CRC computation has completed. An interrupt is generated if CRCENDIE is set. It is not necessary to reset CRCEND before restarting CRC computation. Setting CRCENDF bit in FLASH_ICR register clears this bit."]
        #[inline(always)]
        pub fn set_crcendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "CRC read error flag This bit is set when a word is found read protected during a CRC operation. An interrupt is generated if CRCRDIE is set. Setting CRCRDERRF bit in FLASH_ICR register clears this bit. This flag is valid only when CRCEND bit is set."]
        #[inline(always)]
        pub const fn crcrderrf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "CRC read error flag This bit is set when a word is found read protected during a CRC operation. An interrupt is generated if CRCRDIE is set. Setting CRCRDERRF bit in FLASH_ICR register clears this bit. This flag is valid only when CRCEND bit is set."]
        #[inline(always)]
        pub fn set_crcrderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("eopf", &self.eopf())
                .field("wrperrf", &self.wrperrf())
                .field("pgserrf", &self.pgserrf())
                .field("strberrf", &self.strberrf())
                .field("oblerrf", &self.oblerrf())
                .field("incerrf", &self.incerrf())
                .field("rdserrf", &self.rdserrf())
                .field("sneccerrf", &self.sneccerrf())
                .field("dbeccerrf", &self.dbeccerrf())
                .field("crcendf", &self.crcendf())
                .field("crcrderrf", &self.crcrderrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ eopf: {=bool:?}, wrperrf: {=bool:?}, pgserrf: {=bool:?}, strberrf: {=bool:?}, oblerrf: {=bool:?}, incerrf: {=bool:?}, rdserrf: {=bool:?}, sneccerrf: {=bool:?}, dbeccerrf: {=bool:?}, crcendf: {=bool:?}, crcrderrf: {=bool:?} }}" , self . eopf () , self . wrperrf () , self . pgserrf () , self . strberrf () , self . oblerrf () , self . incerrf () , self . rdserrf () , self . sneccerrf () , self . dbeccerrf () , self . crcendf () , self . crcrderrf ())
        }
    }
    #[doc = "FLASH control key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Control unlock key Following values must be written to FLASH_KEYR consecutively to unlock FLASH_CR register: 1st key = 0x4567 0123 2nd key = 0xCDEF 89AB Reads to this register returns zero. If above sequence is wrong or performed twice, the FLASH_CR register is locked until the next system reset, and access to it generates a bus error."]
        #[inline(always)]
        pub const fn cukey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Control unlock key Following values must be written to FLASH_KEYR consecutively to unlock FLASH_CR register: 1st key = 0x4567 0123 2nd key = 0xCDEF 89AB Reads to this register returns zero. If above sequence is wrong or performed twice, the FLASH_CR register is locked until the next system reset, and access to it generates a bus error."]
        #[inline(always)]
        pub fn set_cukey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("cukey", &self.cukey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Keyr {{ cukey: {=u32:?} }}", self.cukey())
        }
    }
    #[doc = "FLASH non-volatile status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nvsr(pub u32);
    impl Nvsr {
        #[doc = "Non-volatile state others: invalid configuration."]
        #[inline(always)]
        pub const fn nvstate(&self) -> super::vals::NvsrNvstate {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::NvsrNvstate::from_bits(val as u8)
        }
        #[doc = "Non-volatile state others: invalid configuration."]
        #[inline(always)]
        pub fn set_nvstate(&mut self, val: super::vals::NvsrNvstate) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Nvsr {
        #[inline(always)]
        fn default() -> Nvsr {
            Nvsr(0)
        }
    }
    impl core::fmt::Debug for Nvsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nvsr").field("nvstate", &self.nvstate()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nvsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nvsr {{ nvstate: {:?} }}", self.nvstate())
        }
    }
    #[doc = "FLASH security status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nvsrp(pub u32);
    impl Nvsrp {
        #[doc = "Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF)."]
        #[inline(always)]
        pub const fn nvstate(&self) -> super::vals::NvsrpNvstate {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::NvsrpNvstate::from_bits(val as u8)
        }
        #[doc = "Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF)."]
        #[inline(always)]
        pub fn set_nvstate(&mut self, val: super::vals::NvsrpNvstate) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Nvsrp {
        #[inline(always)]
        fn default() -> Nvsrp {
            Nvsrp(0)
        }
    }
    impl core::fmt::Debug for Nvsrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nvsrp").field("nvstate", &self.nvstate()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nvsrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nvsrp {{ nvstate: {:?} }}", self.nvstate())
        }
    }
    #[doc = "FLASH option byte key control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obkcr(pub u32);
    impl Obkcr {
        #[doc = "Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them."]
        #[inline(always)]
        pub const fn obkindex(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them."]
        #[inline(always)]
        pub fn set_obkindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Next key level 10 or 11: reserved."]
        #[inline(always)]
        pub const fn nextkl(&self) -> super::vals::Nextkl {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Nextkl::from_bits(val as u8)
        }
        #[doc = "Next key level 10 or 11: reserved."]
        #[inline(always)]
        pub fn set_nextkl(&mut self, val: super::vals::Nextkl) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key."]
        #[inline(always)]
        pub const fn obksize(&self) -> super::vals::Obksize {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Obksize::from_bits(val as u8)
        }
        #[doc = "Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key."]
        #[inline(always)]
        pub fn set_obksize(&mut self, val: super::vals::Obksize) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Key program This bit must be set to write option byte keys (keys are read otherwise)."]
        #[inline(always)]
        pub const fn keyprog(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Key program This bit must be set to write option byte keys (keys are read otherwise)."]
        #[inline(always)]
        pub fn set_keyprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged."]
        #[inline(always)]
        pub const fn keystart(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged."]
        #[inline(always)]
        pub fn set_keystart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Obkcr {
        #[inline(always)]
        fn default() -> Obkcr {
            Obkcr(0)
        }
    }
    impl core::fmt::Debug for Obkcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obkcr")
                .field("obkindex", &self.obkindex())
                .field("nextkl", &self.nextkl())
                .field("obksize", &self.obksize())
                .field("keyprog", &self.keyprog())
                .field("keystart", &self.keystart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obkcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Obkcr {{ obkindex: {=u8:?}, nextkl: {:?}, obksize: {:?}, keyprog: {=bool:?}, keystart: {=bool:?} }}",
                self.obkindex(),
                self.nextkl(),
                self.obksize(),
                self.keyprog(),
                self.keystart()
            )
        }
    }
    #[doc = "FLASH option byte word 1 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obw1sr(pub u32);
    impl Obw1sr {
        #[doc = "Brownout level These bits reflects the power level that generates a system reset."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::BorLev {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::BorLev::from_bits(val as u8)
        }
        #[doc = "Brownout level These bits reflects the power level that generates a system reset."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::BorLev) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Independent watchdog HW Control."]
        #[inline(always)]
        pub const fn iwdg_hw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog HW Control."]
        #[inline(always)]
        pub fn set_iwdg_hw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Reset on stop mode."]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on stop mode."]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Reset on standby mode."]
        #[inline(always)]
        pub const fn nrst_stby(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on standby mode."]
        #[inline(always)]
        pub fn set_nrst_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "XSPIM_P1 High-Speed at Low-Voltage."]
        #[inline(always)]
        pub const fn octo1_hslv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P1 High-Speed at Low-Voltage."]
        #[inline(always)]
        pub fn set_octo1_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "XSPIM_P2 High-Speed at Low-Voltage."]
        #[inline(always)]
        pub const fn octo2_hslv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P2 High-Speed at Low-Voltage."]
        #[inline(always)]
        pub fn set_octo2_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IWDG stop mode freeze When set the independent watchdog IWDG is frozen in system Stop mode."]
        #[inline(always)]
        pub const fn iwdg_fz_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop mode freeze When set the independent watchdog IWDG is frozen in system Stop mode."]
        #[inline(always)]
        pub fn set_iwdg_fz_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IWDG standby mode freeze When set the independent watchdog IWDG is frozen in system Standby mode."]
        #[inline(always)]
        pub const fn iwdg_fz_sdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG standby mode freeze When set the independent watchdog IWDG is frozen in system Standby mode."]
        #[inline(always)]
        pub fn set_iwdg_fz_sdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Personalization OK This bit is set on STMicroelectronics production line."]
        #[inline(always)]
        pub const fn perso_ok(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Personalization OK This bit is set on STMicroelectronics production line."]
        #[inline(always)]
        pub fn set_perso_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "I/O High-Speed at Low-Voltage This bit indicates that the product operates below 2.5 V."]
        #[inline(always)]
        pub const fn vddio_hslv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "I/O High-Speed at Low-Voltage This bit indicates that the product operates below 2.5 V."]
        #[inline(always)]
        pub fn set_vddio_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Obw1sr {
        #[inline(always)]
        fn default() -> Obw1sr {
            Obw1sr(0)
        }
    }
    impl core::fmt::Debug for Obw1sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obw1sr")
                .field("bor_lev", &self.bor_lev())
                .field("iwdg_hw", &self.iwdg_hw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stby", &self.nrst_stby())
                .field("octo1_hslv", &self.octo1_hslv())
                .field("octo2_hslv", &self.octo2_hslv())
                .field("iwdg_fz_stop", &self.iwdg_fz_stop())
                .field("iwdg_fz_sdby", &self.iwdg_fz_sdby())
                .field("perso_ok", &self.perso_ok())
                .field("vddio_hslv", &self.vddio_hslv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obw1sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Obw1sr {{ bor_lev: {:?}, iwdg_hw: {=bool:?}, nrst_stop: {=bool:?}, nrst_stby: {=bool:?}, octo1_hslv: {=bool:?}, octo2_hslv: {=bool:?}, iwdg_fz_stop: {=bool:?}, iwdg_fz_sdby: {=bool:?}, perso_ok: {=bool:?}, vddio_hslv: {=bool:?} }}" , self . bor_lev () , self . iwdg_hw () , self . nrst_stop () , self . nrst_stby () , self . octo1_hslv () , self . octo2_hslv () , self . iwdg_fz_stop () , self . iwdg_fz_sdby () , self . perso_ok () , self . vddio_hslv ())
        }
    }
    #[doc = "FLASH option byte word 1 status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obw1srp(pub u32);
    impl Obw1srp {
        #[doc = "Brownout level Write to change corresponding bits in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Brownout level Write to change corresponding bits in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn iwdg_hw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_iwdg_hw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn nrst_stby(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_nrst_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn octo1_hslv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_octo1_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn octo2_hslv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_octo2_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn iwdg_fz_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_iwdg_fz_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn iwdg_fz_sdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_iwdg_fz_sdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub const fn vddio_hslv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register."]
        #[inline(always)]
        pub fn set_vddio_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Obw1srp {
        #[inline(always)]
        fn default() -> Obw1srp {
            Obw1srp(0)
        }
    }
    impl core::fmt::Debug for Obw1srp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obw1srp")
                .field("bor_lev", &self.bor_lev())
                .field("iwdg_hw", &self.iwdg_hw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stby", &self.nrst_stby())
                .field("octo1_hslv", &self.octo1_hslv())
                .field("octo2_hslv", &self.octo2_hslv())
                .field("iwdg_fz_stop", &self.iwdg_fz_stop())
                .field("iwdg_fz_sdby", &self.iwdg_fz_sdby())
                .field("vddio_hslv", &self.vddio_hslv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obw1srp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Obw1srp {{ bor_lev: {=u8:?}, iwdg_hw: {=bool:?}, nrst_stop: {=bool:?}, nrst_stby: {=bool:?}, octo1_hslv: {=bool:?}, octo2_hslv: {=bool:?}, iwdg_fz_stop: {=bool:?}, iwdg_fz_sdby: {=bool:?}, vddio_hslv: {=bool:?} }}" , self . bor_lev () , self . iwdg_hw () , self . nrst_stop () , self . nrst_stby () , self . octo1_hslv () , self . octo2_hslv () , self . iwdg_fz_stop () , self . iwdg_fz_sdby () , self . vddio_hslv ())
        }
    }
    #[doc = "FLASH option byte word 2 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obw2sr(pub u32);
    impl Obw2sr {
        #[doc = "ITCM SRAM configuration."]
        #[inline(always)]
        pub const fn itcm_axi_share(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "ITCM SRAM configuration."]
        #[inline(always)]
        pub fn set_itcm_axi_share(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "DTCM SRAM configuration."]
        #[inline(always)]
        pub const fn dtcm_axi_share(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "DTCM SRAM configuration."]
        #[inline(always)]
        pub fn set_dtcm_axi_share(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "ECC on SRAM."]
        #[inline(always)]
        pub const fn ecc_on_sram(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ECC on SRAM."]
        #[inline(always)]
        pub fn set_ecc_on_sram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I2C Not I3C."]
        #[inline(always)]
        pub const fn i2c_ni3c(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I2C Not I3C."]
        #[inline(always)]
        pub fn set_i2c_ni3c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Obw2sr {
        #[inline(always)]
        fn default() -> Obw2sr {
            Obw2sr(0)
        }
    }
    impl core::fmt::Debug for Obw2sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obw2sr")
                .field("itcm_axi_share", &self.itcm_axi_share())
                .field("dtcm_axi_share", &self.dtcm_axi_share())
                .field("ecc_on_sram", &self.ecc_on_sram())
                .field("i2c_ni3c", &self.i2c_ni3c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obw2sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Obw2sr {{ itcm_axi_share: {=u8:?}, dtcm_axi_share: {=u8:?}, ecc_on_sram: {=bool:?}, i2c_ni3c: {=bool:?} }}" , self . itcm_axi_share () , self . dtcm_axi_share () , self . ecc_on_sram () , self . i2c_ni3c ())
        }
    }
    #[doc = "FLASH option byte word 2 status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obw2srp(pub u32);
    impl Obw2srp {
        #[doc = "ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes."]
        #[inline(always)]
        pub const fn itcm_axi_share(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes."]
        #[inline(always)]
        pub fn set_itcm_axi_share(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes."]
        #[inline(always)]
        pub const fn dtcm_axi_share(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes."]
        #[inline(always)]
        pub fn set_dtcm_axi_share(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register."]
        #[inline(always)]
        pub const fn ecc_on_sram(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register."]
        #[inline(always)]
        pub fn set_ecc_on_sram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register."]
        #[inline(always)]
        pub const fn i2c_ni3c(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register."]
        #[inline(always)]
        pub fn set_i2c_ni3c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Obw2srp {
        #[inline(always)]
        fn default() -> Obw2srp {
            Obw2srp(0)
        }
    }
    impl core::fmt::Debug for Obw2srp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obw2srp")
                .field("itcm_axi_share", &self.itcm_axi_share())
                .field("dtcm_axi_share", &self.dtcm_axi_share())
                .field("ecc_on_sram", &self.ecc_on_sram())
                .field("i2c_ni3c", &self.i2c_ni3c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obw2srp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Obw2srp {{ itcm_axi_share: {=u8:?}, dtcm_axi_share: {=u8:?}, ecc_on_sram: {=bool:?}, i2c_ni3c: {=bool:?} }}" , self . itcm_axi_share () , self . dtcm_axi_share () , self . ecc_on_sram () , self . i2c_ni3c ())
        }
    }
    #[doc = "FLASH options control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[doc = "Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Program options."]
        #[inline(always)]
        pub const fn pg_opt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Program options."]
        #[inline(always)]
        pub fn set_pg_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR."]
        #[inline(always)]
        pub const fn kveie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR."]
        #[inline(always)]
        pub fn set_kveie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR."]
        #[inline(always)]
        pub const fn kteie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR."]
        #[inline(always)]
        pub fn set_kteie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change."]
        #[inline(always)]
        pub const fn opterrie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change."]
        #[inline(always)]
        pub fn set_opterrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    impl core::fmt::Debug for Optcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr")
                .field("optlock", &self.optlock())
                .field("pg_opt", &self.pg_opt())
                .field("kveie", &self.kveie())
                .field("kteie", &self.kteie())
                .field("opterrie", &self.opterrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optcr {{ optlock: {=bool:?}, pg_opt: {=bool:?}, kveie: {=bool:?}, kteie: {=bool:?}, opterrie: {=bool:?} }}" , self . optlock () , self . pg_opt () , self . kveie () , self . kteie () , self . opterrie ())
        }
    }
    #[doc = "FLASH options interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opticr(pub u32);
    impl Opticr {
        #[doc = "key valid error flag Set this bit to clear KVEF flag in FLASH_OPTISR register."]
        #[inline(always)]
        pub const fn kvef(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "key valid error flag Set this bit to clear KVEF flag in FLASH_OPTISR register."]
        #[inline(always)]
        pub fn set_kvef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "key transfer error flag Set this bit to clear KTEF flag in FLASH_OPTISR register."]
        #[inline(always)]
        pub const fn ktef(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "key transfer error flag Set this bit to clear KTEF flag in FLASH_OPTISR register."]
        #[inline(always)]
        pub fn set_ktef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Option byte change error flag Set this bit to clear OPTERRF flag in FLASH_OPTISR register."]
        #[inline(always)]
        pub const fn opterrf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error flag Set this bit to clear OPTERRF flag in FLASH_OPTISR register."]
        #[inline(always)]
        pub fn set_opterrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Opticr {
        #[inline(always)]
        fn default() -> Opticr {
            Opticr(0)
        }
    }
    impl core::fmt::Debug for Opticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opticr")
                .field("kvef", &self.kvef())
                .field("ktef", &self.ktef())
                .field("opterrf", &self.opterrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opticr {{ kvef: {=bool:?}, ktef: {=bool:?}, opterrf: {=bool:?} }}",
                self.kvef(),
                self.ktef(),
                self.opterrf()
            )
        }
    }
    #[doc = "FLASH options interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optisr(pub u32);
    impl Optisr {
        #[doc = "Key valid error flag This bit is set when loading an unknown or corrupted option byte key. More specifically: Embedded Flash did not find an option byte key that corresponds to the given OBKINDEX\\[4:0\\]
and the requested HDPL (optionally modified by NEXTKL\\[1:0\\]). It can happen for example when requested key has not being provisioned. A double error detection was found when loading the requested option byte key. In this case, if this key is provisioned again the error should disappear. When KVEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KVEIE bit of FLASH_OPTCR register is set. Setting KVEF bit of register FLASH_OPTICR clears this bit."]
        #[inline(always)]
        pub const fn kvef(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Key valid error flag This bit is set when loading an unknown or corrupted option byte key. More specifically: Embedded Flash did not find an option byte key that corresponds to the given OBKINDEX\\[4:0\\]
and the requested HDPL (optionally modified by NEXTKL\\[1:0\\]). It can happen for example when requested key has not being provisioned. A double error detection was found when loading the requested option byte key. In this case, if this key is provisioned again the error should disappear. When KVEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KVEIE bit of FLASH_OPTCR register is set. Setting KVEF bit of register FLASH_OPTICR clears this bit."]
        #[inline(always)]
        pub fn set_kvef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Key transfer error flag This bit is set when embedded Flash signals an error to the SAES peripheral. It happens when the key size (128-bit or 256-bit) is not matching between embedded Flash OBKSIZE\\[1:0\\]
and KEYSIZE bit in SAES_CR register. It also happen when an ECC dual error detection occurred while embedded Flash loaded an option byte key for the SAES peripheral. When KTEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KTEIE bit of FLASH_OPTCR register is set. Setting KTEF bit of register FLASH_OPTICR clears this bit."]
        #[inline(always)]
        pub const fn ktef(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Key transfer error flag This bit is set when embedded Flash signals an error to the SAES peripheral. It happens when the key size (128-bit or 256-bit) is not matching between embedded Flash OBKSIZE\\[1:0\\]
and KEYSIZE bit in SAES_CR register. It also happen when an ECC dual error detection occurred while embedded Flash loaded an option byte key for the SAES peripheral. When KTEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KTEIE bit of FLASH_OPTCR register is set. Setting KTEF bit of register FLASH_OPTICR clears this bit."]
        #[inline(always)]
        pub fn set_ktef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Option byte change error flag When OPTERRF is set, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTERRIE bit of FLASH_OPTCR register is set. Setting OPTERRF of register FLASH_OPTICR clears this bit."]
        #[inline(always)]
        pub const fn opterrf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error flag When OPTERRF is set, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTERRIE bit of FLASH_OPTCR register is set. Setting OPTERRF of register FLASH_OPTICR clears this bit."]
        #[inline(always)]
        pub fn set_opterrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Optisr {
        #[inline(always)]
        fn default() -> Optisr {
            Optisr(0)
        }
    }
    impl core::fmt::Debug for Optisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optisr")
                .field("kvef", &self.kvef())
                .field("ktef", &self.ktef())
                .field("opterrf", &self.opterrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optisr {{ kvef: {=bool:?}, ktef: {=bool:?}, opterrf: {=bool:?} }}",
                self.kvef(),
                self.ktef(),
                self.opterrf()
            )
        }
    }
    #[doc = "FLASH options key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optkeyr(pub u32);
    impl Optkeyr {
        #[doc = "Options configuration unlock key Following values must be written to FLASH_OPTKEYR consecutively to unlock FLASH_OPTCR register: 1st key = 0x0819 2A3B 2nd key = 0x4C5D 6E7F Reads to this register returns zero. If above sequence is performed twice locks up the corresponding register/bit until the next system reset, and generates a bus error."]
        #[inline(always)]
        pub const fn ocukey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Options configuration unlock key Following values must be written to FLASH_OPTKEYR consecutively to unlock FLASH_OPTCR register: 1st key = 0x0819 2A3B 2nd key = 0x4C5D 6E7F Reads to this register returns zero. If above sequence is performed twice locks up the corresponding register/bit until the next system reset, and generates a bus error."]
        #[inline(always)]
        pub fn set_ocukey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Optkeyr {
        #[inline(always)]
        fn default() -> Optkeyr {
            Optkeyr(0)
        }
    }
    impl core::fmt::Debug for Optkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optkeyr").field("ocukey", &self.ocukey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Optkeyr {{ ocukey: {=u32:?} }}", self.ocukey())
        }
    }
    #[doc = "FLASH OTP lock status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otplsr(pub u32);
    impl Otplsr {
        #[doc = "OTP lock n Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. OTPL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. OTPL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified."]
        #[inline(always)]
        pub const fn otpl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "OTP lock n Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. OTPL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. OTPL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified."]
        #[inline(always)]
        pub fn set_otpl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Otplsr {
        #[inline(always)]
        fn default() -> Otplsr {
            Otplsr(0)
        }
    }
    impl core::fmt::Debug for Otplsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otplsr").field("otpl", &self.otpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otplsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Otplsr {{ otpl: {=u16:?} }}", self.otpl())
        }
    }
    #[doc = "FLASH OTP lock status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otplsrp(pub u32);
    impl Otplsrp {
        #[doc = "OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared."]
        #[inline(always)]
        pub const fn otpl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared."]
        #[inline(always)]
        pub fn set_otpl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Otplsrp {
        #[inline(always)]
        fn default() -> Otplsrp {
            Otplsrp(0)
        }
    }
    impl core::fmt::Debug for Otplsrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otplsrp").field("otpl", &self.otpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otplsrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Otplsrp {{ otpl: {=u16:?} }}", self.otpl())
        }
    }
    #[doc = "FLASH RoT status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rotsr(pub u32);
    impl Rotsr {
        #[doc = "OEM provisioned device Any other value: device is not provisioned by the OEM."]
        #[inline(always)]
        pub const fn oem_provd(&self) -> super::vals::OemProvd {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::OemProvd::from_bits(val as u8)
        }
        #[doc = "OEM provisioned device Any other value: device is not provisioned by the OEM."]
        #[inline(always)]
        pub fn set_oem_provd(&mut self, val: super::vals::OemProvd) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Debug authentication method Any other value: no authentication method selected (NotSet)."]
        #[inline(always)]
        pub const fn dbg_auth(&self) -> super::vals::DbgAuth {
            let val = (self.0 >> 8usize) & 0xff;
            super::vals::DbgAuth::from_bits(val as u8)
        }
        #[doc = "Debug authentication method Any other value: no authentication method selected (NotSet)."]
        #[inline(always)]
        pub fn set_dbg_auth(&mut self, val: super::vals::DbgAuth) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
        }
        #[doc = "iRoT selection This option is ignored for STM32H7R devices (OEM iRoT is always selected). Any other value: OEM iRoT is selected at boot."]
        #[inline(always)]
        pub const fn irot_select(&self) -> super::vals::IrotSelect {
            let val = (self.0 >> 24usize) & 0xff;
            super::vals::IrotSelect::from_bits(val as u8)
        }
        #[doc = "iRoT selection This option is ignored for STM32H7R devices (OEM iRoT is always selected). Any other value: OEM iRoT is selected at boot."]
        #[inline(always)]
        pub fn set_irot_select(&mut self, val: super::vals::IrotSelect) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rotsr {
        #[inline(always)]
        fn default() -> Rotsr {
            Rotsr(0)
        }
    }
    impl core::fmt::Debug for Rotsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rotsr")
                .field("oem_provd", &self.oem_provd())
                .field("dbg_auth", &self.dbg_auth())
                .field("irot_select", &self.irot_select())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rotsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rotsr {{ oem_provd: {:?}, dbg_auth: {:?}, irot_select: {:?} }}",
                self.oem_provd(),
                self.dbg_auth(),
                self.irot_select()
            )
        }
    }
    #[doc = "FLASH RoT status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rotsrp(pub u32);
    impl Rotsrp {
        #[doc = "OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1."]
        #[inline(always)]
        pub const fn oem_provd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1."]
        #[inline(always)]
        pub fn set_oem_provd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0."]
        #[inline(always)]
        pub const fn dbg_auth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0."]
        #[inline(always)]
        pub fn set_dbg_auth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "iRoT selection This option is ignored for STM32H7R devices. Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1 and if NVSTATE is not 0xB4 (OPEN)."]
        #[inline(always)]
        pub const fn irot_select(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "iRoT selection This option is ignored for STM32H7R devices. Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1 and if NVSTATE is not 0xB4 (OPEN)."]
        #[inline(always)]
        pub fn set_irot_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rotsrp {
        #[inline(always)]
        fn default() -> Rotsrp {
            Rotsrp(0)
        }
    }
    impl core::fmt::Debug for Rotsrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rotsrp")
                .field("oem_provd", &self.oem_provd())
                .field("dbg_auth", &self.dbg_auth())
                .field("irot_select", &self.irot_select())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rotsrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rotsrp {{ oem_provd: {=u8:?}, dbg_auth: {=u8:?}, irot_select: {=u8:?} }}",
                self.oem_provd(),
                self.dbg_auth(),
                self.irot_select()
            )
        }
    }
    #[doc = "FLASH status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Busy flag This bit is set when an effective write, erase or option byte change operation is ongoing. It is possible to know what type of operation is being executed reading the flags IS_PROGRAM, IS_ERASE and IS_OPTCHANGE. BUSY cannot be cleared by application. It is automatically reset by hardware every time a step in a write, erase or option byte change operation completes. It is not recommended to do software polling on BUSY to know when one operation completed because, depending of operation, more pulses are possible for one only operation. For software polling it is therefore better to use QW flag or to check the EOPF flag."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag This bit is set when an effective write, erase or option byte change operation is ongoing. It is possible to know what type of operation is being executed reading the flags IS_PROGRAM, IS_ERASE and IS_OPTCHANGE. BUSY cannot be cleared by application. It is automatically reset by hardware every time a step in a write, erase or option byte change operation completes. It is not recommended to do software polling on BUSY to know when one operation completed because, depending of operation, more pulses are possible for one only operation. For software polling it is therefore better to use QW flag or to check the EOPF flag."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write buffer not empty flag This bit is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_CR the embedded Flash memory detects an error that involves data loss the application software has disabled write operations This bit cannot be forced to 0. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write buffer not empty flag This bit is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_CR the embedded Flash memory detects an error that involves data loss the application software has disabled write operations This bit cannot be forced to 0. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wait queue flag This bit is set when a write, erase or option byte change operation is pending in the command queue buffer. It is not possible to know what type of programming operation is present in the queue. This flag is reset by hardware when all write, erase or option byte change operations have been executed and thus removed from the waiting queue(s). This bit cannot be forced to 0. It is reset after a deterministic time if no other operations are requested."]
        #[inline(always)]
        pub const fn qw(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wait queue flag This bit is set when a write, erase or option byte change operation is pending in the command queue buffer. It is not possible to know what type of programming operation is present in the queue. This flag is reset by hardware when all write, erase or option byte change operations have been executed and thus removed from the waiting queue(s). This bit cannot be forced to 0. It is reset after a deterministic time if no other operations are requested."]
        #[inline(always)]
        pub fn set_qw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CRC busy flag This bit is set when a CRC calculation is ongoing. This bit cannot be forced to 0. The user must wait until the CRC calculation has completed or disable CRC computation using CRC_EN bit in FLASH_CR register."]
        #[inline(always)]
        pub const fn crc_busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CRC busy flag This bit is set when a CRC calculation is ongoing. This bit cannot be forced to 0. The user must wait until the CRC calculation has completed or disable CRC computation using CRC_EN bit in FLASH_CR register."]
        #[inline(always)]
        pub fn set_crc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Is a program This bit is set together with BUSY when a program operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an program operation can happen during an option change."]
        #[inline(always)]
        pub const fn is_program(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Is a program This bit is set together with BUSY when a program operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an program operation can happen during an option change."]
        #[inline(always)]
        pub fn set_is_program(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Is an erase This bit is set together with BUSY when an erase operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an erase operation can happen during an option change."]
        #[inline(always)]
        pub const fn is_erase(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Is an erase This bit is set together with BUSY when an erase operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an erase operation can happen during an option change."]
        #[inline(always)]
        pub fn set_is_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Is an option change This bit is set together with BUSY when an option change operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_PROGRAM or IS_ERASE, because a program or erase step is ongoing during option change."]
        #[inline(always)]
        pub const fn is_optchange(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Is an option change This bit is set together with BUSY when an option change operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_PROGRAM or IS_ERASE, because a program or erase step is ongoing during option change."]
        #[inline(always)]
        pub fn set_is_optchange(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Root code check flag This bit returns the status of the root code check performed following the first access to the Flash. This bit is cleared with RCHECKF bit in FLASH_FCR (optional)."]
        #[inline(always)]
        pub const fn rcheckf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Root code check flag This bit returns the status of the root code check performed following the first access to the Flash. This bit is cleared with RCHECKF bit in FLASH_FCR (optional)."]
        #[inline(always)]
        pub fn set_rcheckf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("busy", &self.busy())
                .field("wbne", &self.wbne())
                .field("qw", &self.qw())
                .field("crc_busy", &self.crc_busy())
                .field("is_program", &self.is_program())
                .field("is_erase", &self.is_erase())
                .field("is_optchange", &self.is_optchange())
                .field("rcheckf", &self.rcheckf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ busy: {=bool:?}, wbne: {=bool:?}, qw: {=bool:?}, crc_busy: {=bool:?}, is_program: {=bool:?}, is_erase: {=bool:?}, is_optchange: {=bool:?}, rcheckf: {=bool:?} }}" , self . busy () , self . wbne () , self . qw () , self . crc_busy () , self . is_program () , self . is_erase () , self . is_optchange () , self . rcheckf ())
        }
    }
    #[doc = "FLASH write protection status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpsr(pub u32);
    impl Wrpsr {
        #[doc = "Write protection for sector n This bit reflects the write protection status of user Flash sector n."]
        #[inline(always)]
        pub const fn wrps(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write protection for sector n This bit reflects the write protection status of user Flash sector n."]
        #[inline(always)]
        pub fn set_wrps(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wrpsr {
        #[inline(always)]
        fn default() -> Wrpsr {
            Wrpsr(0)
        }
    }
    impl core::fmt::Debug for Wrpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpsr").field("wrps", &self.wrps()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrpsr {{ wrps: {=u8:?} }}", self.wrps())
        }
    }
    #[doc = "FLASH write protection status register programming."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpsrp(pub u32);
    impl Wrpsrp {
        #[doc = "Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR."]
        #[inline(always)]
        pub const fn wrps(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR."]
        #[inline(always)]
        pub fn set_wrps(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wrpsrp {
        #[inline(always)]
        fn default() -> Wrpsrp {
            Wrpsrp(0)
        }
    }
    impl core::fmt::Debug for Wrpsrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpsrp").field("wrps", &self.wrps()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpsrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrpsrp {{ wrps: {=u8:?} }}", self.wrps())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BorLev {
        #[doc = "BOR OFF, POR/PDR reset threshold level is applied."]
        DISABLED = 0x0,
        #[doc = "BOR Level 1, the threshold level is low (around 2.1 V)."]
        LEVEL1 = 0x01,
        #[doc = "BOR Level 2, the threshold level is medium (around 2.4 V)."]
        LEVEL2 = 0x02,
        #[doc = "BOR Level 3, the threshold level is high (around 2.7 V)."]
        LEVEL3 = 0x03,
    }
    impl BorLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BorLev {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BorLev {
        #[inline(always)]
        fn from(val: u8) -> BorLev {
            BorLev::from_bits(val)
        }
    }
    impl From<BorLev> for u8 {
        #[inline(always)]
        fn from(val: BorLev) -> u8 {
            BorLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CrcBurst {
        #[doc = "every burst has a size of 4 Flash words (64 Bytes)."]
        WORD4 = 0x0,
        #[doc = "every burst has a size of 16 Flash words (256 Bytes)."]
        WORD16 = 0x01,
        #[doc = "every burst has a size of 64 Flash words (1 Kbytes)."]
        WORD64 = 0x02,
        #[doc = "every burst has a size of 256 Flash words (4 Kbytes)."]
        WORD256 = 0x03,
    }
    impl CrcBurst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CrcBurst {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CrcBurst {
        #[inline(always)]
        fn from(val: u8) -> CrcBurst {
            CrcBurst::from_bits(val)
        }
    }
    impl From<CrcBurst> for u8 {
        #[inline(always)]
        fn from(val: CrcBurst) -> u8 {
            CrcBurst::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgAuth(u8);
    impl DbgAuth {
        #[doc = "Authentication method using ECDSA signature (NIST P256)."]
        pub const ECDSA: Self = Self(0x51);
        #[doc = "Delegated debug (to OEM iRoT code in user Flash)."]
        pub const DELEGATED: Self = Self(0x6f);
        #[doc = "Authentication method using password."]
        pub const PASSWORD: Self = Self(0x8a);
        #[doc = "Locked device (no debug allowed)."]
        pub const LOCKED: Self = Self(0xb4);
    }
    impl DbgAuth {
        pub const fn from_bits(val: u8) -> DbgAuth {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for DbgAuth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("ECDSA"),
                0x6f => f.write_str("DELEGATED"),
                0x8a => f.write_str("PASSWORD"),
                0xb4 => f.write_str("LOCKED"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuth {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "ECDSA"),
                0x6f => defmt::write!(f, "DELEGATED"),
                0x8a => defmt::write!(f, "PASSWORD"),
                0xb4 => defmt::write!(f, "LOCKED"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for DbgAuth {
        #[inline(always)]
        fn from(val: u8) -> DbgAuth {
            DbgAuth::from_bits(val)
        }
    }
    impl From<DbgAuth> for u8 {
        #[inline(always)]
        fn from(val: DbgAuth) -> u8 {
            DbgAuth::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct IrotSelect(u8);
    impl IrotSelect {
        #[doc = "ST iRoT is selected at boot."]
        pub const SELECTED: Self = Self(0xb4);
    }
    impl IrotSelect {
        pub const fn from_bits(val: u8) -> IrotSelect {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for IrotSelect {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("SELECTED"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrotSelect {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "SELECTED"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for IrotSelect {
        #[inline(always)]
        fn from(val: u8) -> IrotSelect {
            IrotSelect::from_bits(val)
        }
    }
    impl From<IrotSelect> for u8 {
        #[inline(always)]
        fn from(val: IrotSelect) -> u8 {
            IrotSelect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nextkl {
        #[doc = "OBKINDEX represents the index of the option byte key stored for the hide protection level indicated in SBS_HDPLSR."]
        PLUS0 = 0x0,
        #[doc = "OBKINDEX represents the index of the option byte key stored for the hide protection level indicated in SBS_HDPLSR plus one (e.g. if HDPL=1 in SBS_HDPLR the key of level 2 is selected)."]
        PLUS1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Nextkl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nextkl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nextkl {
        #[inline(always)]
        fn from(val: u8) -> Nextkl {
            Nextkl::from_bits(val)
        }
    }
    impl From<Nextkl> for u8 {
        #[inline(always)]
        fn from(val: Nextkl) -> u8 {
            Nextkl::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NvsrNvstate(u8);
    impl NvsrNvstate {
        #[doc = "CLOSED device."]
        pub const CLOSED: Self = Self(0x51);
        #[doc = "OPEN device."]
        pub const OPEN: Self = Self(0xb4);
    }
    impl NvsrNvstate {
        pub const fn from_bits(val: u8) -> NvsrNvstate {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for NvsrNvstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("CLOSED"),
                0xb4 => f.write_str("OPEN"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NvsrNvstate {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "CLOSED"),
                0xb4 => defmt::write!(f, "OPEN"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for NvsrNvstate {
        #[inline(always)]
        fn from(val: u8) -> NvsrNvstate {
            NvsrNvstate::from_bits(val)
        }
    }
    impl From<NvsrNvstate> for u8 {
        #[inline(always)]
        fn from(val: NvsrNvstate) -> u8 {
            NvsrNvstate::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NvsrpNvstate(u8);
    impl NvsrpNvstate {
        #[doc = "CLOSE."]
        pub const CLOSE: Self = Self(0x51);
        #[doc = "OPEN."]
        pub const OPEN: Self = Self(0xb4);
    }
    impl NvsrpNvstate {
        pub const fn from_bits(val: u8) -> NvsrpNvstate {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for NvsrpNvstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("CLOSE"),
                0xb4 => f.write_str("OPEN"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NvsrpNvstate {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "CLOSE"),
                0xb4 => defmt::write!(f, "OPEN"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for NvsrpNvstate {
        #[inline(always)]
        fn from(val: u8) -> NvsrpNvstate {
            NvsrpNvstate::from_bits(val)
        }
    }
    impl From<NvsrpNvstate> for u8 {
        #[inline(always)]
        fn from(val: NvsrpNvstate) -> u8 {
            NvsrpNvstate::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Obksize {
        #[doc = "Key size is 32 bits."]
        BITS32 = 0x0,
        #[doc = "Key size is 64 bits."]
        BITS64 = 0x01,
        #[doc = "Key size is 128 bits."]
        BITS128 = 0x02,
        #[doc = "Key size is 256 bits."]
        BITS256 = 0x03,
    }
    impl Obksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Obksize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Obksize {
        #[inline(always)]
        fn from(val: u8) -> Obksize {
            Obksize::from_bits(val)
        }
    }
    impl From<Obksize> for u8 {
        #[inline(always)]
        fn from(val: Obksize) -> u8 {
            Obksize::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OemProvd(u8);
    impl OemProvd {
        #[doc = "Device has been provisioned by the OEM."]
        pub const PROVISIONED: Self = Self(0xb4);
    }
    impl OemProvd {
        pub const fn from_bits(val: u8) -> OemProvd {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for OemProvd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("PROVISIONED"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OemProvd {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "PROVISIONED"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OemProvd {
        #[inline(always)]
        fn from(val: u8) -> OemProvd {
            OemProvd::from_bits(val)
        }
    }
    impl From<OemProvd> for u8 {
        #[inline(always)]
        fn from(val: OemProvd) -> u8 {
            OemProvd::to_bits(val)
        }
    }
}
