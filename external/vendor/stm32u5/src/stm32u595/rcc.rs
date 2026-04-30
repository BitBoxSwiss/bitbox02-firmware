#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x04],
    icscr1: ICSCR1,
    icscr2: ICSCR2,
    icscr3: ICSCR3,
    crrcr: CRRCR,
    _reserved5: [u8; 0x04],
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    cfgr3: CFGR3,
    pll1cfgr: PLL1CFGR,
    pll2cfgr: PLL2CFGR,
    pll3cfgr: PLL3CFGR,
    pll1divr: PLL1DIVR,
    pll1fracr: PLL1FRACR,
    pll2divr: PLL2DIVR,
    pll2fracr: PLL2FRACR,
    pll3divr: PLL3DIVR,
    pll3fracr: PLL3FRACR,
    _reserved17: [u8; 0x04],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved20: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr1: AHB2RSTR1,
    ahb2rstr2: AHB2RSTR2,
    ahb3rstr: AHB3RSTR,
    _reserved24: [u8; 0x04],
    apb1rstr1: APB1RSTR1,
    apb1rstr2: APB1RSTR2,
    apb2rstr: APB2RSTR,
    apb3rstr: APB3RSTR,
    _reserved28: [u8; 0x04],
    ahb1enr: AHB1ENR,
    ahb2enr1: AHB2ENR1,
    ahb2enr2: AHB2ENR2,
    ahb3enr: AHB3ENR,
    _reserved32: [u8; 0x04],
    apb1enr1: APB1ENR1,
    apb1enr2: APB1ENR2,
    apb2enr: APB2ENR,
    apb3enr: APB3ENR,
    _reserved36: [u8; 0x04],
    ahb1smenr: AHB1SMENR,
    ahb2smenr1: AHB2SMENR1,
    ahb2smenr2: AHB2SMENR2,
    ahb3smenr: AHB3SMENR,
    _reserved40: [u8; 0x04],
    apb1smenr1: APB1SMENR1,
    apb1smenr2: APB1SMENR2,
    apb2smenr: APB2SMENR,
    apb3smenr: APB3SMENR,
    _reserved44: [u8; 0x04],
    srdamr: SRDAMR,
    _reserved45: [u8; 0x04],
    ccipr1: CCIPR1,
    ccipr2: CCIPR2,
    ccipr3: CCIPR3,
    _reserved48: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    _reserved50: [u8; 0x18],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    ///0x00 - RCC clock control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - RCC internal clock sources calibration register 1
    #[inline(always)]
    pub const fn icscr1(&self) -> &ICSCR1 {
        &self.icscr1
    }
    ///0x0c - RCC internal clock sources calibration register 2
    #[inline(always)]
    pub const fn icscr2(&self) -> &ICSCR2 {
        &self.icscr2
    }
    ///0x10 - RCC internal clock sources calibration register 3
    #[inline(always)]
    pub const fn icscr3(&self) -> &ICSCR3 {
        &self.icscr3
    }
    ///0x14 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    ///0x1c - RCC clock configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x20 - RCC clock configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x24 - RCC clock configuration register 3
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    ///0x28 - RCC PLL1 configuration register
    #[inline(always)]
    pub const fn pll1cfgr(&self) -> &PLL1CFGR {
        &self.pll1cfgr
    }
    ///0x2c - RCC PLL2 configuration register
    #[inline(always)]
    pub const fn pll2cfgr(&self) -> &PLL2CFGR {
        &self.pll2cfgr
    }
    ///0x30 - RCC PLL3 configuration register
    #[inline(always)]
    pub const fn pll3cfgr(&self) -> &PLL3CFGR {
        &self.pll3cfgr
    }
    ///0x34 - RCC PLL1 dividers register
    #[inline(always)]
    pub const fn pll1divr(&self) -> &PLL1DIVR {
        &self.pll1divr
    }
    ///0x38 - RCC PLL1 fractional divider register
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    ///0x3c - RCC PLL2 dividers configuration register
    #[inline(always)]
    pub const fn pll2divr(&self) -> &PLL2DIVR {
        &self.pll2divr
    }
    ///0x40 - RCC PLL2 fractional divider register
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    ///0x44 - RCC PLL3 dividers configuration register
    #[inline(always)]
    pub const fn pll3divr(&self) -> &PLL3DIVR {
        &self.pll3divr
    }
    ///0x48 - RCC PLL3 fractional divider register
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    ///0x50 - RCC clock interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x54 - RCC clock interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x58 - RCC clock interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x60 - RCC AHB1 peripheral reset register
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    ///0x64 - RCC AHB2 peripheral reset register 1
    #[inline(always)]
    pub const fn ahb2rstr1(&self) -> &AHB2RSTR1 {
        &self.ahb2rstr1
    }
    ///0x68 - RCC AHB2 peripheral reset register 2
    #[inline(always)]
    pub const fn ahb2rstr2(&self) -> &AHB2RSTR2 {
        &self.ahb2rstr2
    }
    ///0x6c - RCC AHB3 peripheral reset register
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    ///0x74 - RCC APB1 peripheral reset register 1
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &APB1RSTR1 {
        &self.apb1rstr1
    }
    ///0x78 - RCC APB1 peripheral reset register 2
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &APB1RSTR2 {
        &self.apb1rstr2
    }
    ///0x7c - RCC APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x80 - RCC APB3 peripheral reset register
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
    }
    ///0x88 - RCC AHB1 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x8c - RCC AHB2 peripheral clock enable register 1
    #[inline(always)]
    pub const fn ahb2enr1(&self) -> &AHB2ENR1 {
        &self.ahb2enr1
    }
    ///0x90 - RCC AHB2 peripheral clock enable register 2
    #[inline(always)]
    pub const fn ahb2enr2(&self) -> &AHB2ENR2 {
        &self.ahb2enr2
    }
    ///0x94 - RCC AHB3 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    ///0x9c - RCC APB1 peripheral clock enable register 1
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &APB1ENR1 {
        &self.apb1enr1
    }
    ///0xa0 - RCC APB1 peripheral clock enable register 2
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &APB1ENR2 {
        &self.apb1enr2
    }
    ///0xa4 - RCC APB2 peripheral clock enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0xa8 - RCC APB3 peripheral clock enable register
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    ///0xb0 - RCC AHB1 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb1smenr(&self) -> &AHB1SMENR {
        &self.ahb1smenr
    }
    ///0xb4 - RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1
    #[inline(always)]
    pub const fn ahb2smenr1(&self) -> &AHB2SMENR1 {
        &self.ahb2smenr1
    }
    ///0xb8 - RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2
    #[inline(always)]
    pub const fn ahb2smenr2(&self) -> &AHB2SMENR2 {
        &self.ahb2smenr2
    }
    ///0xbc - RCC AHB3 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb3smenr(&self) -> &AHB3SMENR {
        &self.ahb3smenr
    }
    ///0xc4 - RCC APB1 peripheral clock enable in Sleep and Stop modes register 1
    #[inline(always)]
    pub const fn apb1smenr1(&self) -> &APB1SMENR1 {
        &self.apb1smenr1
    }
    ///0xc8 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
    #[inline(always)]
    pub const fn apb1smenr2(&self) -> &APB1SMENR2 {
        &self.apb1smenr2
    }
    ///0xcc - RCC APB2 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    ///0xd0 - RCC APB3 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn apb3smenr(&self) -> &APB3SMENR {
        &self.apb3smenr
    }
    ///0xd8 - RCC SmartRun domain peripheral autonomous mode register
    #[inline(always)]
    pub const fn srdamr(&self) -> &SRDAMR {
        &self.srdamr
    }
    ///0xe0 - RCC peripherals independent clock configuration register 1
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    ///0xe4 - RCC peripherals independent clock configuration register 2
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0xe8 - RCC peripherals independent clock configuration register 3
    #[inline(always)]
    pub const fn ccipr3(&self) -> &CCIPR3 {
        &self.ccipr3
    }
    ///0xf0 - RCC backup domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0xf4 - RCC control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x110 - RCC secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x114 - RCC privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
