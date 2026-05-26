#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ier: IER,
    fcr: FCR,
    hmonr: HMONR,
    mmonr: MMONR,
    _reserved6: [u8; 0x08],
    crr0: CRR0,
    crr1: CRR1,
    crr2: CRR2,
    crr3: CRR3,
}
impl RegisterBlock {
    ///0x00 - ICACHE control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - ICACHE status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - ICACHE interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x0c - ICACHE flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - ICACHE hit monitor register
    #[inline(always)]
    pub const fn hmonr(&self) -> &HMONR {
        &self.hmonr
    }
    ///0x14 - ICACHE miss monitor register
    #[inline(always)]
    pub const fn mmonr(&self) -> &MMONR {
        &self.mmonr
    }
    ///0x20 - ICACHE region configuration register
    #[inline(always)]
    pub const fn crr0(&self) -> &CRR0 {
        &self.crr0
    }
    ///0x24 - ICACHE region configuration register
    #[inline(always)]
    pub const fn crr1(&self) -> &CRR1 {
        &self.crr1
    }
    ///0x28 - ICACHE region configuration register
    #[inline(always)]
    pub const fn crr2(&self) -> &CRR2 {
        &self.crr2
    }
    ///0x2c - ICACHE region configuration register
    #[inline(always)]
    pub const fn crr3(&self) -> &CRR3 {
        &self.crr3
    }
}
/**CR (rw) register accessor: ICACHE control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///ICACHE control register
pub mod cr;
/**SR (r) register accessor: ICACHE status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///ICACHE status register
pub mod sr;
/**IER (rw) register accessor: ICACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///ICACHE interrupt enable register
pub mod ier;
/**FCR (w) register accessor: ICACHE flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///ICACHE flag clear register
pub mod fcr;
/**HMONR (r) register accessor: ICACHE hit monitor register

You can [`read`](crate::Reg::read) this register and get [`hmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:HMONR)

For information about available fields see [`mod@hmonr`] module*/
pub type HMONR = crate::Reg<hmonr::HMONRrs>;
///ICACHE hit monitor register
pub mod hmonr;
/**MMONR (r) register accessor: ICACHE miss monitor register

You can [`read`](crate::Reg::read) this register and get [`mmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:MMONR)

For information about available fields see [`mod@mmonr`] module*/
pub type MMONR = crate::Reg<mmonr::MMONRrs>;
///ICACHE miss monitor register
pub mod mmonr;
/**CRR0 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`crr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:CRR0)

For information about available fields see [`mod@crr0`] module*/
pub type CRR0 = crate::Reg<crr0::CRR0rs>;
///ICACHE region configuration register
pub mod crr0;
/**CRR1 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`crr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:CRR1)

For information about available fields see [`mod@crr1`] module*/
pub type CRR1 = crate::Reg<crr1::CRR1rs>;
///ICACHE region configuration register
pub mod crr1;
/**CRR2 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`crr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:CRR2)

For information about available fields see [`mod@crr2`] module*/
pub type CRR2 = crate::Reg<crr2::CRR2rs>;
///ICACHE region configuration register
pub mod crr2;
/**CRR3 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`crr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ICACHE:CRR3)

For information about available fields see [`mod@crr3`] module*/
pub type CRR3 = crate::Reg<crr3::CRR3rs>;
///ICACHE region configuration register
pub mod crr3;
