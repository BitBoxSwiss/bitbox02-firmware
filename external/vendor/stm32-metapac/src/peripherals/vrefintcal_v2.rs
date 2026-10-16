#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "VREFINT Factory Calibration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrefintcal {
    ptr: *mut u8,
}
unsafe impl Send for Vrefintcal {}
unsafe impl Sync for Vrefintcal {}
impl Vrefintcal {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Factory calibration"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::VrefintcalData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Factory calibration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VrefintcalData(pub u32);
    impl VrefintcalData {
        #[doc = "VREFINT calibration value"]
        #[inline(always)]
        pub const fn vrefint_cal(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x0fff;
            val as u16
        }
        #[doc = "VREFINT calibration value"]
        #[inline(always)]
        pub fn set_vrefint_cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
        }
    }
    impl Default for VrefintcalData {
        #[inline(always)]
        fn default() -> VrefintcalData {
            VrefintcalData(0)
        }
    }
    impl core::fmt::Debug for VrefintcalData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VrefintcalData")
                .field("vrefint_cal", &self.vrefint_cal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VrefintcalData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VrefintcalData {{ vrefint_cal: {=u16:?} }}", self.vrefint_cal())
        }
    }
}
