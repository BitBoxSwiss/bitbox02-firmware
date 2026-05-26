#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    vosr: VOSR,
    svmcr: SVMCR,
    wucr1: WUCR1,
    wucr2: WUCR2,
    wucr3: WUCR3,
    bdcr1: BDCR1,
    bdcr2: BDCR2,
    dbpr: DBPR,
    ucpdr: UCPDR,
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    sr: SR,
    svmsr: SVMSR,
    bdsr: BDSR,
    wusr: WUSR,
    wuscr: WUSCR,
    apcr: APCR,
    pucra: PUCRA,
    pdcra: PDCRA,
    pucrb: PUCRB,
    pdcrb: PDCRB,
    pucrc: PUCRC,
    pdcrc: PDCRC,
    pucrd: PUCRD,
    pdcrd: PDCRD,
    pucre: PUCRE,
    pdcre: PDCRE,
    pucrf: PUCRF,
    pdcrf: PDCRF,
    pucrg: PUCRG,
    pdcrg: PDCRG,
    pucrh: PUCRH,
    pdcrh: PDCRH,
    pucri: PUCRI,
    pdcri: PDCRI,
    pucrj: PUCRJ,
    pdcrj: PDCRJ,
    _reserved40: [u8; 0x08],
    cr4: CR4,
    cr5: CR5,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - PWR control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - PWR control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - PWR voltage scaling register
    #[inline(always)]
    pub const fn vosr(&self) -> &VOSR {
        &self.vosr
    }
    ///0x10 - PWR supply voltage monitoring control register
    #[inline(always)]
    pub const fn svmcr(&self) -> &SVMCR {
        &self.svmcr
    }
    ///0x14 - PWR wakeup control register 1
    #[inline(always)]
    pub const fn wucr1(&self) -> &WUCR1 {
        &self.wucr1
    }
    ///0x18 - PWR wakeup control register 2
    #[inline(always)]
    pub const fn wucr2(&self) -> &WUCR2 {
        &self.wucr2
    }
    ///0x1c - PWR wakeup control register 3
    #[inline(always)]
    pub const fn wucr3(&self) -> &WUCR3 {
        &self.wucr3
    }
    ///0x20 - PWR Backup domain control register 1
    #[inline(always)]
    pub const fn bdcr1(&self) -> &BDCR1 {
        &self.bdcr1
    }
    ///0x24 - PWR Backup domain control register 2
    #[inline(always)]
    pub const fn bdcr2(&self) -> &BDCR2 {
        &self.bdcr2
    }
    ///0x28 - PWR disable Backup domain register
    #[inline(always)]
    pub const fn dbpr(&self) -> &DBPR {
        &self.dbpr
    }
    ///0x2c - PWR USB Type-C™ and Power Delivery register
    #[inline(always)]
    pub const fn ucpdr(&self) -> &UCPDR {
        &self.ucpdr
    }
    ///0x30 - PWR security configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x34 - PWR privilege control register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x38 - PWR status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x3c - PWR supply voltage monitoring status register
    #[inline(always)]
    pub const fn svmsr(&self) -> &SVMSR {
        &self.svmsr
    }
    ///0x40 - PWR Backup domain status register
    #[inline(always)]
    pub const fn bdsr(&self) -> &BDSR {
        &self.bdsr
    }
    ///0x44 - PWR wakeup status register
    #[inline(always)]
    pub const fn wusr(&self) -> &WUSR {
        &self.wusr
    }
    ///0x48 - PWR wakeup status clear register
    #[inline(always)]
    pub const fn wuscr(&self) -> &WUSCR {
        &self.wuscr
    }
    ///0x4c - PWR apply pull configuration register
    #[inline(always)]
    pub const fn apcr(&self) -> &APCR {
        &self.apcr
    }
    ///0x50 - PWR port A pull-up control register
    #[inline(always)]
    pub const fn pucra(&self) -> &PUCRA {
        &self.pucra
    }
    ///0x54 - PWR port A pull-down control register
    #[inline(always)]
    pub const fn pdcra(&self) -> &PDCRA {
        &self.pdcra
    }
    ///0x58 - PWR port B pull-up control register
    #[inline(always)]
    pub const fn pucrb(&self) -> &PUCRB {
        &self.pucrb
    }
    ///0x5c - PWR port B pull-down control register
    #[inline(always)]
    pub const fn pdcrb(&self) -> &PDCRB {
        &self.pdcrb
    }
    ///0x60 - Power port C pull up control register
    #[inline(always)]
    pub const fn pucrc(&self) -> &PUCRC {
        &self.pucrc
    }
    ///0x64 - PWR port C pull-down control register
    #[inline(always)]
    pub const fn pdcrc(&self) -> &PDCRC {
        &self.pdcrc
    }
    ///0x68 - PWR port D pull-up control register
    #[inline(always)]
    pub const fn pucrd(&self) -> &PUCRD {
        &self.pucrd
    }
    ///0x6c - PWR port D pull-down control register
    #[inline(always)]
    pub const fn pdcrd(&self) -> &PDCRD {
        &self.pdcrd
    }
    ///0x70 - PWR port E pull-up control register
    #[inline(always)]
    pub const fn pucre(&self) -> &PUCRE {
        &self.pucre
    }
    ///0x74 - PWR port E pull-down control register
    #[inline(always)]
    pub const fn pdcre(&self) -> &PDCRE {
        &self.pdcre
    }
    ///0x78 - PWR port F pull-up control register
    #[inline(always)]
    pub const fn pucrf(&self) -> &PUCRF {
        &self.pucrf
    }
    ///0x7c - PWR port F pull-down control register
    #[inline(always)]
    pub const fn pdcrf(&self) -> &PDCRF {
        &self.pdcrf
    }
    ///0x80 - PWR port G pull-up control register
    #[inline(always)]
    pub const fn pucrg(&self) -> &PUCRG {
        &self.pucrg
    }
    ///0x84 - PWR port G pull-down control register
    #[inline(always)]
    pub const fn pdcrg(&self) -> &PDCRG {
        &self.pdcrg
    }
    ///0x88 - PWR port H pull-up control register
    #[inline(always)]
    pub const fn pucrh(&self) -> &PUCRH {
        &self.pucrh
    }
    ///0x8c - PWR port H pull-down control register
    #[inline(always)]
    pub const fn pdcrh(&self) -> &PDCRH {
        &self.pdcrh
    }
    ///0x90 - PWR port I pull-up control register
    #[inline(always)]
    pub const fn pucri(&self) -> &PUCRI {
        &self.pucri
    }
    ///0x94 - PWR port I pull-down control register
    #[inline(always)]
    pub const fn pdcri(&self) -> &PDCRI {
        &self.pdcri
    }
    ///0x98 - PWR port J pull-up control register
    #[inline(always)]
    pub const fn pucrj(&self) -> &PUCRJ {
        &self.pucrj
    }
    ///0x9c - PWR port J pull-down control register
    #[inline(always)]
    pub const fn pdcrj(&self) -> &PDCRJ {
        &self.pdcrj
    }
    ///0xa8 - PWR control register 4
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    ///0xac -
    #[inline(always)]
    pub const fn cr5(&self) -> &CR5 {
        &self.cr5
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**CR2 (rw) register accessor: PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///PWR control register 2
pub mod cr2;
/**CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///PWR control register 3
pub mod cr3;
/**VOSR (rw) register accessor: PWR voltage scaling register

You can [`read`](crate::Reg::read) this register and get [`vosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:VOSR)

For information about available fields see [`mod@vosr`] module*/
pub type VOSR = crate::Reg<vosr::VOSRrs>;
///PWR voltage scaling register
pub mod vosr;
/**SVMCR (rw) register accessor: PWR supply voltage monitoring control register

You can [`read`](crate::Reg::read) this register and get [`svmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:SVMCR)

For information about available fields see [`mod@svmcr`] module*/
pub type SVMCR = crate::Reg<svmcr::SVMCRrs>;
///PWR supply voltage monitoring control register
pub mod svmcr;
/**WUCR1 (rw) register accessor: PWR wakeup control register 1

You can [`read`](crate::Reg::read) this register and get [`wucr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:WUCR1)

For information about available fields see [`mod@wucr1`] module*/
pub type WUCR1 = crate::Reg<wucr1::WUCR1rs>;
///PWR wakeup control register 1
pub mod wucr1;
/**WUCR2 (rw) register accessor: PWR wakeup control register 2

You can [`read`](crate::Reg::read) this register and get [`wucr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:WUCR2)

For information about available fields see [`mod@wucr2`] module*/
pub type WUCR2 = crate::Reg<wucr2::WUCR2rs>;
///PWR wakeup control register 2
pub mod wucr2;
/**WUCR3 (rw) register accessor: PWR wakeup control register 3

You can [`read`](crate::Reg::read) this register and get [`wucr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:WUCR3)

For information about available fields see [`mod@wucr3`] module*/
pub type WUCR3 = crate::Reg<wucr3::WUCR3rs>;
///PWR wakeup control register 3
pub mod wucr3;
/**BDCR1 (rw) register accessor: PWR Backup domain control register 1

You can [`read`](crate::Reg::read) this register and get [`bdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:BDCR1)

For information about available fields see [`mod@bdcr1`] module*/
pub type BDCR1 = crate::Reg<bdcr1::BDCR1rs>;
///PWR Backup domain control register 1
pub mod bdcr1;
/**BDCR2 (rw) register accessor: PWR Backup domain control register 2

You can [`read`](crate::Reg::read) this register and get [`bdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:BDCR2)

For information about available fields see [`mod@bdcr2`] module*/
pub type BDCR2 = crate::Reg<bdcr2::BDCR2rs>;
///PWR Backup domain control register 2
pub mod bdcr2;
/**DBPR (rw) register accessor: PWR disable Backup domain register

You can [`read`](crate::Reg::read) this register and get [`dbpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:DBPR)

For information about available fields see [`mod@dbpr`] module*/
pub type DBPR = crate::Reg<dbpr::DBPRrs>;
///PWR disable Backup domain register
pub mod dbpr;
/**UCPDR (rw) register accessor: PWR USB Type-C™ and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:UCPDR)

For information about available fields see [`mod@ucpdr`] module*/
pub type UCPDR = crate::Reg<ucpdr::UCPDRrs>;
///PWR USB Type-C™ and Power Delivery register
pub mod ucpdr;
/**SECCFGR (rw) register accessor: PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///PWR security configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: PWR privilege control register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///PWR privilege control register
pub mod privcfgr;
/**SR (rw) register accessor: PWR status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///PWR status register
pub mod sr;
/**SVMSR (r) register accessor: PWR supply voltage monitoring status register

You can [`read`](crate::Reg::read) this register and get [`svmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:SVMSR)

For information about available fields see [`mod@svmsr`] module*/
pub type SVMSR = crate::Reg<svmsr::SVMSRrs>;
///PWR supply voltage monitoring status register
pub mod svmsr;
/**BDSR (r) register accessor: PWR Backup domain status register

You can [`read`](crate::Reg::read) this register and get [`bdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:BDSR)

For information about available fields see [`mod@bdsr`] module*/
pub type BDSR = crate::Reg<bdsr::BDSRrs>;
///PWR Backup domain status register
pub mod bdsr;
/**WUSR (r) register accessor: PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`wusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:WUSR)

For information about available fields see [`mod@wusr`] module*/
pub type WUSR = crate::Reg<wusr::WUSRrs>;
///PWR wakeup status register
pub mod wusr;
/**WUSCR (rw) register accessor: PWR wakeup status clear register

You can [`read`](crate::Reg::read) this register and get [`wuscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:WUSCR)

For information about available fields see [`mod@wuscr`] module*/
pub type WUSCR = crate::Reg<wuscr::WUSCRrs>;
///PWR wakeup status clear register
pub mod wuscr;
/**APCR (rw) register accessor: PWR apply pull configuration register

You can [`read`](crate::Reg::read) this register and get [`apcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:APCR)

For information about available fields see [`mod@apcr`] module*/
pub type APCR = crate::Reg<apcr::APCRrs>;
///PWR apply pull configuration register
pub mod apcr;
/**PUCRA (rw) register accessor: PWR port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRA)

For information about available fields see [`mod@pucra`] module*/
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
///PWR port A pull-up control register
pub mod pucra;
/**PDCRA (rw) register accessor: PWR port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRA)

For information about available fields see [`mod@pdcra`] module*/
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
///PWR port A pull-down control register
pub mod pdcra;
/**PUCRB (rw) register accessor: PWR port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRB)

For information about available fields see [`mod@pucrb`] module*/
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
///PWR port B pull-up control register
pub mod pucrb;
/**PDCRB (rw) register accessor: PWR port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRB)

For information about available fields see [`mod@pdcrb`] module*/
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
///PWR port B pull-down control register
pub mod pdcrb;
/**PUCRC (rw) register accessor: Power port C pull up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRC)

For information about available fields see [`mod@pucrc`] module*/
pub type PUCRC = crate::Reg<pucrc::PUCRCrs>;
///Power port C pull up control register
pub mod pucrc;
/**PDCRC (rw) register accessor: PWR port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRC)

For information about available fields see [`mod@pdcrc`] module*/
pub type PDCRC = crate::Reg<pdcrc::PDCRCrs>;
///PWR port C pull-down control register
pub mod pdcrc;
/**PUCRD (rw) register accessor: PWR port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRD)

For information about available fields see [`mod@pucrd`] module*/
pub type PUCRD = crate::Reg<pucrd::PUCRDrs>;
///PWR port D pull-up control register
pub mod pucrd;
/**PDCRD (rw) register accessor: PWR port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRD)

For information about available fields see [`mod@pdcrd`] module*/
pub type PDCRD = crate::Reg<pdcrd::PDCRDrs>;
///PWR port D pull-down control register
pub mod pdcrd;
/**PUCRE (rw) register accessor: PWR port E pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRE)

For information about available fields see [`mod@pucre`] module*/
pub type PUCRE = crate::Reg<pucre::PUCRErs>;
///PWR port E pull-up control register
pub mod pucre;
/**PDCRE (rw) register accessor: PWR port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRE)

For information about available fields see [`mod@pdcre`] module*/
pub type PDCRE = crate::Reg<pdcre::PDCRErs>;
///PWR port E pull-down control register
pub mod pdcre;
/**PUCRF (rw) register accessor: PWR port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRF)

For information about available fields see [`mod@pucrf`] module*/
pub type PUCRF = crate::Reg<pucrf::PUCRFrs>;
///PWR port F pull-up control register
pub mod pucrf;
/**PDCRF (rw) register accessor: PWR port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRF)

For information about available fields see [`mod@pdcrf`] module*/
pub type PDCRF = crate::Reg<pdcrf::PDCRFrs>;
///PWR port F pull-down control register
pub mod pdcrf;
/**PUCRG (rw) register accessor: PWR port G pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRG)

For information about available fields see [`mod@pucrg`] module*/
pub type PUCRG = crate::Reg<pucrg::PUCRGrs>;
///PWR port G pull-up control register
pub mod pucrg;
/**PDCRG (rw) register accessor: PWR port G pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRG)

For information about available fields see [`mod@pdcrg`] module*/
pub type PDCRG = crate::Reg<pdcrg::PDCRGrs>;
///PWR port G pull-down control register
pub mod pdcrg;
/**PUCRH (rw) register accessor: PWR port H pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRH)

For information about available fields see [`mod@pucrh`] module*/
pub type PUCRH = crate::Reg<pucrh::PUCRHrs>;
///PWR port H pull-up control register
pub mod pucrh;
/**PDCRH (rw) register accessor: PWR port H pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRH)

For information about available fields see [`mod@pdcrh`] module*/
pub type PDCRH = crate::Reg<pdcrh::PDCRHrs>;
///PWR port H pull-down control register
pub mod pdcrh;
/**PUCRI (rw) register accessor: PWR port I pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRI)

For information about available fields see [`mod@pucri`] module*/
pub type PUCRI = crate::Reg<pucri::PUCRIrs>;
///PWR port I pull-up control register
pub mod pucri;
/**PDCRI (rw) register accessor: PWR port I pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRI)

For information about available fields see [`mod@pdcri`] module*/
pub type PDCRI = crate::Reg<pdcri::PDCRIrs>;
///PWR port I pull-down control register
pub mod pdcri;
/**PUCRJ (rw) register accessor: PWR port J pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PUCRJ)

For information about available fields see [`mod@pucrj`] module*/
pub type PUCRJ = crate::Reg<pucrj::PUCRJrs>;
///PWR port J pull-up control register
pub mod pucrj;
/**PDCRJ (rw) register accessor: PWR port J pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PDCRJ)

For information about available fields see [`mod@pdcrj`] module*/
pub type PDCRJ = crate::Reg<pdcrj::PDCRJrs>;
///PWR port J pull-down control register
pub mod pdcrj;
/**CR4 (rw) register accessor: PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:CR4)

For information about available fields see [`mod@cr4`] module*/
pub type CR4 = crate::Reg<cr4::CR4rs>;
///PWR control register 4
pub mod cr4;
/**CR5 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:CR5)

For information about available fields see [`mod@cr5`] module*/
pub type CR5 = crate::Reg<cr5::CR5rs>;
///
pub mod cr5;
