#[repr(C)]
#[derive(Debug)]
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
pub struct LAYER {
    cr: CR,
    whpcr: WHPCR,
    wvpcr: WVPCR,
    ckcr: CKCR,
    pfcr: PFCR,
    cacr: CACR,
    dccr: DCCR,
    bfcr: BFCR,
    _reserved8: [u8; 0x08],
    cfbar: CFBAR,
    cfblr: CFBLR,
    cfblnr: CFBLNR,
    _reserved11: [u8; 0x0c],
    clutwr: CLUTWR,
    _reserved_end: [u8; 0x3c],
}
impl LAYER {
    ///0x00 -
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - LTDC layer 1 window horizontal position configuration register
    #[inline(always)]
    pub const fn whpcr(&self) -> &WHPCR {
        &self.whpcr
    }
    ///0x08 - LTDC layer 1 window vertical position configuration register
    #[inline(always)]
    pub const fn wvpcr(&self) -> &WVPCR {
        &self.wvpcr
    }
    ///0x0c - LTDC layer 1 color keying configuration register
    #[inline(always)]
    pub const fn ckcr(&self) -> &CKCR {
        &self.ckcr
    }
    ///0x10 - LTDC layer 1 pixel format configuration register
    #[inline(always)]
    pub const fn pfcr(&self) -> &PFCR {
        &self.pfcr
    }
    ///0x14 - LTDC layer 1 constant alpha configuration register
    #[inline(always)]
    pub const fn cacr(&self) -> &CACR {
        &self.cacr
    }
    ///0x18 - LTDC layer 1 default color configuration register
    #[inline(always)]
    pub const fn dccr(&self) -> &DCCR {
        &self.dccr
    }
    ///0x1c - LTDC layer 1 blending factors configuration register
    #[inline(always)]
    pub const fn bfcr(&self) -> &BFCR {
        &self.bfcr
    }
    ///0x28 - LTDC layer 1 color frame buffer address register
    #[inline(always)]
    pub const fn cfbar(&self) -> &CFBAR {
        &self.cfbar
    }
    ///0x2c - LTDC layer 1 color frame buffer length register
    #[inline(always)]
    pub const fn cfblr(&self) -> &CFBLR {
        &self.cfblr
    }
    ///0x30 - LTDC layer 1 color frame buffer line number register
    #[inline(always)]
    pub const fn cfblnr(&self) -> &CFBLNR {
        &self.cfblnr
    }
    ///0x40 - LTDC layer 1 CLUT write register
    #[inline(always)]
    pub const fn clutwr(&self) -> &CLUTWR {
        &self.clutwr
    }
}
/**CR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///
pub mod cr;
/**WHPCR (rw) register accessor: LTDC layer 1 window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@whpcr`] module*/
pub type WHPCR = crate::Reg<whpcr::WHPCRrs>;
///LTDC layer 1 window horizontal position configuration register
pub mod whpcr;
/**WVPCR (rw) register accessor: LTDC layer 1 window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wvpcr`] module*/
pub type WVPCR = crate::Reg<wvpcr::WVPCRrs>;
///LTDC layer 1 window vertical position configuration register
pub mod wvpcr;
/**CKCR (rw) register accessor: LTDC layer 1 color keying configuration register

You can [`read`](crate::Reg::read) this register and get [`ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckcr`] module*/
pub type CKCR = crate::Reg<ckcr::CKCRrs>;
///LTDC layer 1 color keying configuration register
pub mod ckcr;
/**PFCR (rw) register accessor: LTDC layer 1 pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pfcr`] module*/
pub type PFCR = crate::Reg<pfcr::PFCRrs>;
///LTDC layer 1 pixel format configuration register
pub mod pfcr;
/**CACR (rw) register accessor: LTDC layer 1 constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr`] module*/
pub type CACR = crate::Reg<cacr::CACRrs>;
///LTDC layer 1 constant alpha configuration register
pub mod cacr;
/**DCCR (rw) register accessor: LTDC layer 1 default color configuration register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dccr`] module*/
pub type DCCR = crate::Reg<dccr::DCCRrs>;
///LTDC layer 1 default color configuration register
pub mod dccr;
/**BFCR (rw) register accessor: LTDC layer 1 blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bfcr`] module*/
pub type BFCR = crate::Reg<bfcr::BFCRrs>;
///LTDC layer 1 blending factors configuration register
pub mod bfcr;
/**CFBAR (rw) register accessor: LTDC layer 1 color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfbar`] module*/
pub type CFBAR = crate::Reg<cfbar::CFBARrs>;
///LTDC layer 1 color frame buffer address register
pub mod cfbar;
/**CFBLR (rw) register accessor: LTDC layer 1 color frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfblr`] module*/
pub type CFBLR = crate::Reg<cfblr::CFBLRrs>;
///LTDC layer 1 color frame buffer length register
pub mod cfblr;
/**CFBLNR (rw) register accessor: LTDC layer 1 color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfblnr`] module*/
pub type CFBLNR = crate::Reg<cfblnr::CFBLNRrs>;
///LTDC layer 1 color frame buffer line number register
pub mod cfblnr;
/**CLUTWR (w) register accessor: LTDC layer 1 CLUT write register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clutwr`] module*/
pub type CLUTWR = crate::Reg<clutwr::CLUTWRrs>;
///LTDC layer 1 CLUT write register
pub mod clutwr;
