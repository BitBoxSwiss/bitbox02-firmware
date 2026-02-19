#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    cfgr3: CFGR3,
    cr: CR,
    imr: IMR,
    sr: SR,
    icr: ICR,
    tx_ordsetr: TX_ORDSETR,
    tx_payszr: TX_PAYSZR,
    txdr: TXDR,
    rx_ordsetr: RX_ORDSETR,
    rx_payszr: RX_PAYSZR,
    rxdr: RXDR,
    rx_ordextr1: RX_ORDEXTR1,
    rx_ordextr2: RX_ORDEXTR2,
}
impl RegisterBlock {
    ///0x00 - UCPD configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x04 - UCPD configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x08 - UCPD configuration register 3
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    ///0x0c - UCPD control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - UCPD Interrupt Mask Register
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x14 - UCPD Status Register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - UCPD Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x1c - UCPD Tx Ordered Set Type Register
    #[inline(always)]
    pub const fn tx_ordsetr(&self) -> &TX_ORDSETR {
        &self.tx_ordsetr
    }
    ///0x20 - UCPD Tx payload size Register
    #[inline(always)]
    pub const fn tx_payszr(&self) -> &TX_PAYSZR {
        &self.tx_payszr
    }
    ///0x24 - UCPD Tx Data Register
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    ///0x28 - UCPD Rx Ordered Set Register
    #[inline(always)]
    pub const fn rx_ordsetr(&self) -> &RX_ORDSETR {
        &self.rx_ordsetr
    }
    ///0x2c - UCPD Rx payload size Register
    #[inline(always)]
    pub const fn rx_payszr(&self) -> &RX_PAYSZR {
        &self.rx_payszr
    }
    ///0x30 - UCPD Receive Data Register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    ///0x34 - UCPD Rx Ordered Set Extension Register 1
    #[inline(always)]
    pub const fn rx_ordextr1(&self) -> &RX_ORDEXTR1 {
        &self.rx_ordextr1
    }
    ///0x38 - UCPD Rx Ordered Set Extension Register 2
    #[inline(always)]
    pub const fn rx_ordextr2(&self) -> &RX_ORDEXTR2 {
        &self.rx_ordextr2
    }
}
/**CFGR1 (rw) register accessor: UCPD configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///UCPD configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///UCPD configuration register 2
pub mod cfgr2;
/**CFGR3 (rw) register accessor: UCPD configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:CFGR3)

For information about available fields see [`mod@cfgr3`] module*/
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
///UCPD configuration register 3
pub mod cfgr3;
/**CR (rw) register accessor: UCPD control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///UCPD control register
pub mod cr;
/**IMR (rw) register accessor: UCPD Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///UCPD Interrupt Mask Register
pub mod imr;
/**SR (r) register accessor: UCPD Status Register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///UCPD Status Register
pub mod sr;
/**ICR (w) register accessor: UCPD Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///UCPD Interrupt Clear Register
pub mod icr;
/**TX_ORDSETR (rw) register accessor: UCPD Tx Ordered Set Type Register

You can [`read`](crate::Reg::read) this register and get [`tx_ordsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ordsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:TX_ORDSETR)

For information about available fields see [`mod@tx_ordsetr`] module*/
pub type TX_ORDSETR = crate::Reg<tx_ordsetr::TX_ORDSETRrs>;
///UCPD Tx Ordered Set Type Register
pub mod tx_ordsetr;
/**TX_PAYSZR (rw) register accessor: UCPD Tx payload size Register

You can [`read`](crate::Reg::read) this register and get [`tx_payszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_payszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:TX_PAYSZR)

For information about available fields see [`mod@tx_payszr`] module*/
pub type TX_PAYSZR = crate::Reg<tx_payszr::TX_PAYSZRrs>;
///UCPD Tx payload size Register
pub mod tx_payszr;
/**TXDR (rw) register accessor: UCPD Tx Data Register

You can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:TXDR)

For information about available fields see [`mod@txdr`] module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///UCPD Tx Data Register
pub mod txdr;
/**RX_ORDSETR (r) register accessor: UCPD Rx Ordered Set Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordsetr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:RX_ORDSETR)

For information about available fields see [`mod@rx_ordsetr`] module*/
pub type RX_ORDSETR = crate::Reg<rx_ordsetr::RX_ORDSETRrs>;
///UCPD Rx Ordered Set Register
pub mod rx_ordsetr;
/**RX_PAYSZR (r) register accessor: UCPD Rx payload size Register

You can [`read`](crate::Reg::read) this register and get [`rx_payszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:RX_PAYSZR)

For information about available fields see [`mod@rx_payszr`] module*/
pub type RX_PAYSZR = crate::Reg<rx_payszr::RX_PAYSZRrs>;
///UCPD Rx payload size Register
pub mod rx_payszr;
/**RXDR (r) register accessor: UCPD Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:RXDR)

For information about available fields see [`mod@rxdr`] module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///UCPD Receive Data Register
pub mod rxdr;
/**RX_ORDEXTR1 (rw) register accessor: UCPD Rx Ordered Set Extension Register 1

You can [`read`](crate::Reg::read) this register and get [`rx_ordextr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordextr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:RX_ORDEXTR1)

For information about available fields see [`mod@rx_ordextr1`] module*/
pub type RX_ORDEXTR1 = crate::Reg<rx_ordextr1::RX_ORDEXTR1rs>;
///UCPD Rx Ordered Set Extension Register 1
pub mod rx_ordextr1;
/**RX_ORDEXTR2 (rw) register accessor: UCPD Rx Ordered Set Extension Register 2

You can [`read`](crate::Reg::read) this register and get [`rx_ordextr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordextr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#UCPD1:RX_ORDEXTR2)

For information about available fields see [`mod@rx_ordextr2`] module*/
pub type RX_ORDEXTR2 = crate::Reg<rx_ordextr2::RX_ORDEXTR2rs>;
///UCPD Rx Ordered Set Extension Register 2
pub mod rx_ordextr2;
