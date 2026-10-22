#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrm {
    ptr: *mut u8,
}
unsafe impl Send for Rrm {}
unsafe impl Sync for Rrm {}
impl Rrm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "UDRA_CTRL0 register."]
    #[inline(always)]
    pub const fn udra_ctrl0(self) -> crate::common::Reg<regs::UdraCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "UDRA_IRQ_ENABLE register."]
    #[inline(always)]
    pub const fn udra_irq_enable(self) -> crate::common::Reg<regs::UdraIrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "UDRA_IRQ_STATUS register."]
    #[inline(always)]
    pub const fn udra_irq_status(self) -> crate::common::Reg<regs::UdraIrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "UDRA_RADIO_CFG_PTR register."]
    #[inline(always)]
    pub const fn udra_radio_cfg_ptr(self) -> crate::common::Reg<regs::UdraRadioCfgPtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SEMA_IRQ_ENABLE register."]
    #[inline(always)]
    pub const fn sema_irq_enable(self) -> crate::common::Reg<regs::SemaIrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SEMA_IRQ_STATUS register."]
    #[inline(always)]
    pub const fn sema_irq_status(self) -> crate::common::Reg<regs::SemaIrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "BLE_IRQ_ENABLE register."]
    #[inline(always)]
    pub const fn ble_irq_enable(self) -> crate::common::Reg<regs::BleIrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "BLE_IRQ_STATUS register."]
    #[inline(always)]
    pub const fn ble_irq_status(self) -> crate::common::Reg<regs::BleIrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "VP_CPU_CMD_BUS register."]
    #[inline(always)]
    pub const fn vp_cpu_cmd_bus(self) -> crate::common::Reg<regs::VpCpuCmdBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "VP_CPU_SEMA_BUS register."]
    #[inline(always)]
    pub const fn vp_cpu_sema_bus(self) -> crate::common::Reg<regs::VpCpuSemaBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "VP_CPU_IRQ_ENABLE register."]
    #[inline(always)]
    pub const fn vp_cpu_irq_enable(self) -> crate::common::Reg<regs::VpCpuIrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "VP_CPU_IRQ_STATUS register."]
    #[inline(always)]
    pub const fn vp_cpu_irq_status(self) -> crate::common::Reg<regs::VpCpuIrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
}
pub mod regs {
    #[doc = "BLE_IRQ_ENABLE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BleIrqEnable(pub u32);
    impl BleIrqEnable {
        #[doc = "IP_BLE Port grant interrupt enable."]
        #[inline(always)]
        pub const fn port_grant(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE Port grant interrupt enable."]
        #[inline(always)]
        pub fn set_port_grant(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IP_BLE Port release interrupt enable."]
        #[inline(always)]
        pub const fn port_release(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE Port release interrupt enable."]
        #[inline(always)]
        pub fn set_port_release(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IP_BLE Port command start interrup enable."]
        #[inline(always)]
        pub const fn port_cmd_start(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE Port command start interrup enable."]
        #[inline(always)]
        pub fn set_port_cmd_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IP_BLE Port command end interrup enable."]
        #[inline(always)]
        pub const fn port_cmd_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE Port command end interrup enable."]
        #[inline(always)]
        pub fn set_port_cmd_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for BleIrqEnable {
        #[inline(always)]
        fn default() -> BleIrqEnable {
            BleIrqEnable(0)
        }
    }
    impl core::fmt::Debug for BleIrqEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BleIrqEnable")
                .field("port_grant", &self.port_grant())
                .field("port_release", &self.port_release())
                .field("port_cmd_start", &self.port_cmd_start())
                .field("port_cmd_end", &self.port_cmd_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BleIrqEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BleIrqEnable {{ port_grant: {=bool:?}, port_release: {=bool:?}, port_cmd_start: {=bool:?}, port_cmd_end: {=bool:?} }}" , self . port_grant () , self . port_release () , self . port_cmd_start () , self . port_cmd_end ())
        }
    }
    #[doc = "BLE_IRQ_STATUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BleIrqStatus(pub u32);
    impl BleIrqStatus {
        #[doc = "IP_BLE hardware port granted interrupt status:."]
        #[inline(always)]
        pub const fn port_grant(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE hardware port granted interrupt status:."]
        #[inline(always)]
        pub fn set_port_grant(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IP_BLE hardware port released interrupt status."]
        #[inline(always)]
        pub const fn port_release(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE hardware port released interrupt status."]
        #[inline(always)]
        pub fn set_port_release(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IP_BLE hardware port command start interrupt status."]
        #[inline(always)]
        pub const fn cmd_start(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE hardware port command start interrupt status."]
        #[inline(always)]
        pub fn set_cmd_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IP_BLE hardware port command end interrupt status."]
        #[inline(always)]
        pub const fn cmd_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IP_BLE hardware port command end interrupt status."]
        #[inline(always)]
        pub fn set_cmd_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for BleIrqStatus {
        #[inline(always)]
        fn default() -> BleIrqStatus {
            BleIrqStatus(0)
        }
    }
    impl core::fmt::Debug for BleIrqStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BleIrqStatus")
                .field("port_grant", &self.port_grant())
                .field("port_release", &self.port_release())
                .field("cmd_start", &self.cmd_start())
                .field("cmd_end", &self.cmd_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BleIrqStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BleIrqStatus {{ port_grant: {=bool:?}, port_release: {=bool:?}, cmd_start: {=bool:?}, cmd_end: {=bool:?} }}" , self . port_grant () , self . port_release () , self . cmd_start () , self . cmd_end ())
        }
    }
    #[doc = "SEMA_IRQ_ENABLE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SemaIrqEnable(pub u32);
    impl SemaIrqEnable {
        #[doc = "semaphore locked (= one port granted) interrupt enable."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "semaphore locked (= one port granted) interrupt enable."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "semaphore unlocked (=no port selected) interrupt enable."]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "semaphore unlocked (=no port selected) interrupt enable."]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SemaIrqEnable {
        #[inline(always)]
        fn default() -> SemaIrqEnable {
            SemaIrqEnable(0)
        }
    }
    impl core::fmt::Debug for SemaIrqEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SemaIrqEnable")
                .field("lock", &self.lock())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SemaIrqEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SemaIrqEnable {{ lock: {=bool:?}, unlock: {=bool:?} }}",
                self.lock(),
                self.unlock()
            )
        }
    }
    #[doc = "SEMA_IRQ_STATUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SemaIrqStatus(pub u32);
    impl SemaIrqStatus {
        #[doc = "On read, returns the semaphore locked interrupt status."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "On read, returns the semaphore locked interrupt status."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "On read, returns the semaphore unlocked interrupt status."]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "On read, returns the semaphore unlocked interrupt status."]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SemaIrqStatus {
        #[inline(always)]
        fn default() -> SemaIrqStatus {
            SemaIrqStatus(0)
        }
    }
    impl core::fmt::Debug for SemaIrqStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SemaIrqStatus")
                .field("lock", &self.lock())
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SemaIrqStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SemaIrqStatus {{ lock: {=bool:?}, unlock: {=bool:?} }}",
                self.lock(),
                self.unlock()
            )
        }
    }
    #[doc = "UDRA_CTRL0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdraCtrl0(pub u32);
    impl UdraCtrl0 {
        #[doc = "reload the radio configuration pointer from RAM."]
        #[inline(always)]
        pub const fn reload_rdcfgptr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "reload the radio configuration pointer from RAM."]
        #[inline(always)]
        pub fn set_reload_rdcfgptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for UdraCtrl0 {
        #[inline(always)]
        fn default() -> UdraCtrl0 {
            UdraCtrl0(0)
        }
    }
    impl core::fmt::Debug for UdraCtrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UdraCtrl0")
                .field("reload_rdcfgptr", &self.reload_rdcfgptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UdraCtrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "UdraCtrl0 {{ reload_rdcfgptr: {=bool:?} }}", self.reload_rdcfgptr())
        }
    }
    #[doc = "UDRA_IRQ_ENABLE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdraIrqEnable(pub u32);
    impl UdraIrqEnable {
        #[doc = "UDRA interrupt enable (reload radio config pointer)."]
        #[inline(always)]
        pub const fn radio_cfg_ptr_reloaded(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UDRA interrupt enable (reload radio config pointer)."]
        #[inline(always)]
        pub fn set_radio_cfg_ptr_reloaded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UDRA interrupt enable (command start)."]
        #[inline(always)]
        pub const fn cmd_start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UDRA interrupt enable (command start)."]
        #[inline(always)]
        pub fn set_cmd_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "UDRA interrupt enable (command end)."]
        #[inline(always)]
        pub const fn cmd_end(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UDRA interrupt enable (command end)."]
        #[inline(always)]
        pub fn set_cmd_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for UdraIrqEnable {
        #[inline(always)]
        fn default() -> UdraIrqEnable {
            UdraIrqEnable(0)
        }
    }
    impl core::fmt::Debug for UdraIrqEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UdraIrqEnable")
                .field("radio_cfg_ptr_reloaded", &self.radio_cfg_ptr_reloaded())
                .field("cmd_start", &self.cmd_start())
                .field("cmd_end", &self.cmd_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UdraIrqEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UdraIrqEnable {{ radio_cfg_ptr_reloaded: {=bool:?}, cmd_start: {=bool:?}, cmd_end: {=bool:?} }}",
                self.radio_cfg_ptr_reloaded(),
                self.cmd_start(),
                self.cmd_end()
            )
        }
    }
    #[doc = "UDRA_IRQ_STATUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdraIrqStatus(pub u32);
    impl UdraIrqStatus {
        #[doc = "On read, returns the UDRA reload radio configuration pointer interrupt status."]
        #[inline(always)]
        pub const fn radio_cfg_ptr_reloaded(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "On read, returns the UDRA reload radio configuration pointer interrupt status."]
        #[inline(always)]
        pub fn set_radio_cfg_ptr_reloaded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "On read, returns the UDRA command start interrupt status."]
        #[inline(always)]
        pub const fn cmd_stard(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "On read, returns the UDRA command start interrupt status."]
        #[inline(always)]
        pub fn set_cmd_stard(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "On read, returns the UDRA command end interrupt status."]
        #[inline(always)]
        pub const fn cmd_end(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "On read, returns the UDRA command end interrupt status."]
        #[inline(always)]
        pub fn set_cmd_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for UdraIrqStatus {
        #[inline(always)]
        fn default() -> UdraIrqStatus {
            UdraIrqStatus(0)
        }
    }
    impl core::fmt::Debug for UdraIrqStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UdraIrqStatus")
                .field("radio_cfg_ptr_reloaded", &self.radio_cfg_ptr_reloaded())
                .field("cmd_stard", &self.cmd_stard())
                .field("cmd_end", &self.cmd_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UdraIrqStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UdraIrqStatus {{ radio_cfg_ptr_reloaded: {=bool:?}, cmd_stard: {=bool:?}, cmd_end: {=bool:?} }}",
                self.radio_cfg_ptr_reloaded(),
                self.cmd_stard(),
                self.cmd_end()
            )
        }
    }
    #[doc = "UDRA_RADIO_CFG_PTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdraRadioCfgPtr(pub u32);
    impl UdraRadioCfgPtr {
        #[doc = "UDRA radio configuration address."]
        #[inline(always)]
        pub const fn radio_config_address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "UDRA radio configuration address."]
        #[inline(always)]
        pub fn set_radio_config_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UdraRadioCfgPtr {
        #[inline(always)]
        fn default() -> UdraRadioCfgPtr {
            UdraRadioCfgPtr(0)
        }
    }
    impl core::fmt::Debug for UdraRadioCfgPtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UdraRadioCfgPtr")
                .field("radio_config_address", &self.radio_config_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UdraRadioCfgPtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UdraRadioCfgPtr {{ radio_config_address: {=u32:?} }}",
                self.radio_config_address()
            )
        }
    }
    #[doc = "VP_CPU_CMD_BUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VpCpuCmdBus(pub u32);
    impl VpCpuCmdBus {
        #[doc = "command number."]
        #[inline(always)]
        pub const fn command(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "command number."]
        #[inline(always)]
        pub fn set_command(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "CPU Virtual port command request:."]
        #[inline(always)]
        pub const fn command_req(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CPU Virtual port command request:."]
        #[inline(always)]
        pub fn set_command_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for VpCpuCmdBus {
        #[inline(always)]
        fn default() -> VpCpuCmdBus {
            VpCpuCmdBus(0)
        }
    }
    impl core::fmt::Debug for VpCpuCmdBus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VpCpuCmdBus")
                .field("command", &self.command())
                .field("command_req", &self.command_req())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VpCpuCmdBus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VpCpuCmdBus {{ command: {=u8:?}, command_req: {=bool:?} }}",
                self.command(),
                self.command_req()
            )
        }
    }
    #[doc = "VP_CPU_IRQ_ENABLE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VpCpuIrqEnable(pub u32);
    impl VpCpuIrqEnable {
        #[doc = "CPU virtual port grant interrupt enable."]
        #[inline(always)]
        pub const fn port_grant(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port grant interrupt enable."]
        #[inline(always)]
        pub fn set_port_grant(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CPU virtual port release interrupt enable."]
        #[inline(always)]
        pub const fn port_release(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port release interrupt enable."]
        #[inline(always)]
        pub fn set_port_release(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CPU virtual port command start interrup enable."]
        #[inline(always)]
        pub const fn port_cmd_start(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port command start interrup enable."]
        #[inline(always)]
        pub fn set_port_cmd_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CPU virtual port command end interrup enable."]
        #[inline(always)]
        pub const fn port_cmd_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port command end interrup enable."]
        #[inline(always)]
        pub fn set_port_cmd_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for VpCpuIrqEnable {
        #[inline(always)]
        fn default() -> VpCpuIrqEnable {
            VpCpuIrqEnable(0)
        }
    }
    impl core::fmt::Debug for VpCpuIrqEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VpCpuIrqEnable")
                .field("port_grant", &self.port_grant())
                .field("port_release", &self.port_release())
                .field("port_cmd_start", &self.port_cmd_start())
                .field("port_cmd_end", &self.port_cmd_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VpCpuIrqEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VpCpuIrqEnable {{ port_grant: {=bool:?}, port_release: {=bool:?}, port_cmd_start: {=bool:?}, port_cmd_end: {=bool:?} }}" , self . port_grant () , self . port_release () , self . port_cmd_start () , self . port_cmd_end ())
        }
    }
    #[doc = "VP_CPU_IRQ_STATUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VpCpuIrqStatus(pub u32);
    impl VpCpuIrqStatus {
        #[doc = "CPU virtual port granted interrupt status."]
        #[inline(always)]
        pub const fn port_grant(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port granted interrupt status."]
        #[inline(always)]
        pub fn set_port_grant(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "virtual port released interrupt status."]
        #[inline(always)]
        pub const fn port_release(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "virtual port released interrupt status."]
        #[inline(always)]
        pub fn set_port_release(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CPU virtual port preemption (at semaphore level) interrupt status."]
        #[inline(always)]
        pub const fn port_preempt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port preemption (at semaphore level) interrupt status."]
        #[inline(always)]
        pub fn set_port_preempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CPU virtual port command start interrupt status."]
        #[inline(always)]
        pub const fn cmd_start(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port command start interrupt status."]
        #[inline(always)]
        pub fn set_cmd_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CPU virtual port command end interrupt status."]
        #[inline(always)]
        pub const fn cmd_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CPU virtual port command end interrupt status."]
        #[inline(always)]
        pub fn set_cmd_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for VpCpuIrqStatus {
        #[inline(always)]
        fn default() -> VpCpuIrqStatus {
            VpCpuIrqStatus(0)
        }
    }
    impl core::fmt::Debug for VpCpuIrqStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VpCpuIrqStatus")
                .field("port_grant", &self.port_grant())
                .field("port_release", &self.port_release())
                .field("port_preempt", &self.port_preempt())
                .field("cmd_start", &self.cmd_start())
                .field("cmd_end", &self.cmd_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VpCpuIrqStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VpCpuIrqStatus {{ port_grant: {=bool:?}, port_release: {=bool:?}, port_preempt: {=bool:?}, cmd_start: {=bool:?}, cmd_end: {=bool:?} }}" , self . port_grant () , self . port_release () , self . port_preempt () , self . cmd_start () , self . cmd_end ())
        }
    }
    #[doc = "VP_CPU_SEMA_BUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VpCpuSemaBus(pub u32);
    impl VpCpuSemaBus {
        #[doc = "semaphore priority: priority value (between 0 and 7) of the take request."]
        #[inline(always)]
        pub const fn take_prio(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "semaphore priority: priority value (between 0 and 7) of the take request."]
        #[inline(always)]
        pub fn set_take_prio(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "semaphore token request:."]
        #[inline(always)]
        pub const fn take_req(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "semaphore token request:."]
        #[inline(always)]
        pub fn set_take_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for VpCpuSemaBus {
        #[inline(always)]
        fn default() -> VpCpuSemaBus {
            VpCpuSemaBus(0)
        }
    }
    impl core::fmt::Debug for VpCpuSemaBus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VpCpuSemaBus")
                .field("take_prio", &self.take_prio())
                .field("take_req", &self.take_req())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VpCpuSemaBus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VpCpuSemaBus {{ take_prio: {=u8:?}, take_req: {=bool:?} }}",
                self.take_prio(),
                self.take_req()
            )
        }
    }
}
