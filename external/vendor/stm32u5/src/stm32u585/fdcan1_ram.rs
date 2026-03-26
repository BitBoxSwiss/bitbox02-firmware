#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crel: CREL,
    endn: ENDN,
    _reserved2: [u8; 0x04],
    dbtp: DBTP,
    test: TEST,
    rwd: RWD,
    cccr: CCCR,
    nbtp: NBTP,
    tscc: TSCC,
    tscv: TSCV,
    tocc: TOCC,
    tocv: TOCV,
    _reserved11: [u8; 0x10],
    ecr: ECR,
    psr: PSR,
    tdcr: TDCR,
    _reserved14: [u8; 0x04],
    ir: IR,
    ie: IE,
    ils: ILS,
    ile: ILE,
    _reserved18: [u8; 0x20],
    rxgfc: RXGFC,
    xidam: XIDAM,
    hpms: HPMS,
    _reserved21: [u8; 0x04],
    rxf0s: RXF0S,
    rxf0a: RXF0A,
    rxf1s: RXF1S,
    rxf1a: RXF1A,
    _reserved25: [u8; 0x20],
    txbc: TXBC,
    txfqs: TXFQS,
    txbrp: TXBRP,
    txbar: TXBAR,
    txbcr: TXBCR,
    txbto: TXBTO,
    txbcf: TXBCF,
    txbtie: TXBTIE,
    txbcie: TXBCIE,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved36: [u8; 0x14],
    ckdiv: CKDIV,
}
impl RegisterBlock {
    ///0x00 - FDCAN Core Release Register
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    ///0x04 - FDCAN endian register
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        &self.endn
    }
    ///0x0c - FDCAN Data Bit Timing and Prescaler Register
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    ///0x10 - FDCAN Test Register
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    ///0x14 - FDCAN RAM Watchdog Register
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    ///0x18 - FDCAN CC Control Register
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    ///0x1c - FDCAN Nominal Bit Timing and Prescaler Register
    #[inline(always)]
    pub const fn nbtp(&self) -> &NBTP {
        &self.nbtp
    }
    ///0x20 - FDCAN Timestamp Counter Configuration Register
    #[inline(always)]
    pub const fn tscc(&self) -> &TSCC {
        &self.tscc
    }
    ///0x24 - FDCAN Timestamp Counter Value Register
    #[inline(always)]
    pub const fn tscv(&self) -> &TSCV {
        &self.tscv
    }
    ///0x28 - FDCAN Timeout Counter Configuration Register
    #[inline(always)]
    pub const fn tocc(&self) -> &TOCC {
        &self.tocc
    }
    ///0x2c - FDCAN Timeout Counter Value Register
    #[inline(always)]
    pub const fn tocv(&self) -> &TOCV {
        &self.tocv
    }
    ///0x40 - FDCAN Error Counter Register
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    ///0x44 - FDCAN Protocol Status Register
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    ///0x48 - FDCAN Transmitter Delay Compensation Register
    #[inline(always)]
    pub const fn tdcr(&self) -> &TDCR {
        &self.tdcr
    }
    ///0x50 - FDCAN Interrupt Register
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x54 - FDCAN Interrupt Enable Register
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    ///0x58 - FDCAN Interrupt Line Select Register
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    ///0x5c - FDCAN Interrupt Line Enable Register
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    ///0x80 - FDCAN Global Filter Configuration Register
    #[inline(always)]
    pub const fn rxgfc(&self) -> &RXGFC {
        &self.rxgfc
    }
    ///0x84 - FDCAN Extended ID and Mask Register
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    ///0x88 - FDCAN High Priority Message Status Register
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    ///0x90 - FDCAN Rx FIFO 0 Status Register
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    ///0x94 - CAN Rx FIFO 0 Acknowledge Register
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    ///0x98 - FDCAN Rx FIFO 1 Status Register
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    ///0x9c - FDCAN Rx FIFO 1 Acknowledge Register
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    ///0xc0 - FDCAN Tx buffer configuration register
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    ///0xc4 - FDCAN Tx FIFO/Queue Status Register
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    ///0xc8 - FDCAN Tx Buffer Request Pending Register
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    ///0xcc - FDCAN Tx Buffer Add Request Register
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    ///0xd0 - FDCAN Tx Buffer Cancellation Request Register
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    ///0xd4 - FDCAN Tx Buffer Transmission Occurred Register
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    ///0xd8 - FDCAN Tx Buffer Cancellation Finished Register
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    ///0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    ///0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    ///0xe4 - FDCAN Tx Event FIFO Status Register
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    ///0xe8 - FDCAN Tx Event FIFO Acknowledge Register
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    ///0x100 - FDCAN CFG clock divider register
    #[inline(always)]
    pub const fn ckdiv(&self) -> &CKDIV {
        &self.ckdiv
    }
}
/**CREL (r) register accessor: FDCAN Core Release Register

You can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:CREL)

For information about available fields see [`mod@crel`] module*/
pub type CREL = crate::Reg<crel::CRELrs>;
///FDCAN Core Release Register
pub mod crel;
/**ENDN (r) register accessor: FDCAN endian register

You can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:ENDN)

For information about available fields see [`mod@endn`] module*/
pub type ENDN = crate::Reg<endn::ENDNrs>;
///FDCAN endian register
pub mod endn;
/**DBTP (rw) register accessor: FDCAN Data Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:DBTP)

For information about available fields see [`mod@dbtp`] module*/
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
///FDCAN Data Bit Timing and Prescaler Register
pub mod dbtp;
/**TEST (rw) register accessor: FDCAN Test Register

You can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TEST)

For information about available fields see [`mod@test`] module*/
pub type TEST = crate::Reg<test::TESTrs>;
///FDCAN Test Register
pub mod test;
/**RWD (rw) register accessor: FDCAN RAM Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:RWD)

For information about available fields see [`mod@rwd`] module*/
pub type RWD = crate::Reg<rwd::RWDrs>;
///FDCAN RAM Watchdog Register
pub mod rwd;
/**CCCR (rw) register accessor: FDCAN CC Control Register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///FDCAN CC Control Register
pub mod cccr;
/**NBTP (rw) register accessor: FDCAN Nominal Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:NBTP)

For information about available fields see [`mod@nbtp`] module*/
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
///FDCAN Nominal Bit Timing and Prescaler Register
pub mod nbtp;
/**TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TSCC)

For information about available fields see [`mod@tscc`] module*/
pub type TSCC = crate::Reg<tscc::TSCCrs>;
///FDCAN Timestamp Counter Configuration Register
pub mod tscc;
/**TSCV (rw) register accessor: FDCAN Timestamp Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TSCV)

For information about available fields see [`mod@tscv`] module*/
pub type TSCV = crate::Reg<tscv::TSCVrs>;
///FDCAN Timestamp Counter Value Register
pub mod tscv;
/**TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TOCC)

For information about available fields see [`mod@tocc`] module*/
pub type TOCC = crate::Reg<tocc::TOCCrs>;
///FDCAN Timeout Counter Configuration Register
pub mod tocc;
/**TOCV (rw) register accessor: FDCAN Timeout Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TOCV)

For information about available fields see [`mod@tocv`] module*/
pub type TOCV = crate::Reg<tocv::TOCVrs>;
///FDCAN Timeout Counter Value Register
pub mod tocv;
/**ECR (rw) register accessor: FDCAN Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:ECR)

For information about available fields see [`mod@ecr`] module*/
pub type ECR = crate::Reg<ecr::ECRrs>;
///FDCAN Error Counter Register
pub mod ecr;
/**PSR (rw) register accessor: FDCAN Protocol Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:PSR)

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSRrs>;
///FDCAN Protocol Status Register
pub mod psr;
/**TDCR (rw) register accessor: FDCAN Transmitter Delay Compensation Register

You can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TDCR)

For information about available fields see [`mod@tdcr`] module*/
pub type TDCR = crate::Reg<tdcr::TDCRrs>;
///FDCAN Transmitter Delay Compensation Register
pub mod tdcr;
/**IR (rw) register accessor: FDCAN Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///FDCAN Interrupt Register
pub mod ir;
/**IE (rw) register accessor: FDCAN Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:IE)

For information about available fields see [`mod@ie`] module*/
pub type IE = crate::Reg<ie::IErs>;
///FDCAN Interrupt Enable Register
pub mod ie;
/**ILS (rw) register accessor: FDCAN Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:ILS)

For information about available fields see [`mod@ils`] module*/
pub type ILS = crate::Reg<ils::ILSrs>;
///FDCAN Interrupt Line Select Register
pub mod ils;
/**ILE (rw) register accessor: FDCAN Interrupt Line Enable Register

You can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:ILE)

For information about available fields see [`mod@ile`] module*/
pub type ILE = crate::Reg<ile::ILErs>;
///FDCAN Interrupt Line Enable Register
pub mod ile;
/**RXGFC (rw) register accessor: FDCAN Global Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxgfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxgfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:RXGFC)

For information about available fields see [`mod@rxgfc`] module*/
pub type RXGFC = crate::Reg<rxgfc::RXGFCrs>;
///FDCAN Global Filter Configuration Register
pub mod rxgfc;
/**XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register

You can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:XIDAM)

For information about available fields see [`mod@xidam`] module*/
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
///FDCAN Extended ID and Mask Register
pub mod xidam;
/**HPMS (r) register accessor: FDCAN High Priority Message Status Register

You can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:HPMS)

For information about available fields see [`mod@hpms`] module*/
pub type HPMS = crate::Reg<hpms::HPMSrs>;
///FDCAN High Priority Message Status Register
pub mod hpms;
/**RXF0S (r) register accessor: FDCAN Rx FIFO 0 Status Register

You can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:RXF0S)

For information about available fields see [`mod@rxf0s`] module*/
pub type RXF0S = crate::Reg<rxf0s::RXF0Srs>;
///FDCAN Rx FIFO 0 Status Register
pub mod rxf0s;
/**RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:RXF0A)

For information about available fields see [`mod@rxf0a`] module*/
pub type RXF0A = crate::Reg<rxf0a::RXF0Ars>;
///CAN Rx FIFO 0 Acknowledge Register
pub mod rxf0a;
/**RXF1S (r) register accessor: FDCAN Rx FIFO 1 Status Register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:RXF1S)

For information about available fields see [`mod@rxf1s`] module*/
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
///FDCAN Rx FIFO 1 Status Register
pub mod rxf1s;
/**RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:RXF1A)

For information about available fields see [`mod@rxf1a`] module*/
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
///FDCAN Rx FIFO 1 Acknowledge Register
pub mod rxf1a;
/**TXBC (rw) register accessor: FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBC)

For information about available fields see [`mod@txbc`] module*/
pub type TXBC = crate::Reg<txbc::TXBCrs>;
///FDCAN Tx buffer configuration register
pub mod txbc;
/**TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register

You can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXFQS)

For information about available fields see [`mod@txfqs`] module*/
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
///FDCAN Tx FIFO/Queue Status Register
pub mod txfqs;
/**TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register

You can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBRP)

For information about available fields see [`mod@txbrp`] module*/
pub type TXBRP = crate::Reg<txbrp::TXBRPrs>;
///FDCAN Tx Buffer Request Pending Register
pub mod txbrp;
/**TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register

You can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBAR)

For information about available fields see [`mod@txbar`] module*/
pub type TXBAR = crate::Reg<txbar::TXBARrs>;
///FDCAN Tx Buffer Add Request Register
pub mod txbar;
/**TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register

You can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBCR)

For information about available fields see [`mod@txbcr`] module*/
pub type TXBCR = crate::Reg<txbcr::TXBCRrs>;
///FDCAN Tx Buffer Cancellation Request Register
pub mod txbcr;
/**TXBTO (r) register accessor: FDCAN Tx Buffer Transmission Occurred Register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBTO)

For information about available fields see [`mod@txbto`] module*/
pub type TXBTO = crate::Reg<txbto::TXBTOrs>;
///FDCAN Tx Buffer Transmission Occurred Register
pub mod txbto;
/**TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register

You can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBCF)

For information about available fields see [`mod@txbcf`] module*/
pub type TXBCF = crate::Reg<txbcf::TXBCFrs>;
///FDCAN Tx Buffer Cancellation Finished Register
pub mod txbcf;
/**TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBTIE)

For information about available fields see [`mod@txbtie`] module*/
pub type TXBTIE = crate::Reg<txbtie::TXBTIErs>;
///FDCAN Tx Buffer Transmission Interrupt Enable Register
pub mod txbtie;
/**TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXBCIE)

For information about available fields see [`mod@txbcie`] module*/
pub type TXBCIE = crate::Reg<txbcie::TXBCIErs>;
///FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
pub mod txbcie;
/**TXEFS (r) register accessor: FDCAN Tx Event FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXEFS)

For information about available fields see [`mod@txefs`] module*/
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
///FDCAN Tx Event FIFO Status Register
pub mod txefs;
/**TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:TXEFA)

For information about available fields see [`mod@txefa`] module*/
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
///FDCAN Tx Event FIFO Acknowledge Register
pub mod txefa;
/**CKDIV (rw) register accessor: FDCAN CFG clock divider register

You can [`read`](crate::Reg::read) this register and get [`ckdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:CKDIV)

For information about available fields see [`mod@ckdiv`] module*/
pub type CKDIV = crate::Reg<ckdiv::CKDIVrs>;
///FDCAN CFG clock divider register
pub mod ckdiv;
