#[repr(C)]
#[derive(Debug)]
///Cluster CH%s, containing C?LBAR, C?FCR, C?SR, C?CR, C?TR1, C?TR2, C?BR1, C?SAR, C?DAR, C?LLR
pub struct CH {
    lbar: LBAR,
    _reserved1: [u8; 0x08],
    fcr: FCR,
    sr: SR,
    cr: CR,
    _reserved4: [u8; 0x28],
    tr1: TR1,
    tr2: TR2,
    br1: BR1,
    sar: SAR,
    dar: DAR,
    _reserved9: [u8; 0x28],
    llr: LLR,
}
impl CH {
    ///0x00 - LPDMA channel 0 linked-list base address register
    #[inline(always)]
    pub const fn lbar(&self) -> &LBAR {
        &self.lbar
    }
    ///0x0c - LPDMA channel 0 flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - LPDMA channel 0 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - LPDMA channel 0 control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x40 - LPDMA channel 0 transfer register 1
    #[inline(always)]
    pub const fn tr1(&self) -> &TR1 {
        &self.tr1
    }
    ///0x44 - LPDMA channel 0 transfer register 2
    #[inline(always)]
    pub const fn tr2(&self) -> &TR2 {
        &self.tr2
    }
    ///0x48 - LPDMA channel 0 block register 1
    #[inline(always)]
    pub const fn br1(&self) -> &BR1 {
        &self.br1
    }
    ///0x4c - LPDMA channel 0 source address register
    #[inline(always)]
    pub const fn sar(&self) -> &SAR {
        &self.sar
    }
    ///0x50 - LPDMA channel 0 destination address register
    #[inline(always)]
    pub const fn dar(&self) -> &DAR {
        &self.dar
    }
    ///0x7c - LPDMA channel 0 linked-list address register
    #[inline(always)]
    pub const fn llr(&self) -> &LLR {
        &self.llr
    }
}
/**LBAR (rw) register accessor: LPDMA channel 0 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lbar`] module*/
pub type LBAR = crate::Reg<lbar::LBARrs>;
///LPDMA channel 0 linked-list base address register
pub mod lbar;
/**FCR (w) register accessor: LPDMA channel 0 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///LPDMA channel 0 flag clear register
pub mod fcr;
/**SR (r) register accessor: LPDMA channel 0 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///LPDMA channel 0 status register
pub mod sr;
/**CR (rw) register accessor: LPDMA channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///LPDMA channel 0 control register
pub mod cr;
/**TR1 (rw) register accessor: LPDMA channel 0 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tr1`] module*/
pub type TR1 = crate::Reg<tr1::TR1rs>;
///LPDMA channel 0 transfer register 1
pub mod tr1;
/**TR2 (rw) register accessor: LPDMA channel 0 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tr2`] module*/
pub type TR2 = crate::Reg<tr2::TR2rs>;
///LPDMA channel 0 transfer register 2
pub mod tr2;
/**BR1 (rw) register accessor: LPDMA channel 0 block register 1

You can [`read`](crate::Reg::read) this register and get [`br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@br1`] module*/
pub type BR1 = crate::Reg<br1::BR1rs>;
///LPDMA channel 0 block register 1
pub mod br1;
/**SAR (rw) register accessor: LPDMA channel 0 source address register

You can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar`] module*/
pub type SAR = crate::Reg<sar::SARrs>;
///LPDMA channel 0 source address register
pub mod sar;
/**DAR (rw) register accessor: LPDMA channel 0 destination address register

You can [`read`](crate::Reg::read) this register and get [`dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dar`] module*/
pub type DAR = crate::Reg<dar::DARrs>;
///LPDMA channel 0 destination address register
pub mod dar;
/**LLR (rw) register accessor: LPDMA channel 0 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@llr`] module*/
pub type LLR = crate::Reg<llr::LLRrs>;
///LPDMA channel 0 linked-list address register
pub mod llr;