/**CR (rw) register accessor: RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RCC clock control register
pub mod cr;
/**ICSCR1 (rw) register accessor: RCC internal clock sources calibration register 1

You can [`read`](crate::Reg::read) this register and get [`icscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:ICSCR1)

For information about available fields see [`mod@icscr1`] module*/
pub type ICSCR1 = crate::Reg<icscr1::ICSCR1rs>;
///RCC internal clock sources calibration register 1
pub mod icscr1;
/**ICSCR2 (rw) register accessor: RCC internal clock sources calibration register 2

You can [`read`](crate::Reg::read) this register and get [`icscr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:ICSCR2)

For information about available fields see [`mod@icscr2`] module*/
pub type ICSCR2 = crate::Reg<icscr2::ICSCR2rs>;
///RCC internal clock sources calibration register 2
pub mod icscr2;
/**ICSCR3 (rw) register accessor: RCC internal clock sources calibration register 3

You can [`read`](crate::Reg::read) this register and get [`icscr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:ICSCR3)

For information about available fields see [`mod@icscr3`] module*/
pub type ICSCR3 = crate::Reg<icscr3::ICSCR3rs>;
///RCC internal clock sources calibration register 3
pub mod icscr3;
/**CRRCR (r) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CRRCR)

For information about available fields see [`mod@crrcr`] module*/
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
///RCC clock recovery RC register
pub mod crrcr;
/**CFGR1 (rw) register accessor: RCC clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///RCC clock configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///RCC clock configuration register 2
pub mod cfgr2;
/**CFGR3 (rw) register accessor: RCC clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CFGR3)

For information about available fields see [`mod@cfgr3`] module*/
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
///RCC clock configuration register 3
pub mod cfgr3;
/**PLL1CFGR (rw) register accessor: RCC PLL1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL1CFGR)

For information about available fields see [`mod@pll1cfgr`] module*/
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGRrs>;
///RCC PLL1 configuration register
pub mod pll1cfgr;
/**PLL2CFGR (rw) register accessor: RCC PLL2 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL2CFGR)

For information about available fields see [`mod@pll2cfgr`] module*/
pub type PLL2CFGR = crate::Reg<pll2cfgr::PLL2CFGRrs>;
///RCC PLL2 configuration register
pub mod pll2cfgr;
/**PLL3CFGR (rw) register accessor: RCC PLL3 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL3CFGR)

For information about available fields see [`mod@pll3cfgr`] module*/
pub type PLL3CFGR = crate::Reg<pll3cfgr::PLL3CFGRrs>;
///RCC PLL3 configuration register
pub mod pll3cfgr;
/**PLL1DIVR (rw) register accessor: RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL1DIVR)

For information about available fields see [`mod@pll1divr`] module*/
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
///RCC PLL1 dividers register
pub mod pll1divr;
/**PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL1FRACR)

For information about available fields see [`mod@pll1fracr`] module*/
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
///RCC PLL1 fractional divider register
pub mod pll1fracr;
/**PLL2DIVR (rw) register accessor: RCC PLL2 dividers configuration register

You can [`read`](crate::Reg::read) this register and get [`pll2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL2DIVR)

For information about available fields see [`mod@pll2divr`] module*/
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVRrs>;
///RCC PLL2 dividers configuration register
pub mod pll2divr;
/**PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL2FRACR)

For information about available fields see [`mod@pll2fracr`] module*/
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
///RCC PLL2 fractional divider register
pub mod pll2fracr;
/**PLL3DIVR (rw) register accessor: RCC PLL3 dividers configuration register

You can [`read`](crate::Reg::read) this register and get [`pll3divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL3DIVR)

For information about available fields see [`mod@pll3divr`] module*/
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVRrs>;
///RCC PLL3 dividers configuration register
pub mod pll3divr;
/**PLL3FRACR (rw) register accessor: RCC PLL3 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PLL3FRACR)

For information about available fields see [`mod@pll3fracr`] module*/
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
///RCC PLL3 fractional divider register
pub mod pll3fracr;
/**CIER (rw) register accessor: RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///RCC clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: RCC clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///RCC clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///RCC clock interrupt clear register
pub mod cicr;
/**AHB1RSTR (rw) register accessor: RCC AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///RCC AHB1 peripheral reset register
pub mod ahb1rstr;
/**AHB2RSTR1 (rw) register accessor: RCC AHB2 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2RSTR1)

For information about available fields see [`mod@ahb2rstr1`] module*/
pub type AHB2RSTR1 = crate::Reg<ahb2rstr1::AHB2RSTR1rs>;
///RCC AHB2 peripheral reset register 1
pub mod ahb2rstr1;
/**AHB2RSTR2 (rw) register accessor: RCC AHB2 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2RSTR2)

For information about available fields see [`mod@ahb2rstr2`] module*/
pub type AHB2RSTR2 = crate::Reg<ahb2rstr2::AHB2RSTR2rs>;
///RCC AHB2 peripheral reset register 2
pub mod ahb2rstr2;
/**AHB3RSTR (rw) register accessor: RCC AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB3RSTR)

For information about available fields see [`mod@ahb3rstr`] module*/
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
///RCC AHB3 peripheral reset register
pub mod ahb3rstr;
/**APB1RSTR1 (rw) register accessor: RCC APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB1RSTR1)

For information about available fields see [`mod@apb1rstr1`] module*/
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1rs>;
///RCC APB1 peripheral reset register 1
pub mod apb1rstr1;
/**APB1RSTR2 (rw) register accessor: RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB1RSTR2)

For information about available fields see [`mod@apb1rstr2`] module*/
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2rs>;
///RCC APB1 peripheral reset register 2
pub mod apb1rstr2;
/**APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///RCC APB2 peripheral reset register
pub mod apb2rstr;
/**APB3RSTR (rw) register accessor: RCC APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB3RSTR)

For information about available fields see [`mod@apb3rstr`] module*/
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
///RCC APB3 peripheral reset register
pub mod apb3rstr;
/**AHB1ENR (rw) register accessor: RCC AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///RCC AHB1 peripheral clock enable register
pub mod ahb1enr;
/**AHB2ENR1 (rw) register accessor: RCC AHB2 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`ahb2enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2ENR1)

For information about available fields see [`mod@ahb2enr1`] module*/
pub type AHB2ENR1 = crate::Reg<ahb2enr1::AHB2ENR1rs>;
///RCC AHB2 peripheral clock enable register 1
pub mod ahb2enr1;
/**AHB2ENR2 (rw) register accessor: RCC AHB2 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`ahb2enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2ENR2)

For information about available fields see [`mod@ahb2enr2`] module*/
pub type AHB2ENR2 = crate::Reg<ahb2enr2::AHB2ENR2rs>;
///RCC AHB2 peripheral clock enable register 2
pub mod ahb2enr2;
/**AHB3ENR (rw) register accessor: RCC AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB3ENR)

For information about available fields see [`mod@ahb3enr`] module*/
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
///RCC AHB3 peripheral clock enable register
pub mod ahb3enr;
/**APB1ENR1 (rw) register accessor: RCC APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB1ENR1)

For information about available fields see [`mod@apb1enr1`] module*/
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1rs>;
///RCC APB1 peripheral clock enable register 1
pub mod apb1enr1;
/**APB1ENR2 (rw) register accessor: RCC APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB1ENR2)

For information about available fields see [`mod@apb1enr2`] module*/
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2rs>;
///RCC APB1 peripheral clock enable register 2
pub mod apb1enr2;
/**APB2ENR (rw) register accessor: RCC APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///RCC APB2 peripheral clock enable register
pub mod apb2enr;
/**APB3ENR (rw) register accessor: RCC APB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB3ENR)

For information about available fields see [`mod@apb3enr`] module*/
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
///RCC APB3 peripheral clock enable register
pub mod apb3enr;
/**AHB1SMENR (rw) register accessor: RCC AHB1 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB1SMENR)

For information about available fields see [`mod@ahb1smenr`] module*/
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENRrs>;
///RCC AHB1 peripheral clock enable in Sleep and Stop modes register
pub mod ahb1smenr;
/**AHB2SMENR1 (rw) register accessor: RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2SMENR1)

For information about available fields see [`mod@ahb2smenr1`] module*/
pub type AHB2SMENR1 = crate::Reg<ahb2smenr1::AHB2SMENR1rs>;
///RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1
pub mod ahb2smenr1;
/**AHB2SMENR2 (rw) register accessor: RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2SMENR2)

For information about available fields see [`mod@ahb2smenr2`] module*/
pub type AHB2SMENR2 = crate::Reg<ahb2smenr2::AHB2SMENR2rs>;
///RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2
pub mod ahb2smenr2;
/**AHB3SMENR (rw) register accessor: RCC AHB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB3SMENR)

For information about available fields see [`mod@ahb3smenr`] module*/
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENRrs>;
///RCC AHB3 peripheral clock enable in Sleep and Stop modes register
pub mod ahb3smenr;
/**APB1SMENR1 (rw) register accessor: RCC APB1 peripheral clock enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB1SMENR1)

For information about available fields see [`mod@apb1smenr1`] module*/
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1rs>;
///RCC APB1 peripheral clock enable in Sleep and Stop modes register 1
pub mod apb1smenr1;
/**APB1SMENR2 (rw) register accessor: RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB1SMENR2)

For information about available fields see [`mod@apb1smenr2`] module*/
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2rs>;
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod apb1smenr2;
/**APB2SMENR (rw) register accessor: RCC APB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB2SMENR)

For information about available fields see [`mod@apb2smenr`] module*/
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
///RCC APB2 peripheral clocks enable in Sleep and Stop modes register
pub mod apb2smenr;
/**APB3SMENR (rw) register accessor: RCC APB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB3SMENR)

For information about available fields see [`mod@apb3smenr`] module*/
pub type APB3SMENR = crate::Reg<apb3smenr::APB3SMENRrs>;
///RCC APB3 peripheral clock enable in Sleep and Stop modes register
pub mod apb3smenr;
/**SRDAMR (rw) register accessor: RCC SmartRun domain peripheral autonomous mode register

You can [`read`](crate::Reg::read) this register and get [`srdamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:SRDAMR)

For information about available fields see [`mod@srdamr`] module*/
pub type SRDAMR = crate::Reg<srdamr::SRDAMRrs>;
///RCC SmartRun domain peripheral autonomous mode register
pub mod srdamr;
/**CCIPR1 (rw) register accessor: RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CCIPR1)

For information about available fields see [`mod@ccipr1`] module*/
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
///RCC peripherals independent clock configuration register 1
pub mod ccipr1;
/**CCIPR2 (rw) register accessor: RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///RCC peripherals independent clock configuration register 2
pub mod ccipr2;
/**CCIPR3 (rw) register accessor: RCC peripherals independent clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CCIPR3)

For information about available fields see [`mod@ccipr3`] module*/
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3rs>;
///RCC peripherals independent clock configuration register 3
pub mod ccipr3;
/**BDCR (rw) register accessor: RCC backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RCC backup domain control register
pub mod bdcr;
/**CSR (rw) register accessor: RCC control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///RCC control/status register
pub mod csr;
/**SECCFGR (rw) register accessor: RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///RCC secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: RCC privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///RCC privilege configuration register
pub mod privcfgr;
