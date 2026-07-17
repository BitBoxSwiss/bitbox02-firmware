#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    cfglock: CFGLOCK,
    _reserved2: [u8; 0xec],
    seccfgr: [SECCFGR; 1],
    _reserved3: [u8; 0xfc],
    privcfgr: [PRIVCFGR; 1],
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - GTZC2 SRAM4 MPCBB configuration lock register
    #[inline(always)]
    pub const fn cfglock(&self) -> &CFGLOCK {
        &self.cfglock
    }
    ///0x100 - MPCBBz security configuration for super-block %s register
    #[inline(always)]
    pub const fn seccfgr(&self, n: usize) -> &SECCFGR {
        &self.seccfgr[n]
    }
    ///Iterator for array of:
    ///0x100 - MPCBBz security configuration for super-block %s register
    #[inline(always)]
    pub fn seccfgr_iter(&self) -> impl Iterator<Item = &SECCFGR> {
        self.seccfgr.iter()
    }
    ///0x200 - MPCBBz privileged configuration for super-block %s register
    #[inline(always)]
    pub const fn privcfgr(&self, n: usize) -> &PRIVCFGR {
        &self.privcfgr[n]
    }
    ///Iterator for array of:
    ///0x200 - MPCBBz privileged configuration for super-block %s register
    #[inline(always)]
    pub fn privcfgr_iter(&self) -> impl Iterator<Item = &PRIVCFGR> {
        self.privcfgr.iter()
    }
}
/**CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_MPCBB4:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MPCBB control register
pub mod cr;
/**CFGLOCK (rw) register accessor: GTZC2 SRAM4 MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`cfglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_MPCBB4:CFGLOCK)

For information about available fields see [`mod@cfglock`] module*/
pub type CFGLOCK = crate::Reg<cfglock::CFGLOCKrs>;
///GTZC2 SRAM4 MPCBB configuration lock register
pub mod cfglock;
/**SECCFGR (rw) register accessor: MPCBBz security configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_MPCBB4:SECCFGR[0])

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///MPCBBz security configuration for super-block %s register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: MPCBBz privileged configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_MPCBB4:PRIVCFGR[0])

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///MPCBBz privileged configuration for super-block %s register
pub mod privcfgr;
