#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    cfglockr1: CFGLOCKR1,
    _reserved2: [u8; 0xec],
    seccfgr: [SECCFGR; 32],
    _reserved3: [u8; 0x80],
    privcfgr: [PRIVCFGR; 32],
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - GTZC1 SRAMz MPCBB configuration lock register
    #[inline(always)]
    pub const fn cfglockr1(&self) -> &CFGLOCKR1 {
        &self.cfglockr1
    }
    ///0x100..0x180 - MPCBBz security configuration for super-block %s register
    #[inline(always)]
    pub const fn seccfgr(&self, n: usize) -> &SECCFGR {
        &self.seccfgr[n]
    }
    ///Iterator for array of:
    ///0x100..0x180 - MPCBBz security configuration for super-block %s register
    #[inline(always)]
    pub fn seccfgr_iter(&self) -> impl Iterator<Item = &SECCFGR> {
        self.seccfgr.iter()
    }
    ///0x200..0x280 - MPCBBz privileged configuration for super-block %s register
    #[inline(always)]
    pub const fn privcfgr(&self, n: usize) -> &PRIVCFGR {
        &self.privcfgr[n]
    }
    ///Iterator for array of:
    ///0x200..0x280 - MPCBBz privileged configuration for super-block %s register
    #[inline(always)]
    pub fn privcfgr_iter(&self) -> impl Iterator<Item = &PRIVCFGR> {
        self.privcfgr.iter()
    }
}
/**CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_MPCBB3:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MPCBB control register
pub mod cr;
/**CFGLOCKR1 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`cfglockr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglockr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_MPCBB3:CFGLOCKR1)

For information about available fields see [`mod@cfglockr1`] module*/
pub type CFGLOCKR1 = crate::Reg<cfglockr1::CFGLOCKR1rs>;
///GTZC1 SRAMz MPCBB configuration lock register
pub mod cfglockr1;
/**SECCFGR (rw) register accessor: MPCBBz security configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_MPCBB3:SECCFGR[0])

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///MPCBBz security configuration for super-block %s register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: MPCBBz privileged configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_MPCBB3:PRIVCFGR[0])

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///MPCBBz privileged configuration for super-block %s register
pub mod privcfgr;
