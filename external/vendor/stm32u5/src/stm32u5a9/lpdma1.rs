#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    _reserved2: [u8; 0x04],
    misr: MISR,
    smisr: SMISR,
    _reserved4: [u8; 0x3c],
    ch: [CH; 4],
}
impl RegisterBlock {
    ///0x00 - LPDMA secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x04 - LPDMA privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x0c - LPDMA non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x10 - LPDMA secure masked interrupt status register
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x50..0x250 - Cluster CH%s, containing C?LBAR, C?FCR, C?SR, C?CR, C?TR1, C?TR2, C?BR1, C?SAR, C?DAR, C?LLR
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x50..0x250 - Cluster CH%s, containing C?LBAR, C?FCR, C?SR, C?CR, C?TR1, C?TR2, C?BR1, C?SAR, C?DAR, C?LLR
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
/**SECCFGR (rw) register accessor: LPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPDMA1:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///LPDMA secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: LPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPDMA1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///LPDMA privileged configuration register
pub mod privcfgr;
/**MISR (r) register accessor: LPDMA non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPDMA1:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///LPDMA non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: LPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPDMA1:SMISR)

For information about available fields see [`mod@smisr`] module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///LPDMA secure masked interrupt status register
pub mod smisr;
///Cluster CH%s, containing C?LBAR, C?FCR, C?SR, C?CR, C?TR1, C?TR2, C?BR1, C?SAR, C?DAR, C?LLR
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing C?LBAR, C?FCR, C?SR, C?CR, C?TR1, C?TR2, C?BR1, C?SAR, C?DAR, C?LLR
pub mod ch;
