#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ier: IER,
    fcr: FCR,
    rhmonr: RHMONR,
    rmmonr: RMMONR,
    _reserved6: [u8; 0x08],
    whmonr: WHMONR,
    wmmonr: WMMONR,
    cmdrsaddrr: CMDRSADDRR,
    cmdreaddrr: CMDREADDRR,
}
impl RegisterBlock {
    ///0x00 - DCACHE control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DCACHE status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - DCACHE interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x0c - DCACHE flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - DCACHE read-hit monitor register
    #[inline(always)]
    pub const fn rhmonr(&self) -> &RHMONR {
        &self.rhmonr
    }
    ///0x14 - DCACHE read-miss monitor register
    #[inline(always)]
    pub const fn rmmonr(&self) -> &RMMONR {
        &self.rmmonr
    }
    ///0x20 - write-hit monitor register
    #[inline(always)]
    pub const fn whmonr(&self) -> &WHMONR {
        &self.whmonr
    }
    ///0x24 - write-miss monitor register
    #[inline(always)]
    pub const fn wmmonr(&self) -> &WMMONR {
        &self.wmmonr
    }
    ///0x28 - command range start address register
    #[inline(always)]
    pub const fn cmdrsaddrr(&self) -> &CMDRSADDRR {
        &self.cmdrsaddrr
    }
    ///0x2c - command range start address register
    #[inline(always)]
    pub const fn cmdreaddrr(&self) -> &CMDREADDRR {
        &self.cmdreaddrr
    }
}
/**CR (rw) register accessor: DCACHE control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DCACHE control register
pub mod cr;
/**SR (r) register accessor: DCACHE status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DCACHE status register
pub mod sr;
/**IER (rw) register accessor: DCACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///DCACHE interrupt enable register
pub mod ier;
/**FCR (w) register accessor: DCACHE flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///DCACHE flag clear register
pub mod fcr;
/**RHMONR (r) register accessor: DCACHE read-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`rhmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:RHMONR)

For information about available fields see [`mod@rhmonr`] module*/
pub type RHMONR = crate::Reg<rhmonr::RHMONRrs>;
///DCACHE read-hit monitor register
pub mod rhmonr;
/**RMMONR (r) register accessor: DCACHE read-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`rmmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:RMMONR)

For information about available fields see [`mod@rmmonr`] module*/
pub type RMMONR = crate::Reg<rmmonr::RMMONRrs>;
///DCACHE read-miss monitor register
pub mod rmmonr;
/**WHMONR (r) register accessor: write-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`whmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:WHMONR)

For information about available fields see [`mod@whmonr`] module*/
pub type WHMONR = crate::Reg<whmonr::WHMONRrs>;
///write-hit monitor register
pub mod whmonr;
/**WMMONR (r) register accessor: write-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`wmmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:WMMONR)

For information about available fields see [`mod@wmmonr`] module*/
pub type WMMONR = crate::Reg<wmmonr::WMMONRrs>;
///write-miss monitor register
pub mod wmmonr;
/**CMDRSADDRR (rw) register accessor: command range start address register

You can [`read`](crate::Reg::read) this register and get [`cmdrsaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdrsaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:CMDRSADDRR)

For information about available fields see [`mod@cmdrsaddrr`] module*/
pub type CMDRSADDRR = crate::Reg<cmdrsaddrr::CMDRSADDRRrs>;
///command range start address register
pub mod cmdrsaddrr;
/**CMDREADDRR (rw) register accessor: command range start address register

You can [`read`](crate::Reg::read) this register and get [`cmdreaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdreaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:CMDREADDRR)

For information about available fields see [`mod@cmdreaddrr`] module*/
pub type CMDREADDRR = crate::Reg<cmdreaddrr::CMDREADDRRrs>;
///command range start address register
pub mod cmdreaddrr;
