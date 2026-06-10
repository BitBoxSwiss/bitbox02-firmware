#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    _reserved1: [u8; 0x04],
    nskeyr: NSKEYR,
    seckeyr: SECKEYR,
    optkeyr: OPTKEYR,
    _reserved4: [u8; 0x04],
    pdkey1r: PDKEY1R,
    pdkey2r: PDKEY2R,
    nssr: NSSR,
    secsr: SECSR,
    nscr: NSCR,
    seccr: SECCR,
    eccr: ECCR,
    opsr: OPSR,
    _reserved12: [u8; 0x08],
    optr: OPTR,
    nsbootadd0r: NSBOOTADD0R,
    nsbootadd1r: NSBOOTADD1R,
    secbootadd0r: SECBOOTADD0R,
    secwm1r1: SECWM1R1,
    secwm1r2: SECWM1R2,
    wrp1ar: WRP1AR,
    wrp1br: WRP1BR,
    secwm2r1: SECWM2R1,
    secwm2r2: SECWM2R2,
    wrp2ar: WRP2AR,
    wrp2br: WRP2BR,
    oem1keyr1: OEM1KEYR1,
    oem1keyr2: OEM1KEYR2,
    oem2keyr1: OEM2KEYR1,
    oem2keyr2: OEM2KEYR2,
    sec1bbr1: SEC1BBR1,
    sec1bbr2: SEC1BBR2,
    sec1bbr3: SEC1BBR3,
    sec1bbr4: SEC1BBR4,
    sec1bbr5: SEC1BBR5,
    sec1bbr6: SEC1BBR6,
    sec1bbr7: SEC1BBR7,
    sec1bbr8: SEC1BBR8,
    sec2bbr1: SEC2BBR1,
    sec2bbr2: SEC2BBR2,
    sec2bbr3: SEC2BBR3,
    sec2bbr4: SEC2BBR4,
    sec2bbr5: SEC2BBR5,
    sec2bbr6: SEC2BBR6,
    sec2bbr7: SEC2BBR7,
    sec2bbr8: SEC2BBR8,
    sechdpcr: SECHDPCR,
    privcfgr: PRIVCFGR,
    _reserved46: [u8; 0x08],
    priv1bbr1: PRIV1BBR1,
    priv1bbr2: PRIV1BBR2,
    priv1bbr3: PRIV1BBR3,
    priv1bbr4: PRIV1BBR4,
    priv1bbr5: PRIV1BBR5,
    priv1bbr6: PRIV1BBR6,
    priv1bbr7: PRIV1BBR7,
    priv1bbr8: PRIV1BBR8,
    priv2bbr1: PRIV2BBR1,
    priv2bbr2: PRIV2BBR2,
    priv2bbr3: PRIV2BBR3,
    priv2bbr4: PRIV2BBR4,
    priv2bbr5: PRIV2BBR5,
    priv2bbr6: PRIV2BBR6,
    priv2bbr7: PRIV2BBR7,
    priv2bbr8: PRIV2BBR8,
}
impl RegisterBlock {
    ///0x00 - FLASH access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x08 - FLASH non-secure key register
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    ///0x0c - FLASH secure key register
    #[inline(always)]
    pub const fn seckeyr(&self) -> &SECKEYR {
        &self.seckeyr
    }
    ///0x10 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x18 - FLASH bank 1 power-down key register
    #[inline(always)]
    pub const fn pdkey1r(&self) -> &PDKEY1R {
        &self.pdkey1r
    }
    ///0x1c - FLASH bank 2 power-down key register
    #[inline(always)]
    pub const fn pdkey2r(&self) -> &PDKEY2R {
        &self.pdkey2r
    }
    ///0x20 - FLASH non-secure status register
    #[inline(always)]
    pub const fn nssr(&self) -> &NSSR {
        &self.nssr
    }
    ///0x24 - FLASH secure status register
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    ///0x28 - FLASH non-secure control register
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    ///0x2c - FLASH secure control register
    #[inline(always)]
    pub const fn seccr(&self) -> &SECCR {
        &self.seccr
    }
    ///0x30 - FLASH ECC register
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x34 - FLASH operation status register
    #[inline(always)]
    pub const fn opsr(&self) -> &OPSR {
        &self.opsr
    }
    ///0x40 - FLASH option register
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    ///0x44 - FLASH non-secure boot address 0 register
    #[inline(always)]
    pub const fn nsbootadd0r(&self) -> &NSBOOTADD0R {
        &self.nsbootadd0r
    }
    ///0x48 - FLASH non-secure boot address 1 register
    #[inline(always)]
    pub const fn nsbootadd1r(&self) -> &NSBOOTADD1R {
        &self.nsbootadd1r
    }
    ///0x4c - FLASH secure boot address 0 register
    #[inline(always)]
    pub const fn secbootadd0r(&self) -> &SECBOOTADD0R {
        &self.secbootadd0r
    }
    ///0x50 - FLASH secure watermark1 register 1
    #[inline(always)]
    pub const fn secwm1r1(&self) -> &SECWM1R1 {
        &self.secwm1r1
    }
    ///0x54 - FLASH secure watermark1 register 2
    #[inline(always)]
    pub const fn secwm1r2(&self) -> &SECWM1R2 {
        &self.secwm1r2
    }
    ///0x58 - FLASH WRP1 area A address register
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    ///0x5c - FLASH WRP1 area B address register
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    ///0x60 - FLASH secure watermark2 register 1
    #[inline(always)]
    pub const fn secwm2r1(&self) -> &SECWM2R1 {
        &self.secwm2r1
    }
    ///0x64 - FLASH secure watermark2 register 2
    #[inline(always)]
    pub const fn secwm2r2(&self) -> &SECWM2R2 {
        &self.secwm2r2
    }
    ///0x68 - FLASH WPR2 area A address register
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &WRP2AR {
        &self.wrp2ar
    }
    ///0x6c - FLASH WPR2 area B address register
    #[inline(always)]
    pub const fn wrp2br(&self) -> &WRP2BR {
        &self.wrp2br
    }
    ///0x70 - FLASH OEM1 key register 1
    #[inline(always)]
    pub const fn oem1keyr1(&self) -> &OEM1KEYR1 {
        &self.oem1keyr1
    }
    ///0x74 - FLASH OEM1 key register 2
    #[inline(always)]
    pub const fn oem1keyr2(&self) -> &OEM1KEYR2 {
        &self.oem1keyr2
    }
    ///0x78 - FLASH OEM2 key register 1
    #[inline(always)]
    pub const fn oem2keyr1(&self) -> &OEM2KEYR1 {
        &self.oem2keyr1
    }
    ///0x7c - FLASH OEM2 key register 2
    #[inline(always)]
    pub const fn oem2keyr2(&self) -> &OEM2KEYR2 {
        &self.oem2keyr2
    }
    ///0x80 - FLASH secure block based bank 1 register 1
    #[inline(always)]
    pub const fn sec1bbr1(&self) -> &SEC1BBR1 {
        &self.sec1bbr1
    }
    ///0x84 - FLASH secure block based bank 1 register 2
    #[inline(always)]
    pub const fn sec1bbr2(&self) -> &SEC1BBR2 {
        &self.sec1bbr2
    }
    ///0x88 - FLASH secure block based bank 1 register 3
    #[inline(always)]
    pub const fn sec1bbr3(&self) -> &SEC1BBR3 {
        &self.sec1bbr3
    }
    ///0x8c - FLASH secure block based bank 1 register 4
    #[inline(always)]
    pub const fn sec1bbr4(&self) -> &SEC1BBR4 {
        &self.sec1bbr4
    }
    ///0x90 - FLASH secure block based bank 1 register 5
    #[inline(always)]
    pub const fn sec1bbr5(&self) -> &SEC1BBR5 {
        &self.sec1bbr5
    }
    ///0x94 - FLASH secure block based bank 1 register 6
    #[inline(always)]
    pub const fn sec1bbr6(&self) -> &SEC1BBR6 {
        &self.sec1bbr6
    }
    ///0x98 - FLASH secure block based bank 1 register 7
    #[inline(always)]
    pub const fn sec1bbr7(&self) -> &SEC1BBR7 {
        &self.sec1bbr7
    }
    ///0x9c - FLASH secure block based bank 1 register 8
    #[inline(always)]
    pub const fn sec1bbr8(&self) -> &SEC1BBR8 {
        &self.sec1bbr8
    }
    ///0xa0 - FLASH secure block based bank 2 register 1
    #[inline(always)]
    pub const fn sec2bbr1(&self) -> &SEC2BBR1 {
        &self.sec2bbr1
    }
    ///0xa4 - FLASH secure block based bank 2 register 2
    #[inline(always)]
    pub const fn sec2bbr2(&self) -> &SEC2BBR2 {
        &self.sec2bbr2
    }
    ///0xa8 - FLASH secure block based bank 2 register 3
    #[inline(always)]
    pub const fn sec2bbr3(&self) -> &SEC2BBR3 {
        &self.sec2bbr3
    }
    ///0xac - FLASH secure block based bank 2 register 4
    #[inline(always)]
    pub const fn sec2bbr4(&self) -> &SEC2BBR4 {
        &self.sec2bbr4
    }
    ///0xb0 - FLASH secure block based bank 2 register 5
    #[inline(always)]
    pub const fn sec2bbr5(&self) -> &SEC2BBR5 {
        &self.sec2bbr5
    }
    ///0xb4 - FLASH secure block based bank 2 register 6
    #[inline(always)]
    pub const fn sec2bbr6(&self) -> &SEC2BBR6 {
        &self.sec2bbr6
    }
    ///0xb8 - FLASH secure block based bank 2 register 7
    #[inline(always)]
    pub const fn sec2bbr7(&self) -> &SEC2BBR7 {
        &self.sec2bbr7
    }
    ///0xbc - FLASH secure block based bank 2 register 8
    #[inline(always)]
    pub const fn sec2bbr8(&self) -> &SEC2BBR8 {
        &self.sec2bbr8
    }
    ///0xc0 - FLASH secure HDP control register
    #[inline(always)]
    pub const fn sechdpcr(&self) -> &SECHDPCR {
        &self.sechdpcr
    }
    ///0xc4 - FLASH privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0xd0 - FLASH privilege block based bank 1 register 1
    #[inline(always)]
    pub const fn priv1bbr1(&self) -> &PRIV1BBR1 {
        &self.priv1bbr1
    }
    ///0xd4 - FLASH privilege block based bank 1 register 2
    #[inline(always)]
    pub const fn priv1bbr2(&self) -> &PRIV1BBR2 {
        &self.priv1bbr2
    }
    ///0xd8 - FLASH privilege block based bank 1 register 3
    #[inline(always)]
    pub const fn priv1bbr3(&self) -> &PRIV1BBR3 {
        &self.priv1bbr3
    }
    ///0xdc - FLASH privilege block based bank 1 register 4
    #[inline(always)]
    pub const fn priv1bbr4(&self) -> &PRIV1BBR4 {
        &self.priv1bbr4
    }
    ///0xe0 - FLASH privilege block based bank 1 register 5
    #[inline(always)]
    pub const fn priv1bbr5(&self) -> &PRIV1BBR5 {
        &self.priv1bbr5
    }
    ///0xe4 - FLASH privilege block based bank 1 register 6
    #[inline(always)]
    pub const fn priv1bbr6(&self) -> &PRIV1BBR6 {
        &self.priv1bbr6
    }
    ///0xe8 - FLASH privilege block based bank 1 register 7
    #[inline(always)]
    pub const fn priv1bbr7(&self) -> &PRIV1BBR7 {
        &self.priv1bbr7
    }
    ///0xec - FLASH privilege block based bank 1 register 8
    #[inline(always)]
    pub const fn priv1bbr8(&self) -> &PRIV1BBR8 {
        &self.priv1bbr8
    }
    ///0xf0 - FLASH privilege block based bank 2 register 1
    #[inline(always)]
    pub const fn priv2bbr1(&self) -> &PRIV2BBR1 {
        &self.priv2bbr1
    }
    ///0xf4 - FLASH privilege block based bank 2 register 2
    #[inline(always)]
    pub const fn priv2bbr2(&self) -> &PRIV2BBR2 {
        &self.priv2bbr2
    }
    ///0xf8 - FLASH privilege block based bank 2 register 3
    #[inline(always)]
    pub const fn priv2bbr3(&self) -> &PRIV2BBR3 {
        &self.priv2bbr3
    }
    ///0xfc - FLASH privilege block based bank 2 register 4
    #[inline(always)]
    pub const fn priv2bbr4(&self) -> &PRIV2BBR4 {
        &self.priv2bbr4
    }
    ///0x100 - FLASH privilege block based bank 2 register 5
    #[inline(always)]
    pub const fn priv2bbr5(&self) -> &PRIV2BBR5 {
        &self.priv2bbr5
    }
    ///0x104 - FLASH privilege block based bank 2 register 6
    #[inline(always)]
    pub const fn priv2bbr6(&self) -> &PRIV2BBR6 {
        &self.priv2bbr6
    }
    ///0x108 - FLASH privilege block based bank 2 register 7
    #[inline(always)]
    pub const fn priv2bbr7(&self) -> &PRIV2BBR7 {
        &self.priv2bbr7
    }
    ///0x10c - FLASH privilege block based bank 2 register 8
    #[inline(always)]
    pub const fn priv2bbr8(&self) -> &PRIV2BBR8 {
        &self.priv2bbr8
    }
}
/**ACR (rw) register accessor: FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///FLASH access control register
pub mod acr;
/**NSKEYR (w) register accessor: FLASH non-secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:NSKEYR)

For information about available fields see [`mod@nskeyr`] module*/
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
///FLASH non-secure key register
pub mod nskeyr;
/**SECKEYR (w) register accessor: FLASH secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECKEYR)

For information about available fields see [`mod@seckeyr`] module*/
pub type SECKEYR = crate::Reg<seckeyr::SECKEYRrs>;
///FLASH secure key register
pub mod seckeyr;
/**OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH option key register
pub mod optkeyr;
/**PDKEY1R (w) register accessor: FLASH bank 1 power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkey1r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PDKEY1R)

For information about available fields see [`mod@pdkey1r`] module*/
pub type PDKEY1R = crate::Reg<pdkey1r::PDKEY1Rrs>;
///FLASH bank 1 power-down key register
pub mod pdkey1r;
/**PDKEY2R (w) register accessor: FLASH bank 2 power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkey2r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PDKEY2R)

For information about available fields see [`mod@pdkey2r`] module*/
pub type PDKEY2R = crate::Reg<pdkey2r::PDKEY2Rrs>;
///FLASH bank 2 power-down key register
pub mod pdkey2r;
/**NSSR (rw) register accessor: FLASH non-secure status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:NSSR)

For information about available fields see [`mod@nssr`] module*/
pub type NSSR = crate::Reg<nssr::NSSRrs>;
///FLASH non-secure status register
pub mod nssr;
/**SECSR (rw) register accessor: FLASH secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECSR)

For information about available fields see [`mod@secsr`] module*/
pub type SECSR = crate::Reg<secsr::SECSRrs>;
///FLASH secure status register
pub mod secsr;
/**NSCR (rw) register accessor: FLASH non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:NSCR)

For information about available fields see [`mod@nscr`] module*/
pub type NSCR = crate::Reg<nscr::NSCRrs>;
///FLASH non-secure control register
pub mod nscr;
/**SECCR (rw) register accessor: FLASH secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECCR)

For information about available fields see [`mod@seccr`] module*/
pub type SECCR = crate::Reg<seccr::SECCRrs>;
///FLASH secure control register
pub mod seccr;
/**ECCR (rw) register accessor: FLASH ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///FLASH ECC register
pub mod eccr;
/**OPSR (r) register accessor: FLASH operation status register

You can [`read`](crate::Reg::read) this register and get [`opsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OPSR)

For information about available fields see [`mod@opsr`] module*/
pub type OPSR = crate::Reg<opsr::OPSRrs>;
///FLASH operation status register
pub mod opsr;
/**OPTR (rw) register accessor: FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OPTR)

For information about available fields see [`mod@optr`] module*/
pub type OPTR = crate::Reg<optr::OPTRrs>;
///FLASH option register
pub mod optr;
/**NSBOOTADD0R (rw) register accessor: FLASH non-secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`nsbootadd0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:NSBOOTADD0R)

For information about available fields see [`mod@nsbootadd0r`] module*/
pub type NSBOOTADD0R = crate::Reg<nsbootadd0r::NSBOOTADD0Rrs>;
///FLASH non-secure boot address 0 register
pub mod nsbootadd0r;
/**NSBOOTADD1R (rw) register accessor: FLASH non-secure boot address 1 register

You can [`read`](crate::Reg::read) this register and get [`nsbootadd1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:NSBOOTADD1R)

For information about available fields see [`mod@nsbootadd1r`] module*/
pub type NSBOOTADD1R = crate::Reg<nsbootadd1r::NSBOOTADD1Rrs>;
///FLASH non-secure boot address 1 register
pub mod nsbootadd1r;
/**SECBOOTADD0R (rw) register accessor: FLASH secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`secbootadd0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbootadd0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECBOOTADD0R)

For information about available fields see [`mod@secbootadd0r`] module*/
pub type SECBOOTADD0R = crate::Reg<secbootadd0r::SECBOOTADD0Rrs>;
///FLASH secure boot address 0 register
pub mod secbootadd0r;
/**SECWM1R1 (rw) register accessor: FLASH secure watermark1 register 1

You can [`read`](crate::Reg::read) this register and get [`secwm1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECWM1R1)

For information about available fields see [`mod@secwm1r1`] module*/
pub type SECWM1R1 = crate::Reg<secwm1r1::SECWM1R1rs>;
///FLASH secure watermark1 register 1
pub mod secwm1r1;
/**SECWM1R2 (rw) register accessor: FLASH secure watermark1 register 2

You can [`read`](crate::Reg::read) this register and get [`secwm1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECWM1R2)

For information about available fields see [`mod@secwm1r2`] module*/
pub type SECWM1R2 = crate::Reg<secwm1r2::SECWM1R2rs>;
///FLASH secure watermark1 register 2
pub mod secwm1r2;
/**WRP1AR (rw) register accessor: FLASH WRP1 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:WRP1AR)

For information about available fields see [`mod@wrp1ar`] module*/
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
///FLASH WRP1 area A address register
pub mod wrp1ar;
/**WRP1BR (rw) register accessor: FLASH WRP1 area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:WRP1BR)

For information about available fields see [`mod@wrp1br`] module*/
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
///FLASH WRP1 area B address register
pub mod wrp1br;
/**SECWM2R1 (rw) register accessor: FLASH secure watermark2 register 1

You can [`read`](crate::Reg::read) this register and get [`secwm2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECWM2R1)

For information about available fields see [`mod@secwm2r1`] module*/
pub type SECWM2R1 = crate::Reg<secwm2r1::SECWM2R1rs>;
///FLASH secure watermark2 register 1
pub mod secwm2r1;
/**SECWM2R2 (rw) register accessor: FLASH secure watermark2 register 2

You can [`read`](crate::Reg::read) this register and get [`secwm2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECWM2R2)

For information about available fields see [`mod@secwm2r2`] module*/
pub type SECWM2R2 = crate::Reg<secwm2r2::SECWM2R2rs>;
///FLASH secure watermark2 register 2
pub mod secwm2r2;
/**WRP2AR (rw) register accessor: FLASH WPR2 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:WRP2AR)

For information about available fields see [`mod@wrp2ar`] module*/
pub type WRP2AR = crate::Reg<wrp2ar::WRP2ARrs>;
///FLASH WPR2 area A address register
pub mod wrp2ar;
/**WRP2BR (rw) register accessor: FLASH WPR2 area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp2br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:WRP2BR)

For information about available fields see [`mod@wrp2br`] module*/
pub type WRP2BR = crate::Reg<wrp2br::WRP2BRrs>;
///FLASH WPR2 area B address register
pub mod wrp2br;
/**OEM1KEYR1 (w) register accessor: FLASH OEM1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem1keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OEM1KEYR1)

For information about available fields see [`mod@oem1keyr1`] module*/
pub type OEM1KEYR1 = crate::Reg<oem1keyr1::OEM1KEYR1rs>;
///FLASH OEM1 key register 1
pub mod oem1keyr1;
/**OEM1KEYR2 (w) register accessor: FLASH OEM1 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem1keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OEM1KEYR2)

For information about available fields see [`mod@oem1keyr2`] module*/
pub type OEM1KEYR2 = crate::Reg<oem1keyr2::OEM1KEYR2rs>;
///FLASH OEM1 key register 2
pub mod oem1keyr2;
/**OEM2KEYR1 (w) register accessor: FLASH OEM2 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem2keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OEM2KEYR1)

For information about available fields see [`mod@oem2keyr1`] module*/
pub type OEM2KEYR1 = crate::Reg<oem2keyr1::OEM2KEYR1rs>;
///FLASH OEM2 key register 1
pub mod oem2keyr1;
/**OEM2KEYR2 (w) register accessor: FLASH OEM2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem2keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:OEM2KEYR2)

For information about available fields see [`mod@oem2keyr2`] module*/
pub type OEM2KEYR2 = crate::Reg<oem2keyr2::OEM2KEYR2rs>;
///FLASH OEM2 key register 2
pub mod oem2keyr2;
/**SEC1BBR1 (rw) register accessor: FLASH secure block based bank 1 register 1

You can [`read`](crate::Reg::read) this register and get [`sec1bbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR1)

For information about available fields see [`mod@sec1bbr1`] module*/
pub type SEC1BBR1 = crate::Reg<sec1bbr1::SEC1BBR1rs>;
///FLASH secure block based bank 1 register 1
pub mod sec1bbr1;
/**SEC1BBR2 (rw) register accessor: FLASH secure block based bank 1 register 2

You can [`read`](crate::Reg::read) this register and get [`sec1bbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR2)

For information about available fields see [`mod@sec1bbr2`] module*/
pub type SEC1BBR2 = crate::Reg<sec1bbr2::SEC1BBR2rs>;
///FLASH secure block based bank 1 register 2
pub mod sec1bbr2;
/**SEC1BBR3 (rw) register accessor: FLASH secure block based bank 1 register 3

You can [`read`](crate::Reg::read) this register and get [`sec1bbr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR3)

For information about available fields see [`mod@sec1bbr3`] module*/
pub type SEC1BBR3 = crate::Reg<sec1bbr3::SEC1BBR3rs>;
///FLASH secure block based bank 1 register 3
pub mod sec1bbr3;
/**SEC1BBR4 (rw) register accessor: FLASH secure block based bank 1 register 4

You can [`read`](crate::Reg::read) this register and get [`sec1bbr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR4)

For information about available fields see [`mod@sec1bbr4`] module*/
pub type SEC1BBR4 = crate::Reg<sec1bbr4::SEC1BBR4rs>;
///FLASH secure block based bank 1 register 4
pub mod sec1bbr4;
/**SEC1BBR5 (rw) register accessor: FLASH secure block based bank 1 register 5

You can [`read`](crate::Reg::read) this register and get [`sec1bbr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR5)

For information about available fields see [`mod@sec1bbr5`] module*/
pub type SEC1BBR5 = crate::Reg<sec1bbr5::SEC1BBR5rs>;
///FLASH secure block based bank 1 register 5
pub mod sec1bbr5;
/**SEC1BBR6 (rw) register accessor: FLASH secure block based bank 1 register 6

You can [`read`](crate::Reg::read) this register and get [`sec1bbr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR6)

For information about available fields see [`mod@sec1bbr6`] module*/
pub type SEC1BBR6 = crate::Reg<sec1bbr6::SEC1BBR6rs>;
///FLASH secure block based bank 1 register 6
pub mod sec1bbr6;
/**SEC1BBR7 (rw) register accessor: FLASH secure block based bank 1 register 7

You can [`read`](crate::Reg::read) this register and get [`sec1bbr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR7)

For information about available fields see [`mod@sec1bbr7`] module*/
pub type SEC1BBR7 = crate::Reg<sec1bbr7::SEC1BBR7rs>;
///FLASH secure block based bank 1 register 7
pub mod sec1bbr7;
/**SEC1BBR8 (rw) register accessor: FLASH secure block based bank 1 register 8

You can [`read`](crate::Reg::read) this register and get [`sec1bbr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1bbr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC1BBR8)

For information about available fields see [`mod@sec1bbr8`] module*/
pub type SEC1BBR8 = crate::Reg<sec1bbr8::SEC1BBR8rs>;
///FLASH secure block based bank 1 register 8
pub mod sec1bbr8;
/**SEC2BBR1 (rw) register accessor: FLASH secure block based bank 2 register 1

You can [`read`](crate::Reg::read) this register and get [`sec2bbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR1)

For information about available fields see [`mod@sec2bbr1`] module*/
pub type SEC2BBR1 = crate::Reg<sec2bbr1::SEC2BBR1rs>;
///FLASH secure block based bank 2 register 1
pub mod sec2bbr1;
/**SEC2BBR2 (rw) register accessor: FLASH secure block based bank 2 register 2

You can [`read`](crate::Reg::read) this register and get [`sec2bbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR2)

For information about available fields see [`mod@sec2bbr2`] module*/
pub type SEC2BBR2 = crate::Reg<sec2bbr2::SEC2BBR2rs>;
///FLASH secure block based bank 2 register 2
pub mod sec2bbr2;
/**SEC2BBR3 (rw) register accessor: FLASH secure block based bank 2 register 3

You can [`read`](crate::Reg::read) this register and get [`sec2bbr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR3)

For information about available fields see [`mod@sec2bbr3`] module*/
pub type SEC2BBR3 = crate::Reg<sec2bbr3::SEC2BBR3rs>;
///FLASH secure block based bank 2 register 3
pub mod sec2bbr3;
/**SEC2BBR4 (rw) register accessor: FLASH secure block based bank 2 register 4

You can [`read`](crate::Reg::read) this register and get [`sec2bbr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR4)

For information about available fields see [`mod@sec2bbr4`] module*/
pub type SEC2BBR4 = crate::Reg<sec2bbr4::SEC2BBR4rs>;
///FLASH secure block based bank 2 register 4
pub mod sec2bbr4;
/**SEC2BBR5 (rw) register accessor: FLASH secure block based bank 2 register 5

You can [`read`](crate::Reg::read) this register and get [`sec2bbr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR5)

For information about available fields see [`mod@sec2bbr5`] module*/
pub type SEC2BBR5 = crate::Reg<sec2bbr5::SEC2BBR5rs>;
///FLASH secure block based bank 2 register 5
pub mod sec2bbr5;
/**SEC2BBR6 (rw) register accessor: FLASH secure block based bank 2 register 6

You can [`read`](crate::Reg::read) this register and get [`sec2bbr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR6)

For information about available fields see [`mod@sec2bbr6`] module*/
pub type SEC2BBR6 = crate::Reg<sec2bbr6::SEC2BBR6rs>;
///FLASH secure block based bank 2 register 6
pub mod sec2bbr6;
/**SEC2BBR7 (rw) register accessor: FLASH secure block based bank 2 register 7

You can [`read`](crate::Reg::read) this register and get [`sec2bbr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR7)

For information about available fields see [`mod@sec2bbr7`] module*/
pub type SEC2BBR7 = crate::Reg<sec2bbr7::SEC2BBR7rs>;
///FLASH secure block based bank 2 register 7
pub mod sec2bbr7;
/**SEC2BBR8 (rw) register accessor: FLASH secure block based bank 2 register 8

You can [`read`](crate::Reg::read) this register and get [`sec2bbr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2bbr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SEC2BBR8)

For information about available fields see [`mod@sec2bbr8`] module*/
pub type SEC2BBR8 = crate::Reg<sec2bbr8::SEC2BBR8rs>;
///FLASH secure block based bank 2 register 8
pub mod sec2bbr8;
/**SECHDPCR (rw) register accessor: FLASH secure HDP control register

You can [`read`](crate::Reg::read) this register and get [`sechdpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sechdpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:SECHDPCR)

For information about available fields see [`mod@sechdpcr`] module*/
pub type SECHDPCR = crate::Reg<sechdpcr::SECHDPCRrs>;
///FLASH secure HDP control register
pub mod sechdpcr;
/**PRIVCFGR (rw) register accessor: FLASH privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///FLASH privilege configuration register
pub mod privcfgr;
/**PRIV1BBR1 (rw) register accessor: FLASH privilege block based bank 1 register 1

You can [`read`](crate::Reg::read) this register and get [`priv1bbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR1)

For information about available fields see [`mod@priv1bbr1`] module*/
pub type PRIV1BBR1 = crate::Reg<priv1bbr1::PRIV1BBR1rs>;
///FLASH privilege block based bank 1 register 1
pub mod priv1bbr1;
/**PRIV1BBR2 (rw) register accessor: FLASH privilege block based bank 1 register 2

You can [`read`](crate::Reg::read) this register and get [`priv1bbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR2)

For information about available fields see [`mod@priv1bbr2`] module*/
pub type PRIV1BBR2 = crate::Reg<priv1bbr2::PRIV1BBR2rs>;
///FLASH privilege block based bank 1 register 2
pub mod priv1bbr2;
/**PRIV1BBR3 (rw) register accessor: FLASH privilege block based bank 1 register 3

You can [`read`](crate::Reg::read) this register and get [`priv1bbr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR3)

For information about available fields see [`mod@priv1bbr3`] module*/
pub type PRIV1BBR3 = crate::Reg<priv1bbr3::PRIV1BBR3rs>;
///FLASH privilege block based bank 1 register 3
pub mod priv1bbr3;
/**PRIV1BBR4 (rw) register accessor: FLASH privilege block based bank 1 register 4

You can [`read`](crate::Reg::read) this register and get [`priv1bbr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR4)

For information about available fields see [`mod@priv1bbr4`] module*/
pub type PRIV1BBR4 = crate::Reg<priv1bbr4::PRIV1BBR4rs>;
///FLASH privilege block based bank 1 register 4
pub mod priv1bbr4;
/**PRIV1BBR5 (rw) register accessor: FLASH privilege block based bank 1 register 5

You can [`read`](crate::Reg::read) this register and get [`priv1bbr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR5)

For information about available fields see [`mod@priv1bbr5`] module*/
pub type PRIV1BBR5 = crate::Reg<priv1bbr5::PRIV1BBR5rs>;
///FLASH privilege block based bank 1 register 5
pub mod priv1bbr5;
/**PRIV1BBR6 (rw) register accessor: FLASH privilege block based bank 1 register 6

You can [`read`](crate::Reg::read) this register and get [`priv1bbr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR6)

For information about available fields see [`mod@priv1bbr6`] module*/
pub type PRIV1BBR6 = crate::Reg<priv1bbr6::PRIV1BBR6rs>;
///FLASH privilege block based bank 1 register 6
pub mod priv1bbr6;
/**PRIV1BBR7 (rw) register accessor: FLASH privilege block based bank 1 register 7

You can [`read`](crate::Reg::read) this register and get [`priv1bbr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR7)

For information about available fields see [`mod@priv1bbr7`] module*/
pub type PRIV1BBR7 = crate::Reg<priv1bbr7::PRIV1BBR7rs>;
///FLASH privilege block based bank 1 register 7
pub mod priv1bbr7;
/**PRIV1BBR8 (rw) register accessor: FLASH privilege block based bank 1 register 8

You can [`read`](crate::Reg::read) this register and get [`priv1bbr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv1bbr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV1BBR8)

For information about available fields see [`mod@priv1bbr8`] module*/
pub type PRIV1BBR8 = crate::Reg<priv1bbr8::PRIV1BBR8rs>;
///FLASH privilege block based bank 1 register 8
pub mod priv1bbr8;
/**PRIV2BBR1 (rw) register accessor: FLASH privilege block based bank 2 register 1

You can [`read`](crate::Reg::read) this register and get [`priv2bbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR1)

For information about available fields see [`mod@priv2bbr1`] module*/
pub type PRIV2BBR1 = crate::Reg<priv2bbr1::PRIV2BBR1rs>;
///FLASH privilege block based bank 2 register 1
pub mod priv2bbr1;
/**PRIV2BBR2 (rw) register accessor: FLASH privilege block based bank 2 register 2

You can [`read`](crate::Reg::read) this register and get [`priv2bbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR2)

For information about available fields see [`mod@priv2bbr2`] module*/
pub type PRIV2BBR2 = crate::Reg<priv2bbr2::PRIV2BBR2rs>;
///FLASH privilege block based bank 2 register 2
pub mod priv2bbr2;
/**PRIV2BBR3 (rw) register accessor: FLASH privilege block based bank 2 register 3

You can [`read`](crate::Reg::read) this register and get [`priv2bbr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR3)

For information about available fields see [`mod@priv2bbr3`] module*/
pub type PRIV2BBR3 = crate::Reg<priv2bbr3::PRIV2BBR3rs>;
///FLASH privilege block based bank 2 register 3
pub mod priv2bbr3;
/**PRIV2BBR4 (rw) register accessor: FLASH privilege block based bank 2 register 4

You can [`read`](crate::Reg::read) this register and get [`priv2bbr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR4)

For information about available fields see [`mod@priv2bbr4`] module*/
pub type PRIV2BBR4 = crate::Reg<priv2bbr4::PRIV2BBR4rs>;
///FLASH privilege block based bank 2 register 4
pub mod priv2bbr4;
/**PRIV2BBR5 (rw) register accessor: FLASH privilege block based bank 2 register 5

You can [`read`](crate::Reg::read) this register and get [`priv2bbr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR5)

For information about available fields see [`mod@priv2bbr5`] module*/
pub type PRIV2BBR5 = crate::Reg<priv2bbr5::PRIV2BBR5rs>;
///FLASH privilege block based bank 2 register 5
pub mod priv2bbr5;
/**PRIV2BBR6 (rw) register accessor: FLASH privilege block based bank 2 register 6

You can [`read`](crate::Reg::read) this register and get [`priv2bbr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR6)

For information about available fields see [`mod@priv2bbr6`] module*/
pub type PRIV2BBR6 = crate::Reg<priv2bbr6::PRIV2BBR6rs>;
///FLASH privilege block based bank 2 register 6
pub mod priv2bbr6;
/**PRIV2BBR7 (rw) register accessor: FLASH privilege block based bank 2 register 7

You can [`read`](crate::Reg::read) this register and get [`priv2bbr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR7)

For information about available fields see [`mod@priv2bbr7`] module*/
pub type PRIV2BBR7 = crate::Reg<priv2bbr7::PRIV2BBR7rs>;
///FLASH privilege block based bank 2 register 7
pub mod priv2bbr7;
/**PRIV2BBR8 (rw) register accessor: FLASH privilege block based bank 2 register 8

You can [`read`](crate::Reg::read) this register and get [`priv2bbr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priv2bbr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:PRIV2BBR8)

For information about available fields see [`mod@priv2bbr8`] module*/
pub type PRIV2BBR8 = crate::Reg<priv2bbr8::PRIV2BBR8rs>;
///FLASH privilege block based bank 2 register 8
pub mod priv2bbr8;
