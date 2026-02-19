#[repr(C)]
#[derive(Debug)]
///Cluster LUT%s, containing LUT*L, LUT*H
pub struct LUT {
    lutl: LUTL,
    luth: LUTH,
}
impl LUT {
    ///0x00 - Graphic MMU LUT entry x low
    #[inline(always)]
    pub const fn lutl(&self) -> &LUTL {
        &self.lutl
    }
    ///0x04 - Graphic MMU LUT entry x high
    #[inline(always)]
    pub const fn luth(&self) -> &LUTH {
        &self.luth
    }
}
/**LUTL (rw) register accessor: Graphic MMU LUT entry x low

You can [`read`](crate::Reg::read) this register and get [`lutl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lutl`] module*/
pub type LUTL = crate::Reg<lutl::LUTLrs>;
///Graphic MMU LUT entry x low
pub mod lutl;
/**LUTH (rw) register accessor: Graphic MMU LUT entry x high

You can [`read`](crate::Reg::read) this register and get [`luth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`luth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@luth`] module*/
pub type LUTH = crate::Reg<luth::LUTHrs>;
///Graphic MMU LUT entry x high
pub mod luth;
